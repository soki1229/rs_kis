import asyncio
import json
import yaml
import re
from playwright.async_api import async_playwright

BASE_URL = "https://apiportal.koreainvestment.com/apiservice-apiservice"
REAL_DOMAIN = "https://openapi.koreainvestment.com:9443"
VTS_DOMAIN = "https://openapivts.koreainvestment.com:29443"
RAW_DATA_FILE = "crates/kis_api/kis-raw-data.json"
OPENAPI_YAML_FILE = "crates/kis_api/kis-openapi.yaml"

WS_CONFIG = {
    "real": "ws://ops.koreainvestment.com:21000",
    "vts": "ws://ops.koreainvestment.com:31000",
}


async def fetch_spec():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context()
        page = await context.new_page()

        # XHR 인터셉션: 포털 백엔드에서 apiPropertys 포함 전체 API 데이터 캡처
        raw_api_by_url: dict[str, dict] = {}

        async def handle_response(response):
            if response.status != 200:
                return
            ct = response.headers.get("content-type", "")
            if "json" not in ct:
                return
            try:
                data = await response.json()
                # 단일 API 상세 응답
                if (
                    isinstance(data, dict)
                    and "accessUrl" in data
                    and "apiPropertys" in data
                ):
                    raw_api_by_url[data["accessUrl"]] = data
                # 목록 응답
                elif (
                    isinstance(data, list)
                    and data
                    and isinstance(data[0], dict)
                    and "accessUrl" in data[0]
                ):
                    for item in data:
                        if "accessUrl" in item:
                            raw_api_by_url[item["accessUrl"]] = item
            except Exception:
                pass

        page.on("response", handle_response)

        print(f"[*] Connecting to KIS Portal: {BASE_URL}")
        await page.goto(BASE_URL, wait_until="networkidle")
        await asyncio.sleep(5)

        links = await page.query_selector_all(
            "#sideBar a[onclick*='goLeftMenuUrl']"
        )
        api_tasks = []
        for link in links:
            onclick = await link.get_attribute("onclick")
            match = re.search(r"goLeftMenuUrl\('(.+?)'\)", onclick)
            if match:
                path = match.group(1)
                name = await link.inner_text()
                api_tasks.append({"name": name.strip(), "path": path})

        print(f"[+] Found {len(api_tasks)} APIs. Scraping details...")

        # HTML 스크래핑 (kis-openapi.yaml 및 XHR 미캡처 시 폴백용)
        yaml_results = []
        for i, api in enumerate(api_tasks):
            print(f"[{i+1}/{len(api_tasks)}] Scraping {api['name']} ({api['path']})")
            try:
                await page.evaluate(f"goLeftMenuUrl('{api['path']}')")
                await page.wait_for_selector("#apiInfoForm", timeout=15000)
                await asyncio.sleep(2)

                detail_data = await page.evaluate(
                    """() => {
                    const getTxt = (sel) => {
                        const el = document.querySelector(sel);
                        return el ? el.innerText.trim() : "";
                    };
                    const parseTable = (id) => {
                        const rows = document.querySelectorAll(`${id} table tbody tr`);
                        return Array.from(rows).map(row => {
                            const cols = row.querySelectorAll('td');
                            if (cols.length < 3) return null;
                            return {
                                name: cols[0].innerText.trim(),
                                korean_name: cols[1].innerText.trim(),
                                type: cols[2].innerText.trim(),
                                required: cols[3] ? cols[3].innerText.trim() : "N",
                                description: cols[5] ? cols[5].innerText.trim() : ""
                            };
                        }).filter(x => x !== null);
                    };
                    const descEls = Array.from(document.querySelectorAll('.txt, .desc, p'));
                    const fullDesc = descEls.map(e => e.innerText.trim()).filter(t => t.length > 5).join('\\n');
                    const sampleCode = document.querySelector('pre, code, .sample');
                    const example = sampleCode ? sampleCode.innerText.trim() : "";
                    return {
                        description: fullDesc,
                        example: example,
                        method: getTxt("td[name='httpMethod']"),
                        real_tr_id: getTxt("td[name='realTrId']"),
                        vts_tr_id: getTxt("td[name='virtualTrId']"),
                        request: parseTable("#reqBody")
                    };
                }"""
                )

                res_fields = []
                response_btn = page.locator("button:has-text('Response')")
                if await response_btn.count() > 0:
                    await response_btn.click()
                    await asyncio.sleep(1.0)
                    res_fields = await page.evaluate(
                        """(id) => {
                        const rows = document.querySelectorAll(`${id} table tbody tr`);
                        return Array.from(rows).map(row => {
                            const cols = row.querySelectorAll('td');
                            if (cols.length < 3) return null;
                            return {
                                name: cols[0].innerText.trim(),
                                korean_name: cols[1].innerText.trim(),
                                type: cols[2].innerText.trim(),
                                required: "N",
                                description: cols[5] ? cols[5].innerText.trim() : ""
                            };
                        }).filter(x => x !== null);
                    }""",
                        "#resBody",
                    )

                yaml_results.append(
                    {
                        "name": api["name"],
                        "description": detail_data["description"],
                        "example": detail_data["example"],
                        "method": detail_data["method"]
                        if detail_data["method"]
                        else "POST",
                        "endpoint": api["path"],
                        "real_tr_id": detail_data["real_tr_id"],
                        "vts_tr_id": detail_data["vts_tr_id"],
                        "request": detail_data["request"],
                        "response": res_fields,
                    }
                )

            except Exception as e:
                print(f"  [!] Skip {api['name']}: {e}")
                yaml_results.append(
                    {
                        "name": api["name"],
                        "endpoint": api["path"],
                        "method": "POST",
                        "request": [],
                        "response": [],
                    }
                )

            if (i + 1) % 20 == 0:
                with open(OPENAPI_YAML_FILE, "w", encoding="utf-8") as f:
                    yaml.dump(
                        {"config": WS_CONFIG, "apis": yaml_results},
                        f,
                        allow_unicode=True,
                        sort_keys=False,
                    )

        with open(OPENAPI_YAML_FILE, "w", encoding="utf-8") as f:
            yaml.dump(
                {"config": WS_CONFIG, "apis": yaml_results},
                f,
                allow_unicode=True,
                sort_keys=False,
            )
        print("[+] Fetching completed.")
        await browser.close()

    # ── kis-raw-data.json 업데이트 결정 ──────────────────────────────────
    if raw_api_by_url:
        # XHR로 전체(또는 과반) 데이터 캡처 성공 → 직접 저장
        raw_list = list(raw_api_by_url.values())
        print(f"[+] XHR captured {len(raw_list)} APIs → saving to {RAW_DATA_FILE}")
        with open(RAW_DATA_FILE, "w", encoding="utf-8") as f:
            json.dump(raw_list, f, ensure_ascii=False, indent=2)
    elif yaml_results:
        # XHR 미캡처 → HTML 스크래핑 결과를 기존 raw-data와 병합하여 저장
        print(
            f"[!] XHR capture empty — merging HTML-scraped data with existing {RAW_DATA_FILE}"
        )
        _merge_html_into_raw(yaml_results)
    else:
        print("[!] No data fetched — keeping existing kis-raw-data.json unchanged")


