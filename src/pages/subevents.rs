use rocket::{http::Status, response::Redirect};
use rocket_dyn_templates::{context, Template};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, entities, orm};

use super::events::get_evt;

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![get]
}

#[rocket::get("/<evtid>/subevts/<id>")]
pub async fn get(
    user: db::User,
    evtid: i32,
    id: i32,
    db: &db::DbConnGuard,
) -> Result<Template, (Status, Redirect)> {
    let db = db as &db::DbConn;
    let hevt = orm::HoldEvent::find()
        .filter(entities::hold_event::Column::Admin.eq(user.id))
        .filter(entities::hold_event::Column::Event.eq(evtid));
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
    let state = if let Some(_) = hevt.one(db).await.expect("can't select holdevents") {
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
#[rocket::get("/<evtid>/subevts/user/<uid>")]
async fn get_user(
    user: db::User,
    evtid: i32,
    uid: i32,
    db: &db::DbConnGuard,
) -> Result<Template, (Status, Redirect)> {
    let db = db as &db::DbConn;
    let hevt = orm::HoldEvent::find()
        .filter(entities::hold_event::Column::Admin.eq(user.id))
        .filter(entities::hold_event::Column::Event.eq(evtid));
    if let None = hevt.one(db).await.expect("can't select holdevents") {
        return Err((Status::Forbidden, Redirect::to(rocket::uri!("/"))));
    }
    todo!()
}
