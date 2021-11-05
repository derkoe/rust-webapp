#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use std::collections::HashMap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("version", VERSION);
    Template::render("index", context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
