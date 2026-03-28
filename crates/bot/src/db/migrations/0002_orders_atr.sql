-- orders: atr (진입 시점 ATR_14) 컬럼 추가.
-- nullable: 과거 주문 및 force-sell 주문은 ATR 없이 삽입될 수 있음.
ALTER TABLE orders ADD COLUMN atr TEXT;
