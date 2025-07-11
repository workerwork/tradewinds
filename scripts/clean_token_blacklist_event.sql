-- 启用 MySQL 事件调度器（只需执行一次）
SET GLOBAL event_scheduler = ON;

-- 创建定时清理事件：每天凌晨2点清理一次 token_blacklist 表中过期的 token
CREATE EVENT IF NOT EXISTS clean_token_blacklist
ON SCHEDULE EVERY 1 DAY
STARTS CURRENT_DATE + INTERVAL 2 HOUR
DO
  DELETE FROM token_blacklist WHERE expires_at < NOW();

-- 查看事件是否创建成功
-- SHOW EVENTS FROM tradewinds; 