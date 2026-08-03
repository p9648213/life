use crate::{
    app::resource::controller::{create_resourse, list_resourse},
    server::Server,
    state::State,
};

pub fn create_routes(server: &mut Server<'_, State>) {
    server.routes.post("/resources/create", create_resourse);
    server.routes.post("/resources/delete", create_resourse);
    server.routes.get("/resources", list_resourse);
}
