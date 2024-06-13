use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, entities, orm};

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![
        create,
        create_submit,
        join,
        get,
        create_subevent,
        create_subevent_submit
    ]
}

#[rocket::get("/create")]
fn create(user: db::User) -> Template {
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
async fn create_submit(form: rocket::form::Form<PostCreate<'_>>, db: &db::DbConnGuard) -> Redirect {
    let db = db as &db::DbConn;
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
    Redirect::to(rocket::uri!(get(evt.id)))
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
    user: db::User,
    db: &db::DbConnGuard,
) -> Redirect {
    let db = db as &db::DbConn;
    let jevt = entities::join_event::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        user: sea_orm::ActiveValue::Set(user.id),
        event: sea_orm::ActiveValue::Set(event),
        invite_admin: sea_orm::ActiveValue::Set(form.invite_admin),
        notes: sea_orm::ActiveValue::Set(form.notes.to_string()),
    };
    let jevt = jevt.insert(db).await.expect("Can't insert new jevent");
    tracing::debug!(?jevt, "New JoinEvent");
    Redirect::to(rocket::uri!(get(event)))
}

pub async fn get_evt(id: i32, db: &sea_orm::DatabaseConnection) -> Option<db::Event> {
    crate::orm::Event::find_by_id(id)
        .one(db)
        .await
        .expect("can't select event")
}

#[rocket::get("/<eventid>")]
pub async fn get(eventid: i32, user: db::User, db: &db::DbConnGuard) -> Result<Template, Status> {
    let db = db as &db::DbConn;
    if db::as_event_admin(db, &user, eventid).await.is_some() {
        // user is admin of this event
        let event = get_evt(eventid, db)
            .await
            .expect("event in hevents not exist");
        let sbevts = orm::SubEvent::find().filter(entities::sub_event::Column::Event.eq(eventid));
        let sbevts = sbevts.all(db).await.expect("can't select subevents");
        Ok(Template::render(
            "events",
            context! { event, sbevts, state: "admin" },
        ))
    } else {
        let joined_events =
            orm::JoinEvent::find().filter(entities::join_event::Column::User.eq(user.id));
        let thisevent = joined_events.filter(entities::join_event::Column::Event.eq(eventid));
        if let Some(_) = thisevent.one(db).await.expect("can't select joinevents") {
            // user has already joined this event
            let event = get_evt(eventid, db)
                .await
                .expect("event in jevents not exist");
            Ok(Template::render(
                "events",
                context! { event, state: "join" },
            ))
        } else if let Some(event) = get_evt(eventid, db).await {
            Ok(Template::render(
                "events",
                context! { event, state: "none" },
            ))
        } else {
            Err(Status::NotFound)
        }
    }
}

#[rocket::get("/<evtid>/subevts/create")]
async fn create_subevent(
    user: db::User,
    evtid: i32,
    db: &db::DbConnGuard,
) -> Result<Template, Result<Status, Redirect>> {
    let db = db as &db::DbConn;
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Err(Err(Redirect::to(rocket::uri!(get(evtid)))));
    }
    let Some(event) = get_evt(evtid, db).await else {
        return Err(Ok(Status::NotFound));
    };
    Ok(Template::render("subevt-create", context! { event }))
}
#[derive(rocket::FromForm)]
struct PostSubCreate<'a> {
    comment: &'a str,
}
#[rocket::post("/<evtid>/subevts/create", data = "<form>")]
async fn create_subevent_submit<'a>(
    user: db::User,
    evtid: i32,
    db: &db::DbConnGuard,
    form: rocket::form::Form<PostSubCreate<'a>>,
) -> Result<Template, Result<Status, Redirect>> {
    // help wtf is this return type
    let db = db as &db::DbConn;
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Err(Ok(Status::Forbidden));
    }
    let new_sbevt = entities::sub_event::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        event: sea_orm::ActiveValue::Set(evtid),
        comment: sea_orm::ActiveValue::Set(form.comment.to_string()),
    };
    tracing::trace!(?new_sbevt, "New Subevent");
    let sbevt = match new_sbevt.insert(db).await {
        Ok(x) => x,
        Err(e) => {
            tracing::error!(?e);
            // probably bad evtid
            return Err(Ok(Status::NotFound));
        }
    };
    Err(Err(Redirect::to(rocket::uri!(super::subevents::get(
        evtid, sbevt.id
    )))))
}
