use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
};

fn hello_word<'buf, 'req>(_: &'req Request<'buf>) -> Response<'req> {
    Response::html(StatusCode::Ok, "")
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.routes.get("/", hello_word);
    server.run("127.0.0.1:8080")
}
