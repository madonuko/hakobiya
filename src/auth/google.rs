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

pub fn routes() -> impl Into<Vec<Route>> {
    todo!()
}

/// User information to be retrieved from Google
#[derive(serde::Deserialize)]
struct GoogleUserInfo {
    #[serde(default)]
    name: String,
    #[serde(default)]
    mail: String,
}

#[rocket::get("/login/google")]
fn google_login(oauth2: OAuth2<GoogleUserInfo>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(
            cookies,
            &[".../auth/userinfo.email", ".../auth/userinfo.profile"],
        )
        .unwrap()
}

#[get("/auth/google")]
async fn github_callback(
    token: TokenResponse<GitHubUserInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Redirect, Debug<Error>> {
    // Use the token to retrieve the user's GitHub account information.
    let user_info: GitHubUserInfo = reqwest::Client::builder()
        .build()
        .context("failed to build reqwest client")?
        .get("https://api.github.com/user")
        .header(AUTHORIZATION, format!("token {}", token.access_token()))
        .header(ACCEPT, "application/vnd.github.v3+json")
        .header(USER_AGENT, "rocket_oauth2 demo application")
        .send()
        .await
        .context("failed to complete request")?
        .json()
        .await
        .context("failed to deserialize response")?;

    // Set a private cookie with the user's name, and redirect to the home page.
    cookies.add_private(
        Cookie::build(("username", user_info.name))
            .same_site(SameSite::Lax)
            .build(),
    );
    Ok(Redirect::to("/"))
}
