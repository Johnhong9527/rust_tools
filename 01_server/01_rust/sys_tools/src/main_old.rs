#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/world
#[get("/world")]
fn world() -> &'static str{
    "hello, hhh world!"
}

// Try visiting:
//   http://127.0.0.1:8000/hello/<name>
#[get("/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[post("/file_path"), format="application/json", data="<user>"]
fn set_file_path(user: User) {
    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hello", routes![world])
        .mount("/hello", routes![hello])
}