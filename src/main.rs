#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template{
    Template::render("index", context!{
        title: "Manchita"
    })
}

#[get("/about")]
fn about() -> &'static str{
    "about"
}

#[get("/")]
fn profile() -> &'static str{
    "profile"
}

#[post("/")]
fn create_profile() -> &'static str{
    "creating profile"
}

#[put("/")]
fn update_profile() -> &'static str{
    "updating profile"
}

#[delete("/")]
fn delete_profile() -> &'static str{
    "deleting profile"
}

#[launch]
fn rocket() -> _{
    rocket::build().mount("/", routes![index, about])
    .mount("/profile", routes![profile, create_profile, update_profile, delete_profile])
    .attach(Template::fairing())
}
