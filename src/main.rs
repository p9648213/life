use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
};

fn hello_word<'a>(_: &'a Request) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>Hello World</h1>")
}

fn check_health<'a>(req: &'a Request) -> Response<'a> {
    println!("{:?}", req.query());
    Response::html(StatusCode::Ok, "<h1>Healthy</h1>")
}

fn post_check_health<'a>(_: &'a Request) -> Response<'a> {
    Response::html(StatusCode::Ok, "<h1>Healthy</h1>")
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();

    server.routes.get("/", hello_word);
    server.routes.get("/health", check_health);
    server.routes.post("/health", post_check_health);

    server.run("127.0.0.1:8080")
}
