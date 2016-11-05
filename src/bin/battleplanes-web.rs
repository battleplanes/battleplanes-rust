#![feature(plugin)]
#![plugin(maud_macros)]

extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate env_logger;
extern crate maud;
extern crate iron_sessionstorage;
extern crate uuid;
extern crate persistent;

extern crate battleplanes;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;
use uuid::Uuid;

use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

struct SessionId(String);

impl iron_sessionstorage::Value for SessionId {
    fn get_key() -> &'static str { "sessionid" }
    fn into_raw(self) -> String { self.0 }
    fn from_raw(value: String) -> Option<Self> {
        // Maybe validate that only 'a's are in the string
        Some(SessionId(value))
    }
}

#[derive(Copy, Clone)]
pub struct GamePool;

impl GamePool {
    fn new() -> GamePool {
        GamePool {
        }
    }
}

impl iron::typemap::Key for GamePool { type Value = i32; }

mod data {
    use std::collections::BTreeMap;

    pub fn make_data() -> BTreeMap<String, String> {
        let mut data = BTreeMap::new();
        data.insert("message".to_string(), "Hello, World!".to_string());

        data
    }
}

mod template {
    use maud;
    pub fn with_layout(inner: maud::Markup) -> maud::Markup {
        html! {
            (maud::PreEscaped("<!doctype html>"))
            html lang="en" {
                head {
                    meta charset="utf-8" /
                    title {
                        "Battleplanes"
                    }
                    meta name="description" content="Battleplanes, a battleships-like game" /
                    meta name="author" content="Flavius Aspra <flavius.as@gmail.com>" /
                    link rel="stylesheet" href="/assets/css/reset.css?v=1.0" /
                    link rel="stylesheet" href="/assets/css/styles.css?v=1.0" /
                    (maud::PreEscaped("<!--[if lt IE 9]>"))
                        script src="https://cdnjs.cloudflare.com/ajax/libs/html5shiv/3.7.3/html5shiv.js" /
                    (maud::PreEscaped("<![endif]-->"))
                }
                body {
                    (inner)
                    script src="/assets/js/script.js" { }
                }
            }
        }
    }
    pub fn battleplanes_board(board: ::battleplanes::Board) -> maud::Markup {
        let grid = get_normalized_grid(board);
        html! {
            table.battleplanes-board {
                thead {
                    td { " " }
                    td { "A" }
                    td { "B" }
                    td { "C" }
                    td { "D" }
                    td { "E" }
                    td { "F" }
                    td { "G" }
                    td { "H" }
                    td { "I" }
                    td { "J" }
                }
                tbody {
                    @for rownum in 0..10 {
                        tr {
                            th {
                                (rownum+1)
                            }
                            @for colnum in 0..10 {
                                td class=(grid[rownum][colnum].class) { (grid[rownum][colnum].content) }
                            }
                        }
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    struct HtmlCellProperties {
        class: String,
        content: String,
    }

    fn get_normalized_grid(board: ::battleplanes::Board) -> Vec<Vec<HtmlCellProperties>> {
        let mut grid : Vec<Vec<HtmlCellProperties>> = Vec::with_capacity(10);
        for i in 0..10 {
            grid.push(Vec::new());
            for _ in 0..10 {
                grid[i].push(HtmlCellProperties {
                    class: "".to_string(),
                    content: " ".to_string(),
                });
            }
        }
        for plane in board.planes() {
            let (head_x, head_y) = plane.head().as_tuple();
            grid[head_x][head_y].class = format!("plane-{}", plane.id());

            for tile in plane.coordinate_iterator().filter_map(|t| t) {
                let (tile_x, tile_y) = tile.as_tuple();
                grid[tile_x][tile_y].content = " ".to_string();
                grid[tile_x][tile_y].class = format!("plane-{}", plane.id());
            }
        }
        for hit in board.hits() {
            let (hit_x, hit_y) = hit.as_tuple();
            grid[hit_x][hit_y].content = "✕".to_string();
        }
        for miss in board.misses() {
            let (miss_x, miss_y) = miss.as_tuple();
            grid[miss_x][miss_y].content = "●".to_string();
        }
        for killed in board.killed_planes() {
            let (killed_x, killed_y) = killed.head().as_tuple();
            grid[killed_x][killed_y].content = "✕".to_string();

            grid[killed_x][killed_y].class = format!("plane-killed-{}", killed.id());
            
            for tile in killed.coordinate_iterator().filter_map(|t| t) {
                let (tile_x, tile_y) = tile.as_tuple();
                grid[tile_x][tile_y].content = " ".to_string();
                grid[tile_x][tile_y].class = format!("plane-killed-{}", killed.id());
            }
        }

        grid
    }
}

fn action_randomgrid(r: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    let mut random_board = battleplanes::Board::new_random();
    for _ in 0..20 {
        let hit = battleplanes::Coordinate::new_random_coordinate();
        random_board.hit_at(hit);
    }

    let index_markup = template::battleplanes_board(random_board);
    let template = template::with_layout(index_markup);
    resp.set_mut(template).set_mut(status::Ok);
    Ok(resp)
}

fn action_index(r: &mut Request) -> IronResult<Response> {
    let sessionid = match try!(r.session().get::<SessionId>()) {
        Some(sessionid) => sessionid,
        None => SessionId(Uuid::new_v4().hyphenated().to_string().to_owned()),
    };
    let mut resp = Ok(Response::with(format!(
                "Reload to add a char {}", sessionid.0
    )));
    try!(r.session().set(sessionid));
    resp
}

fn action_hits(req: &mut Request) -> IronResult<Response> {
    let lock = req.get::<persistent::State<GamePool>>().unwrap();
    let mut gamepool = lock.write().unwrap();

    *gamepool += 1;
    Ok(Response::with((status::Ok, format!("Hits: {}", *gamepool))))
}

fn main() {
    //TODO: load secret from env
    let my_secret = b"verysecret".to_vec();
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/", action_index);
    router.get("/randomgrid", action_randomgrid);
    router.get("/hits", action_hits);

    let mut assets_mount = Mount::new();
    assets_mount
        .mount("/", router)
        .mount("/assets/", Static::new(Path::new("./src/bin/battleplanes-web/assets/")));
    let mut chain = Chain::new(assets_mount);
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));
    chain.link(persistent::State::<GamePool>::both(GamePool::new()));
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
