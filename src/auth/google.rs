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

use rocket::http::{Cookie, CookieJar, SameSite, Status};
use rocket::response::Redirect;
use rocket_oauth2::{OAuth2, TokenResponse};
use tracing::error;

pub fn routes() -> impl Into<Vec<rocket::Route>> {
    rocket::routes![google_login, google_callback]
}

#[derive(serde::Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

#[rocket::get("/login/google")]
fn google_login(oauth2: OAuth2<GoogleUser>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(
            cookies,
            &[".../auth/userinfo.email", ".../auth/userinfo.profile"],
        )
        .unwrap()
}

#[rocket::get("/auth/google")]
async fn google_callback(
    token: TokenResponse<GoogleUser>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Status> {
    let Some(id_token) = token.as_value().get("id_token") else {
        error!("Cannot get id_token from google's response");
        return Err(Status::InternalServerError);
    };
    let user_info: GoogleUser = match reqwest::Client::new()
        .get(format!(
            "https://www.googleapis.com/oauth2/v1/userinfo?alt=json&access_token={}",
            token.access_token()
        ))
        .bearer_auth(id_token)
        .send()
        .await
    {
        Ok(res) => match res.json().await {
            Ok(user_info) => user_info,
            Err(e) => {
                error!(?e, "Cannot turn user_info as json");
                return Err(Status::InternalServerError);
            }
        },
        Err(e) => {
            error!(?e, "Cannot get request from google oauth2 userinfo api");
            return Err(Status::Unauthorized);
        }
    };

    // Set a private cookie with the user's name, and redirect to the home page.
    cookies.add_private(
        Cookie::build(("username", user_info.name))
            .same_site(SameSite::Lax)
            .build(),
    );
    Ok(Redirect::to("/"))
}
