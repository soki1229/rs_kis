-- orders: add exchange_code for KR market routing (KOSPI "J" / KOSDAQ "Q")
-- NULL = US or unknown; read back by reconcile_kr_submitted_orders on restart
ALTER TABLE orders ADD COLUMN exchange_code TEXT;
