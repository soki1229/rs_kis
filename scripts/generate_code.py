import json
import os
import re
import yaml
import inflection
import subprocess

# --- Constants ---
RAW_DATA_FILE = "crates/kis_api/kis-raw-data.json"
OUTPUT_DIR = "crates/kis_api/src/generated"

# --- Utility Functions ---

def to_standard_camel(text):
    if not text: return ""
    # Standardize delimiters
    text = text.replace('-', ' ').replace('_', ' ').replace('.', ' ')
    # Camelize each part
    parts = text.split()
    return "".join([p.capitalize() for p in parts])

def to_safe_snake(name):
    if not name: return "field"
    # Convert to snake_case but keep standard characters
    # First, replace common problematic characters with underscore
    name = name.replace('-', '_').replace('.', '_').replace(' ', '_').replace('~', '_').replace('…', '_')
    # Remove any other non-alphanumeric/non-underscore characters
    name = re.sub(r'[^a-zA-Z0-9_]', '', name)

    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    name = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()
    # Cleanup consecutive underscores
    name = re.sub(r'_+', '_', name).strip('_')
    # Reserved keywords
    if name in ['type', 'continue', 'break', 'match', 'as', 'loop', 'move', 'struct', 'enum', 'pub', 'mod', 'use', 'impl', 'trait', 'fn', 'let', 'mut', 'ref', 'dyn', 'where', 'for', 'crate', 'self', 'super']:
        return f"r#{name}"
    if not name: return "field"
    if name[0].isdigit():
        return f"v_{name}"
    return name

def to_struct_name(api):
    endpoint = api.get('accessUrl', '')
    parts = [p for p in endpoint.strip('/').split('/') if p != "uapi"]
    # Join with spaces to use standard camelizer
    return to_standard_camel(" ".join(parts))

def format_doc(text, indent=""):
    if not text: return ""
    text = str(text).replace('\t', '    ')
    lines = text.strip().split('\n')
    return "\n".join([f"{indent}/// {line}" for line in lines])

def _extract_params(api):
    # Authoritative source: portal apiPropertys with bodyType='req_b' only.
    # reqExample and children are examples/metadata, not formal spec — excluded to prevent contamination.
    JUNK_FIELDS = {
        'headerMap', 'methodList', 'contentTypeList', 'pathList',
        'queryMap', 'formMap', 'jsonBody', 'jsonResponse', 'address',
        'enabled', 'byAppKey', 'refreshTimeUnit', 'requestLimit',
    }
    params = []
    seen_rust_names = set()
    props = api.get('apiPropertys', [])
    for p in props:
        if p.get('bodyType') == 'req_b':
            name = p.get('propertyCd')
            if name and name not in JUNK_FIELDS:
                rust_name = to_safe_snake(name)
                if rust_name not in seen_rust_names:
                    params.append({
                        'name': name,
                        'korean_name': p.get('propertyNm', name),
                        'type': 'String',
                        'required': p.get('requireYn', 'N'),
                        'description': p.get('description')
                    })
                    seen_rust_names.add(rust_name)
    return params


def _extract_response_groups(api):
    """Parse res_b fields into output groups for response struct generation.

    Returns (groups, output_types) where:
      groups      = {'root': [...fields], 'output': [...fields], 'output1': [...], ...}
      output_types = {'output': 'option'|'vec', 'output1': 'vec', ...}

    Heuristic for array vs. object:
      - Numbered outputs (output1, output2, ...) → Vec<T>
      - Plain 'output' with pagination fields in root  → Vec<T>
      - Plain 'output' without pagination fields       → Option<T>
      - Endpoints in FORCE_VEC_OUTPUT → output always Vec<T>
    """
    OUTPUT_RE = re.compile(r'^output\d*$', re.IGNORECASE)
    PAGINATION = {'ctx_area_fk100', 'ctx_area_nk100', 'ctx_area_fk200', 'ctx_area_nk200',
                  'CTX_AREA_FK100', 'CTX_AREA_NK100', 'CTX_AREA_FK200', 'CTX_AREA_NK200'}
    ROOT_ALWAYS = {'rt_cd', 'msg_cd', 'msg1'}
    FORCE_VEC_OUTPUT = {
        '/uapi/domestic-stock/v1/trading/inquire-psbl-rvsecncl',
        '/uapi/domestic-stock/v1/quotations/chk-holiday',
        '/uapi/domestic-stock/v1/quotations/volume-rank',
        '/uapi/overseas-stock/v1/quotations/countries-holiday',
    }

    endpoint = api.get('accessUrl', '')
    force_vec = endpoint in FORCE_VEC_OUTPUT

    props = api.get('apiPropertys', [])
    res_b = [p for p in props if p.get('bodyType') == 'res_b' and p.get('propertyCd')]

    groups = {'root': []}
    current = 'root'
    has_pagination = False
    seen_rust = {'root': set()}

    for p in res_b:
        name = p.get('propertyCd', '')
        if not name:
            continue
        if OUTPUT_RE.match(name):
            current = name.lower()  # normalize: 'Output' → 'output'
            if current not in groups:
                groups[current] = []
                seen_rust[current] = set()
        elif name in ROOT_ALWAYS or name in PAGINATION:
            if name in PAGINATION:
                has_pagination = True
            rn = to_safe_snake(name)
            if rn not in seen_rust['root']:
                groups['root'].append(p)
                seen_rust['root'].add(rn)
        else:
            if current not in groups:
                groups[current] = []
                seen_rust[current] = set()
            rn = to_safe_snake(name)
            if rn not in seen_rust[current]:
                groups[current].append(p)
                seen_rust[current].add(rn)

    output_types = {}
    for key in groups:
        if key == 'root':
            continue
        numbered = bool(re.search(r'\d', key))
        if numbered or has_pagination or force_vec:
            output_types[key] = 'vec'
        else:
            output_types[key] = 'option'

    return groups, output_types


