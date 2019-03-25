use actix;
use actix_web;
use env_logger;

use self::actix_web::{middleware, server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello-world");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index))
    }).bind("127.0.0.1:80")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:80");
    let _ = sys.run();
}
