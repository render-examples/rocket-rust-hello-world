#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/")]
fn home() -> String {
    format!("<html>
    <head>
        <title>Barcode API</title>
    </head>
    <body>
        <h1>Barcode API</h1>
        <p>Use the following URL to get a barcode:</p>
        <p><a href=\"https://barcode-api.herokuapp.com/barcode/1234\">https://barcode-api.herokuapp.com/barcode/1234</a></p>
    </body>
</html>
")
}

#[get("/barcode/<barcode>")]
fn barcode(barcode: String) -> String {
    format!("{{\"barcode\": \"{barcode}\", \
    \"name\": \"Grape Test\", \
    \"fruit\": \"grapes\", \
    \"process\": \"main\", \
    \"standard\": \"premium\", \
    \"market\": \"Woolworths\", \
    \"image\": \"https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png\"\
    }}\
")
}

fn main() {
    rocket::ignite().mount("/", routes![barcode, home]).launch();
}
