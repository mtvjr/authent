use rocket::request::Form;

#[derive(FromForm, Debug)]
pub struct Info {
    email: String,
    username: String,
    password: String,
    remember: bool,
}

#[post("/login", data="<info>")]
pub fn route(info: Form<Info>) -> String {
    format!("You tried to login with {:?}", info)
}
