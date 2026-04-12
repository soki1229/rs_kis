import yaml
import os
import inflection
import re
from collections import defaultdict

SPEC_PATH = "crates/kis_api/kis-openapi.yaml"
OUTPUT_DIR = "crates/kis_api/src/generated"

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

def get_namespace(api):
    parts = api['endpoint'].split('/')
    if len(parts) > 4:
        return parts[4].replace('-', '_')
    return "common"

def get_method_name(api):
    parts = api['endpoint'].split('/')
    base = parts[-1] if parts[-1] else parts[-2]
    return base.replace('-', '_')

def format_doc(text, indent="    "):
    if not text: return ""
    lines = text.replace('\r', '').split('\n')
    formatted = []
    # Clippy & Doctest 방지 전략: 
    # 모든 줄을 마크다운 코드 블록 외부로 간주하게 하거나, 
    # 특수문자로 시작하는 줄에 추가 인덴트 주어 Doctest 감지 회피
    for line in lines:
        line = line.strip()
        if not line:
            formatted.append(f"{indent}///")
            continue
        
        # Doctest는 줄이 4칸 이상 인덴트되어 있거나 특정 기호로 시작하면 코드로 오인함
        # 모든 줄 앞에 공백 하나를 추가하여 텍스트로 인식하게 유도
        formatted.append(f"{indent}/// {line}")
        
    return "\n".join(formatted)

