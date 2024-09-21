use crate::models;

// NOTE: this one is just to check if the tables are created make this part of the init run plan
// that happens whenever you first use this app and shit like that

#[allow(dead_code)]
pub async fn init_db(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<(), sqlx::Error> {
    let create_users = sqlx::query(
        "create table if not exists users(id text primary key, username text, password text)",
    );
    let create_groups = sqlx::query(
        "create table if not exists groups(id text primary key, name text, datetime text)",
    );
    let create_groups_chat = sqlx::query("create table if not exists group_chat_msg(id text primary key, group_id text, datetime text, message text, foreign key (group_id) references groups(id) on delete cascade on update cascade);");
    let create_personal_chat = sqlx::query("create table if not exists personal_chat_msg(id text primary key, from_id text, to_id text, datetime text, message text, foreign key (from_id) references users(id) on delete cascade on update cascade, foreign key (to_id) references users(id) on delete cascade on update cascade);");
    create_users.execute(pool).await?;
    create_groups.execute(pool).await?;
    create_groups_chat.execute(pool).await?;
    create_personal_chat.execute(pool).await?;
    return Ok(());
}

// NOTE: this is create queries i should make this for all 4 tables

#[allow(dead_code)]
pub async fn insert_users(
    pool: &sqlx::Pool<sqlx::Postgres>,
    username: String,
    password: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let new_users = models::User {
        id: sqlx::types::uuid::Uuid::new_v4().to_string(),
        username,
        password,
    };
    let insert_users = sqlx::query("INSERT INTO users(id, username, password) VALUES ($1, $2, $3)")
        .bind(new_users.id)
        .bind(new_users.username)
        .bind(new_users.password)
        .execute(pool)
        .await?;
    return Ok(insert_users);
}

#[allow(dead_code)]
pub async fn insert_groups(
    pool: &sqlx::Pool<sqlx::Postgres>,
    groups_name: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let new_groups = models::Group {
        id: sqlx::types::uuid::Uuid::new_v4().to_string(),
        name: groups_name,
        datetime: chrono::Utc::now().to_string(),
    };
    let insert_groups = sqlx::query("INSERT INTO users (id, username, password) VALUES ($1, $2, $3)")
        .bind(new_groups.id)
        .bind(new_groups.name)
        .bind(new_groups.datetime)
        .execute(pool)
        .await?;
    return Ok(insert_groups);
}

#[allow(dead_code)]
pub async fn insert_groups_chat(
    pool: &sqlx::Pool<sqlx::Postgres>,
    group_id: String,
    message: String,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let groups_chat = models::GroupChatMsg {
        id: uuid::Uuid::new_v4().to_string(),
        group_id,
        datetime: chrono::Utc::now().to_string(),
        message,
    };

    let res = sqlx::query("insert into groups_chat_msg values ($1, $2, $3, $4)")
        .bind(groups_chat.id)
        .bind(groups_chat.group_id)
        .bind(groups_chat.datetime)
        .bind(groups_chat.message)
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
    let groups_chat = models::PersonalChatMsg {
        id: uuid::Uuid::new_v4().to_string(),
        from_id,
        to_id,
        datetime: chrono::Utc::now().to_string(),
        message,
    };

    let res = sqlx::query("insert into groups_chat_msg values ($1, $2, $3, $4)")
        .bind(groups_chat.id)
        .bind(groups_chat.from_id)
        .bind(groups_chat.to_id)
        .bind(groups_chat.datetime)
        .bind(groups_chat.message)
        .execute(pool)
        .await?;

    return Ok(res);
}

// NOTE: this is select queries i should make these for all 4 tables

#[allow(dead_code)]
pub async fn fetch_one_users_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<sqlx::postgres::PgRow, sqlx::Error> {
    let fetched_users = sqlx::query("select * from user where id = $1")
        .bind(id)
        .fetch_one(pool)
        .await?;

    return Ok(fetched_users);
}

#[allow(dead_code)]
pub async fn fetch_all_userss_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let fetched_userss = sqlx::query("select * from user where id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    return Ok(fetched_userss);
}

// NOTE: follow this function't pattern please for everthing else
#[allow(dead_code)]
pub async fn fetch_all_users(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<models::User>, sqlx::Error> {
    let all_users = sqlx::query_as::<_, models::User>("select * from users")
        .fetch_all(pool)
        .await?;
    return Ok(all_users);
}

#[allow(dead_code)]
pub async fn fetch_all_groupss(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let all_groupss = sqlx::query("select * from groups")
        .fetch_all(pool)
        .await?;
    return Ok(all_groupss);
}

#[allow(dead_code)]
pub async fn fetch_all_groups_chat_msg_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let fetched_groups_chat_msgs = sqlx::query("select * from groups_chat_msg where groups_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    return Ok(fetched_groups_chat_msgs);
}

#[allow(dead_code)]
pub async fn fetch_all_personal_chat_msg_with_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: String,
) -> Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> {
    let fetched_personal_chat_msgs = sqlx::query("select * from groups_chat_msg where from_id = $1 or to_id = $1")
        .bind(id)
        .fetch_all(pool)
        .await?;
    return Ok(fetched_personal_chat_msgs);
}

// i think this is pretty much available
