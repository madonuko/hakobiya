use rocket::http::Status;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, insert, select, setup_db};

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
async fn create_submit(
    form: rocket::form::Form<PostCreate<'_>>,
    db: &db::DbConnGuard,
    user: db::User,
) -> Redirect {
    setup_db!(db);
    let evt = insert!(Event { [id], name: form.name.to_string() });
    let hevt = insert!(HoldEvent { [id], event: evt.id, admin: user.id });
    tracing::info!(?evt, ?hevt, "Created new event");
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
    setup_db!(db);
    let jevt = insert!(JoinEvent {
        [id],
        user: user.id,
        event,
        invite_admin: form.invite_admin,
        notes: form.notes.to_string()
    });
    tracing::debug!(?jevt, "New JoinEvent");
    Redirect::to(rocket::uri!(get(event)))
}

#[deprecated]
pub async fn get_evt(id: i32, db: &sea_orm::DatabaseConnection) -> Option<db::Event> {
    crate::orm::Event::find_by_id(id)
        .one(db)
        .await
        .expect("can't select event")
}

#[rocket::get("/<eventid>")]
pub async fn get(eventid: i32, user: db::User, db: &db::DbConnGuard) -> Result<Template, Status> {
    setup_db!(db);
    let Some(event) = select!(Event(eventid)) else {
        return Err(Status::NotFound);
    };
    let sbevts = select!(SubEvent[event=eventid]@all);
    Ok(Template::render(
        "events",
        context! {
            event,
            sbevts,
            state: if db::as_event_admin(db, &user, eventid).await.is_some() {
                "admin"
            } else if select!(JoinEvent[user=user.id]@one).is_some() {
                "join"
            } else {
                "none"
            },
        },
    ))
}

#[rocket::get("/<evtid>/subevts/create")]
async fn create_subevent(
    user: db::User,
    evtid: i32,
    db: &db::DbConnGuard,
) -> Result<Template, Result<Status, Redirect>> {
    setup_db!(db);
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Err(Err(Redirect::to(rocket::uri!(get(evtid)))));
    }
    let Some(event) = select!(Event(evtid)) else {
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
    setup_db!(db);
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Err(Ok(Status::Forbidden));
    }
    // FIXME
    let new_sbevt = insert!(SubEvent { [id], event: evtid, comment: form.comment.to_string() }~);
    let sbevt = match new_sbevt {
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