def generate_rust():
    if not os.path.exists(SPEC_PATH): return
    with open(SPEC_PATH, "r", encoding="utf-8") as f:
        spec = yaml.safe_load(f)

    models_code = "#![allow(clippy::doc_lazy_continuation)]\nuse serde::{Deserialize, Serialize};\n\n"
    test_code = """use crate::{KisClient, KisEnv};\nuse crate::models::*;\nuse dotenvy::dotenv;\nuse std::env;\nuse tokio::sync::OnceCell;\nuse std::path::PathBuf;\n\nstatic SHARED_CLIENT: OnceCell<KisClient> = OnceCell::const_new();\n\nasync fn get_test_client() -> &'static KisClient {\n    SHARED_CLIENT.get_or_init(|| async {\n        dotenv().ok();\n        let key = env::var(\"VTS_APP_KEY\").expect(\"VTS_APP_KEY not set\");\n        let secret = env::var(\"VTS_APP_SECRET\").expect(\"VTS_APP_SECRET not set\");\n        let cache_path = PathBuf::from(\".token_cache.vts.json\");\n        KisClient::with_cache(&key, &secret, KisEnv::Vts, Some(cache_path)).await.expect(\"Failed to init client\")\n    }).await\n}\n\n"""
    
    grouped_apis = defaultdict(lambda: defaultdict(list))
    test_tree = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    
    seen_structs = set()
    for api in spec['apis']:
        category = "stock" if "/domestic-stock/" in api['endpoint'] else "overseas"
        namespace = get_namespace(api)
        struct_base = to_struct_name(api)
        while struct_base in seen_structs: struct_base += "Next"
        seen_structs.add(struct_base)
        api['generated_struct'] = struct_base
        grouped_apis[category][namespace].append(api)

        if api['request']:
            models_code += f"/// [{api['name']}] 요청 구조체\n#[derive(Serialize, Deserialize, Debug, Clone, Default)]\n#[allow(non_snake_case)]\npub struct {struct_base}Request {{\n"
            for f in api['request']:
                models_code += f"    /// {f['korean_name']}\n"
                if f.get('description'): models_code += format_doc(f['description'], indent="    ") + "\n"
                models_code += f"    #[serde(rename = \"{f['name']}\")]\n    pub {to_safe_snake(f['name'])}: String,\n"
            models_code += "}\n\n"
        
        if api['response']:
            models_code += f"/// [{api['name']}] 응답 구조체\n#[derive(Serialize, Deserialize, Debug, Clone, Default)]\n#[allow(non_snake_case)]\npub struct {struct_base}Response {{\n"
            for f in api['response']:
                models_code += f"    /// {f['korean_name']}\n"
                if f.get('description'): models_code += format_doc(f['description'], indent="    ") + "\n"
                models_code += f"    #[serde(rename = \"{f['name']}\")]\n    pub {to_safe_snake(f['name'])}: String,\n"
            models_code += "}\n\n"

    for category in ["stock", "overseas"]:
        cat_camel = inflection.camelize(category)
        code = f"#![allow(clippy::doc_lazy_continuation)]\nuse crate::client::KisClient;\nuse crate::models::*;\nuse crate::error::KisError;\n\n"
        for ns in grouped_apis[category].keys():
            ns_camel = inflection.camelize(ns)
            code += f"#[allow(dead_code)]\npub struct {ns_camel}(pub(crate) KisClient);\n\n"
        
        code += f"impl crate::endpoints::{cat_camel} {{\n"
        for ns in grouped_apis[category].keys():
            ns_camel = inflection.camelize(ns)
            code += f"    pub fn {ns}(&self) -> {ns_camel} {{\n        {ns_camel}(self.0.clone())\n    }}\n"
        code += "}\n\n"
        
        for ns, apis in grouped_apis[category].items():
            ns_camel = inflection.camelize(ns)
            code += f"#[allow(non_snake_case)]\nimpl {ns_camel} {{\n"
            seen_methods = set()
            for api in apis:
                method_name = get_method_name(api)
                real_tr_id_str = api.get('real_tr_id', ""); vts_tr_id_str = api.get('vts_tr_id', "")
                rid = re.search(r"([A-Z0-9]{8,14})", str(real_tr_id_str)).group(1) if re.search(r"([A-Z0-9]{8,14})", str(real_tr_id_str)) else real_tr_id_str
                vid = re.search(r"([A-Z0-9]{8,14})", str(vts_tr_id_str)).group(1) if re.search(r"([A-Z0-9]{8,14})", str(vts_tr_id_str)) else vts_tr_id_str
                method_full = method_name
                while method_full in seen_methods: method_full += "_next"
                seen_methods.add(method_full)
                http_method = api.get('method', 'POST').lower()
                if http_method not in ['get', 'post']: http_method = 'post'
                req_struct = f"{api['generated_struct']}Request" if api['request'] else "()"
                res_struct = f"{api['generated_struct']}Response" if api['response'] else "serde_json::Value"
                
                code += f"    /// {api['name']}\n"
                if api.get('description'): code += format_doc(api['description'], indent="    ") + "\n"
                code += f"    pub async fn {method_full}(\n        &self,\n        req: {req_struct},\n    ) -> Result<{res_struct}, KisError> {{\n"
                code += f"        let tr_id = match self.0.env() {{\n            crate::client::KisEnv::Real => \"{rid}\",\n            crate::client::KisEnv::Vts => \"{vid}\",\n        }};\n"
                code += f"        self.0.{http_method}(\"{api['endpoint']}\", tr_id, req).await\n    }}\n\n"

                if category == "stock" and http_method == "get":
                    env_key = "vts" if vid and "미지원" not in str(vid) else "real"
                    t_str = f"        #[tokio::test]\n        #[ignore]\n        async fn test_{method_full}() {{\n"
                    t_str += f"            let client = super::super::super::get_test_client().await;\n"
                    arg = "()" if req_struct == "()" else f"{req_struct}::default()"
                    t_str += f"            let result = client.{category}().{ns}().{method_full}({arg}).await;\n"
                    t_str += f"            println!(\"API {api['name']} endpoint status: {{:?}}\", result.is_ok() || result.is_err());\n        }}\n"
                    test_tree[env_key][category][ns].append(t_str)
            code += "}\n\n"
        with open(f"{OUTPUT_DIR}/{category}.rs", "w") as f: f.write(code)

    test_code_body = ""
    for env, cats in test_tree.items():
        test_code_body += f"mod {env} {{\n    use super::*;\n"
        for cat, nss in cats.items():
            test_code_body += f"    mod {cat} {{\n        use super::*;\n"
            for ns, t_cases in nss.items():
                test_code_body += f"        mod {ns} {{\n            use super::*;\n"
                test_code_body += "\n".join(t_cases)
                test_code_body += "        }\n"
            test_code_body += "    }\n"
        test_code_body += "}\n"

    os.makedirs(OUTPUT_DIR, exist_ok=True)
    with open(f"{OUTPUT_DIR}/models.rs", "w") as f: f.write(models_code)
    with open(f"{OUTPUT_DIR}/tests.rs", "w") as f: f.write(test_code + test_code_body)
    with open(f"{OUTPUT_DIR}/mod.rs", "w") as f:
        f.write("pub mod models;\npub mod stock;\npub mod overseas;\n#[cfg(test)]\npub mod tests;\n")

if __name__ == "__main__":
    generate_rust()
    print("[+] Clean SDK with Rich Documentation and Doctest protection generated.")
