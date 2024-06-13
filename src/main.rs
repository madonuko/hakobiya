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
pub mod entities;
pub mod pages;

pub use entities::prelude as orm;
pub use rocket::fs as rfs;
use rocket::{
    http::{Cookie, CookieJar, Status},
    response::Redirect,
};
use rocket_dyn_templates::Template;
use rocket_oauth2::OAuth2;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[rocket::get("/")]
async fn index(
    user: db::User,
    db: &db::DbConnGuard,
) -> Result<Template, Status> {
    let db: &sea_orm::prelude::DatabaseConnection = db as &sea_orm::DatabaseConnection;
    let hevents = orm::HoldEvent::find()
        .filter(entities::hold_event::Column::Admin.eq(user.id.clone()))
        .limit(10)
        .all(db)
        .await
        .expect("find all user holdevents");
    let mut new_hevents = vec![];
    for hevent in hevents {
        let Ok(Some(hevent)) = orm::Event::find_by_id(hevent.event).one(db).await else {
            tracing::error!(?hevent, "Cannot get HoldEvent");
            return Err(Status::InternalServerError);
        };
        new_hevents.push(hevent);
    }
    let jevents = orm::JoinEvent::find()
        .filter(entities::join_event::Column::User.eq(user.id.clone()))
        .limit(10)
        .all(db)
        .await
        .expect("find all user joinevents");
    let mut new_jevents = vec![];
    for jevent in jevents {
        let Ok(Some(jevent)) = orm::Event::find_by_id(jevent.event).one(db).await else {
            tracing::error!(?jevent, "Cannot get JoinEvent");
            return Err(Status::InternalServerError);
        };
        new_jevents.push(jevent);
    }
    Ok(Template::render(
        "index",
        rocket_dyn_templates::context! {
            user,
            hevents: new_hevents,
            jevents: new_jevents,
        },
    ))
}

#[rocket::get("/", rank = 2)]
fn index_anonymous() -> Template {
    Template::render("login-required", rocket_dyn_templates::context! {})
}

#[rocket::get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Redirect {
    cookies.remove(Cookie::from("username"));
    cookies.remove(Cookie::from("mail"));
    Redirect::to(rocket::uri!(index_anonymous()))
}

#[rocket::catch(401)] // Unauthorized
fn unauthorized_access() -> Redirect {
    Redirect::to(rocket::uri!(index_anonymous()))
}

#[rocket::catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/404.html")
}

#[rocket::launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    #[cfg(not(debug_assertions))]
    let loglayer = tracing_logfmt::layer();
    #[cfg(debug_assertions)]
    let loglayer = tracing_subscriber::fmt::layer();
    tracing_subscriber::Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(loglayer)
        .init();

    tracing::info!("📚 Setting up sea_orm database");
    let db = db::set_up_db().await.expect("Cannot setup db");

    tracing::info!("🚀 Launching rocket");
    rocket::build()
        .manage(db)
        .attach(Template::fairing())
        .mount("/", rfs::FileServer::from(rfs::relative!("static")))
        .mount("/", rocket::routes![index, index_anonymous, logout])
        .mount("/", auth::google::routes())
        .register("/", rocket::catchers![unauthorized_access, not_found])
        .mount("/events", pages::events::routes())
        .attach(OAuth2::<auth::google::GoogleUser>::fairing("google"))
}
