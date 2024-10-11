use actix_web_opentelemetry::RequestMetrics;
use actix_web::dev::ServiceRequest;
use opentelemetry::{sdk::propagation::TraceContextPropagator, global};
use tracing_subscriber::Registry;
use tracing_subscriber::prelude::*;

use crate::configuration::config::Configuration;

pub fn init_observability(configuration: &Configuration) -> RequestMetrics<impl Fn(&ServiceRequest) -> bool + Send + Clone> {
    // Start an (optional) otel prometheus metrics pipeline
    let metrics_exporter = opentelemetry_prometheus::exporter().init();
    let request_metrics = actix_web_opentelemetry::RequestMetrics::new(
        opentelemetry::global::meter("actix_http_tracing"),
        Some(|req: &actix_web::dev::ServiceRequest| {
            req.path() == "/metrics" && req.method() == actix_web::http::Method::GET
        }),
        Some(metrics_exporter),
    );

    // Start an otel jaeger trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(&configuration.config.app_name)
        .install_simple()
        .unwrap();

    // Initialize `tracing` using `opentelemetry-tracing` and configure logging
    Registry::default()
        .with(tracing_subscriber::EnvFilter::new(&configuration.log.level))
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    request_metrics
}