def _merge_html_into_raw(yaml_results: list[dict]):
    """HTML 스크래핑 결과를 기존 kis-raw-data.json에 병합.

    - request 필드(req_b): 스크래핑 결과로 갱신
    - response 필드(res_b): 스크래핑 결과가 있으면 갱신, 없으면 기존 유지
    - 기존에 없는 새 엔드포인트: REST 기본값으로 추가
    - 기존 항목이 스크래핑 결과에 없으면 삭제하지 않고 유지
    """
    # 기존 raw data 로드
    existing: dict[str, dict] = {}
    try:
        with open(RAW_DATA_FILE, encoding="utf-8") as f:
            for item in json.load(f):
                ep = item.get("accessUrl", "")
                if ep:
                    existing[ep] = item
    except Exception:
        pass

    for api in yaml_results:
        endpoint = api.get("endpoint", "")
        if not endpoint:
            continue

        old = existing.get(endpoint, {})

        # req_b 속성 구성
        req_props = [
            {
                "bodyType": "req_b",
                "propertyCd": f.get("name", ""),
                "propertyNm": f.get("korean_name", f.get("name", "")),
                "propertyType": "A0001",
                "requireYn": "Y" if f.get("required", "N") == "Y" else "N",
                "description": f.get("description", ""),
            }
            for f in api.get("request", [])
            if f.get("name")
        ]

        # res_b 속성: 스크래핑 결과 우선, 없으면 기존 보존
        html_res = api.get("response", [])
        if html_res:
            res_props = [
                {
                    "bodyType": "res_b",
                    "propertyCd": f.get("name", ""),
                    "propertyNm": f.get("korean_name", f.get("name", "")),
                    "propertyType": "A0001",
                    "requireYn": "N",
                    "description": f.get("description", ""),
                }
                for f in html_res
                if f.get("name")
            ]
        else:
            res_props = [
                p
                for p in old.get("apiPropertys", [])
                if p.get("bodyType") == "res_b"
            ]

        existing[endpoint] = {
            **old,
            "name": api.get("name", old.get("name", "")),
            "accessUrl": endpoint,
            "realDomain": old.get("realDomain", REAL_DOMAIN),
            "virtualDomain": old.get("virtualDomain", VTS_DOMAIN),
            "httpMethod": api.get("method", old.get("httpMethod", "POST")),
            "apiType": old.get("apiType", "REST"),
            "realTrId": api.get("real_tr_id", old.get("realTrId", "")),
            "virtualTrId": api.get("vts_tr_id", old.get("virtualTrId", "")),
            "description": api.get("description", old.get("description", "")),
            "apiPropertys": req_props + res_props,
        }

    # WebSocket 합성 항목 보존 (config.rs 생성에 필요)
    for ep, item in list(existing.items()):
        if item.get("apiType") == "WEBSOCKET" and ep not in {
            a["endpoint"] for a in yaml_results if a.get("endpoint")
        }:
            pass  # 이미 existing에 있음

    merged = list(existing.values())
    with open(RAW_DATA_FILE, "w", encoding="utf-8") as f:
        json.dump(merged, f, ensure_ascii=False, indent=2)
    print(f"[+] Merged {len(yaml_results)} APIs → saved to {RAW_DATA_FILE}")


if __name__ == "__main__":
    asyncio.run(fetch_spec())