def parse_compound_tr_id(tr_id_str):
    """Parse a compound TR_ID string into a list of (label, code) tuples.

    Examples
    --------
    "(매도) TTTC0011U (매수) TTTC0012U"
        → [("sell", "TTTC0011U"), ("buy", "TTTC0012U")]
    "(3개월이내) TTTC0081R (3개월이전) CTSC9215R"
        → [("recent", "TTTC0081R"), ("old", "CTSC9215R")]
    "(미국매수) TTTT1002U (미국매도) TTTT1006U ..."
        → [("buy", "TTTT1002U"), ("sell", "TTTT1006U")]
    "TTTC0013U"
        → [("", "TTTC0013U")]
    "모의투자 미지원" / "미지원"
        → "모의투자 미지원"  (sentinel string)
    None / ""
        → ""
    """
    NOT_SUPPORTED = "모의투자 미지원"

    if not tr_id_str:
        return ""
    s = tr_id_str.strip()
    if s in ("모의투자 미지원", "미지원", "None", "없음"):
        return NOT_SUPPORTED

    # Label keyword → English suffix used in method names
    LABEL_MAP = {
        "매수": "buy", "매도": "sell",
        "미국매수": "buy", "미국매도": "sell",
        "주간매수": "buy", "주간매도": "sell",
        "3개월이내": "recent", "3개월이전": "old",
        "예약취소": "cancel_resv", "예약정정": "mod_resv",
        "정정": "modify", "취소": "cancel",
        "주간": "day", "야간": "night",
        "구": "legacy", "신": "new",
        "구버전": "legacy", "신버전": "new",
        # 해외주식 지역 구분
        "미국": "us",
        "미국예약매수": "us_buy_resv",
        "미국예약매도": "us_sell_resv",
        "일본/중국/홍콩/베트남": "asia",
        "중국/홍콩/일본/베트남 예약주문": "asia_resv",
        "미국 예약주문 취소접수": "us_cancel_resv",
        "미국 정정·취소": "us_modify_cancel",
        # 선물옵션 주간/야간 매수매도 구분
        "주간 매수/매도": "day_order",
        "주간 정정/취소": "day_modify",
        "야간 매수/매도": "night_order",
        "야간 정정/취소": "night_modify",
        # 아시아 국가 (정보 없음 처리)
        "아시아국가 미제공": "asia_not_supported",
        "아시아 국가 하단 규격서 참고": "asia_see_spec",
    }

    # Find (label) CODE pairs
    pattern = r'\(([^)]+)\)\s*([A-Z][A-Z0-9]{5,})'
    matches = re.findall(pattern, s)
    if matches:
        result = []
        for label, code in matches:
            label = label.strip()
            suffix = LABEL_MAP.get(label, to_safe_snake(label))
            result.append((suffix, code))
        return result

    # Single TR_ID (possibly with trailing noise)
    codes = re.findall(r'[A-Z][A-Z0-9]{5,}', s)
    if codes:
        return [("", codes[0])]

    return ""


