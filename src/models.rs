#[derive(serde::Serialize, serde::Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub datetime: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GroupChatMsg {
    pub id: String,
    pub group_id: String,
    pub datetime: String,
    pub message: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PersonalChatMsg {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub datetime: String,
    pub message: String,
}
