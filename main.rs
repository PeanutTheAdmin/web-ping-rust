// How to use: http://YOUR_IP_HERE:8080/ping/IP_TO_PING
// ex: http://192.168.1.2:8080/ping/8.8.8.8

use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::process::Command;

async fn ping_device(Path(ip): Path<String>) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("ping")
            .arg("-n")
            .arg("4") // send 4 ping requests on Windows
            .arg(&ip)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("ping")
            .arg("-c")
            .arg("4") // send 4 ping requests on Linux
            .arg(&ip)
            .output()
            .expect("failed to execute process")
    };

    let response = String::from_utf8_lossy(&output.stdout);

    let is_online = if cfg!(target_os = "windows") {
        // Check Windows
        !(response.contains("Request timed out") || response.contains("Destination host unreachable"))
    } else {
        // Check Linux
        !(response.contains("0 received") || response.contains("100% packet loss"))
    };

    if is_online {
        format!("{} is online:\n{}", ip, response)
    } else {
        format!("{} is offline:\n{}", ip, response)
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping/:ip", get(ping_device));

    // run it on port 8080
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
