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
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

pub use entities::event::Model as Event;
pub use entities::hold_event::Model as HoldEvent;
pub use entities::join_event::Model as JoinEvent;
pub use entities::join_sub_event::Model as JoinSubEvent;
pub use entities::sub_event::Model as SubEvent;
pub use entities::user::Model as User;

use crate::entities;

static DATABASE_URL: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    std::env::var("DATABASE_URL").expect("$DATABASE_URL not defined")
});

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
            let db = request
                .guard::<&rocket::State<sea_orm::DatabaseConnection>>()
                .await
                .expect("get db");
            let db = db as &sea_orm::DatabaseConnection;
            let (name, mail) = (username.value().to_string(), mail.value().to_string());
            let Some(user) = crate::orm::User::find()
                .filter(entities::user::Column::Mail.contains(&mail))
                .one(db)
                .await
                .expect("find user from request")
            else {
                let newuser = entities::user::ActiveModel {
                    id: sea_orm::ActiveValue::NotSet,
                    name: sea_orm::ActiveValue::Set(name),
                    mail: sea_orm::ActiveValue::Set(mail),
                };
                tracing::info!(?newuser, "insert new user");
                let user = newuser.insert(db).await.expect("insert new user");
                return request::Outcome::Success(user);
            };
            request::Outcome::Success(user)
        } else {
            request::Outcome::Forward(Status::Unauthorized)
        }
    }
}

pub(super) async fn set_up_db() -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
    // let db = sea_orm::Database::connect(&*DATABASE_URL).await?;
    // create db if not exists
    // db.execute(sea_orm::Statement::from_string(
    //     db.get_database_backend(),
    //     format!(
    //         r#"SELECT 'CREATE DATABASE {DB_NAME}'
    //     WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '{DB_NAME}')\gexec"#
    //     ),
    // ))
    // .await?;

    let db = sea_orm::Database::connect(&*DATABASE_URL).await?;
    migration::Migrator::up(&db, None).await?;
    Ok(db)
}
