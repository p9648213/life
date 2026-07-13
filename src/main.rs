use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
    templates,
};

fn home<'buf, 'req>(_: &'req Request<'buf>) -> Response<'req> {
    let mut html = String::new();
    templates::render_home_index(&mut html);
    Response::html(StatusCode::Ok, &html)
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.routes.get("/", home);
    server.run("127.0.0.1:8080")
}
