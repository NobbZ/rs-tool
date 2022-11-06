use std::io::Result as IoResult;

use actix_web::{web, App, HttpServer};
use rs_tool::{echo, hello, manual_hello};

#[actix_web::main]
async fn main() -> IoResult<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}
