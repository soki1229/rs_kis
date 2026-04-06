-- daily_ohlc: 일봉 히스토리 (시그널 평가용, ATR-14 계산 기준).
-- (symbol, date) 복합 PK — INSERT OR REPLACE로 중복 없이 갱신.
CREATE TABLE IF NOT EXISTS daily_ohlc (
    symbol  TEXT NOT NULL,
    date    TEXT NOT NULL,   -- 'YYYYMMDD'
    open    TEXT NOT NULL,
    high    TEXT NOT NULL,
    low     TEXT NOT NULL,
    close   TEXT NOT NULL,
    volume  TEXT NOT NULL,
    PRIMARY KEY (symbol, date)
);

-- audit_log: 시스템 이벤트 이력 (디버깅/감사용).
CREATE TABLE IF NOT EXISTS audit_log (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,
    market     TEXT,
    symbol     TEXT,
    detail     TEXT NOT NULL,
    created_at TEXT NOT NULL
);
