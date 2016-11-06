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
extern crate concurrent_hashmap;
extern crate plugin;
extern crate urlparse;

extern crate battleplanes;

use std::path::Path;
use std::sync::{Arc, RwLock};
use std::fmt;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use staticfile::Static;
use uuid::Uuid;

use iron_sessionstorage::traits::*;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;

use concurrent_hashmap::ConcHashMap;

#[derive(Clone)]
struct SessionId(String);

impl SessionId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl iron_sessionstorage::Value for SessionId {
    fn get_key() -> &'static str { "sessionid" }
    fn into_raw(self) -> String { self.0 }
    fn from_raw(value: String) -> Option<Self> {
        // TODO: validate uuid right format
        Some(SessionId(value))
    }
}

// TODO: wrap data in the hashmap in Arc's
#[derive(Clone)]
pub struct GamePool {
    games: ConcHashMap<String, battleplanes::Game>,
    ai_initial_boards: ConcHashMap<String, battleplanes::Board>,
}

impl GamePool {
    fn find_initial_ai_board(&mut self, key: String) -> battleplanes::Board {
        match self.ai_initial_boards.find_mut(&key) {
            Some(mut board) => board.get().clone(),
            None => {
                self.ai_initial_boards.insert(key.clone(), battleplanes::Board::new_random());
                self.ai_initial_boards.find(&key).unwrap().get().clone()
            },
        }
    }
    fn find_game(&mut self, key: String) -> &mut battleplanes::Game {
        match self.games.find_mut(&key) {
            Some(mut game) => game.get(),
            None => {
                self.games.insert(key.clone(), battleplanes::Game::new_random_starter());
                self.games.find_mut(&key).unwrap().get()
            }
        }
    }
}
impl fmt::Display for GamePool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut len : usize = 0;
        for (key, value) in self.ai_initial_boards.iter() {
            //println!("map: {} = {}", key, value);
            len += 1;
        }
        write!(f, "{}", len)
    }
}

#[derive(Clone)]
pub struct GamePoolMiddleware {
    data: Arc<RwLock<GamePool>>,
}
impl GamePoolMiddleware {
    fn new() -> GamePoolMiddleware {
        GamePoolMiddleware {
            data: Arc::new(RwLock::new(GamePool {
                games: ConcHashMap::<String, battleplanes::Game>::new(),
                ai_initial_boards: ConcHashMap::<String, battleplanes::Board>::new(),
            })),
        }
    }
}

impl iron::typemap::Key for GamePoolMiddleware { type Value = Arc<RwLock<GamePool>>; }

impl iron::BeforeMiddleware for GamePoolMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<GamePoolMiddleware>(self.data.clone());
        Ok(())
    }
}

impl<'a, 'b> plugin::Plugin<Request<'a, 'b>> for GamePoolMiddleware {
    type Error = String;
    fn eval(req: &mut Request<'a, 'b>) -> Result<Arc<RwLock<GamePool>>, String> {
        req.extensions.get::<GamePoolMiddleware>().cloned().ok_or("Not found".to_string())
    }
}

