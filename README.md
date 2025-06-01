# 接口文档

## 项目概述

这是一个基于Rust和Poem框架的后端API模板项目，提供了模块化的API开发结构和示例。

本人原本是做前端开发，也尝试使用Python做后端开发，考虑到性能问题最终决定学习使用Rust，为方便快速上手故决定做一个Rust模板。我的构想是能够直接上手，开箱即用，模板可以自动生成api文档并且未来有一定的扩展性。

由于本人不是后端工程师，纯属自学，如有问题及修改建议欢迎联系！

## 快速开始

### 环境要求

- Rust 1.70.0+
- Cargo

### 安装与运行

1. 克隆项目

```bash
git clone https://github.com/snow-xf/rust-poem-api-template
cd rust-poem-api-template
```

2. 编译并运行

```bash
cargo run
```

3. 访问API文档

打开浏览器访问: [http://localhost:3000/api/docs](http://localhost:3000/api/docs)

OpenAPI规范JSON:[http://localhost:3000/api/docs/json](http://localhost:3000/api/docs/json)

## API接口

### 用户管理

- `GET /api/users` - 获取用户列表
- `POST /api/users` - 创建新用户
- `GET /api/users/:id` - 获取用户详情
- `PUT /api/users/:id` - 更新用户信息
- `DELETE /api/users/:id` - 删除用户

### 系统管理

- `GET /api/system/status` - 获取系统状态
- `GET /api/system/config` - 获取系统配置

## 项目结构

```
src/
├── api/            # API 接口定义
│   ├── mod.rs      # API 模块聚合
│   ├── user/       # 用户功能域
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   └── dto.rs
│   └── system/     # 系统功能域
│       ├── mod.rs
│       ├── controller.rs
│       └── dto.rs
├── config/         # 配置管理
├── middlewares/    # 中间件
├── models/         # 数据模型
├── services/       # 业务逻辑服务
├── utils/          # 工具函数
├── lib.rs          # 库入口
└── main.rs         # 应用入口
```

## 基础扩展示例

### 添加新API端点

在`controller.rs`中添加新端点:

```rust
/// 获取用户统计
/// 
/// 获取系统用户统计信息
#[oai(path = "/users/stats", method = "get")]
async fn get_user_stats(&self) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let stats = serde_json::json!({
        "total": 100,
        "active": 85,
        "new_today": 5
    });
    
    success_json(stats)
}
```

### 添加新功能域

1. 创建目录结构:

```bash
mkdir -p src/api/product/{controller.rs,dto.rs,mod.rs}
```

2. 在`api/mod.rs`中添加:

```rust
pub mod product;

// 在create_api_service函数中添加
OpenApiService::new(
    (
        user::UserController::default(),
        system::SystemController::default(),
        product::ProductController::default(), // 新增
    ),
    // ...
)
```

## 许可证

MIT

## 联系方式

- 项目地址: [https://github.com/snow-xf/rust-poem-api-template](https://github.com/snow-xf/rust-poem-api-template)
