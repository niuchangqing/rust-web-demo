use std::ops::Add;

use salvo::prelude::*;
use tracing::Level;

use rust_web_demo::controller;
use rust_web_demo::domain::response::ResultVo;
use rust_web_demo::service::SERVICE;
use rust_web_demo::error::CustomError;

#[tokio::main]
async fn main() {
    //初始化
    let address = String::from("127.0.0.1:").add(SERVICE.config.port.as_str());
    log::info!(">>>>>>>>>> server address {}", address);
    let router = Router::new()
        .push(
            Router::with_path("/user")
                .get(controller::user_controller::get_user)
                .post(controller::user_controller::add_user)
                .push(
                    Router::with_path("/file")
                        .get(controller::user_controller::test_error),
                ),
        )
        ;
    Server::new(TcpListener::bind(address.as_str())).serve(router).await;
}