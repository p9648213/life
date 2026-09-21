use crate::{
    app::{
        login::controller::{login, logout, register},
        resource::controller::{create_resourse, delete_resourse, list_resourse, update_resource},
    },
    server::Server,
    state::State,
};

pub fn create_routes(server: &mut Server<State>) {
    server.routes.get("/login", login);
    server.routes.post("/login", login);
    server.routes.get("/register", register);
    server.routes.post("/register", register);
    server.routes.post("/logout", logout);
    server.routes.post("/resources/create", create_resourse);
    server.routes.post("/resources/delete", delete_resourse);
    server.routes.post("/resources/update", update_resource);
    server.routes.get("/resources", list_resourse);
    server.routes.static_files("/static/", "./static");
}
