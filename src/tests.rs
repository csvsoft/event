
use rocket::local::Client;
use rocket::http::Status;

fn rocket() -> rocket::Rocket {
rocket::ignite()
    .mount("/", routes![super::index])
    .mount("/getfile", routes![super::get_file])
}

#[test]
fn test_hello() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}


#[test]
fn test_get_file() {
    let client = Client::new(rocket()).unwrap();
    let mut response = client.get("/getfile/mysongs.txt").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello , this is my song.".into()));
}