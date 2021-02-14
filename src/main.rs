pub(crate) mod backend;
pub(crate) mod common;
pub(crate) mod gateway;
pub(crate) mod store;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    gateway::start().await
}