class TypeMapper:
    def __init__(self, yaml_path):
        with open(yaml_path, 'r') as f:
            self.mapping = yaml.safe_load(f) or {}
        self.required_imports = set()
        # Pre-compile pattern rules
        self._patterns = []
        for rule in self.mapping.get('patterns', []):
            try:
                self._patterns.append((re.compile(rule['pattern']), rule['type']))
            except re.error:
                pass
        # Field-level overrides: name → type
        self._fields = {f['name']: f['type'] for f in self.mapping.get('fields', [])}

    def get_rust_type(self, field_name):
        # 1. Explicit field override wins
        if field_name in self._fields:
            rtype = self._fields[field_name]
        else:
            # 2. Pattern matching (first match wins)
            rtype = 'String'
            for compiled, typ in self._patterns:
                if compiled.match(field_name):
                    rtype = typ
                    break
        if rtype == "Decimal":
            self.required_imports.add("rust_decimal::Decimal")
        return rtype


class CodeGenerator:
    def __init__(self):
        with open(RAW_DATA_FILE, 'r') as f:
            raw_data = json.load(f)

        # Load TR_ID overrides from overrides.yaml
        overrides_path = "scripts/overrides.yaml"
        self._tr_id_overrides = {}  # endpoint → {real, vts}
        try:
            with open(overrides_path, 'r') as f:
                ov = yaml.safe_load(f) or {}
            for entry in ov.get('tr_id_overrides', []):
                ep = entry.get('endpoint', '')
                self._tr_id_overrides[ep] = {
                    'real': entry.get('real', ''),
                    'vts': entry.get('vts', ''),
                }
        except FileNotFoundError:
            pass

        self.spec = []
        for api in raw_data:
            ep = api.get('accessUrl', '')
            real_tr = api.get('realTrId', '') or ''
            vts_tr = api.get('virtualTrId', '') or ''

            # Apply TR_ID overrides from overrides.yaml
            if ep in self._tr_id_overrides:
                ov_entry = self._tr_id_overrides[ep]
                if ov_entry.get('real'):
                    real_tr = ov_entry['real']
                if ov_entry.get('vts'):
                    vts_tr = ov_entry['vts']

            resp_groups, resp_output_types = _extract_response_groups(api)
            self.spec.append({
                'name': api.get('name'),
                'accessUrl': ep,
                'realTrId': real_tr,
                'virtualTrId': vts_tr,
                'realDomain': api.get('realDomain', ''),
                'virtualDomain': api.get('virtualDomain', ''),
                'method': api.get('httpMethod', 'POST'),
                'description': api.get('description', ''),
                'request': _extract_params(api),
                'resp_groups': resp_groups,
                'resp_output_types': resp_output_types,
            })
        self.type_mapper = TypeMapper("scripts/type_map.yaml")
        os.makedirs(OUTPUT_DIR, exist_ok=True)

    def generate(self):
        self._write_models()
        self._write_config_module()
        self._write_api_module("stock")
        self._write_api_module("overseas")
        self._write_mod_rs()
        self._run_fmt()

    def _write_models(self):
        output = ["#![allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::doc_markdown)]"]
        for api in self.spec:
            for f in api.get('request', []): self.type_mapper.get_rust_type(f.get('name', ''))
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
            if req_fields:
                output.append(f"/// [{api.get('name', 'Unknown')}] 요청 구조체")
                output.append(format_doc(api.get('description', '')))
                output.append("#[derive(Serialize, Deserialize, Debug, Clone, Default)]")
                output.append("#[allow(non_snake_case)]")
                output.append(f"pub struct {struct_base}Request {{")
                struct_seen_rust_names = set()
                for f in req_fields:
                    fname = f.get('name', 'unknown')
                    rust_name = to_safe_snake(fname)
                    if rust_name in struct_seen_rust_names: continue
                    struct_seen_rust_names.add(rust_name)
                    rtype = self.type_mapper.get_rust_type(fname)
                    kname = f.get('korean_name', fname).replace('\t', ' ')
                    req_str = '필수' if f.get('required') == 'Y' else '선택'
                    output.append(f"    /// {kname} ({f.get('type', 'String')}, {req_str})")
                    output.append(f'    #[serde(rename = "{fname}")]')
                    output.append(f"    pub {rust_name}: {rtype},")
                output.append("}\n")

        # ── Response structs (res_b) ────────────────────────────────────────
        seen_resp_structs = set()
        for api in self.spec:
            struct_base = api.get('generated_struct', '')
            if not struct_base or struct_base in seen_resp_structs:
                continue
            seen_resp_structs.add(struct_base)

            groups = api.get('resp_groups', {'root': []})
            output_types = api.get('resp_output_types', {})

            # Inner structs for each output group
            for group_key, fields in groups.items():
                if group_key == 'root' or not fields:
                    continue
                suffix = to_standard_camel(group_key)  # Output / Output1 / Output2
                inner_name = f"{struct_base}{suffix}Item"
                output.append(f"/// [{api.get('name', '')}] {group_key} 항목")
                output.append("#[derive(Serialize, Deserialize, Debug, Clone, Default)]")
                output.append("#[allow(non_snake_case)]")
                output.append(f"pub struct {inner_name} {{")
                for p in fields:
                    fname = p.get('propertyCd', '')
                    if not fname:
                        continue
                    rust_name = to_safe_snake(fname)
                    kname = (p.get('propertyNm') or fname).replace('\t', ' ')
                    rtype = self.type_mapper.get_rust_type(fname)
                    output.append(f"    /// {kname}")
                    output.append(f'    #[serde(default, rename = "{fname}")]')
                    output.append(f"    pub {rust_name}: {rtype},")
                output.append("}\n")

            # Top-level Response struct
            resp_name = f"{struct_base}Response"
            output.append(f"/// [{api.get('name', '')}] 응답 구조체")
            output.append("#[derive(Serialize, Deserialize, Debug, Clone, Default)]")
            output.append(f"pub struct {resp_name} {{")
            for p in groups.get('root', []):
                fname = p.get('propertyCd', '')
                if not fname:
                    continue
                rust_name = to_safe_snake(fname)
                kname = (p.get('propertyNm') or fname).replace('\t', ' ')
                output.append(f"    /// {kname}")
                output.append(f'    #[serde(default, rename = "{fname}")]')
                output.append(f"    pub {rust_name}: String,")
            for group_key in groups:
                if group_key == 'root' or not groups[group_key]:
                    continue
                suffix = to_standard_camel(group_key)
                inner_name = f"{struct_base}{suffix}Item"
                otype = output_types.get(group_key, 'option')
                if otype == 'vec':
                    output.append(f'    #[serde(default)]')
                    output.append(f"    pub {group_key}: Vec<{inner_name}>,")
                else:
                    output.append(f'    #[serde(default)]')
                    output.append(f"    pub {group_key}: Option<{inner_name}>,")
            output.append("}\n")

        with open(os.path.join(OUTPUT_DIR, "models.rs"), "w") as f:
            f.write("\n".join(output))

    def _write_config_module(self):
        with open(RAW_DATA_FILE, 'r') as f:
            raw_data = json.load(f)
        output = ["// This file is generated from kis-openapi.yaml. Do not edit manually.", ""]
        # Use .strip() and trust metadata for protocol
        real_ws = next((api.get('realDomain', '').strip() for api in raw_data if api.get('apiType') == 'WEBSOCKET' and '21000' in api.get('realDomain', '')), "")
        vts_ws = next((api.get('virtualDomain', '').strip() for api in raw_data if api.get('apiType') == 'WEBSOCKET' and '31000' in api.get('virtualDomain', '')), "")
        output.append(f'pub const REAL_WS_URL: &str = "{real_ws}";')
        output.append(f'pub const VTS_WS_URL: &str = "{vts_ws}";')
        with open(os.path.join(OUTPUT_DIR, "config.rs"), "w") as f:
            f.write("\n".join(output))

    def _emit_method(self, output, api, method_name, real_tr, vts_tr, req_struct, method_call, endpoint):
        """Emit a single async fn into `output`."""
        NOT_SUPPORTED = "모의투자 미지원"
        vts_tr_safe = vts_tr if vts_tr else NOT_SUPPORTED
        real_domain = api.get('realDomain', '')
        vts_domain = api.get('virtualDomain', '')
        resp_struct = f"{api['generated_struct']}Response"

        output.append(format_doc(api.get('name', 'Unknown'), "    "))
        output.append(f"    /// - TR_ID: Real={real_tr} / VTS={vts_tr_safe}")
        output.append(f"    /// - Endpoint: {endpoint}")
        output.append(f"    pub async fn {method_name}(&self, req: {req_struct}) -> Result<{resp_struct}, KisError> {{")
        output.append("        let (tr_id, base_url) = match self.0.env() {")
        output.append(f'            crate::client::KisEnv::Real => ("{real_tr}", "{real_domain}"),')
        output.append(f'            crate::client::KisEnv::Vts => ("{vts_tr_safe}", "{vts_domain}"),')
        output.append("        };")
        output.append(f'        self.0.{method_call}("{endpoint}", tr_id, base_url, req).await')
        output.append("    }\n")

    def _write_api_module(self, module_name):
        output = [
            "#![allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::doc_markdown)]",
            "use crate::client::KisClient;",
            "use crate::error::KisError;",
            "use crate::models::*;",
            ""
        ]
        filtered_apis = []
        for api in self.spec:
            ep = api.get('accessUrl', '')
            if module_name == "stock" and ("domestic-stock" in ep or "domestic-futureoption" in ep):
                filtered_apis.append(api)
            elif module_name == "overseas" and ("overseas-stock" in ep or "overseas-price" in ep or "oauth2" in ep or "/uapi/hashkey" in ep):
                filtered_apis.append(api)
        groups = {}
        for api in filtered_apis:
            parts = api.get('accessUrl', '').strip('/').split('/')
            group_name = "Common"
            for p in parts:
                if p in ["quotations", "trading", "ranking", "order", "account"]:
                    group_name = to_standard_camel(p)
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
            method_name = to_safe_snake(group)
            output.append(f"    pub fn {method_name}(&self) -> {struct_name} {{ {struct_name}(self.0.clone()) }}")
        output.append("}\n")

        for group, apis in groups.items():
            struct_name = f"{module_prefix}{group}"
            output.append("#[allow(non_snake_case)]")
            output.append(f"impl {struct_name} {{")
            for api in apis:
                endpoint = api.get('accessUrl', '')
                parts = [p for p in endpoint.strip('/').split('/') if p != "uapi"]
                base_method_name = to_safe_snake("_".join(parts))
                if base_method_name.startswith("r#"): base_method_name = base_method_name[2:]
                req_struct = f"{api['generated_struct']}Request" if api.get('request') else "()"
                method_call = "post" if api.get('method', 'POST') == "POST" else "get"

                real_variants = parse_compound_tr_id(api.get('realTrId', '') or '')
                vts_variants  = parse_compound_tr_id(api.get('virtualTrId', '') or '')

                NOT_SUPPORTED = "모의투자 미지원"

                # Both simple (single TR_ID each)
                if isinstance(real_variants, list) and len(real_variants) == 1 and real_variants[0][0] == "":
                    real_tr = real_variants[0][1]
                    vts_tr  = vts_variants[0][1] if isinstance(vts_variants, list) and vts_variants else (vts_variants if isinstance(vts_variants, str) else NOT_SUPPORTED)
                    self._emit_method(output, api, base_method_name, real_tr, vts_tr, req_struct, method_call, endpoint)

                # Compound on real side → emit one method per variant
                elif isinstance(real_variants, list) and len(real_variants) > 1:
                    # Build vts lookup by suffix
                    vts_by_suffix = {}
                    if isinstance(vts_variants, list):
                        for suffix, code in vts_variants:
                            vts_by_suffix[suffix] = code
                    elif isinstance(vts_variants, str):
                        # Single sentinel or simple code — apply to all variants
                        for suffix, _ in real_variants:
                            vts_by_suffix[suffix] = vts_variants

                    for suffix, real_tr in real_variants:
                        variant_method = f"{base_method_name}_{suffix}" if suffix else base_method_name
                        vts_tr = vts_by_suffix.get(suffix, NOT_SUPPORTED)
                        self._emit_method(output, api, variant_method, real_tr, vts_tr, req_struct, method_call, endpoint)

                else:
                    # Fallback: emit as-is (e.g., real is not-supported string)
                    real_tr = real_variants if isinstance(real_variants, str) else NOT_SUPPORTED
                    vts_tr  = vts_variants  if isinstance(vts_variants,  str) else NOT_SUPPORTED
                    self._emit_method(output, api, base_method_name, real_tr, vts_tr, req_struct, method_call, endpoint)

            output.append("}\n")
        with open(os.path.join(OUTPUT_DIR, f"{module_name}.rs"), "w") as f:
            f.write("\n".join(output))

    def _write_mod_rs(self):
        with open(os.path.join(OUTPUT_DIR, "mod.rs"), "w") as f:
            f.write("pub mod config;\npub mod models;\npub mod overseas;\npub mod stock;\n")

    def _run_fmt(self):
        try:
            subprocess.run(["cargo", "fmt"], check=True)
            print("[+] Auto-formatted generated code.")
        except:
            print("[!] Failed to run cargo fmt.")

if __name__ == "__main__":
    generator = CodeGenerator()
    generator.generate()
    print("[+] Structured SDK generated with Lint fixes.")
