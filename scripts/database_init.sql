-- ==============================================
-- Tradewinds 项目数据库初始化脚本
-- 版本: 1.0
-- 描述: 完整的数据库表结构和初始数据
-- ==============================================

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- 创建数据库（如果不存在）
CREATE DATABASE IF NOT EXISTS `tradewinds` 
CHARACTER SET utf8mb4 
COLLATE utf8mb4_unicode_ci;

USE `tradewinds`;

-- 设置时区
SET time_zone = '+00:00';

-- ==============================================
-- 表结构定义
-- ==============================================

-- 用户表
DROP TABLE IF EXISTS `users`;
CREATE TABLE `users` (
  `id` varchar(255) NOT NULL COMMENT '用户ID（UUID）',
  `username` varchar(50) NOT NULL COMMENT '用户名',
  `email` varchar(100) NOT NULL COMMENT '邮箱',
  `password` varchar(255) NOT NULL COMMENT '密码哈希',
  `real_name` varchar(50) DEFAULT NULL COMMENT '真实姓名',
  `phone` varchar(20) DEFAULT NULL COMMENT '手机号',
  `avatar` varchar(255) DEFAULT NULL COMMENT '头像URL',
  `status` int NOT NULL DEFAULT '1' COMMENT '状态：0-禁用，1-启用',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_username` (`username`),
  UNIQUE KEY `idx_email` (`email`),
  KEY `idx_status` (`status`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';

-- 角色表
DROP TABLE IF EXISTS `roles`;
CREATE TABLE `roles` (
  `id` varchar(255) NOT NULL COMMENT '角色ID（UUID）',
  `code` varchar(50) NOT NULL COMMENT '角色唯一标识',
  `name` varchar(50) NOT NULL COMMENT '角色名称',
  `description` varchar(255) DEFAULT NULL COMMENT '角色描述',
  `status` int NOT NULL DEFAULT '1' COMMENT '状态：0-禁用，1-启用',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_code` (`code`),
  UNIQUE KEY `idx_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色表';

-- 权限表
DROP TABLE IF EXISTS `permissions`;
CREATE TABLE `permissions` (
  `id` varchar(255) NOT NULL COMMENT '权限ID（UUID）',
  `name` varchar(100) NOT NULL COMMENT '权限名称',
  `code` varchar(100) DEFAULT NULL COMMENT '权限代码',
  `type` int NOT NULL DEFAULT '0' COMMENT '权限类型：0-菜单，1-按钮，2-API',
  `parent_id` varchar(255) DEFAULT NULL COMMENT '父权限ID',
  `path` varchar(255) DEFAULT NULL COMMENT '路径',
  `component` varchar(255) DEFAULT NULL COMMENT '组件路径',
  `icon` varchar(100) DEFAULT NULL COMMENT '图标',
  `sort` int NOT NULL DEFAULT '0' COMMENT '排序',
  `status` int NOT NULL DEFAULT '1' COMMENT '状态：0-禁用，1-启用',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `idx_code` (`code`),
  KEY `idx_parent_id` (`parent_id`),
  KEY `idx_type` (`type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='权限表';

-- 用户角色关联表
DROP TABLE IF EXISTS `user_roles`;
CREATE TABLE `user_roles` (
  `id` varchar(255) NOT NULL COMMENT '关联ID（UUID）',
  `user_id` varchar(255) NOT NULL COMMENT '用户ID',
  `role_id` varchar(255) NOT NULL COMMENT '角色ID',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_user_role` (`user_id`,`role_id`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_role_id` (`role_id`),
  CONSTRAINT `fk_user_roles_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_user_roles_role` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户角色关联表';

-- 角色权限关联表
DROP TABLE IF EXISTS `role_permissions`;
CREATE TABLE `role_permissions` (
  `id` varchar(255) NOT NULL COMMENT '关联ID（UUID）',
  `role_id` varchar(255) NOT NULL COMMENT '角色ID',
  `permission_id` varchar(255) NOT NULL COMMENT '权限ID',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_role_permission` (`role_id`,`permission_id`),
  KEY `idx_role_id` (`role_id`),
  KEY `idx_permission_id` (`permission_id`),
  CONSTRAINT `fk_role_permissions_role` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_role_permissions_permission` FOREIGN KEY (`permission_id`) REFERENCES `permissions` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='角色权限关联表';

-- Token 黑名单表
DROP TABLE IF EXISTS `token_blacklist`;
CREATE TABLE `token_blacklist` (
  `id` varchar(255) NOT NULL COMMENT 'Token黑名单ID（UUID）',
  `jti` varchar(255) NOT NULL COMMENT 'JWT ID',
  `user_id` varchar(255) NOT NULL COMMENT '用户ID',
  `expires_at` timestamp NOT NULL COMMENT '过期时间',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_jti` (`jti`),
  KEY `idx_user_id` (`user_id`),
  KEY `idx_expires_at` (`expires_at`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='Token黑名单表';

-- 系统参数表
DROP TABLE IF EXISTS `system_settings`;
CREATE TABLE `system_settings` (
  `id` varchar(64) NOT NULL PRIMARY KEY,
  `key` varchar(64) NOT NULL UNIQUE COMMENT '参数名',
  `value` varchar(255) NOT NULL COMMENT '参数值',
  `description` varchar(255) DEFAULT NULL COMMENT '描述',
  `updated_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ==============================================
-- 初始数据插入
-- ==============================================

-- 插入角色数据
INSERT INTO `roles` (`id`, `code`, `name`, `description`, `status`, `created_at`, `updated_at`) VALUES
('550e8400-e29b-41d4-a716-446655440002', 'super_admin', '超级管理员', '系统超级管理员，拥有所有权限', 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440003', 'normal_admin', '普通管理员', '普通管理员角色，拥有系统管理权限', 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440004', 'normal_user', '普通用户', '普通用户角色，基础权限', 1, NOW(), NOW());

-- 插入用户数据（密码：admin123）
INSERT INTO `users` (`id`, `username`, `password`, `email`, `phone`, `real_name`, `avatar`, `status`, `created_at`, `updated_at`) VALUES
('550e8400-e29b-41d4-a716-446655440001', 'admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5jZjyYx9vBmLK', 'admin@example.com', '13800138000', '系统管理员', NULL, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440008', 'manager', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5jZjyYx9vBmLK', 'manager@example.com', '13800138001', '普通管理员', NULL, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440009', 'user', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5jZjyYx9vBmLK', 'user@example.com', '13800138002', '普通用户', NULL, 1, NOW(), NOW());

-- 分配用户角色
INSERT INTO `user_roles` (`id`, `user_id`, `role_id`, `created_at`, `updated_at`) VALUES
('550e8400-e29b-41d4-a716-446655440030', '550e8400-e29b-41d4-a716-446655440001', '550e8400-e29b-41d4-a716-446655440002', NOW(), NOW()),  -- admin -> 超级管理员
('550e8400-e29b-41d4-a716-446655440031', '550e8400-e29b-41d4-a716-446655440008', '550e8400-e29b-41d4-a716-446655440003', NOW(), NOW()),  -- manager -> 普通管理员
('550e8400-e29b-41d4-a716-446655440032', '550e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440004', NOW(), NOW());  -- user -> 普通用户

-- 插入权限数据
INSERT INTO `permissions` (`id`, `name`, `code`, `type`, `parent_id`, `path`, `component`, `icon`, `sort`, `status`, `created_at`, `updated_at`) VALUES
-- 超级管理员控制台（顶级菜单）
('550e8400-e29b-41d4-a716-446655440010', '超级管理员', 'super_admin', 0, NULL, '/super-admin', NULL, 'crown', 0, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440011', '管理员仪表盘', 'super_admin:dashboard', 0, '550e8400-e29b-41d4-a716-446655440010', '/super-admin/dashboard', 'super-admin/dashboard', 'dashboard', 1, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440012', '系统监控', 'super_admin:monitor', 0, '550e8400-e29b-41d4-a716-446655440010', '/super-admin/monitor', 'super-admin/monitor', 'monitor', 2, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440013', '数据备份', 'super_admin:backup', 0, '550e8400-e29b-41d4-a716-446655440010', '/super-admin/backup', 'super-admin/backup', 'database', 3, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440014', '系统配置', 'super_admin:config', 0, '550e8400-e29b-41d4-a716-446655440010', '/super-admin/config', 'super-admin/config', 'setting', 4, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440015', '操作日志', 'super_admin:logs', 0, '550e8400-e29b-41d4-a716-446655440010', '/super-admin/logs', 'super-admin/logs', 'file-text', 5, 1, NOW(), NOW()),

-- 系统管理（三级菜单结构）
('550e8400-e29b-41d4-a716-446655440004', '系统管理', 'system', 0, NULL, '/system', NULL, 'setting', 1, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440016', '用户管理', 'user_management', 0, '550e8400-e29b-41d4-a716-446655440004', '/system/user-management', NULL, 'user', 1, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440005', '用户列表', 'user:list', 0, '550e8400-e29b-41d4-a716-446655440016', '/system/user-management/users', 'system/users', 'user', 1, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440006', '角色管理', 'role:list', 0, '550e8400-e29b-41d4-a716-446655440016', '/system/user-management/roles', 'system/roles', 'team', 2, 1, NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440007', '权限管理', 'permission:list', 0, '550e8400-e29b-41d4-a716-446655440016', '/system/user-management/permissions', 'system/permissions', 'lock', 3, 1, NOW(), NOW());

-- 分配超级管理员权限（所有权限）
INSERT INTO `role_permissions` (`id`, `role_id`, `permission_id`, `created_at`, `updated_at`) VALUES
-- 超级管理员控制台权限
('550e8400-e29b-41d4-a716-446655440020', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440010', NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440021', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440011', NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440022', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440012', NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440023', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440013', NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440024', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440014', NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440025', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440015', NOW(), NOW()),

-- 系统管理权限
('550e8400-e29b-41d4-a716-446655440009', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440004', NOW(), NOW()),
('550e8400-e29b-41d4-a716-446655440060', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440016', NOW(), NOW()),  -- 用户管理（二级菜单）
('550e8400-e29b-41d4-a716-44665544000a', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440005', NOW(), NOW()),
('550e8400-e29b-41d4-a716-44665544000b', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440006', NOW(), NOW()),
('550e8400-e29b-41d4-a716-44665544000c', '550e8400-e29b-41d4-a716-446655440002', '550e8400-e29b-41d4-a716-446655440007', NOW(), NOW()),

-- 分配普通管理员权限（系统管理权限，但不包括超级管理员控制台）
('550e8400-e29b-41d4-a716-446655440040', '550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440004', NOW(), NOW()),  -- 系统管理
('550e8400-e29b-41d4-a716-446655440061', '550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440016', NOW(), NOW()),  -- 用户管理（二级菜单）
('550e8400-e29b-41d4-a716-446655440041', '550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440005', NOW(), NOW()),  -- 用户列表
('550e8400-e29b-41d4-a716-446655440042', '550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440006', NOW(), NOW()),  -- 角色管理
('550e8400-e29b-41d4-a716-446655440043', '550e8400-e29b-41d4-a716-446655440003', '550e8400-e29b-41d4-a716-446655440007', NOW(), NOW()),  -- 权限管理

-- 分配普通用户权限（只有基础查看权限）
('550e8400-e29b-41d4-a716-446655440050', '550e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440004', NOW(), NOW()),  -- 系统管理（查看）
('550e8400-e29b-41d4-a716-446655440062', '550e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440016', NOW(), NOW()),  -- 用户管理（二级菜单）（查看）
('550e8400-e29b-41d4-a716-446655440051', '550e8400-e29b-41d4-a716-446655440004', '550e8400-e29b-41d4-a716-446655440005', NOW(), NOW());  -- 用户列表（查看）

-- 默认密码配置
INSERT INTO `system_settings` (`id`, `key`, `value`, `description`) VALUES
('550e8400-e29b-41d4-a716-446655440099', 'default_password', '123456', '用户重置密码默认值');

-- ==============================================
-- 重置外键检查
-- ==============================================

SET FOREIGN_KEY_CHECKS = 1;

-- ==============================================
-- 初始化完成
-- ==============================================
-- 默认用户账号：
-- 超级管理员: admin / admin123     （拥有所有权限，包括超级管理员控制台）
-- 普通管理员: manager / admin123   （拥有系统管理权限，不包括超级管理员控制台）
-- 普通用户:   user / admin123      （只有基础查看权限）
-- ============================================== 