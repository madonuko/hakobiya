use rocket_dyn_templates::{context, Template};
use sea_orm::ActiveModelTrait;

use crate::entities;

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![create, create_submit, join, join_submit]
}

#[rocket::get("/create")]
fn create(user: crate::db::User) -> Template {
    Template::render(
        "events-create",
        context! {
            user,
        },
    )
}
#[derive(rocket::FromForm)]
struct PostCreate<'a> {
    name: &'a str,
}
#[rocket::post("/create", data = "<form>")]
async fn create_submit(
    form: rocket::form::Form<PostCreate<'_>>,
    db: &rocket::State<sea_orm::DatabaseConnection>,
) {
    let db: &sea_orm::prelude::DatabaseConnection = db as &sea_orm::DatabaseConnection;
    let new_event = entities::event::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        name: sea_orm::ActiveValue::Set(form.name.to_string()),
    };
    tracing::trace!(?new_event, "New Event");
    let evt = new_event
        .insert(db)
        .await
        .expect("cannot insert new event into db");
    tracing::info!(?evt, "Created new event");
    // redirect to something like "new-event-created.html.tera"
    todo!()
}

#[rocket::get("/join")]
fn join(user: crate::db::User) -> Template {
    Template::render(
        "events-join",
        context! {
            user,
        },
    )
}
#[derive(rocket::FromForm)]
struct PostJoin<'a> {
    event: &'a str,
}
#[rocket::post("/join", data = "<form>")]
fn join_submit(form: rocket::form::Form<PostJoin<'_>>) {
    todo!()
}
