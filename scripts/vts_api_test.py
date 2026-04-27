#!/usr/bin/env python3
"""
VTS(모의투자) API 전수검사 스크립트
- 토큰 발급/캐시 재사용
- VTS TR_ID 보유 API 42개 순차 호출
- TPS 준수 (0.6초 간격 = ~100 req/min 이하)
"""

import json, os, re, time, datetime, requests
from pathlib import Path

# ─── 환경변수 로드 ──────────────────────────────────────────────
ENV_PATH = Path(__file__).parent.parent.parent / "rs_kis_pilot" / ".env"

def load_env(path):
    env = {}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#"): continue
            k, _, v = line.partition("=")
            env[k.strip()] = v.strip().strip('"')
    return env

ENV = load_env(ENV_PATH)
VTS_KEY    = ENV["VTS_APP_KEY"]
VTS_SECRET = ENV["VTS_APP_SECRET"]
CANO       = ENV["KIS_VTS_ACCOUNT_NO"]
ACNT_CD    = ENV["KIS_VTS_ACCOUNT_CD"]

VTS_BASE  = "https://openapivts.koreainvestment.com:29443"
TOKEN_CACHE = Path.home() / ".kis_vts_api_test_token.json"
TPS_DELAY   = 0.65   # 초

# ─── 토큰 발급/캐시 ────────────────────────────────────────────
def get_token():
    if TOKEN_CACHE.exists():
        cache = json.loads(TOKEN_CACHE.read_text())
        expires = datetime.datetime.fromisoformat(cache["access_token_token_expired"])
        if datetime.datetime.now() < expires - datetime.timedelta(minutes=10):
            print(f"[토큰] 캐시 재사용 (만료: {cache['access_token_token_expired']})")
            return cache["access_token"]

    print("[토큰] 신규 발급 중...")
    resp = requests.post(
        f"{VTS_BASE}/oauth2/tokenP",
        json={
            "grant_type": "client_credentials",
            "appkey":    VTS_KEY,
            "appsecret": VTS_SECRET,
        },
        timeout=10,
    )
    resp.raise_for_status()
    data = resp.json()
    TOKEN_CACHE.write_text(json.dumps(data, ensure_ascii=False))
    print(f"[토큰] 발급 완료 (만료: {data.get('access_token_token_expired','?')})")
    return data["access_token"]

# ─── 공통 요청 헬퍼 ───────────────────────────────────────────
def call(token, method, endpoint, tr_id, params=None, body=None):
    headers = {
        "content-type":  "application/json",
        "authorization": f"Bearer {token}",
        "appkey":        VTS_KEY,
        "appsecret":     VTS_SECRET,
        "tr_id":         tr_id,
        "custtype":      "P",
    }
    url = VTS_BASE + endpoint
    try:
        if method == "GET":
            r = requests.get(url, headers=headers, params=params, timeout=10)
        else:
            r = requests.post(url, headers=headers, json=body or {}, timeout=10)
        r.raise_for_status()
        data = r.json()
        rt_cd = data.get("rt_cd", "?")
        msg   = data.get("msg1", "")
        return rt_cd, msg, data
    except Exception as e:
        return "ERR", str(e), {}

# ─── 결과 출력 헬퍼 ──────────────────────────────────────────
results = []

def log(name, tr_id, rt_cd, msg, note=""):
    ok = rt_cd == "0"
    icon = "✅" if ok else ("⚠️" if rt_cd in ("1","7") else "❌")
    line = f"{icon} [{tr_id}] {name}"
    if not ok:
        line += f" → rt_cd={rt_cd} {msg[:80]}"
    if note:
        line += f"  ({note})"
    print(line)
    results.append({"name": name, "tr_id": tr_id, "rt_cd": rt_cd, "msg": msg, "ok": ok})

def pause(label=""):
    if label:
        print(f"   ⏳ {label}...")
    time.sleep(TPS_DELAY)

