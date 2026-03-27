-- orders: 모든 주문 기록. broker_order_id 없으면 재시작 복구 불가.
CREATE TABLE IF NOT EXISTS orders (
    id                 TEXT PRIMARY KEY,
    broker_order_id    TEXT,                       -- KIS 반환 주문번호, 제출 직후 저장 필수
    symbol             TEXT NOT NULL,
    side               TEXT NOT NULL CHECK(side IN ('buy','sell')),
    state              TEXT NOT NULL,              -- OrderState JSON
    order_type         TEXT NOT NULL DEFAULT 'marketable_limit',
    qty                TEXT NOT NULL,              -- Decimal as string
    price              TEXT,                       -- nullable: 재호가 전 미정
    filled_qty         TEXT NOT NULL DEFAULT '0',
    submitted_at       TEXT,
    updated_at         TEXT NOT NULL
);

-- positions: 현재 보유 포지션.
-- atr_at_entry: 진입 시점 ATR_14 (손절가 기준, 이후 갱신 없음).
-- trailing_stop_price: 1차 목표 달성 후에만 설정됨.
CREATE TABLE IF NOT EXISTS positions (
    id                   TEXT PRIMARY KEY,
    order_id             TEXT NOT NULL REFERENCES orders(id),
    symbol               TEXT NOT NULL,
    qty                  TEXT NOT NULL,
    entry_price          TEXT NOT NULL,
    stop_price           TEXT NOT NULL,
    trailing_stop_price  TEXT,
    atr_at_entry         TEXT NOT NULL,
    profit_target_1      TEXT NOT NULL,
    profit_target_2      TEXT NOT NULL,
    partial_exit_done    INTEGER NOT NULL DEFAULT 0, -- 1차 익절 완료 여부
    regime_at_entry      TEXT NOT NULL,              -- MarketRegime JSON
    entered_at           TEXT NOT NULL,
    updated_at           TEXT NOT NULL
);

-- session_stats: 당일 통계 (날짜별 1행).
CREATE TABLE IF NOT EXISTS session_stats (
    date                TEXT PRIMARY KEY,  -- 'YYYY-MM-DD' KST
    pnl                 TEXT NOT NULL DEFAULT '0',
    consecutive_losses  INTEGER NOT NULL DEFAULT 0,
    trade_count         INTEGER NOT NULL DEFAULT 0,
    max_drawdown        TEXT NOT NULL DEFAULT '0',
    profile_active      TEXT NOT NULL DEFAULT 'default',
    updated_at          TEXT NOT NULL
);

-- signal_log: 모든 신호 판정 이력 (분석용).
CREATE TABLE IF NOT EXISTS signal_log (
    id               INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol           TEXT NOT NULL,
    setup_score      INTEGER NOT NULL,
    rule_direction   TEXT,              -- 'long' | 'short' | NULL (미달)
    rule_strength    REAL,
    llm_verdict      TEXT,              -- 'enter' | 'watch' | 'block' | NULL (미호출)
    llm_block_reason TEXT,
    action           TEXT NOT NULL,    -- 'enter' | 'skip' | 'block' | 'expired'
    expire_reason    TEXT,             -- '2_day_elapsed' | 'event_window_ended'
    regime           TEXT NOT NULL,
    logged_at        TEXT NOT NULL
);

-- strategy_stats: 드리프트 모니터링 지표 (섹션 15).
CREATE TABLE IF NOT EXISTS strategy_stats (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    date       TEXT NOT NULL,   -- 'YYYY-MM-DD'
    metric     TEXT NOT NULL,   -- 'score_60_69_winrate' | 'llm_enter_avg_r' | ...
    value      REAL NOT NULL,
    updated_at TEXT NOT NULL
);
