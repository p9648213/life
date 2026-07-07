use life::{
    html::render::render,
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
};

fn hello_word<'buf, 'req>(_: &'req Request<'buf>) -> Response<'req> {
    let span = render("span", "Hello World", "", "");
    let div = render("div", &span, "", "");
    Response::html(StatusCode::Ok, &div)
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.routes.get("/", hello_word);
    server.run("127.0.0.1:8080")
}
