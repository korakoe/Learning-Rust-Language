#[macro_use] extern crate rocket;
use rocket::http::ContentType;


#[get("/")]
fn helloworld() -> (ContentType, &'static str) {
    let data = r#"<link rel=\"stylesheet\" type="text/css\" href=\"static/style.css\">
        <h1>Hello World!</h1>"#;
    (ContentType::HTML, data)
}

#[get("/style.css")]
fn style() -> (ContentType, &'static str) {
    let data = r#"h1 {
	color: Blue;
}"#;
    (ContentType::CSS, data)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![helloworld, style])
}