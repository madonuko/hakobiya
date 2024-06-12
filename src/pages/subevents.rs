use rocket::{http::Status, response::Redirect};
use rocket_dyn_templates::{context, Template};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, entities, orm};

use super::events::get_evt;

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![get, get_user, scanned]
}

#[rocket::get("/<evtid>/subevts/<id>")]
pub async fn get(
    user: db::User,
    evtid: i32,
    id: i32,
    db: &db::DbConnGuard,
) -> Result<Template, (Status, Redirect)> {
    let db = db as &db::DbConn;
    let jevt = orm::JoinEvent::find()
        .filter(entities::join_event::Column::User.eq(user.id))
        .filter(entities::join_event::Column::Event.eq(evtid));
    let event = get_evt(evtid, db)
        .await
        .expect("event in hevents not exist");
    let Some(subevt) = orm::SubEvent::find_by_id(id)
        .one(db)
        .await
        .expect("can't select subevent")
    else {
        return Err((
            Status::NotFound,
            Redirect::to(rocket::uri!(super::events::get(evtid))),
        ));
    };
    let state = if db::as_event_admin(db, &user, evtid).await.is_some() {
        "admin"
    } else if let Some(_) = jevt.one(db).await.expect("can't select joinevents") {
        "join"
    } else {
        return Err((Status::Forbidden, Redirect::to(rocket::uri!("/"))));
    };
    Ok(Template::render(
        "subevents",
        context! { event, subevt, state },
    ))
}

#[derive(rocket::FromForm)]
struct FormSubmit {
    uid: i32,
}
#[rocket::post("/<evtid>/subevts/<id>/scanned", data = "<form>")]
async fn scanned(
    user: db::User,
    evtid: i32,
    id: i32,
    db: &db::DbConnGuard,
    form: rocket::form::Form<FormSubmit>,
) -> Status {
    todo!()
}

#[rocket::get("/<evtid>/subevts/user/<uid>")]
async fn get_user(
    user: db::User,
    evtid: i32,
    uid: i32,
    db: &db::DbConnGuard,
) -> Result<Template, (Status, Redirect)> {
    let db = db as &db::DbConn;
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Err((Status::Forbidden, Redirect::to(rocket::uri!("/"))));
    }
    let jevt = orm::JoinEvent::find()
        .filter(entities::join_event::Column::Event.eq(evtid))
        .filter(entities::join_event::Column::User.eq(uid));
    let Some(jevt) = jevt.one(db).await.expect("can't select joinevents") else {
        return Err((
            Status::NotFound,
            Redirect::to(rocket::uri!(super::events::get(evtid))),
        ));
    };
    Ok(Template::render("subevt-user", context! { user, jevt }))
}
