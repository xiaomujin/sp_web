use std::env;
use rocket::{Build, Rocket, routes, get, launch};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 开启 cmd 的 AlwaysAnsi 功能
    if env::consts::OS == "windows" {
        ansi_term::enable_ansi_support().unwrap();
    }

    rocket::build().mount("/", routes![hello, bye]).launch().await?;
    Ok(())
}


#[get("/hello")]
async fn hello() -> String {
    "Hello, world!".to_string()
}

#[get("/bye")]
async fn bye() -> String {
    "bye!".to_string()
}

// #[launch]
// fn rocket() -> Rocket<Build> {
//     rocket::build().mount("/", routes![hello, bye])
// }