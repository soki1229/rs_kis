-- positions: add exchange_code for KR market routing (KOSPI "J" / KOSDAQ "Q")
-- NULL = US or unknown (defaults to KOSPI-safe behaviour in process_kr_order)
ALTER TABLE positions ADD COLUMN exchange_code TEXT;
