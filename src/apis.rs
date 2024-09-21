use crate::crud;
use crate::models;
use crate::AppState;

#[allow(dead_code)]
pub async fn get_user_list(req: tide::Request<AppState>) -> tide::Result<tide::Body> {
    let res = crud::fetch_all_users(&req.state().pool).await?;
    return Ok(tide::Body::from_json(&res)?);
}

#[allow(dead_code)]
pub async fn post_user(mut req: tide::Request<AppState>) -> tide::Result<String> {
    let body: models::Usr = req.body_json().await?;
    dbg!(&body);
    let res = crud::insert_users(&req.state().pool, body.username, body.password).await?;
    return Ok(format!("{:#?}", res));
}
