use life::{
    constant::{RESOURCE_COLLECTION, STORAGE_URL}, route::create_routes, server::Server, state::State,
    storage::store::Store,
};

fn main() -> std::io::Result<()> {
    let store = Store::connect(STORAGE_URL).map_err(std::io::Error::other)?;
    store.create_collection(RESOURCE_COLLECTION)?;
    let state = State { store };
    let mut server = Server::new(state);
    create_routes(&mut server);
    server.run("127.0.0.1:8080")
}
