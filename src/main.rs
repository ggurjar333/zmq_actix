use actix_web::{web, App, HttpServer, Responder};
use crate::zmq::{start_zmq_listener, send_zmq_message};

mod zmq;


async fn index() -> impl Responder {
    send_zmq_message("Hello from Actix!");
    "Message sent to ZeroMQ"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_zmq_listener();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
