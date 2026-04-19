import yaml
import os
import inflection
import re
import textwrap
from collections import defaultdict

# --- Constants & Paths ---
SPEC_PATH = "crates/kis_api/kis-openapi.yaml"
TYPE_MAP_PATH = "scripts/type_map.yaml"
OVERRIDES_PATH = "scripts/overrides.yaml"
OUTPUT_DIR = "crates/kis_api/src/generated"

# --- Utility Functions ---

def to_struct_name(api):
    parts = api['endpoint'].split('/')
    base = parts[-1] if parts[-1] else parts[-2]
    name = inflection.camelize(inflection.underscore(base.replace('-', '_')))
    if len(name) < 3:
        clean_name = re.sub(r'\(.*?\)', '', api['name'])
        clean_name = re.sub(r'[^\w\s]', '', clean_name)
        name = inflection.camelize(inflection.underscore(clean_name.strip().replace(' ', '_')))
    return name

def to_safe_snake(text):
    text = re.sub(r'[^\w\s]', '', text)
    snake = inflection.underscore(text.strip().replace(' ', '_'))
    snake = re.sub(r'_+', '_', snake)
    if snake in ["type", "mod", "struct", "enum", "fn", "use"]:
        snake = f"r#{snake}"
    return snake

def format_doc(text, indent=""):
    if not text: return ""
    lines = text.replace('\r', '').split('\n')
    formatted = []
    for line in lines:
        line = line.strip()
        if not line:
            formatted.append(f"{indent}///")
            continue
        # Avoid Doctest by ensuring it doesn't look like code
        formatted.append(f"{indent}/// {line}")
    return "\n".join(formatted)

# --- Metadata Classes ---

class TypeMapper:
    def __init__(self, config_path):
        with open(config_path, "r", encoding="utf-8") as f:
            self.config = yaml.safe_load(f)
        self.patterns = [(re.compile(p['pattern'], re.I), p['type'], p.get('import')) 
                         for p in self.config.get('patterns', [])]
        self.explicit = {f['name']: (f['type'], f.get('import')) 
                         for f in self.config.get('fields', [])}
        self.required_imports = set()

    def get_rust_type(self, field_name):
        # 1. Check explicit overrides
        if field_name in self.explicit:
            rtype, imp = self.explicit[field_name]
            if imp: self.required_imports.add(imp)
            return rtype
        
        # 2. Check pattern matching
        for pattern, rtype, imp in self.patterns:
            if pattern.match(field_name):
                if imp: self.required_imports.add(imp)
                return rtype
        
        # 3. Default
        return "String"

class Overrider:
    def __init__(self, config_path):
        if os.path.exists(config_path):
            with open(config_path, "r", encoding="utf-8") as f:
                self.config = yaml.safe_load(f)
        else:
            self.config = {}
        
        self.api_patches = {a['endpoint']: a['patch'] 
                           for a in self.config.get('apis', []) if 'patch' in a}
        self.global_replaces = self.config.get('replacements', [])

    def apply(self, api):
        endpoint = api['endpoint']
        if endpoint in self.api_patches:
            api.update(self.api_patches[endpoint])
        
        # Apply global text replacements in descriptions
        if api.get('description'):
            for r in self.global_replaces:
                api['description'] = api['description'].replace(r['search'], r['replace'])
        return api

# --- Generator Engine ---

