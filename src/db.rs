// This file is part of Hakobiya.
//
// Hakobiya is free software: you can redistribute it and/or modify it under the terms of
// the GNU Affero General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// Hakobiya is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Hakobiya.
// If not, see <https://www.gnu.org/licenses/>.
//

use migration::MigratorTrait;
use rocket::{
    http::{CookieJar, Status},
    request,
};
use rocket_db_pools::{sqlx::PgPool, Database};
use sea_orm::ConnectionTrait;

static DATABASE_URL: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    std::env::var("DATABASE_URL").expect("$DATABASE_URL not defined")
});

const DB_NAME: &'static str = "hakobiya";

#[derive(Database)]
#[database("hakobiya")]
pub struct Hakobiya(PgPool);

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct User {
    pub name: String,
    pub mail: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Event {
    pub id: usize,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct SubEvent {
    pub id: usize,
    pub event: usize,
    pub comment: String,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct JoinEvent {
    pub usrmail: String,
    pub event: usize,
}

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct JoinSubEvent {
    pub usrmail: String,
    pub subevt: usize,
    pub scanned: bool,
}

#[rocket::async_trait]
impl<'r> request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r request::Request<'_>) -> request::Outcome<User, ()> {
        let cookies = request
            .guard::<&CookieJar<'_>>()
            .await
            .expect("request cookies");
        if let Some((username, mail)) = cookies
            .get_private("username")
            .zip(cookies.get_private("mail"))
        {
            return request::Outcome::Success(User {
                name: username.value().to_string(),
                mail: mail.value().to_string(),
            });
        }

        request::Outcome::Forward(Status::Unauthorized)
    }
}

pub(super) async fn set_up_db() -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    let db = sea_orm::Database::connect(&*DATABASE_URL).await?;
    // create db if not exists
    db.execute(sea_orm::Statement::from_string(
        db.get_database_backend(),
        format!(
            r#"SELECT 'CREATE DATABASE {DB_NAME}'
        WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '{DB_NAME}')\gexec"#
        ),
    ))
    .await?;

    let url = format!("{}/{DB_NAME}", *DATABASE_URL);
    let db = sea_orm::Database::connect(&url).await?;
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}
