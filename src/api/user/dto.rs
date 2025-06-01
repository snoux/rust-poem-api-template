// 这里可以定义用户模块特有的DTO（数据传输对象）
// 目前我们直接使用models/user.rs中的结构，所以这个文件暂时为空
// 如果有特定于API的请求/响应结构，可以在这里定义

// 例如，如果需要定义特定于API的用户搜索请求：
// 
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserSearchRequest {
//     pub username_contains: Option<String>,
//     pub email_contains: Option<String>,
//     pub created_after: Option<String>,
//     pub created_before: Option<String>,
// }
