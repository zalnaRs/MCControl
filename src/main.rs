#[macro_use]
extern crate rocket;
pub mod admin;

use clap::Parser;
use rocket::request::FromRequest;
use rocket::serde::Deserialize;
use rocket::State;
use std::sync::Arc;
use tokio::process::Child;
use tokio::sync::Mutex;

pub struct AppState {
    pub child: Option<Child>,
    pub stdin: Option<Arc<Mutex<tokio::process::ChildStdin>>>,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub workdir: String,
}

#[get("/status")]
async fn status(state: &State<Arc<Mutex<AppState>>>) -> &'static str {
    let state = state.lock().await;
    if state.child.is_some() {
        "1" // Minecraft server is running
    } else {
        "0" // Minecraft server is not running
    }
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    let app_state = Arc::new(Mutex::new(AppState {
        child: None,
        stdin: None,
    }));

    rocket::build()
        .manage(args)
        .manage(app_state)
        .mount("/static", rocket::fs::FileServer::from("static"))
        .mount("/", routes![status])
        .mount("/admin", routes![admin::index])
        .mount("/admin/api", routes![admin::start, admin::kill, admin::status, admin::stop, admin::logs, admin::execute])
}
