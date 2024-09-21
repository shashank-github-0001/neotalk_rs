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
    let models::Usr { username, password } = req.body_json().await?;
    let res = crud::insert_users(&req.state().pool, username, password).await?;
    return Ok(format!("{:#?}", res));
}

#[allow(dead_code)]
pub async fn post_group(mut req: tide::Request<AppState>) -> tide::Result<String> {
    let models::Grp { name } = req.body_json().await?;
    dbg!(&name);
    let res = crud::insert_groups(&req.state().pool, name).await?;
    return Ok(format!("{:#?}", res));
}
