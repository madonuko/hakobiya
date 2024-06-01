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

use rocket::{
    http::{CookieJar, Status},
    request,
};
use rocket_db_pools::{sqlx::PgPool, Database};

#[derive(Database)]
#[database("hakobiya")]
pub struct Hakobiya(PgPool);

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub name: String,
    pub mail: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Event {
    pub id: usize,
    pub name: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct SubEvent {
    pub id: usize,
    pub event: usize,
    pub comment: String,
}

#[derive(sqlx::FromRow, Debug)]
pub struct JoinEvent {
    pub usrmail: String,
    pub event: usize,
}

#[derive(sqlx::FromRow, Debug)]
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
