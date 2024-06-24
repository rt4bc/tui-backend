use actix_web::{web, App, HttpResponse, HttpServer};
use actix_files as fs;

async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
	    .service(fs::Files::new("/", "../frontend/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

fn main() {
    std::thread::spawn(|| {
        let rt = actix_web::rt::Runtime::new().unwrap();
        rt.block_on(start_server()).unwrap();
    });

    // Your main program logic here
    println!("Main thread is running...");
    std::thread::sleep(std::time::Duration::from_secs(20));

    println!("Stop");
}
