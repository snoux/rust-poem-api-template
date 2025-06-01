use poem_openapi::Tags;

#[derive(Tags)]
// 对api进行分类
pub enum ApiTags {
    /// 用户模块
    User,
}