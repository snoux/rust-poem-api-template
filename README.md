# rust-poem-api-template

## 项目概述

这是一个基于 Rust 和 Poem 框架构建的后端 API 模板项目，旨在提供模块化的 API 开发结构与参考示例，帮助开发者更高效地进行服务端开发。

作为一名前端开发者，我曾尝试使用 Python 编写后端服务，但出于对性能和类型安全的考虑，最终选择学习 Rust。为了降低学习曲线并加快开发效率，我构建了这个模板项目，力求做到开箱即用，具备以下特点：

- ✅ 基于 Poem 框架，结构清晰，易于扩展  
- ✅ 支持模块化的 API 开发模式  
- ✅ 自动生成 API 文档（使用 OpenAPI 标准）  
- ✅ 面向学习者和小团队友好，便于快速上手  

欢迎大家提出宝贵意见或建议。如有任何问题或改进建议，欢迎联系我交流！

---

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

---

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
    运行前请将`src/api/mod.rs`中的`{{ doc_title }}`及`{{ project_description }}`修改为自己的标题及描述。
    将`src/main.rs`中的`{{crate_name}}`修改为自己的项目名称
    
```bash
cargo run
```

3. 访问 API 文档

浏览器打开：

- 📘 Swagger UI：[http://localhost:3000/api/docs](http://localhost:3000/api/docs)
- 📄 OpenAPI JSON：[http://localhost:3000/api/docs/json](http://localhost:3000/api/docs/json)

![文档界面](https://github.com/user-attachments/assets/249385a9-ee50-4473-8ce3-46013e52b528)

---

## 通过 cargo-generate 创建项目

如果你想基于本模板快速创建自己的新项目，推荐使用 [`cargo-generate`](https://github.com/cargo-generate)。

### 安装 cargo-generate（如未安装）

```bash
cargo install cargo-generate
```

### 以 hello_world 为例创建项目

```bash
cargo generate --git https://github.com/snow-xf/rust-poem-api-template
```

按照要求输入项目信息:
![image](https://github.com/user-attachments/assets/965133a6-c7c2-40fd-a942-7fdca63438e0)


然后进入目录：

```bash
cd hello_world
cargo run
```

访问文档：

- [http://localhost:3000/api/docs](http://localhost:3000/api/docs)

---

## API接口示例

### 用户管理模块

- `GET /api/users` - 获取用户列表  
- `POST /api/users` - 创建新用户  
- `GET /api/users/:id` - 获取用户详情  
- `PUT /api/users/:id` - 更新用户信息  
- `DELETE /api/users/:id` - 删除用户  

---

## 基础扩展示例

### 添加新功能域

1. 创建目录结构:

```bash
mkdir -p src/api/test
touch src/api/test/{controller,dto,mod}.rs
```

2. 在 `api/mod.rs` 中添加:

```rust
pub mod test;

// 在 create_api_service 函数中添加：
OpenApiService::new(
    (
        user::UserController::default(),
        test::TestController::default(), // 新增
    ),
    // ...
)
```

3. 在 `controller.rs` 添加新接口：

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

---

### 配置 API 分类（tags）

在 `src/config/tags.rs` 中定义新分类：

```rust
pub enum ApiTags {
    /// 用户模块
    User,
    /// 测试模块
    Test,
}
```

然后在接口中指定 tag：

```rust
#[oai(path = "/test", method = "get", tag = ApiTags::Test)]
```

---

## 联系方式

- 项目地址: [https://github.com/snow-xf/rust-poem-api-template](https://github.com/snow-xf/rust-poem-api-template)
- 邮箱: [livefei@live.com](mailto:livefei@live.com)

---

如需进一步自定义或添加功能模块，欢迎 Issue / PR / Star！🎯
