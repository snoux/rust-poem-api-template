use poem_openapi::payload::Json;
use poem_openapi::{Object, types::Type, types::ToJSON, types::ParseFromJSON};
use serde::{Deserialize, Serialize};

/// 统一API响应结构体
/// 用于封装所有接口的返回数据
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct ApiResponse<T: Send + Sync + Serialize + Type + ToJSON + ParseFromJSON> {
    /// 状态码，例如 200 表示成功，400 表示客户端错误，500 表示服务器错误
    #[oai(validator(minimum(value = "100"), maximum(value = "599")))]
    pub code: u16,
    /// 响应消息，通常在出错时提供额外信息
    pub msg: String,
    /// 实际响应数据，可以是任意实现了 Serialize 和 poem_openapi::types::Type 的类型
    pub data: Option<T>,
}

impl<T: Send + Sync + Serialize + Type + ToJSON + ParseFromJSON> ApiResponse<T> {
    /// 创建一个成功的响应
    ///
    /// # Arguments
    ///
    /// * `data` - 成功时返回的数据
    ///
    /// # Returns
    ///
    /// 包含成功状态码和数据的 `ApiResponse`
    pub fn success(data: T) -> Self {
        ApiResponse {
            code: 200,
            msg: "Success".to_string(),
            data: Some(data),
        }
    }

    /// 创建一个成功的响应（无数据）
    ///
    /// # Returns
    ///
    /// 包含成功状态码但无数据的 `ApiResponse`
    pub fn success_nodata() -> Self {
        ApiResponse {
            code: 200,
            msg: "Success".to_string(),
            data: None,
        }
    }

    /// 创建一个失败的响应
    ///
    /// # Arguments
    ///
    /// * `code` - 错误状态码
    /// * `msg` - 错误消息
    ///
    /// # Returns
    ///
    /// 包含错误状态码和消息的 `ApiResponse`
    pub fn error(code: u16, msg: String) -> Self {
        ApiResponse {
            code,
            msg,
            data: None,
        }
    }

    /// 将 `ApiResponse` 转换为 `poem::Result<Json<Self>>`
    /// 便于在 Poem 路由处理函数中直接返回
    ///
    /// # Returns
    ///
    /// Poem 框架能识别的 Result 类型
    pub fn to_json_result(self) -> poem::Result<Json<Self>> {
        Ok(Json(self))
    }
}

/// 创建一个表示成功的 `poem::Result<Json<ApiResponse<T>>>`
///
/// # Arguments
///
/// * `data` - 成功时返回的数据
///
/// # Returns
///
/// 包含成功响应的 `poem::Result`
pub fn success_json<T: Send + Sync + Serialize + Type + ToJSON + ParseFromJSON>(data: T) -> poem::Result<Json<ApiResponse<T>>> {
    ApiResponse::success(data).to_json_result()
}

/// 创建一个表示成功的 `poem::Result<Json<ApiResponse<T>>>` (无数据)
///
/// # Returns
///
/// 包含成功响应（无数据）的 `poem::Result`
pub fn success_nodata_json<T: Send + Sync + Serialize + Type + ToJSON + ParseFromJSON>() -> poem::Result<Json<ApiResponse<T>>> {
    ApiResponse::<T>::success_nodata().to_json_result()
}

/// 创建一个表示失败的 `poem::Result<Json<ApiResponse<T>>>`
///
/// # Arguments
///
/// * `code` - 错误状态码
/// * `msg` - 错误消息
///
/// # Returns
///
/// 包含失败响应的 `poem::Result`
pub fn error_json<T: Send + Sync + Serialize + Type + ToJSON + ParseFromJSON>(code: u16, msg: String) -> poem::Result<Json<ApiResponse<T>>> {
    ApiResponse::<T>::error(code, msg).to_json_result()
}

/// 空响应类型，用于不需要返回数据的API
#[derive(Debug, Serialize, Deserialize, Object)]
pub struct EmptyResponse {}

/// 创建一个空响应实例
pub fn empty() -> EmptyResponse {
    EmptyResponse {}
}
