use actix_web::{get, post, route, HttpRequest, HttpResponse, Responder};

use serde_json::Value;

use crate::services::model::{capitalize, JsonResponse};

#[get("/liveness")]
pub async fn liveness(req: HttpRequest) -> impl Responder {
    println!("Calling GET {}", req.path());
    HttpResponse::Ok().body("Liveness probe")
}

#[get("/readiness")]
pub async fn readiness(req: HttpRequest) -> impl Responder {
    println!("Calling GET {}", req.path());
    HttpResponse::Ok().body("Readiness probe")
}

pub async fn manual_hello(req: HttpRequest) -> impl Responder {
    println!("Calling GET {}", req.path());
    HttpResponse::Ok().body("Hey there!")
}

#[route("/echo", method="GET", method="POST")]
#[tracing::instrument]
async fn echo_multiple_method(req_body: String, req: HttpRequest) -> impl Responder {
    println!("Calling GET {}", req.path());
    tracing::info!("Calling GET {}", req.path());
    HttpResponse::Ok().body(req_body)
}

#[post("/anything{value:.*}")]
pub async fn echo_post(req_body: String, req: HttpRequest) -> impl Responder {
    echo(req_body, req).await
}

#[get("/anything{value:.*}")]
pub async fn echo_get(req_body: String, req: HttpRequest) -> impl Responder {
    echo(req_body, req).await
}

// httpbin response
// {
//     "args": {},
//     "data": "",
//     "files": {},
//     "form": {},
//     "headers": {
//         "Accept": "application/json",
//         "Accept-Encoding": "gzip, deflate, br",
//         "Accept-Language": "it,it-IT;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6",
//         "Content-Length": "0",
//         "Host": "httpbin.org",
//         "Origin": "https://httpbin.org",
//         "Referer": "https://httpbin.org/",
//         "Sec-Ch-Ua": "\"Chromium\";v=\"104\", \" Not A;Brand\";v=\"99\", \"Microsoft Edge\";v=\"104\"",
//         "Sec-Ch-Ua-Mobile": "?0",
//         "Sec-Ch-Ua-Platform": "\"Windows\"",
//         "Sec-Fetch-Dest": "empty",
//         "Sec-Fetch-Mode": "cors",
//         "Sec-Fetch-Site": "same-origin",
//         "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.81 Safari/537.36 Edg/104.0.1293.54",
//         "X-Amzn-Trace-Id": "Root=1-646f796d-7dcc40a90c71ff3c49839a3a"
//     },
//     "json": null,
//     "method": "POST",
//     "origin": "15.160.249.205",
//     "url": "https://httpbin.org/anything"
// }

// needed to quiet 'headers_json' and 'json_resp' warning that switch on because in the next 'match' pattern
// it stops execution with return 400 and so these 2 values could not be used
#[allow(unused_assignments)]
async fn echo(req_body: String, req: HttpRequest) -> impl Responder {
    println!("Calling POST {}", req.path());

    // get all headers and set them as tuple with index to check later if it's the last element
    let head = req.headers()
    .into_iter()
    .enumerate()
    .map(|(index, (name, value))| {
        let is_last = index == req.headers().len() - 1;
        (name, value, is_last)
    });
    
    // extract headers name and value and format them, even based on their index, to allow below json serialization
    let headers = head.map(|(head_name, head_val, is_last)| {
        if is_last {
            // not add the final ',' otherwise the serde_json::from_str will give error
            format!(
                "\"{}\": \"{}\" ", 
                capitalize(head_name.as_str()).as_str(), 
                head_val.to_str().unwrap_or(""),
            )
        } else {
            // add the final ',' so every key-value pair has divided correctly
            format!(
                "\"{}\": \"{}\", ", 
                capitalize(head_name.as_str()).as_str(), 
                head_val.to_str().unwrap_or(""),
            )
        }
    })
    .collect::<Vec<_>>()
    .join("\n");

    // parsed all headers as json
    let parsed_headers: Result<Value, serde_json::Error> = serde_json::from_str(format!("{{ {} }}", headers).as_str());
    let mut headers_json = Value::Null;

    match parsed_headers {
        Ok(json_value) => {
            headers_json = json_value;
        }
        Err(val) => {
            return HttpResponse::BadRequest().body(format!("Invalid JSON headers {val}"))
        },
    }

    // parsed request body as json
    let parsed_json: Result<Value, serde_json::Error> = serde_json::from_str(req_body.as_str());
    let mut json_resp = Value::Null;

    match parsed_json {
        Ok(json_value) => {
            json_resp = json_value;
        }
        Err(val) => {
            return HttpResponse::BadRequest().body(format!("Invalid JSON body {val}"))
        },
    }

    // prepare the response
    let resp = JsonResponse::new(
        req.query_string().to_string(),
        req_body,
        "".to_string(),
        "".to_string(),
        headers_json,
        json_resp,
        req.method().to_string(),
        req.peer_addr().unwrap().to_string(),
        req.uri().to_string(),
    );

    // response with 200 and json body
    // HttpResponse::Ok().json(web::Json(resp)) // for json response but not pretty formatted
    HttpResponse::Ok().body(serde_json::to_string_pretty(&resp).unwrap())
}