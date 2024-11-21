#[macro_use]
extern crate rocket;

use clap::Parser;
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize};
use rocket::State;
use std::fs::File;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

struct AppState {
    child: Option<Child>,
    stdin: Option<Arc<Mutex<tokio::process::ChildStdin>>>,
}

struct Password(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Password {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header == "Bearer ".to_owned() + std::env::var("PASSWORD").unwrap().as_str() {
                return Outcome::Success(Password(auth_header.to_string()));
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("pages/index.html")).await.ok()
}

#[get("/logs")]
async fn logs(_password: Password) -> Option<NamedFile> {
    NamedFile::open(Path::new("output.txt")).await.ok()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    workdir: String,
}

#[post("/start")]
fn start(_password: Password, state: &State<Arc<Mutex<AppState>>>, args: &State<Args>) -> &'static str {
    let state_clone: Arc<Mutex<AppState>> = Arc::clone(state);
    let workdir = args.workdir.clone();

    tokio::spawn(async move {
        let mut state = state_clone.lock().await;
        if state.child.is_none() {
            let mut child = Command::new("sh")
                .arg("./start.sh")
                .current_dir(workdir)
                .stdin(Stdio::piped())
                .stdout(Stdio::from(
                    File::create("output.txt")
                        .expect("Failed to create output file"),
                ))
                .spawn()
                .expect("Failed to start process");

            let child_stdin = child.stdin.take().unwrap();
            state.stdin = Some(Arc::new(Mutex::new(child_stdin)));
            state.child = Some(child);
        }
    });

    "Process started"
}
/*
#[post("/start")]
fn start(args: &State<Args>) {
    Command::new("sh")
        .arg("./start.sh")
        .current_dir(&args.workdir)
        .stdout(Stdio::from(File::create("output.txt").expect("Failed to create output file")))
        .spawn()
        .expect("Failed to start script");
}
*/
#[post("/kill")]
async fn kill(_password: Password, state: &State<Arc<Mutex<AppState>>>) -> &'static str {
    let mut state = state.lock().await;

    if let Some(child) = state.child.as_mut() {
        match child.kill().await {
            Ok(_) => {
                state.child = None;
                state.stdin = None;
                println!("Process killed successfully");
                "Process killed"
            }
            Err(e) => {
                println!("Failed to kill process: {:?}", e);
                "Failed to kill process"
            }
        }
    } else {
        println!("No process running");
        "No process running"
    }
}

#[get("/status")]
async fn status(_password: Password, state: &State<Arc<Mutex<AppState>>>) -> &'static str {
    let mut state = state.lock().await;

    if let Some(child) = state.child.as_mut() {
        match child.try_wait() {
            Ok(None) => "Process is running",
            Ok(Some(_)) => {
                state.child = None;
                state.stdin = None;
                "Process has finished"
            }
            Err(_) => "Failed to check process status",
        }
    } else {
        "No process running"
    }
}

#[post("/stop")]
async fn stop(_password: Password, state: &State<Arc<Mutex<AppState>>>) -> &'static str {
    let mut state = state.lock().await;

    if let Some(stdin) = state.stdin.as_mut() {
        let mut stdin = stdin.lock().await;
        use tokio::io::AsyncWriteExt;
        stdin.write_all(b"stop\n").await.expect("Failed to write to stdin");
        "Stop command sent"
    } else {
        "No process running"
    }
}

#[derive(Deserialize)]
struct CommandRequest {
    command: String,
}

#[post("/execute", format = "json", data = "<command_request>")]
async fn execute(_password: Password, command_request: Json<CommandRequest>, state: &State<Arc<Mutex<AppState>>>) -> &'static str {
    let command = &command_request.command;
    let mut state = state.lock().await;

    if let Some(stdin) = state.stdin.as_mut() {
        let mut stdin = stdin.lock().await;
        use tokio::io::AsyncWriteExt;
        stdin.write_all(command.as_bytes()).await.expect("Failed to write to stdin");
        stdin.write_all(b"\n").await.expect("Failed to write newline to stdin");
        "Command executed"
    } else {
        "No process running"
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
        .mount("/", routes![index])
        .mount("/static", rocket::fs::FileServer::from("static"))
        .mount("/api", routes![start, kill, status, stop, logs, execute])
}
