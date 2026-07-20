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
    templates::render_form(&mut html);
    Response::html(StatusCode::Ok, &html)
}

fn form_post<'buf, 'req>(req: &'req Request<'buf>) -> Response<'req> {
    let result = req.extract_form(["name", "message"]);
    match result {
        Ok(values) => {
            let name = values[0];
            let message = values[1];
            let html = format!("<div>Name: {} - Message: {}</div>", name, message);
            Response::html(StatusCode::Ok, &html)
        }
        Err(error) => Response::text_plain(StatusCode::BadRequest, &error.to_string()),
    }
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.routes.get("/demo/form", home);
    server.routes.post("/demo/form", form_post);
    server.run("127.0.0.1:8080")
}
