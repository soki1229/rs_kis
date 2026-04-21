import json
import os
import re
import yaml
import inflection

# --- Constants ---
RAW_DATA_FILE = "crates/kis_api/kis-raw-data.json"
OUTPUT_DIR = "crates/kis_api/src/generated"

# --- Utility Functions ---

def to_struct_name(api):
    # 경로 전체를 활용하여 고유한 이름 생성
    # /uapi/overseas-stock/v1/trading/order -> OverseasStockV1TradingOrder
    endpoint = api.get('accessUrl', '')
    parts = [p for p in endpoint.strip('/').split('/') if p != "uapi"]
    
    # rs_kis_server 기대치에 정확히 맞춘 접두사 결정
    if "domestic-stock" in endpoint:
        prefix = "DomesticStock"
    elif "overseas-stock" in endpoint:
        prefix = "OverseasStock"
    elif "domestic-futureoption" in endpoint:
        prefix = "DomesticFutureoption"
    elif "overseas-price" in endpoint:
        prefix = "OverseasPrice"
    else:
        prefix = "Auth"
        
    useful_parts = [p for p in parts if p not in ["domestic-stock", "overseas-stock", "domestic-futureoption", "overseas-price"]]
    name_parts = [prefix] + [inflection.camelize(p.replace('-', '_')) for p in useful_parts]
    name = "".join(name_parts)
    
    if len(name) < 10:
        clean_name = re.sub(r'\(.*?\)', '', api.get('name', 'Unknown'))
        clean_name = re.sub(r'[^\w\s]', '', clean_name)
        name += inflection.camelize(inflection.underscore(clean_name.strip().replace(' ', '_')))
        
    return name

def to_safe_snake(text):
    snake = inflection.underscore(text.strip())
    snake = re.sub(r'[^\w\s]', '', snake).replace(' ', '_')
    snake = re.sub(r'_+', '_', snake)
    if snake in ["type", "mod", "struct", "enum", "fn", "use", "loop", "match"]:
        snake = f"r#{snake}"
    return snake

def format_doc(text, indent=""):
    if not text: return ""
    lines = str(text).strip().split('\n')
    return "\n".join([f"{indent}/// {line}" for line in lines])

# --- Type Mapper ---

class TypeMapper:
    def __init__(self, config_path):
        with open(config_path, 'r') as f:
            self.config = yaml.safe_load(f)
        
        self.patterns = []
        for p in self.config.get('patterns', []):
            self.patterns.append((re.compile(p['pattern']), p['type'], p.get('import')))
            
        self.explicit = {str(f['name']).lower(): (f['type'], f.get('import')) 
                         for f in self.config.get('fields', [])}

        self.required_imports = set()

    def get_rust_type(self, field_name):
        field_name_lower = str(field_name).lower()
        if field_name_lower in self.explicit:
            rtype, imp = self.explicit[field_name_lower]
            if imp: self.required_imports.add(imp)
            return rtype
        
        for pattern, rtype, imp in self.patterns:
            if pattern.fullmatch(field_name_lower):
                if imp: self.required_imports.add(imp)
                return rtype
        
        return "String"

# --- Generator ---

