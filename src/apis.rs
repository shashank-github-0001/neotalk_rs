use crate::crud;
use crate::models;
use crate::AppState;
use serde_json::json;

#[allow(dead_code)]
pub async fn get_user_list(req: tide::Request<AppState>) -> tide::Result<tide::Body> {
    let res = crud::fetch_all_users(&req.state().pool).await?;
    return Ok(tide::Body::from_json(&res)?);
}

#[allow(dead_code)]
pub async fn post_user(mut req: tide::Request<AppState>) -> tide::Result {
    let body_bytes = req.body_bytes().await?;
    eprintln!(
        "Raw request body: {:?}",
        String::from_utf8_lossy(&body_bytes)
    );

    let parse_result: Result<models::Usr, serde_json::Error> = serde_json::from_slice(&body_bytes);

    match parse_result {
        Ok(body) => {
            eprintln!("Parsed body: {:?}", body);

            match crud::insert_users(&req.state().pool, body.username, body.password).await {
                Ok(res) => {
                    eprintln!("Insert successful: {:?}", res);
                    Ok(tide::Response::builder(200)
                        .content_type(tide::http::mime::JSON)
                        .body(json!({ "status": "success", "message": "User created" }))
                        .build())
                }
                Err(e) => {
                    eprintln!("Database error: {:?}", e);
                    Ok(tide::Response::builder(500)
                        .content_type(tide::http::mime::JSON)
                        .body(json!({ "status": "error", "message": format!("Database error: {}", e) }))
                        .build())
                }
            }
        }
        Err(e) => {
            eprintln!("JSON parsing error: {:?}", e);
            Ok(tide::Response::builder(422)
                .content_type(tide::http::mime::JSON)
                .body(json!({ "status": "error", "message": format!("Invalid JSON: {}", e) }))
                .build())
        }
    }
}
