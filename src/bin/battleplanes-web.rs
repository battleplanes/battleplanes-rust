extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate env_logger;
extern crate handlebars_iron as hbs;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use hbs::{Template, HandlebarsEngine, DirectorySource};
use router::Router;
use staticfile::Static;
use std::path::Path;

mod data {
    use std::collections::BTreeMap;

    pub fn make_data() -> BTreeMap<String, String> {
        let mut data = BTreeMap::new();
        data.insert("message".to_string(), "Hello, World!".to_string());

        data
    }
}

fn action_index(_: &mut Request) -> IronResult<Response> {
    use data::*;

    let mut resp = Response::new();
    let data = make_data();
    resp.set_mut(Template::new("layout", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main() {
    env_logger::init().unwrap();

    let mut hbse = HandlebarsEngine::new();
    hbse.add(Box::new(DirectorySource::new("./src/bin/battleplanes-web/templates/", ".hbs")));
    if let Err(r) = hbse.reload() {
        panic!("{}", r);
    }

    let mut router = Router::new();
    router.get("/", action_index);

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/assets/", Static::new(Path::new("./src/bin/battleplanes-web/assets/")));
    let mut chain = Chain::new(assets_mount);
    chain.link_after(hbse);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
