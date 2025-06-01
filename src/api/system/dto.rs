// 这里可以定义系统模块特有的DTO（数据传输对象）
// 目前我们直接使用serde_json::Value，所以这个文件暂时为空
// 如果有特定于API的请求/响应结构，可以在这里定义

// 例如，如果需要定义特定于API的系统配置请求：
// 
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SystemConfigRequest {
//     pub max_connections: Option<u32>,
//     pub timeout: Option<u32>,
//     pub debug_mode: Option<bool>,
//     pub log_level: Option<String>,
// }