class CodeGenerator:
    def __init__(self):
        self.type_mapper = TypeMapper(TYPE_MAP_PATH)
        self.overrider = Overrider(OVERRIDES_PATH)
        self.models_content = ""
        self.grouped_apis = defaultdict(lambda: defaultdict(list))
        self.test_tree = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))

    def generate(self):
        if not os.path.exists(SPEC_PATH): return
        with open(SPEC_PATH, "r", encoding="utf-8") as f:
            spec = yaml.safe_load(f)

        # 1. Config generation
        self._generate_config(spec.get('config', {}))

        # 2. IR Building & Model Generation
        seen_structs = set()
        for api in spec['apis']:
            api = self.overrider.apply(api)
            category = "stock" if "/domestic-stock/" in api['endpoint'] else "overseas"
            namespace = self._get_namespace(api)
            
            struct_base = to_struct_name(api)
            while struct_base in seen_structs: struct_base += "Next"
            seen_structs.add(struct_base)
            api['generated_struct'] = struct_base
            
            self.grouped_apis[category][namespace].append(api)
            self._add_model_structs(api)

        # 3. Write Models
        self._write_models()

        # 4. Write API Modules (stock.rs, overseas.rs)
        for category in ["stock", "overseas"]:
            self._write_api_module(category)

        # 5. Write mod.rs & tests.rs
        self._write_metadata()

    def _get_namespace(self, api):
        parts = api['endpoint'].split('/')
        if len(parts) > 4:
            return parts[4].replace('-', '_')
        return "common"

    def _generate_config(self, config_data):
        content = "// This file is generated from kis-openapi.yaml. Do not edit manually.\n\n"
        if not config_data:
            config_data = {
                "real": "ws://ops.koreainvestment.com:21000",
                "vts": "ws://ops.koreainvestment.com:31000"
            }
        for key, value in config_data.items():
            content += f"pub const {key.upper()}_WS_URL: &str = \"{value}\";\n"
        
        os.makedirs(OUTPUT_DIR, exist_ok=True)
        with open(f"{OUTPUT_DIR}/config.rs", "w", encoding="utf-8") as f:
            f.write(content)

    def _add_model_structs(self, api):
        struct_name = api['generated_struct']
        
        # Request Struct
        if api['request']:
            doc = f"/// [{api['name']}] 요청 구조체\n"
            if api.get('description'): doc += format_doc(api['description']) + "\n"
            
            fields = ""
            for f in api['request']:
                rtype = self.type_mapper.get_rust_type(f['name'])
                fields += f"    /// {f['korean_name']} ({f['type']}, {'필수' if f['required'] == 'Y' else '선택'})\n"
                if f.get('description'): fields += format_doc(f['description'], indent="    ") + "\n"
                fields += f"    #[serde(rename = \"{f['name']}\")]\n    pub {to_safe_snake(f['name'])}: {rtype},\n"
            
            self.models_content += f"\n{doc}#[derive(Serialize, Deserialize, Debug, Clone, Default)]\n"
            self.models_content += f"#[allow(non_snake_case)]\n"
            self.models_content += f"pub struct {struct_name}Request {{\n{fields}}}\n"

        # Response Struct
        if api['response']:
            doc = f"/// [{api['name']}] 응답 구조체\n"
            if api.get('description'): doc += format_doc(api['description']) + "\n"
            
            fields = ""
            for f in api['response']:
                rtype = self.type_mapper.get_rust_type(f['name'])
                fields += f"    /// {f['korean_name']} ({f['type']})\n"
                if f.get('description'): fields += format_doc(f['description'], indent="    ") + "\n"
                fields += f"    #[serde(rename = \"{f['name']}\")]\n    pub {to_safe_snake(f['name'])}: {rtype},\n"
            
            self.models_content += f"\n{doc}#[derive(Serialize, Deserialize, Debug, Clone, Default)]\n"
            self.models_content += f"#[allow(non_snake_case)]\n"
            self.models_content += f"pub struct {struct_name}Response {{\n{fields}}}\n"

    def _write_models(self):
        imports = "#![allow(clippy::doc_lazy_continuation)]\nuse serde::{Deserialize, Serialize};\n"
        for imp in sorted(list(self.type_mapper.required_imports)):
            imports += f"use {imp};\n"
        
        with open(f"{OUTPUT_DIR}/models.rs", "w", encoding="utf-8") as f:
            f.write(imports + self.models_content)

    def _write_api_module(self, category):
        cat_camel = inflection.camelize(category)
        code = textwrap.dedent(f"""
            #![allow(clippy::doc_lazy_continuation)]
            use crate::client::KisClient;
            use crate::models::*;
            use crate::error::KisError;

        """)
        
        for ns in self.grouped_apis[category].keys():
            ns_camel = inflection.camelize(ns)
            code += f"#[allow(dead_code)]\npub struct {ns_camel}(pub(crate) KisClient);\n\n"
        
        code += f"impl crate::endpoints::{cat_camel} {{\n"
        for ns in self.grouped_apis[category].keys():
            ns_camel = inflection.camelize(ns)
            code += f"    pub fn {ns}(&self) -> {ns_camel} {{\n        {ns_camel}(self.0.clone())\n    }}\n"
        code += "}\n\n"
        
        for ns, apis in self.grouped_apis[category].items():
            ns_camel = inflection.camelize(ns)
            code += f"#[allow(non_snake_case)]\nimpl {ns_camel} {{\n"
            seen_methods = set()
            for api in apis:
                method_name = api['endpoint'].split('/')[-1].replace('-', '_')
                if not method_name: method_name = api['endpoint'].split('/')[-2].replace('-', '_')
                
                method_full = method_name
                while method_full in seen_methods: method_full += "_next"
                seen_methods.add(method_full)
                
                real_tr_id_str = api.get('real_tr_id', ""); vts_tr_id_str = api.get('vts_tr_id', "")
                rid = re.search(r"([A-Z0-9]{8,14})", str(real_tr_id_str)).group(1) if re.search(r"([A-Z0-9]{8,14})", str(real_tr_id_str)) else real_tr_id_str
                vid = re.search(r"([A-Z0-9]{8,14})", str(vts_tr_id_str)).group(1) if re.search(r"([A-Z0-9]{8,14})", str(vts_tr_id_str)) else vts_tr_id_str
                
                http_method = api.get('method', 'POST').lower()
                if http_method not in ['get', 'post']: http_method = 'post'
                
                req_struct = f"{api['generated_struct']}Request" if api['request'] else "()"
                res_struct = f"{api['generated_struct']}Response" if api['response'] else "serde_json::Value"
                
                # Doc comment for method
                code += f"    /// {api['name']}\n    /// \n"
                code += f"    /// - TR_ID: Real={rid} / VTS={vid}\n"
                code += f"    /// - Endpoint: {api['endpoint']}\n    /// \n"
                if api.get('description'): code += format_doc(api['description'], indent="    ") + "\n"
                if api.get('example'):
                    code += f"    /// \n    /// # Example (Scraped)\n"
                    code += format_doc(api['example'], indent="    ") + "\n"

                method_impl = f"    pub async fn {method_full}(\n"
                method_impl += f"        &self,\n"
                method_impl += f"        req: {req_struct},\n"
                method_impl += f"    ) -> Result<{res_struct}, KisError> {{\n"
                method_impl += f"        let tr_id = match self.0.env() {{\n"
                method_impl += f"            crate::client::KisEnv::Real => \"{rid}\",\n"
                method_impl += f"            crate::client::KisEnv::Vts => \"{vid}\",\n"
                method_impl += f"        }};\n"
                method_impl += f"        self.0.{http_method}(\"{api['endpoint']}\", tr_id, req).await\n"
                method_impl += f"    }}\n"
                
                code += method_impl + "\n"

                # Prepare test data
                if category == "stock" and http_method == "get":
                    env_key = "vts" if vid and "미지원" not in str(vid) else "real"
                    t_str = textwrap.indent(textwrap.dedent(f"""
                        #[tokio::test]
                        #[ignore]
                        async fn test_{method_full}() {{
                            let client = super::super::super::get_test_client().await;
                            let arg = {"()" if req_struct == "()" else f"{req_struct}::default()"};
                            let result = client.{category}().{ns}().{method_full}(arg).await;
                            println!("API {api['name']} status: {{:?}}", result.is_ok() || result.is_err());
                        }}
                    """), "        ")
                    self.test_tree[env_key][category][ns].append(t_str)
                    
            code += "}\n\n"
        
        with open(f"{OUTPUT_DIR}/{category}.rs", "w", encoding="utf-8") as f:
            f.write(code)

    def _write_metadata(self):
        # 1. Tests
        test_content = textwrap.dedent("""
            use crate::{KisClient, KisEnv};
            use crate::models::*;
            use dotenvy::dotenv;
            use std::env;
            use tokio::sync::OnceCell;
            use std::path::PathBuf;

            static SHARED_CLIENT: OnceCell<KisClient> = OnceCell::const_new();

            async fn get_test_client() -> &'static KisClient {
                SHARED_CLIENT.get_or_init(|| async {
                    dotenv().ok();
                    let key = env::var("VTS_APP_KEY").expect("VTS_APP_KEY not set");
                    let secret = env::var("VTS_APP_SECRET").expect("VTS_APP_SECRET not set");
                    let cache_path = PathBuf::from(".token_cache.vts.json");
                    KisClient::with_cache(&key, &secret, KisEnv::Vts, Some(cache_path)).await.expect("Failed to init client")
                }).await
            }

        """)
        for env, cats in self.test_tree.items():
            test_content += f"mod {env} {{\n    use super::*;\n"
            for cat, nss in cats.items():
                test_content += f"    mod {cat} {{\n        use super::*;\n"
                for ns, t_cases in nss.items():
                    test_content += f"        mod {ns} {{\n            use super::*;\n"
                    test_content += "\n".join(t_cases)
                    test_content += "        }\n"
                test_content += "    }\n"
            test_content += "}\n"
        
        with open(f"{OUTPUT_DIR}/tests.rs", "w", encoding="utf-8") as f:
            f.write(test_content)

        # 2. mod.rs
        with open(f"{OUTPUT_DIR}/mod.rs", "w", encoding="utf-8") as f:
            f.write("pub mod config;\npub mod models;\npub mod stock;\npub mod overseas;\n#[cfg(test)]\npub mod tests;\n")

if __name__ == "__main__":
    gen = CodeGenerator()
    gen.generate()
    print("[+] Structured SDK generated with Smart Type Mapping and Overrides.")
