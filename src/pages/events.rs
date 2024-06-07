use rocket::http::Status;
use rocket_dyn_templates::{context, Template};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::entities;

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![create, create_submit, join, get]
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

#[derive(rocket::FromForm)]
struct PostJoin<'a> {
    invite_admin: i32,
    notes: &'a str,
}
#[rocket::post("/<event>/join", data = "<form>")]
async fn join<'a>(
    form: rocket::form::Form<PostJoin<'a>>,
    event: i32,
    user: crate::db::User,
    db: &rocket::State<sea_orm::DatabaseConnection>,
) {
    let db = db as &sea_orm::DatabaseConnection;
    let jevt = entities::join_event::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        user: sea_orm::ActiveValue::Set(user.id),
        event: sea_orm::ActiveValue::Set(event),
        invite_admin: sea_orm::ActiveValue::Set(form.invite_admin),
        notes: sea_orm::ActiveValue::Set(form.notes.to_string()),
    };
    let jevt = jevt.insert(db).await.expect("Can't insert new jevent");
    tracing::debug!(?jevt, "New JoinEvent");
    todo!()
}

#[rocket::get("/<eventid>")]
async fn get(
    eventid: i32,
    user: crate::db::User,
    db: &rocket::State<sea_orm::DatabaseConnection>,
) -> Result<Template, Status> {
    let db = db as &sea_orm::DatabaseConnection;
    let managed_events =
        crate::orm::HoldEvent::find().filter(entities::hold_event::Column::Admin.eq(user.id));
    let thisevent = managed_events
        .filter(entities::hold_event::Column::Event.eq(eventid))
        .one(db)
        .await;
    if let Some(thisevent) = thisevent.expect("can't select holdevents") {
        // user is admin of this event
        Ok(Template::render("events", context! { event, state: "hold" }))
    } else {
        let joined_events = crate::orm::JoinEvent::find().filter(entities::join_event::Column::User.eq(user.id));
        let thisevent = joined_events.filter(entities::jion_event::Column::Event.eq(eventid)).one(db).await;
        if let Some(thisevent) = thisevent.expect("can't select joinevents") {
            // user has already joined this event
            Ok(Template::render("events", context! {event, state: "join" }))

        } else if let Some(event) = crate::orm::Event::find_by_id(eventid).one(db).await.expect("can't select event") {
            Ok(Template::render("events", context! { event, state: "none" }))
        } else {
            Err(Status::NotFound)
        }
    }
}
