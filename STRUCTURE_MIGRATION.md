# API 模块结构迁移说明

## 结构优化概述

本次结构优化将原有的扁平式 API 文件组织调整为按功能域（Domain）分组的文件夹结构，提高了代码的组织性、可维护性和可扩展性。

## 新旧结构对比

### 旧结构

```
src/
├── api/
│   ├── mod.rs           # API 模块聚合
│   ├── user.rs          # 用户相关 API 实现
│   ├── user_management.rs # 用户管理 API 实现
│   └── system_management.rs # 系统管理 API 实现
```

### 新结构

```
src/
├── api/
│   ├── mod.rs           # API 模块聚合
│   ├── user/            # 用户功能域
│   │   ├── mod.rs       # 用户模块聚合
│   │   ├── controller.rs # 用户 API 控制器
│   │   └── dto.rs       # 用户数据传输对象
│   └── system/          # 系统功能域
│       ├── mod.rs       # 系统模块聚合
│       ├── controller.rs # 系统 API 控制器
│       └── dto.rs       # 系统数据传输对象
```

## 迁移动机

1. **关注点分离**：将 API 定义、数据传输对象和业务逻辑分离，使代码结构更清晰
2. **可维护性提升**：避免单个文件过大，便于团队协作和代码审查
3. **可扩展性增强**：新增功能域只需添加新文件夹，不影响现有代码
4. **更好的测试性**：组件职责单一，便于单元测试和模拟依赖
5. **符合行业最佳实践**：采用类似领域驱动设计（DDD）的结构，更符合大型项目需求

## 目录和文件说明

### api/mod.rs

API 模块的入口点，负责聚合和导出所有功能域的 API 控制器，并提供 OpenAPI 服务创建函数。

### api/[domain]/mod.rs

功能域模块的入口点，负责聚合和导出该功能域的控制器和 DTO。

### api/[domain]/controller.rs

功能域的 API 控制器，包含所有 API 路由定义和请求处理逻辑。

### api/[domain]/dto.rs

功能域的数据传输对象（Data Transfer Objects），定义 API 请求和响应的数据结构。

## 未来扩展建议

1. **添加新功能域**

   要添加新的功能域（如订单管理），只需创建新的文件夹结构：

   ```
   src/api/order/
   ├── mod.rs
   ├── controller.rs
   └── dto.rs
   ```

   然后在 `api/mod.rs` 中添加引用和聚合：

   ```rust
   pub mod order;
   
   // 在 create_api_service 函数中添加
   OpenApiService::new(
       (
           user::UserController::default(),
           system::SystemController::default(),
           order::OrderController::default(), // 新增的控制器
       ),
       // ...
   )
   ```

2. **添加服务层**

   对于复杂业务逻辑，建议在功能域文件夹中添加 `service.rs`：

   ```
   src/api/user/
   ├── mod.rs
   ├── controller.rs
   ├── dto.rs
   └── service.rs  # 处理业务逻辑
   ```

3. **添加验证层**

   对于复杂的请求验证逻辑，可以添加 `validator.rs`：

   ```
   src/api/user/
   ├── mod.rs
   ├── controller.rs
   ├── dto.rs
   ├── service.rs
   └── validator.rs  # 处理请求验证
   ```

## 结构优势总结

1. **模块化**：每个功能域独立封装，降低耦合
2. **可测试性**：职责单一的组件便于测试
3. **可维护性**：文件结构清晰，便于理解和修改
4. **可扩展性**：轻松添加新功能域和组件
5. **一致性**：统一的结构模式，便于团队协作
