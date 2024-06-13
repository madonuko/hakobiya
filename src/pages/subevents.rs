use rocket::{http::Status, response::Redirect};
use rocket_dyn_templates::{context, Template};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{db, insert, select, setup_db};

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
    setup_db!(db);
    let Some(event) = select!(Event(evtid)) else {
        return Err((
            Status::NotFound,
            Redirect::to(rocket::uri!("/")),
        ));
    };
    let Some(subevt) = select!(SubEvent(id)) else {
        return Err((
            Status::NotFound,
            Redirect::to(rocket::uri!(super::events::get(evtid))),
        ));
    };
    let state = if db::as_event_admin(db, &user, evtid).await.is_some() {
        "admin"
    } else if let Some(_) = select!(JoinEvent[user=user.id, event=evtid]@one) {
        "join"
    } else {
        return Err((Status::Forbidden, Redirect::to(rocket::uri!("/"))));
    };
    Ok(Template::render(
        "subevent",
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
    setup_db!(db);
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Status::Forbidden;
    }
    let Some(_) = select!(JoinEvent[event=evtid, user=form.uid]@one) else {
        return Status::NotFound;
    };
    if let None = select!(SubEvent(id)) {
        return Status::NotFound;
    }
    insert!(JoinSubEvent {
        [id],
        sub_event: id,
        user: form.uid,
        scanned: true
    });
    Status::NoContent
}

#[rocket::get("/<evtid>/subevts/user/<uid>")]
async fn get_user(
    user: db::User,
    evtid: i32,
    uid: i32,
    db: &db::DbConnGuard,
) -> Result<Template, (Status, Redirect)> {
    setup_db!(db);
    if db::as_event_admin(db, &user, evtid).await.is_none() {
        return Err((Status::Forbidden, Redirect::to(rocket::uri!("/"))));
    }
    let Some(jevt) = select!(JoinEvent[event=evtid, user=uid]@one) else {
        return Err((
            Status::NotFound,
            Redirect::to(rocket::uri!(super::events::get(evtid))),
        ));
    };
    Ok(Template::render("subevt-user", context! { user, jevt }))
}
