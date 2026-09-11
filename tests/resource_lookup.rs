use life::{
    app::resource::model::Resource, constant::RESOURCE_COLLECTION, http::request::Request,
    route::create_routes, server::Server, state::State, storage::store::Store,
};
use std::{fs, path::PathBuf};

struct TestDirectory(PathBuf);

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn resource_routes_list_all_or_find_by_id() {
    let directory = TestDirectory(
        std::env::temp_dir().join(format!("life-resource-lookup-{}", std::process::id())),
    );
    fs::create_dir(&directory.0).unwrap();
    let store = Store::connect(directory.0.join("storage").to_str().unwrap()).unwrap();
    store.create_collection(RESOURCE_COLLECTION).unwrap();
    let mut collection = store.collection::<Resource>(RESOURCE_COLLECTION).unwrap();
    collection
        .insert_one(Resource::new("alpha".into(), 10))
        .unwrap();
    collection
        .insert_one(Resource::new("beta".into(), 20))
        .unwrap();
    collection
        .insert_one(Resource::new("deleted".into(), 30))
        .unwrap();
    collection.delete_one(3).unwrap();
    let mut server = Server::new(State { store });
    create_routes(&mut server);

    for path in ["/resources"] {
        for (query, status, expected, absent) in [
            ("", "200 OK", "Total: 2", "<h3>deleted</h3>"),
            ("?id=1", "200 OK", "<h3>alpha</h3>", "<h3>beta</h3>"),
            ("?id=2", "200 OK", "<h3>beta</h3>", "<h3>alpha</h3>"),
            (
                "?id=3",
                "404 Not Found",
                "Resource not found",
                "<h3>deleted</h3>",
            ),
            (
                "?id=99",
                "404 Not Found",
                "Resource not found",
                "<h3>alpha</h3>",
            ),
            (
                "?id=0",
                "400 Bad Request",
                "Invalid resource ID",
                "<h3>alpha</h3>",
            ),
            (
                "?id=abc",
                "400 Bad Request",
                "Invalid resource ID",
                "<h3>alpha</h3>",
            ),
            (
                "?id=",
                "400 Bad Request",
                "Invalid resource ID",
                "<h3>alpha</h3>",
            ),
            (
                "?id=4294967296",
                "400 Bad Request",
                "Invalid resource ID",
                "<h3>alpha</h3>",
            ),
        ] {
            let wire = format!("GET {path}{query} HTTP/1.1\r\nHost: localhost\r\n\r\n");
            let request = Request::parse(wire.as_bytes()).unwrap();
            let response = server.routes.handle_request(&request, &mut server.state);
            let response = String::from_utf8(response.to_bytes()).unwrap();
            assert!(
                response.starts_with(&format!("HTTP/1.1 {status}\r\n")),
                "{path}{query}: {response}"
            );
            assert!(response.contains(expected), "{path}{query}: {response}");
            assert!(!response.contains(absent), "{path}{query}: {response}");
            if status == "200 OK" {
                assert!(response.contains("method=\"GET\" action=\"/resources\""));
                if query.is_empty() {
                    assert_eq!(
                        response
                            .matches("<article class=\"resource-card\">")
                            .count(),
                        2
                    );
                    assert!(response.contains("<h3>alpha</h3>"));
                    assert!(response.contains("<h3>beta</h3>"));
                } else {
                    assert!(response.contains("Total: 1"));
                    assert_eq!(
                        response
                            .matches("<article class=\"resource-card\">")
                            .count(),
                        1
                    );
                }
            }
        }
    }
}
