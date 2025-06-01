# 使用说明

## 项目概述

这是一个基于 Rust 和 Poem 框架构建的后端 API 模板项目，旨在提供模块化的 API 开发结构与参考示例，帮助开发者更高效地进行服务端开发。

作为一名前端开发者，我曾尝试使用 Python 编写后端服务，但出于对性能和类型安全的考虑，最终选择学习 Rust。为了降低学习曲线并加快开发效率，我构建了这个模板项目，力求做到开箱即用，具备以下特点：
* ✅ 基于 Poem 框架，结构清晰，易于扩展
* ✅ 支持模块化的 API 开发模式
* ✅ 自动生成 API 文档（使用 OpenAPI 标准）
* ✅ 面向学习者和小团队友好，便于快速上手
	
欢迎大家提出宝贵意见或建议。如有任何问题或改进建议，欢迎联系我交流！



## 项目结构

```
src/
├── api/            # API 接口定义
│   ├── mod.rs      # API 模块聚合
│   ├── user/       # 用户功能域（示例）
│   │   ├── mod.rs
│   │   ├── controller.rs
│   │   └── dto.rs
├── config/         # 配置管理
├── middlewares/    # 中间件
├── models/         # 数据模型
├── services/       # 业务逻辑服务
├── utils/          # 工具函数
├── lib.rs          # 库入口
└── main.rs         # 应用入口
```


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

![文档界面](https://github.com/user-attachments/assets/249385a9-ee50-4473-8ce3-46013e52b528)


OpenAPI规范JSON:[http://localhost:3000/api/docs/json](http://localhost:3000/api/docs/json)

## API接口

### 用户管理

- `GET /api/users` - 获取用户列表
- `POST /api/users` - 创建新用户
- `GET /api/users/:id` - 获取用户详情
- `PUT /api/users/:id` - 更新用户信息
- `DELETE /api/users/:id` - 删除用户

## 基础扩展示例

### 添加新功能域

1. 创建目录结构:

```bash
mkdir -p src/api/test
touch touch src/api/test/{controller,dto,mod}.rs
```

2. 在`api/mod.rs`中添加:

```rust
pub mod test;

// 在create_api_service函数中添加
OpenApiService::new(
    (
        user::UserController::default(),
        test::TestController::default(), // 新增
    ),
    // ...
)
```

### 添加新API端点

在`controller.rs`中添加新端点:

```rust
/// 测试
///
/// 测试增加
#[oai(path = "/test", method = "get")]
async fn get_user_stats(&self) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let stats = serde_json::json!({
        "test": "这是一条测试数据",
    });
    success_json(stats)
}
```

## API分类

在`src/config/tags.rs`中添加新的分类，如`Test`:

```
pub enum ApiTags {
    /// 用户模块
    User,
    /// 测试
    Test
}
```


在`controller.rs`指定tag:

```rust
/// 测试
///
/// 测试增加
#[oai(path = "/test", method = "get" , tag = ApiTags::Test)]
async fn get_user_stats(&self) -> Result<Json<ApiResponse<serde_json::Value>>> {
    let stats = serde_json::json!({
        "test": "这是一条测试数据",
    });
    success_json(stats)
}
```


![最终结果](https://github.com/user-attachments/assets/466383b8-300a-4fc0-b929-770a8e103f8d)


## 联系方式

- 项目地址: [https://github.com/snow-xf/rust-poem-api-template](https://github.com/snow-xf/rust-poem-api-template)
- Email: [livefei@live.com](mailto:livefei@live.com)
