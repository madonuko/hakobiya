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

pub mod auth;
pub mod db;
use std::path::Path;

use rocket::{
    fs::NamedFile,
    http::{Cookie, CookieJar, SameSite},
    response::Redirect,
};
use rocket_oauth2::{OAuth2, TokenResponse};

#[rocket::get("/")]
fn index(user: db::User) -> String {
    format!("Hi, {}!", user.name)
}

#[rocket::get("/", rank = 2)]
async fn index_anonymous() -> NamedFile {
    let path = Path::new(rocket::fs::relative!("static")).join("login-required.html");
    NamedFile::open(path)
        .await
        .expect("No static/login-required.html")
}

#[rocket::get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    Redirect::to("/")
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index, index_anonymous, logout])
        .mount(
            "/",
            rocket::fs::FileServer::from(rocket::fs::relative!("static")),
        )
        // The string "github" here matches [default.oauth.github] in `Rocket.toml`
        .attach(OAuth2::<Google>::fairing("google"))
}
