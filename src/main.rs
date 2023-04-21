use rocket::form::Form;
use rocket::http::{ContentType, Cookie, CookieJar};
use rocket::serde::json::Json;
// use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::tokio::fs::File;
use rocket::{Either, Shutdown, State};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate rocket;
use Either::*;

static INDEX: &str = include_str!("../index.txt");

#[get("/")]
fn index() -> &'static str {
    INDEX
}

#[get("/login")]
async fn login(cookies: &CookieJar<'_>) -> Either<(ContentType, File), Redirect> {
    match cookies.get("user") {
        Some(_) => Right(Redirect::to(uri!("/"))),
        None => Left((ContentType::HTML, File::open("login.html").await.unwrap())),
    }
}

#[derive(FromForm)]
struct Login<'a> {
    username: &'a str,
}

// Now this, this is atrocious.
struct AppState {
    user_data: Arc<Mutex<HashMap<String, Vec<f32>>>>,
}

#[post("/login", data = "<login>")]
fn login_user(cookies: &CookieJar, login: Form<Login<'_>>, state: &State<AppState>) -> Redirect {
    match cookies.get("user") {
        Some(_) => Redirect::to(uri!("/")),
        None => {
            let requested_username = login.username;
            let mut user_data = state.user_data.lock().unwrap();
            match user_data.get(requested_username) {
                Some(_) => Redirect::to(uri!("/login")),
                None => {
                    user_data.insert(requested_username.to_string(), Vec::new());
                    cookies.add(Cookie::new("user", requested_username.to_string()));
                    Redirect::to(uri!("/try"))
                }
            }
        }
    }
}

#[get("/try")]
fn tryout(cookies: &CookieJar) -> Either<&'static str, Redirect> {
    match cookies.get("user") {
        Some(_) => Left("This is where the demo goes."),
        None => Right(Redirect::to(uri!("/login"))),
    }
}

#[post("/output")]
fn output(
    cookies: &CookieJar,
    data: Json<Vec<f32>>,
    state: &State<AppState>,
) -> Either<&'static str, Redirect> {
    if let Some(user) = cookies.get("user") {
        let mut user_data = state.user_data.lock().unwrap();

        Left("Sucess")
    } else {
        Right(Redirect::to(uri!("/login")))
    }
}

#[get("/data/<name>")]
fn retrieve(name: &str, state: &State<AppState>, mut end: Shutdown) -> EventStream![] {
    let stream = EventStream! {
        let state = state.into_inner();
        loop {

            yield Event::json()
        }
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, login, login_user, tryout])
        .manage(AppState {
            user_data: Arc::new(Mutex::new(HashMap::new())),
        })
}
