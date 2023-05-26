use super::rocket;
use rocket::local::Client;

#[test]
fn hello_world() {
    let rocket = rocket::ignite().mount("/", routes![super::hello]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/barcode/1234").dispatch();
    assert_eq!(response.body_string(), Some("{\"barcode\": \"1234\", \
    \"name\": \"Test Product\", \
    \"price\": 100, \
    \"image\": \"https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png\"\
    }\
".into()));
}
