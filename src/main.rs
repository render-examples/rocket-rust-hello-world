#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/barcode/<barcode>")]
fn hello(barcode: String) -> String {
    format!("{{\"barcode\": \"{barcode}\", \
        \"name\": \"Test Product\", \
        \"price\": 100, \
        \"image\": \"https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png\"\
        }}\
    ")
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