# ─── 메인 검사 ──────────────────────────────────────────────
def run():
    token = get_token()
    print(f"\n{'='*60}")
    print(f" VTS API 전수검사  계좌: {CANO}-{ACNT_CD}")
    print(f"{'='*60}\n")

    today = datetime.date.today().strftime("%Y%m%d")
    week_ago = (datetime.date.today() - datetime.timedelta(days=7)).strftime("%Y%m%d")
    test_kr_symbol   = "005930"   # 삼성전자
    test_us_symbol   = "AAPL"
    test_us_exchange = "NASD"

    # ── 1. 국내주식 시세 조회 (읽기, 장외도 항상 동작) ────────────────

    print("── [1] 국내주식 시세 조회 ──")
    rt, msg, data = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-price",
        "FHKST01010100",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol})
    log("inquire-price (삼성전자)", "FHKST01010100", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-daily-price",
        "FHKST01010400",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol,
                "FID_PERIOD_DIV_CODE": "D", "FID_ORG_ADJ_PRC": "0"})
    log("inquire-daily-price", "FHKST01010400", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-ccnl",
        "FHKST01010300",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol})
    log("inquire-ccnl (체결)", "FHKST01010300", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-asking-price-exp-ccn",
        "FHKST01010200",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol})
    log("inquire-asking-price-exp-ccn (호가)", "FHKST01010200", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-investor",
        "FHKST01010900",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol})
    log("inquire-investor (투자자)", "FHKST01010900", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-member",
        "FHKST01010600",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol})
    log("inquire-member (회원사)", "FHKST01010600", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-daily-itemchartprice",
        "FHKST03010100",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol,
                "FID_INPUT_DATE_1": week_ago, "FID_INPUT_DATE_2": today,
                "FID_PERIOD_DIV_CODE": "D", "FID_ORG_ADJ_PRC": "0",
                "FID_ETC_CLS_CODE": ""})
    log("inquire-daily-itemchartprice", "FHKST03010100", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-time-itemchartprice",
        "FHKST03010200",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol,
                "FID_INPUT_HOUR_1": "090000", "FID_PW_DATA_INCU_YN": "N",
                "FID_ETC_CLS_CODE": ""})
    log("inquire-time-itemchartprice (분봉)", "FHKST03010200", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-time-itemconclusion",
        "FHPST01060000",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol,
                "FID_INPUT_HOUR_1": "090000"})
    log("inquire-time-itemconclusion (시간대별체결)", "FHPST01060000", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-daily-overtimeprice",
        "FHPST02320000",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol})
    log("inquire-daily-overtimeprice (시간외)", "FHPST02320000", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-time-overtimeconclusion",
        "FHPST02310000",
        params={"FID_COND_MRKT_DIV_CODE": "J", "FID_INPUT_ISCD": test_kr_symbol,
                "FID_INPUT_HOUR_1": "162000", "FID_HOUR_CLS_CODE": "0"})
    log("inquire-time-overtimeconclusion (시간외체결)", "FHPST02310000", rt, msg)
    pause()

    # ELW (삼성전자 ELW 코드로 조회, 없어도 rt_cd=0 반환 가능)
    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-elw-price",
        "FHKEW15010000",
        params={"FID_COND_MRKT_DIV_CODE": "W", "FID_INPUT_ISCD": "57LGHA"})
    log("inquire-elw-price (ELW)", "FHKEW15010000", rt, msg, note="ELW코드 57LGHA")
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/quotations/inquire-daily-indexchartprice",
        "FHKUP03500100",
        params={"FID_COND_MRKT_DIV_CODE": "U", "FID_INPUT_ISCD": "0001",
                "FID_INPUT_DATE_1": week_ago, "FID_INPUT_DATE_2": today,
                "FID_PERIOD_DIV_CODE": "D"})
    log("inquire-daily-indexchartprice (코스피)", "FHKUP03500100", rt, msg)
    pause()

    # ── 2. 국내주식 계좌/거래 ──────────────────────────────────────

    print("\n── [2] 국내주식 계좌/거래 ──")

    rt, msg, bal_data = call(token, "GET",
        "/uapi/domestic-stock/v1/trading/inquire-balance",
        "VTTC8434R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "AFHR_FLPR_YN": "N", "OFL_YN": "",
                "INQR_DVSN": "02", "UNPR_DVSN": "01",
                "FUND_STTL_ICLD_YN": "N", "FNCG_AMT_AUTO_RDPT_YN": "N",
                "PRCS_DVSN": "01", "CTX_AREA_FK100": "", "CTX_AREA_NK100": ""})
    log("inquire-balance (잔고)", "VTTC8434R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/trading/inquire-psbl-order",
        "VTTC8908R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "PDNO": test_kr_symbol, "ORD_UNPR": "0",
                "ORD_DVSN": "01", "CMA_EVLU_AMT_ICLD_YN": "N",
                "OVRS_ICLD_YN": "N"})
    log("inquire-psbl-order (주문가능)", "VTTC8908R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
        "VTTC0081R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "INQR_STRT_DT": week_ago, "INQR_END_DT": today,
                "SLL_BUY_DVSN_CD": "00", "INQR_DVSN": "00",
                "PDNO": "", "CCLD_DVSN": "01", "ORD_GNO_BRNO": "",
                "ODNO": "", "INQR_DVSN_3": "00", "INQR_DVSN_1": "",
                "CTX_AREA_FK100": "", "CTX_AREA_NK100": ""})
    log("inquire-daily-ccld_recent (체결내역)", "VTTC0081R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-stock/v1/trading/inquire-daily-ccld",
        "VTSC9215R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "INQR_STRT_DT": "20250101", "INQR_END_DT": today,
                "SLL_BUY_DVSN_CD": "00", "INQR_DVSN": "00",
                "PDNO": "", "CCLD_DVSN": "01", "ORD_GNO_BRNO": "",
                "ODNO": "", "INQR_DVSN_3": "00", "INQR_DVSN_1": "",
                "CTX_AREA_FK100": "", "CTX_AREA_NK100": ""})
    log("inquire-daily-ccld_old (3개월이전)", "VTSC9215R", rt, msg)
    pause()

    # ── 3. 국내주식 매수/매도/정정취소 ─────────────────────────────
    print("\n── [3] 국내주식 주문 (1주, 장외이므로 미처리 정상) ──")

    buy_odno = ""
    rt, msg, order_data = call(token, "POST",
        "/uapi/domestic-stock/v1/trading/order-cash",
        "VTTC0012U",
        body={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
              "PDNO": test_kr_symbol, "ORD_DVSN": "01",
              "ORD_QTY": "1", "ORD_UNPR": "0"})
    log("order-cash buy 삼성전자 1주 (시장가)", "VTTC0012U", rt, msg)
    if rt == "0":
        buy_odno = order_data.get("output", {}).get("ODNO", "")
        print(f"   → 주문번호: {buy_odno}")
    pause()

    # 정정/취소 테스트 (주문번호 있을 때만)
    if buy_odno:
        rt, msg, _ = call(token, "POST",
            "/uapi/domestic-stock/v1/trading/order-rvsecncl",
            "VTTC0013U",
            body={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                  "KRX_FWDG_ORD_ORGNO": "", "ORGN_ODNO": buy_odno,
                  "ORD_DVSN": "01", "RVSE_CNCL_DVSN_CD": "02",
                  "ORD_QTY": "1", "ORD_UNPR": "0", "QTY_ALL_ORD_YN": "Y"})
        log(f"order-rvsecncl 취소 (주문번호: {buy_odno})", "VTTC0013U", rt, msg)
        pause()

    # 매도 (보유 종목 없어도 API 자체 동작 확인)
    rt, msg, _ = call(token, "POST",
        "/uapi/domestic-stock/v1/trading/order-cash",
        "VTTC0011U",
        body={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
              "PDNO": test_kr_symbol, "ORD_DVSN": "01",
              "ORD_QTY": "1", "ORD_UNPR": "0"})
    log("order-cash sell 삼성전자 1주", "VTTC0011U", rt, msg,
        note="보유수량 부족 시 오류 정상")
    pause()

    # ── 4. 선물옵션 시세 ──────────────────────────────────────────
    print("\n── [4] 국내 선물옵션 시세 ──")

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-futureoption/v1/quotations/inquire-price",
        "FHMIF10000000",
        params={"FID_COND_MRKT_DIV_CODE": "F", "FID_INPUT_ISCD": "101W9000"})
    log("futureoption inquire-price", "FHMIF10000000", rt, msg, note="코스피200선물 근월물")
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-futureoption/v1/quotations/inquire-asking-price",
        "FHMIF10010000",
        params={"FID_COND_MRKT_DIV_CODE": "F", "FID_INPUT_ISCD": "101W9000"})
    log("futureoption inquire-asking-price (호가)", "FHMIF10010000", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-futureoption/v1/quotations/inquire-daily-fuopchartprice",
        "FHKIF03020100",
        params={"FID_COND_MRKT_DIV_CODE": "F", "FID_INPUT_ISCD": "101W9000",
                "FID_INPUT_DATE_1": week_ago, "FID_INPUT_DATE_2": today,
                "FID_PERIOD_DIV_CODE": "D"})
    log("futureoption inquire-daily-fuopchartprice", "FHKIF03020100", rt, msg)
    pause()

    # ── 5. 선물옵션 계좌 조회 ─────────────────────────────────────
    print("\n── [5] 선물옵션 계좌 조회 ──")

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-futureoption/v1/trading/inquire-balance",
        "VTFO6118R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "MGNA_DVSN": "01", "EXCC_STAT_CD": "1",
                "CTX_AREA_FK200": "", "CTX_AREA_NK200": ""})
    log("futureoption inquire-balance", "VTFO6118R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-futureoption/v1/trading/inquire-ccnl",
        "VTTO5201R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "SLL_BUY_DVSN_CD": "00", "CCLD_NCCS_DVSN": "00",
                "SORT_SQN": "DS", "MKET_ID_CD": "KRX",
                "STRT_ORD_DT": week_ago, "END_ORD_DT": today,
                "STRT_ODNO": "", "PDNO": "",
                "CTX_AREA_FK200": "", "CTX_AREA_NK200": ""})
    log("futureoption inquire-ccnl (체결내역)", "VTTO5201R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/domestic-futureoption/v1/trading/inquire-psbl-order",
        "VTTO5105R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "SLL_BUY_DVSN_CD": "02", "PDNO": "101W9000",
                "ORD_UNPR": "0", "ORD_DVSN_CD": "01", "UNIT_PRICE": "0"})
    log("futureoption inquire-psbl-order (주문가능)", "VTTO5105R", rt, msg)
    pause()

    # ── 6. 해외주식 시세 ──────────────────────────────────────────
    print("\n── [6] 해외주식 시세 ──")

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-price/v1/quotations/price",
        "HHDFS00000300",
        params={"AUTH": "", "EXCD": "NAS", "SYMB": test_us_symbol})
    log("overseas price (AAPL 현재가)", "HHDFS00000300", rt, msg, note="EXCD=NAS(NASDAQ 3글자코드)")
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-price/v1/quotations/dailyprice",
        "HHDFS76240000",
        params={"AUTH": "", "EXCD": test_us_exchange, "SYMB": test_us_symbol,
                "GUBN": "0", "BYMD": "", "MODP": "0"})
    log("overseas dailyprice (AAPL 일별)", "HHDFS76240000", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-price/v1/quotations/inquire-daily-chartprice",
        "FHKST03030100",
        params={"FID_COND_MRKT_DIV_CODE": "N", "FID_INPUT_ISCD": test_us_symbol,
                "FID_INPUT_DATE_1": week_ago, "FID_INPUT_DATE_2": today,
                "FID_PERIOD_DIV_CODE": "D"})
    log("overseas inquire-daily-chartprice (분봉차트)", "FHKST03030100", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-price/v1/quotations/inquire-search",
        "HHDFS76410000",
        params={"AUTH": "", "EXCD": test_us_exchange, "CO_YN_PRICECUR": "1",
                "CO_ST_PRICECUR": "100", "CO_EN_PRICECUR": "200",
                "CO_YN_RATE": "", "CO_ST_RATE": "", "CO_EN_RATE": "",
                "CO_YN_VOLUME": "", "CO_ST_VOLUME": "", "CO_EN_VOLUME": "",
                "CO_YN_AMT": "", "CO_ST_AMT": "", "CO_EN_AMT": "",
                "CO_YN_EPS": "", "CO_ST_EPS": "", "CO_EN_EPS": "",
                "CO_YN_PER": "", "CO_ST_PER": "", "CO_EN_PER": ""})
    log("overseas inquire-search (종목 검색)", "HHDFS76410000", rt, msg)
    pause()

    # ── 7. 해외주식 계좌 조회 ─────────────────────────────────────
    print("\n── [7] 해외주식 계좌/거래 조회 ──")

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-stock/v1/trading/inquire-balance",
        "VTTS3012R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "OVRS_EXCG_CD": test_us_exchange, "TR_CRCY_CD": "USD",
                "CTX_AREA_FK200": "", "CTX_AREA_NK200": ""})
    log("overseas inquire-balance (해외잔고)", "VTTS3012R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-stock/v1/trading/inquire-psamount",
        "VTTS3007R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "OVRS_EXCG_CD": test_us_exchange, "OVRS_ORD_UNPR": "150",
                "ITEM_CD": test_us_symbol})
    log("overseas inquire-psamount (주문가능금액)", "VTTS3007R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-stock/v1/trading/inquire-ccnl",
        "VTTS3035R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "ORD_STRT_DT": week_ago, "ORD_END_DT": today, "ORD_DT": week_ago,
                "SLL_BUY_DVSN": "00", "CCLD_NCCS_DVSN": "00", "SORT_SQN": "DS",
                "OVRS_EXCG_CD": "NASD", "PDNO": "",
                "ORD_GNO_BRNO": "", "ODNO": "",
                "CTX_AREA_FK200": "", "CTX_AREA_NK200": ""})
    log("overseas inquire-ccnl (해외체결)", "VTTS3035R", rt, msg)
    pause()

    rt, msg, _ = call(token, "GET",
        "/uapi/overseas-stock/v1/trading/inquire-present-balance",
        "VTRP6504R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "WCRC_FRCR_DVSN_CD": "01", "NATN_CD": "840",
                "TR_MKET_CD": "00", "INQR_DVSN_CD": "00"})
    log("overseas inquire-present-balance (현재잔고)", "VTRP6504R", rt, msg)
    pause()

    # ── 8. 해외주식 매수/매도 ─────────────────────────────────────
    print("\n── [8] 해외주식 주문 ──")

    us_buy_odno = ""
    rt, msg, us_order = call(token, "POST",
        "/uapi/overseas-stock/v1/trading/order",
        "VTTT1002U",
        body={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
              "OVRS_EXCG_CD": test_us_exchange, "PDNO": test_us_symbol,
              "ORD_DVSN": "00", "ORD_QTY": "1", "OVRS_ORD_UNPR": "1.00",
              "ORD_SVR_DVSN_CD": "0"})
    log("overseas order buy AAPL 1주 ($1 지정가)", "VTTT1002U", rt, msg)
    if rt == "0":
        us_buy_odno = us_order.get("output", {}).get("ODNO", "")
        print(f"   → ODNO (대문자): '{us_buy_odno}'")
        odno_lower = us_order.get("output", {}).get("odno", "")
        print(f"   → odno (소문자): '{odno_lower}'")
        print(f"   → output 전체: {json.dumps(us_order.get('output', {}), ensure_ascii=False)}")
    pause()

    if us_buy_odno:
        rt, msg, _ = call(token, "POST",
            "/uapi/overseas-stock/v1/trading/order-rvsecncl",
            "VTTT1004U",
            body={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                  "OVRS_EXCG_CD": test_us_exchange, "PDNO": test_us_symbol,
                  "ORGN_ODNO": us_buy_odno, "ORD_SVR_DVSN_CD": "0",
                  "RVSE_CNCL_DVSN_CD": "02", "ORD_QTY": "1",
                  "OVRS_ORD_UNPR": "0", "QTY_ALL_ORD_YN": "Y"})
        log(f"overseas order-rvsecncl 취소 (주문번호: {us_buy_odno})", "VTTT1004U", rt, msg)
        pause()

    # 매도: VTS 매도 TR_ID = VTTT1001U (포털 원본, VTS 검증 완료)
    rt, msg, us_sell = call(token, "POST",
        "/uapi/overseas-stock/v1/trading/order",
        "VTTT1001U",
        body={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
              "OVRS_EXCG_CD": test_us_exchange, "PDNO": test_us_symbol,
              "ORD_DVSN": "00", "ORD_QTY": "1", "OVRS_ORD_UNPR": "99999.00",
              "ORD_SVR_DVSN_CD": "0"})
    log("overseas order sell AAPL 1주 (VTTT1001U, $99999 지정가)", "VTTT1001U", rt, msg,
        note="보유수량 없으면 오류 정상")
    pause()

    # ── 9. 추가 검증: 미검증 항목 ──────────────────────────────────
    print("\n── [9] 추가 검증 (res_b 필드, 휴일체크) ──")

    # 9-1. chk-holiday: 포털 realTrId=CTCA0903R, virtualTrId=모의투자 미지원
    # VTS에서 호출 불가 → 실서버 전용 API. 서버 코드는 unwrap_or(false)로 처리됨.
    print("   ℹ️  chk-holiday (CTCA0903R) = VTS 미지원, 실서버 전용. 스킵.")
    pause()

    # 9-2. inquire-present-balance output2 구조 검증
    rt, msg, pbal_data = call(token, "GET",
        "/uapi/overseas-stock/v1/trading/inquire-present-balance",
        "VTRP6504R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "WCRC_FRCR_DVSN_CD": "01", "NATN_CD": "840",
                "TR_MKET_CD": "00", "INQR_DVSN_CD": "00"})
    log("overseas inquire-present-balance (output2 구조 검증)", "VTRP6504R", rt, msg)
    if rt == "0":
        out2 = pbal_data.get("output2", "MISSING")
        print(f"   → output2 타입: {type(out2).__name__}")
        if isinstance(out2, list):
            print(f"   → output2 길이: {len(out2)}")
            if out2:
                frcr = out2[0].get("frcr_dncl_amt_2", "MISSING")
                print(f"   → output2[0].frcr_dncl_amt_2: '{frcr}'")
        elif isinstance(out2, dict):
            frcr = out2.get("frcr_dncl_amt_2", "MISSING")
            print(f"   → output2.frcr_dncl_amt_2: '{frcr}'")
        print(f"   → output3 존재: {'output3' in pbal_data}")
    pause()

    # 9-3. domestic balance: dnca_tot_amt 위치 검증
    rt, msg, kr_bal = call(token, "GET",
        "/uapi/domestic-stock/v1/trading/inquire-balance",
        "VTTC8434R",
        params={"CANO": CANO, "ACNT_PRDT_CD": ACNT_CD,
                "AFHR_FLPR_YN": "N", "OFL_YN": "",
                "INQR_DVSN": "02", "UNPR_DVSN": "01",
                "FUND_STTL_ICLD_YN": "N", "FNCG_AMT_AUTO_RDPT_YN": "N",
                "PRCS_DVSN": "01", "CTX_AREA_FK100": "", "CTX_AREA_NK100": ""})
    log("domestic inquire-balance (dnca_tot_amt 검증)", "VTTC8434R", rt, msg)
    if rt == "0":
        out2 = kr_bal.get("output2", [])
        if isinstance(out2, list) and out2:
            dnca = out2[0].get("dnca_tot_amt", "MISSING")
            print(f"   → output2[0].dnca_tot_amt: '{dnca}'")
        elif isinstance(out2, dict):
            dnca = out2.get("dnca_tot_amt", "MISSING")
            print(f"   → output2.dnca_tot_amt: '{dnca}'")
    pause()

    # ── 최종 요약 ──────────────────────────────────────────────
    print(f"\n{'='*60}")
    total   = len(results)
    ok_cnt  = sum(1 for r in results if r["ok"])
    warn_cnt = sum(1 for r in results if r["rt_cd"] in ("1","7"))
    fail_cnt = total - ok_cnt

    print(f" 결과: {ok_cnt}/{total} 성공  |  경고(rt≠0) {fail_cnt}건")
    print(f"{'='*60}")

    if fail_cnt:
        print("\n[실패/경고 목록]")
        for r in results:
            if not r["ok"]:
                print(f"  ❌ [{r['tr_id']}] {r['name']}")
                print(f"     rt_cd={r['rt_cd']}  {r['msg'][:100]}")

if __name__ == "__main__":
    run()
