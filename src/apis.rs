use crate::crud;
use crate::AppState;

#[allow(dead_code)]
pub async fn serve_user_list(req: tide::Request<AppState>) -> tide::Result<tide::Body> {
    let res = crud::fetch_all_users(&req.state().pool).await?;
    return Ok(tide::Body::from_json(&res)?);
}