/*
// TODO: helper method attached to request
// example of clean implementation of middleware:
// https://github.com/iron/iron-sessionstorage/blob/master/src/lib.rs
pub trait GamePoolRequestExt {
    fn gamepool(&mut self) -> &mut GamePool;
}

impl<'a, 'b> GamePoolRequestExt for Request<'a, 'b> {
    fn gamepool(&mut self) -> &mut GamePool {
        let mut middleware = self.extensions.get_mut::<GamePoolMiddleware>().unwrap();
        middleware.write().ok().unwrap()
    }
}
*/

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
                    script src="https://ajax.googleapis.com/ajax/libs/jquery/3.1.1/jquery.min.js" { }
                    script src="/assets/js/script.js" { }
                }
            }
        }
    }
    pub fn battleplanes_board(board: &::battleplanes::Board, id: &String) -> maud::Markup {
        let grid = get_normalized_grid(board);
        html! {
            table.battleplanes-board id=(id) {
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

    pub fn player_boards_as_html(left: &::battleplanes::Board, right: &::battleplanes::Board, gameplay: &::battleplanes::GamePlay) -> maud::Markup {
        let left_markup = battleplanes_board(left, &"own_board".to_string());
        let right_markup = battleplanes_board(right, &"own_scrapbook".to_string());
        let left_form = match gameplay {
            &::battleplanes::GamePlay::YouPlaceNewPlane => {
                html! {
                    form {
                        input name="new_head" id="new_head" /
                        input name="new_orientation" id="new_orientation" /
                        input type="submit" value="Position New Plane" /
                    }
                }
            },
            _ => {
                html! {
                }
            }
        };
        html! {
            table {
                tbody {
                    tr {
                        td id="player_board_wrapper" {
                            (left_markup)
                            (left_form)
                        }
                        td id="player_scrapbook_wrapper" {
                            (right_markup)
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

    fn get_normalized_grid(board: &::battleplanes::Board) -> Vec<Vec<HtmlCellProperties>> {
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
            grid[head_y][head_x].class = format!("plane-{}", plane.id());

            for tile in plane.coordinate_iterator().filter_map(|t| t) {
                let (tile_x, tile_y) = tile.as_tuple();
                grid[tile_y][tile_x].content = " ".to_string();
                grid[tile_y][tile_x].class = format!("plane-{}", plane.id());
            }
        }
        for hit in board.hits() {
            let (hit_x, hit_y) = hit.as_tuple();
            grid[hit_y][hit_x].content = "✕".to_string();
        }
        for miss in board.misses() {
            let (miss_x, miss_y) = miss.as_tuple();
            grid[miss_y][miss_x].content = "●".to_string();
        }
        for killed in board.killed_planes() {
            let (killed_x, killed_y) = killed.head().as_tuple();
            grid[killed_y][killed_x].content = "✕".to_string();

            grid[killed_y][killed_x].class = format!("plane-killed-{}", killed.id());
            
            for tile in killed.coordinate_iterator().filter_map(|t| t) {
                let (tile_x, tile_y) = tile.as_tuple();
                grid[tile_y][tile_x].content = " ".to_string();
                grid[tile_y][tile_x].class = format!("plane-killed-{}", killed.id());
            }
        }

        grid
    }
}

fn action_randomgrid(req: &mut Request) -> IronResult<Response> {
    let sessionid = match try!(req.session().get::<SessionId>()) {
        Some(sessionid) => sessionid,
        None => SessionId(Uuid::new_v4().hyphenated().to_string().to_owned()),
    };

    let mut t = req.get::<GamePoolMiddleware>();
    let mut arc : Arc<RwLock<GamePool>> = t.ok().unwrap();
    let mut gamepool = arc.write().ok().unwrap();
    let mut resp = Response::new();

    let ai_board = gamepool.find_initial_ai_board(sessionid.clone().to_string());

    let index_markup = template::battleplanes_board(&ai_board, &"ai_board".to_string());
    let template = template::with_layout(index_markup);
    try!(req.session().set(sessionid));
    resp.set_mut(template).set_mut(status::Ok);
    Ok(resp)
}

fn action_index(req: &mut Request) -> IronResult<Response> {
    let sessionid = match try!(req.session().get::<SessionId>()) {
        Some(sessionid) => sessionid,
        None => SessionId(Uuid::new_v4().hyphenated().to_string().to_owned()),
    };

    let mut t = req.get::<GamePoolMiddleware>();
    let mut arc : Arc<RwLock<GamePool>> = t.ok().unwrap();
    let mut gamepool = arc.write().ok().unwrap();
    let mut resp = Response::new();

    let ai_board = { gamepool.find_initial_ai_board(sessionid.clone().to_string()) };
    let mut game = { gamepool.find_game(sessionid.clone().to_string()) };
    match game.gameplay {
        battleplanes::GamePlay::YouPlaceNewPlane => {
            match req.url.query() {
                Some(query) => {
                    let params = urlparse::parse_qs(query);
                    match (params.get(&"new_head".to_string()), params.get(&"new_orientation".to_string())) {
                        (Some(maybe_new_head), Some(maybe_new_orientation)) => {
                            let new_head = maybe_new_head.get(0).unwrap().as_str();
                            let new_orientation = maybe_new_orientation.get(0).unwrap().as_str();
                            match game.board_you.add_new_plane_at(new_head, new_orientation) {
                                Ok(_) => {
                                    game.next_logical_state();
                                },
                                Err(msg) => {
                                    println!("Error in {} on {}: {}", file!(), line!(), msg);
                                }
                            };
                        },
                        _ => {
                            //TODO: error feedback
                            println!("Error in {} on {}: invalid head or orientation in query: {}", file!(), line!(), query);
                        },
                    };
                },
                None => { },
            }
        },
        _ => {
            // TODO: more pattern matching
        }
    }

    let index_markup = template::player_boards_as_html(&game.board_you, &game.scrapbook_you, &game.gameplay);
    let template = template::with_layout(index_markup);
    try!(req.session().set(sessionid));
    resp.set_mut(template).set_mut(status::Ok);
    Ok(resp)
}

fn action_hits(req: &mut Request) -> IronResult<Response> {
    let sessionid = match try!(req.session().get::<SessionId>()) {
        Some(sessionid) => sessionid,
        None => SessionId(Uuid::new_v4().hyphenated().to_string().to_owned()),
    };

    let mut t = req.get::<GamePoolMiddleware>();
    let mut arc : Arc<RwLock<GamePool>> = t.ok().unwrap();
    let mut gamepool = arc.write().ok().unwrap();

    //TODO: better handling, without clone possible?
    try!(req.session().set(sessionid));
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
    let gamepool = GamePoolMiddleware::new();
    chain.link_before(gamepool);
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
