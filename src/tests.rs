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
    \"market\": \"Woolworths\", \
    \"image\": \"https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png\"\
    }\
".into()));
}

#[test]
fn home() {
    let rocket = rocket::ignite().mount("/", routes![super::home]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("<html>
    <head>
        <title>Barcode API</title>
    </head>
    <body>
        <h1>Barcode API</h1>
        <p>Use the following URL to get a barcode:</p>
        <p><a href=\"https://barcode-api.herokuapp.com/barcode/1234\">https://barcode-api.herokuapp.com/barcode/1234</a></p>
    </body>
</html>
".into()))
}
