use rocket::{form::Form, get, post, response::status, Route};
use rocket_auth::{Auth, Error, Login, Signup};

#[post("/signup", data = "<form>")]
async fn signup(form: Form<Signup>, auth: Auth<'_>) {
    auth.signup(&form);
}

#[post("/login", data = "<form>")]
async fn login(form: Form<Login>, auth: Auth<'_>) -> Result<status::NoContent, Error> {
    auth.login(&form).await?;
    Ok(status::NoContent)
}

#[get("/logout")]
async fn logout(auth: Auth<'_>) -> Result<&str, status::Unauthorized<String>> {
    match auth.logout() {
        Ok(()) => Ok("Logged out"),
        Err(e) => Err(status::Unauthorized(Some(e.to_string()))),
    }
}

pub fn get_routes() -> Vec<Route> {
    routes![signup, login, logout]
}
