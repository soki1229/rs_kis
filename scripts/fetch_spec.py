import asyncio
import yaml
import re
from playwright.async_api import async_playwright

BASE_URL = "https://apiportal.koreainvestment.com/apiservice-apiservice"

async def fetch_spec():
    async with async_playwright() as p:
        browser = await p.chromium.launch(headless=True)
        context = await browser.new_context()
        page = await context.new_page()
        
        print(f"[*] Connecting to KIS Portal: {BASE_URL}")
        await page.goto(BASE_URL, wait_until="networkidle")
        await asyncio.sleep(5)
        
        links = await page.query_selector_all("#sideBar a[onclick*='goLeftMenuUrl']")
        api_tasks = []
        for link in links:
            onclick = await link.get_attribute("onclick")
            match = re.search(r"goLeftMenuUrl\('(.+?)'\)", onclick)
            if match:
                path = match.group(1)
                name = await link.inner_text()
                api_tasks.append({"name": name.strip(), "path": path})
        
        print(f"[+] Found {len(api_tasks)} APIs. Scraping details...")
        
        results = []
        for i, api in enumerate(api_tasks):
            print(f"[{i+1}/{len(api_tasks)}] Scraping {api['name']} ({api['path']})")
            try:
                await page.evaluate(f"goLeftMenuUrl('{api['path']}')")
                await page.wait_for_selector("#apiInfoForm", timeout=15000)
                await asyncio.sleep(2)
                
                detail_data = await page.evaluate("""() => {
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

                    // 설명 및 샘플 데이터 추출
                    const descEls = Array.from(document.querySelectorAll('.txt'));
                    const mainDesc = descEls.reduce((a, b) => a.length > b.innerText.length ? a : b.innerText, "");
                    
                    // JSON Body 샘플 찾기 (보통 pre 태그 등에 있음)
                    const sampleCode = document.querySelector('pre, code');
                    const example = sampleCode ? sampleCode.innerText.trim() : "";

                    return {
                        description: mainDesc,
                        example: example,
                        method: getTxt("td[name='httpMethod']"),
                        real_tr_id: getTxt("td[name='realTrId']"),
                        vts_tr_id: getTxt("td[name='virtualTrId']"),
                        request: parseTable("#reqBody")
                    };
                }""")

                res_fields = []
                response_btn = page.locator("button:has-text('Response')")
                if await response_btn.count() > 0:
                    await response_btn.click()
                    await asyncio.sleep(1.0)
                    res_fields = await page.evaluate("""(id) => {
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
                    }""", "#resBody")

                results.append({
                    "name": api['name'],
                    "description": detail_data['description'],
                    "example": detail_data['example'],
                    "method": detail_data['method'] if detail_data['method'] else "POST",
                    "endpoint": api['path'],
                    "real_tr_id": detail_data['real_tr_id'],
                    "vts_tr_id": detail_data['vts_tr_id'],
                    "request": detail_data['request'],
                    "response": res_fields
                })
                
            except Exception as e:
                print(f"  [!] Skip {api['name']}: {e}")
                results.append({"name": api['name'], "endpoint": api['path'], "method": "POST", "request": [], "response": []})

            if (i + 1) % 20 == 0:
                with open("crates/kis_api/kis-openapi.yaml", "w", encoding="utf-8") as f:
                    yaml.dump({"apis": results}, f, allow_unicode=True, sort_keys=False)

        with open("crates/kis_api/kis-openapi.yaml", "w", encoding="utf-8") as f:
            yaml.dump({"apis": results}, f, allow_unicode=True, sort_keys=False)
        print("[+] Fetching completed.")
        await browser.close()

if __name__ == "__main__":
    asyncio.run(fetch_spec())
