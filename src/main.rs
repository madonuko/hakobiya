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

use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
};
use rocket_db_pools::Database;
use rocket_dyn_templates::Template;
use rocket_oauth2::OAuth2;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[rocket::get("/")]
fn index(user: db::User) -> Template {
    Template::render("index", &user)
}

#[rocket::get("/", rank = 2)]
fn index_anonymous() -> (rocket::http::ContentType, &'static str) {
    (
        rocket::http::ContentType::HTML,
        include_str!("../static/login-required.html"),
    )
}

#[rocket::get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    cookies.remove(Cookie::from("mail"));
    Redirect::to("/")
}

#[rocket::catch(401)] // Unauthorized
fn unauthorized_access() -> Redirect {
    Redirect::to(rocket::uri!(index_anonymous()))
}

#[rocket::launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    #[cfg(not(debug_assertions))]
    let loglayer = tracing_logfmt::layer();
    #[cfg(debug_assertions)]
    let loglayer = tracing_subscriber::fmt::layer();
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(loglayer)
        .init();
    tracing::info!("Launching rocket 🚀");
    rocket::build()
        .attach(db::Hakobiya::init())
        .attach(rocket::fairing::AdHoc::try_on_ignite(
            "Migrations",
            db::migrate,
        ))
        .attach(Template::fairing())
        .mount("/", rocket::routes![index, index_anonymous, logout])
        .mount("/", auth::google::routes())
        .register("/", rocket::catchers![unauthorized_access])
        .attach(OAuth2::<auth::google::GoogleUser>::fairing("google"))
}
