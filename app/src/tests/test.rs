// to run test without having to build all every time and to see println! output
// it's necessary to use this command: 'cargo test --bin app -- --nocapture'

use actix_web::{http::header::ContentType, test, App};

use crate::services::handler::echo_multiple_method;

// since our methods have macros above them to make Actix works correctly,
// we have to start a server to call these methods, cause they are not 
// simply functions but become struct for Actix (when using macros)
#[actix_web::test]
async fn test_index_get() {
    // start the server
    let app = test::init_service(
        App::new().service(echo_multiple_method)
    ).await;

    // prepare the request
    let req = test::TestRequest::get()
        .insert_header(ContentType::plaintext())
        .uri("/echo")
        .to_request();

    // make the call to endpoint
    let resp = test::call_service(&app, req).await;

    // check if response is 200 status code
    println!("{}", resp.status());
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_index_post() {
    let app = test::init_service(
        App::new().service(echo_multiple_method)
    ).await;

    let req = test::TestRequest::post().uri("/echo").to_request();

    let resp = test::call_service(&app, req).await;
    
    println!("{}", resp.status());
    assert!(resp.status().is_success());
}