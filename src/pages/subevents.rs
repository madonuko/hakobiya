use rocket::{http::Status, response::Redirect};
use rocket_dyn_templates::{context, Template};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, entities, orm};

use super::events::get_evt;

pub fn ruotes() -> impl Into<Vec<rocket::Route>> {
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
    let managed_events =
        orm::HoldEvent::find().filter(entities::hold_event::Column::Admin.eq(user.id));
    let thisevent = managed_events.filter(entities::hold_event::Column::Event.eq(evtid));
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
    Ok(Template::render(
        "subevents",
        context! {
            event,
            subevt,
            state: if thisevent.one(db).await.expect("can't select holdevents").is_some() { "admin" } else if             orm::JoinEvent::find().filter(entities::join_event::Column::User.eq(user.id)).filter(entities::join_event::Column::Event.eq(evtid)).one(db).await.expect("can't select joinevents").is_some()
            { "join" } else { return Err((Status::Forbidden, Redirect::to(rocket::uri!("/")))) }
        },
    ))
}
