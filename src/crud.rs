use crate::models;

// NOTE: this one is just to check if the tables are created make this part of the init run plan
// that happens whenever you first use this app and shit like that

#[allow(dead_code)]
pub async fn init_db(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    let create_user = sqlx::query(
        "create table if not exists user(id text primary key, username text, password text)",
    );
    let create_group = sqlx::query(
        "create table if not exists group(id text primary key, name text, datetime text)",
    );
    let create_group_chat = sqlx::query("create table if not exists group_chat_msg(id text primary key, group_id text, datetime text, message text, foreign key(group_id) references group(id) on delete cascade on update cascade");
    let create_personal_chat = sqlx::query("create table if not exists personal_chat_msg(from_id text, to_id text, datetime text, message text, foreign key(from_id) references user(id) on delete cascade on update cascade, foreign key(to_id) references users(id) on delete cascade on update cascade);");
    create_user.execute(pool).await?;
    create_group.execute(pool).await?;
    create_group_chat.execute(pool).await?;
    create_personal_chat.execute(pool).await?;
    return Ok(());
}

// NOTE: this is create queries i should make this for all 4 tables

#[allow(dead_code)]
pub async fn insert_user(
    pool: &sqlx::Pool<sqlx::Postgres>,
    username: String,
    password: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let new_user = models::User {
        id: sqlx::types::uuid::Uuid::new_v4().to_string(),
        username,
        password,
    };
    let insert_user = sqlx::query("INSERT INTO user (id, username, password) VALUES ($1, $2, $3)")
        .bind(new_user.id)
        .bind(new_user.username)
        .bind(new_user.password)
        .execute(pool)
        .await?;
    return Ok(insert_user);
}

#[allow(dead_code)]
pub async fn insert_group(
    pool: &sqlx::Pool<sqlx::Postgres>,
    group_name: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let new_group = models::Group {
        id: sqlx::types::uuid::Uuid::new_v4().to_string(),
        name: group_name,
        datetime: chrono::Utc::now().to_string(),
    };
    let insert_group = sqlx::query("INSERT INTO user (id, username, password) VALUES ($1, $2, $3)")
        .bind(new_group.id)
        .bind(new_group.name)
        .bind(new_group.datetime)
        .execute(pool)
        .await?;
    return Ok(insert_group);
}

#[allow(dead_code)]
pub async fn insert_group_chat(
    pool: &sqlx::Pool<sqlx::Postgres>,
    group_id: String,
    message: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let group_chat = models::GroupChatMsg {
        id: uuid::Uuid::new_v4().to_string(),
        group_id,
        datetime: chrono::Utc::now().to_string(),
        message,
    };

    let res = sqlx::query("insert into group_chat_msg values ($1, $2, $3, $4)")
        .bind(group_chat.id)
        .bind(group_chat.group_id)
        .bind(group_chat.datetime)
        .bind(group_chat.message)
        .execute(pool)
        .await?;

    return Ok(res);
}

#[allow(dead_code)]
pub async fn insert_personal_chat(
    pool: &sqlx::Pool<sqlx::Postgres>,
    from_id: String,
    to_id: String,
    message: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let group_chat = models::PersonalChatMsg {
        id: uuid::Uuid::new_v4().to_string(),
        from_id,
        to_id,
        datetime: chrono::Utc::now().to_string(),
        message,
    };

    let res = sqlx::query("insert into group_chat_msg values ($1, $2, $3, $4)")
        .bind(group_chat.id)
        .bind(group_chat.from_id)
        .bind(group_chat.to_id)
        .bind(group_chat.datetime)
        .bind(group_chat.message)
        .execute(pool)
        .await?;

    return Ok(res);
}

// NOTE: this is select queries i should make these for all 4 tables

#[allow(dead_code)]
pub async fn fetch_one_user_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
    let fetched_user = sqlx::query("select * from users where id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    return Ok(fetched_user);
}

#[allow(dead_code)]
pub async fn fetch_all_users_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let fetched_users = sqlx::query("select * from users where id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    return Ok(fetched_users);
}

#[allow(dead_code)]
pub async fn fetch_all_users(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let all_users = sqlx::query("select * from users")
        .fetch_all(pool)
        .await?;
    return Ok(all_users);
}

#[allow(dead_code)]
pub async fn fetch_all_groups(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let all_groups = sqlx::query("select * from groups")
        .fetch_all(pool)
        .await?;
    return Ok(all_groups);
}

#[allow(dead_code)]
pub async fn fetch_all_group_chat_msg_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let fetched_group_chat_msgs = sqlx::query("select * from group_chat_msg where group_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    return Ok(fetched_group_chat_msgs);
}

#[allow(dead_code)]
pub async fn fetch_all_personal_chat_msg_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let fetched_personal_chat_msgs = sqlx::query("select * from personal_chat_msg where from_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    return Ok(fetched_personal_chat_msgs);
}
