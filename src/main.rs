use life::{
    http::{
        request::Request,
        response::{Response, StatusCode},
    },
    server::Server,
    templates::render_resource,
};

fn create_resourse<'buf, 'req>(request: &'req Request<'buf>) -> Response<'req> {
    if let Ok(form) = request.extract_form(["r_name"]) {
        println!("{:?}", form);
        Response::html(StatusCode::Ok, "Resource created")
    } else {
        Response::text_plain(StatusCode::InternalServerError, "Error Parsing Form")
    }
}

fn list_resourse<'buf, 'req>(request: &'req Request<'buf>) -> Response<'req> {
    let query = request.query().get("id");
    println!("{:?}", query);
    let mut html = String::new();
    render_resource(&mut html);
    Response::html(StatusCode::Ok, &html)
}

fn main() -> std::io::Result<()> {
    let mut server = Server::new();
    server.routes.post("/resources", create_resourse);
    server.routes.get("/resources", list_resourse);
    server.run("127.0.0.1:8080")
}
