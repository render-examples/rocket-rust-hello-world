use super::rocket;
use rocket::local::Client;

#[test]
fn barcode() {
    let rocket = rocket::ignite().mount("/", routes![super::barcode]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/barcode/1234").dispatch();
    assert_eq!(response.body_string(), Some("{\"barcode\": \"1234\", \
    \"name\": \"Grape Test\", \
    \"fruit\": \"grapes\", \
    \"process\": \"main\", \
    \"standard\": \"premium\", \
    \"market\": \"Harris Farms\", \
    \"image\": \"https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png\"\
    }\
".into()));
}