class CodeGenerator:
    def __init__(self):
        with open(RAW_DATA_FILE, 'r') as f:
            self.spec = json.load(f)
        self.type_mapper = TypeMapper("scripts/type_map.yaml")
        os.makedirs(OUTPUT_DIR, exist_ok=True)

    def generate(self):
        self._write_models()
        self._write_api_module("stock")
        self._write_api_module("overseas")
        self._write_mod_rs()

    def _write_models(self):
        output = ["#![allow(clippy::doc_lazy_continuation)]"]
        for api in self.spec:
            req_fields = api.get('request', [])
            if isinstance(req_fields, list):
                for f in req_fields: self.type_mapper.get_rust_type(f.get('name', ''))
            res_fields = api.get('response', [])
            if isinstance(res_fields, list):
                for f in res_fields: self.type_mapper.get_rust_type(f.get('name', ''))
        
        for imp in sorted(list(self.type_mapper.required_imports)):
            output.append(f"use {imp};")
        output.append("use serde::{Deserialize, Serialize};")
        output.append("")

        seen_structs = set()
        for api in self.spec:
            struct_base = to_struct_name(api)
            original_base = struct_base
            counter = 2
            while struct_base in seen_structs:
                struct_base = f"{original_base}V{counter}"
                counter += 1
            seen_structs.add(struct_base)
            api['generated_struct'] = struct_base

            req_fields = api.get('request', [])
            if isinstance(req_fields, list) and len(req_fields) > 0:
                output.append(f"/// [{api.get('name', 'Unknown')}] 요청 구조체")
                output.append(format_doc(api.get('description', '')))
                output.append("#[derive(Serialize, Deserialize, Debug, Clone, Default)]")
                output.append("#[allow(non_snake_case)]")
                output.append(f"pub struct {struct_base}Request {{")
                for f in req_fields:
                    fname = f.get('name', 'unknown')
                    rtype = self.type_mapper.get_rust_type(fname)
                    output.append(f"    /// {f.get('korean_name', fname)} ({f.get('type', 'String')}, {'필수' if f.get('required') == 'Y' else '선택'})")
                    if f.get('description'):
                        output.append(format_doc(f['description'], "    "))
                    output.append(f'    #[serde(rename = "{fname}")]')
                    output.append(f"    pub {to_safe_snake(fname)}: {rtype},")
                output.append("}\n")

        with open(os.path.join(OUTPUT_DIR, "models.rs"), "w") as f:
            f.write("\n".join(output))

    def _write_api_module(self, module_name):
        output = [
            "#![allow(clippy::doc_lazy_continuation)]",
            "use crate::client::KisClient;",
            "use crate::error::KisError;",
            "use crate::models::*;",
            ""
        ]

        filtered_apis = []
        for api in self.spec:
            ep = api.get('accessUrl', '')
            if module_name == "stock":
                if "domestic-stock" in ep or "domestic-futureoption" in ep:
                    filtered_apis.append(api)
            elif module_name == "overseas":
                if "overseas-stock" in ep or "overseas-price" in ep or "oauth2" in ep or "/uapi/hashkey" in ep:
                    filtered_apis.append(api)
        
        groups = {}
        for api in filtered_apis:
            parts = api.get('accessUrl', '').strip('/').split('/')
            group_name = "Common"
            for p in parts:
                if p in ["quotations", "trading", "ranking", "order", "account"]:
                    group_name = inflection.camelize(p)
                    break
            if group_name not in groups: groups[group_name] = []
            groups[group_name].append(api)

        module_prefix = "Stock" if module_name == "stock" else "Overseas"
        
        for group in groups:
            struct_name = f"{module_prefix}{group}"
            output.append(f"#[allow(dead_code)]\npub struct {struct_name}(pub(crate) KisClient);\n")

        target_endpoint_type = f"crate::endpoints::{module_prefix}"
        output.append(f"impl {target_endpoint_type} {{")
        for group in groups:
            struct_name = f"{module_prefix}{group}"
            method_name = inflection.underscore(group)
            output.append(f"    pub fn {method_name}(&self) -> {struct_name} {{ {struct_name}(self.0.clone()) }}")
        output.append("}\n")

        for group, apis in groups.items():
            struct_name = f"{module_prefix}{group}"
            output.append("#[allow(non_snake_case)]")
            output.append(f"impl {struct_name} {{")
            for api in apis:
                endpoint = api.get('accessUrl', '')
                # 중복 메서드 방지를 위해 전체 경로를 메서드 명으로 사용 (uapi 제외)
                parts = [p for p in endpoint.strip('/').split('/') if p != "uapi"]
                method_name = to_safe_snake("_".join(parts))
                if method_name.startswith("r#"): method_name = method_name[2:]
                
                req_fields = api.get('request', [])
                req_struct = f"{api['generated_struct']}Request" if isinstance(req_fields, list) and len(req_fields) > 0 else "()"
                
                output.append(format_doc(api.get('name', 'Unknown'), "    "))
                output.append(f"    /// - TR_ID: Real={api.get('realTrId', '')} / VTS={api.get('virtualTrId', '')}")
                output.append(f"    /// - Endpoint: {endpoint}")
                if api.get('description'): output.append(format_doc(api['description'], "    "))
                
                output.append(f"    pub async fn {method_name}(&self, req: {req_struct}) -> Result<serde_json::Value, KisError> {{")
                output.append("        let tr_id = match self.0.env() {")
                output.append(f'            crate::client::KisEnv::Real => "{api.get("realTrId", "")}",')
                output.append(f'            crate::client::KisEnv::Vts => "{api.get("virtualTrId", "")}",')
                output.append("        };")
                method_call = "post" if api.get('method', 'POST') == "POST" else "get"
                output.append(f'        self.0.{method_call}("{endpoint}", tr_id, req).await')
                output.append("    }\n")
            output.append("}\n")

        with open(os.path.join(OUTPUT_DIR, f"{module_name}.rs"), "w") as f:
            f.write("\n".join(output))

    def _write_mod_rs(self):
        with open(os.path.join(OUTPUT_DIR, "mod.rs"), "w") as f:
            f.write("pub mod models;\npub mod stock;\npub mod overseas;\npub mod config;\npub mod tests;\n")

if __name__ == "__main__":
    generator = CodeGenerator()
    generator.generate()
    print("[+] Structured SDK generated with Smart Type Mapping and Robust Unique Methods.")
