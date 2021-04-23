use async_graphql::*;

/// 定义查询根节点
pub struct Query;

#[Object(extends)]
impl Query {

    /// 临时方法
    async fn add(&self, a: i32, b: i32) -> i32 {
        log::info!("add(a:1, b:2)");
        a + b
    }
}