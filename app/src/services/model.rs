use serde::{Serialize};

#[derive(Serialize)]
pub struct JsonResponse {
    args: String,
    data: String,
    file: String,
    form: String,
    headers: serde_json::Value,
    json: serde_json::Value,
    method: String,
    origin: String,
    url: String,
}

// the below method defined for JsonResponse allow us to create
// this type of struct outside here and without have to define
// 'pub' every single struct field in addition to struct itself
impl JsonResponse {
    // A public constructor method
    pub fn new(
        args: String, data: String, file: String, form: String, 
        headers: serde_json::Value, json: serde_json::Value, 
        method: String, origin: String, url: String
    ) -> JsonResponse {
        JsonResponse {
            args, // it's not necessary to write 'args: args' cause Rust infers it
            data,
            file,
            form,
            headers,
            json,
            method,
            origin,
            url,
        }
    }
}

pub fn capitalize(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c.is_whitespace() || !c.is_alphabetic() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().to_string());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}