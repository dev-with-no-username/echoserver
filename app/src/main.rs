use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web_opentelemetry::RequestTracing;

mod configuration;
mod services;
mod tests; // needed to make cargo recognizes tests
mod observability;

use crate::{
    configuration::config::get_configuration, 
    services::handler::{
        echo_post, echo_get, echo_multiple_method, liveness, readiness, manual_hello
    }, observability::observability::init_observability
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration();
    println!("Listening on http://0.0.0.0:{}", configuration.config.port);

    let request_metrics = init_observability(&configuration);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .wrap(request_metrics.clone())
            .service(echo_post)
            .service(echo_get)
            .service(echo_multiple_method)
            .service(liveness)
            .service(readiness)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", configuration.config.port as u16))?
    .run()
    .await
}
