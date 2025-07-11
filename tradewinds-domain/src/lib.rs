//! 领域层
//!
//! 领域层是应用程序的核心，负责业务逻辑的实现。
//!
//! 领域层包含以下几个模块：
//!
//! - entities: 实体层，负责业务实体的定义。
//! - repositories: 仓储层，负责业务实体的持久化。
//! - services: 服务层，负责业务逻辑的实现以及业务接口的定义。
//! - value_objects: 值对象层，负责业务对象的属性。
//! - specifications: 规范层，负责业务对象的规范。
//! - aggregates: 聚合层，负责业务对象的聚合。
//!
//! 领域层的设计原则是：
//!
//! - 实体层：实体是业务对象的抽象，具有唯一标识和属性。
//! - 仓储层：仓储是实体的持久化接口，负责与数据库的交互。
//! - 服务层：服务是业务逻辑的实现，负责业务流程的控制。
//! - 值对象层：值对象是业务对象的属性，具有唯一标识和属性。
//! - 规范层：规范是业务对象的规范，负责业务对象的规范。
//! - 聚合层：聚合是业务对象的聚合，负责业务对象的聚合。
pub mod aggregates;
pub mod entities;
pub mod events;
pub mod factories;
pub mod policies;
pub mod repositories;
pub mod services;
pub mod specifications;
pub mod value_objects;
