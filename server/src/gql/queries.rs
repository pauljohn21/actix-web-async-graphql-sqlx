/// 定义查询根节点
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        log::info!("add(a:1, b:2)");
        a + b
    }
}