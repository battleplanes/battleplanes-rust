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

    pub fn player_boards_as_html(left: &::battleplanes::Board,
                                 right: &::battleplanes::Board,
                                 gameplay: &::battleplanes::GamePlay,
    ) -> maud::Markup {
        let left_markup = battleplanes_board(left, &"own_board".to_string());
        let right_markup = battleplanes_board(right, &"own_scrapbook".to_string());
        let left_form = match gameplay {
            &::battleplanes::GamePlay::YouPlaceNewPlane => {
                html! {
                    form {
                        input name="new_head" id="new_head" type="hidden" /
                        input name="new_orientation" id="new_orientation" type="hidden" /
                        input type="submit" value="Send Gray Plane to Mission" id="send_to_mission" /
                    }
                }
            },
            _ => {
                html! {
                }
            }
        };
        let right_form = match gameplay {
            &::battleplanes::GamePlay::YouBombard => {
                html! {
                    form id="bombard_form" {
                        input name="new_hit" id="new_hit" /
                        input type="submit" value="Bombard" /
                    }
                }
            },
            _ => {
                html! {
                }
            }
        };
        let top_notice = match gameplay {
            &::battleplanes::GamePlay::YouPlaceNewPlane => {
                html! {
                    tr {
                        td colspan="2" {
                            p {
                                "Use left click on the left board to position plane,"
                            }
                            p {
                                "Right click to rotate,"
                            }
                            p {
                                "And click submit to send plane to mission."
                            }
                        }
                    }
                }
            },
            &::battleplanes::GamePlay::YouBombard => {
                html! {
                    tr {
                        td colspan="2" {
                            p {
                                "Left click on the righthandside board to bombard the opponent."
                            }
                        }
                    }
                }
            },
            &::battleplanes::GamePlay::YouWon => {
                html! {
                    tr {
                        td colspan="2" {
                            "You won!"
                        }
                    }
                }
            },
            &::battleplanes::GamePlay::OpponentWon => {
                html! {
                    tr {
                        td colspan="2" {
                            "Opponent won!"
                        }
                    }
                }
            },
            _ => {
                html! {
                }
            },
        };
        html! {
            table {
                tbody {
                    (top_notice)
                    tr {
                        td id="player_board_wrapper" {
                            (left_markup)
                            div class="centered" {
                                (left_form)
                            }
                        }
                        td id="player_scrapbook_wrapper" {
                            (right_markup)
                            (right_form)
                        }
                    }
                    tr {
                        td.centered {
                            (left.get_previous_hit_message())
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
        for kill in board.kills() {
            let (kill_x, kill_y) = kill.as_tuple();
            grid[kill_y][kill_x].content = "✕".to_string();
            grid[kill_y][kill_x].class = format!("plane-killed");
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
    pub fn single_link_page(title: &String, link: &String) -> maud::Markup {
        html! {
            a href=(link) {
                (title)
            }
        }
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

    let mut will_redirect = false;

    let ai_board = { gamepool.find_initial_ai_board(sessionid.clone().to_string()) };
    println!("{}", ai_board);
    let mut game = { gamepool.find_game(sessionid.clone().to_string()) };
    match game.gameplay {
        battleplanes::GamePlay::YouPlaceNewPlane => {
            match req.url.query() {
                Some(query) => {
                    resp.headers.set(iron::headers::Location("/".to_string()));
                    will_redirect = true;
                    resp.set_mut(status::Found);
                    let params = urlparse::parse_qs(query);
                    match (params.get(&"new_head".to_string()), params.get(&"new_orientation".to_string())) {
                        (Some(maybe_new_head), Some(maybe_new_orientation)) => {
                            let new_head = maybe_new_head.get(0).unwrap().as_str();
                            let new_orientation = maybe_new_orientation.get(0).unwrap().as_str();
                            match game.board_you.add_new_plane_at(new_head, new_orientation) {
                                Ok(_) => {
                                    game.next_logical_state();
                                    if game.gameplay == battleplanes::GamePlay::OpponentPlacesNewPlane {
                                        let current_index = &game.board_opponent.planes().len();
                                        let new_plane = &ai_board.planes()[*current_index];
                                        let new_head = format!("{}", new_plane.head());
                                        let new_orientation = format!("{}", new_plane.orientation());
                                        match game.board_opponent.add_new_plane_at(new_head.as_str(), new_orientation.as_str()) {
                                            Ok(_) => {
                                                game.next_logical_state();
                                            },
                                            Err(msg) => {
                                                println!("Error in {} on {}: {}", file!(), line!(), msg);
                                            }
                                        };
                                    }
                                    if game.gameplay == battleplanes::GamePlay::OpponentBombards {
                                        game.opponent_hits_randomly();
                                        game.next_logical_state();
                                    }
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
        battleplanes::GamePlay::OpponentPlacesNewPlane => {
            let current_index = &game.board_opponent.planes().len();
            let new_plane = &ai_board.planes()[*current_index];
            let new_head = format!("{}", new_plane.head());
            let new_orientation = format!("{}", new_plane.orientation());
            match game.board_opponent.add_new_plane_at(new_head.as_str(), new_orientation.as_str()) {
                Ok(_) => {
                    game.next_logical_state();
                },
                Err(msg) => {
                    println!("Error in {} on {}: {}", file!(), line!(), msg);
                }
            };
        },
        battleplanes::GamePlay::YouBombard => {
            match req.url.query() {
                Some(query) => {
                    resp.headers.set(iron::headers::Location("/".to_string()));
                    will_redirect = true;
                    resp.set_mut(status::Found);
                    let params = urlparse::parse_qs(query);
                    match params.get(&"new_hit".to_string()) {
                        Some(maybe_new_hit) => {
                            let new_hit = maybe_new_hit.get(0).unwrap().as_str();
                            match game.you_hit_at(new_hit) {
                                battleplanes::BombardmentResult::Hit => {
                                    println!("You've hit at {}", new_hit);
                                    game.next_logical_state();
                                },
                                battleplanes::BombardmentResult::Miss => {
                                    println!("You've missed at {}", new_hit);
                                    game.next_logical_state();
                                },
                                battleplanes::BombardmentResult::Kill => {
                                    println!("You've killed at {}", new_hit);
                                    game.next_logical_state();
                                },
                                battleplanes::BombardmentResult::Retry => {
                                    println!("Retry");
                                },
                            };
                            game.opponent_hits_randomly();
                            game.next_logical_state();
                        },
                        None => {
                        },
                    };
                },
                None => {
                },
            };
        },
        battleplanes::GamePlay::OpponentBombards => {
            game.opponent_hits_randomly();
            game.next_logical_state();
        },
        battleplanes::GamePlay::YouWon => {
            resp.status = Some(iron::status::Found);
            resp.headers.set(iron::headers::Location("/youwon".to_string()));
            return Ok(resp);
        },
        battleplanes::GamePlay::OpponentWon => {
            resp.status = Some(iron::status::Found);
            resp.headers.set(iron::headers::Location("/youlost".to_string()));
            return Ok(resp);
        },
    }

    let index_markup = template::player_boards_as_html(&game.board_you, &game.scrapbook_you, &game.gameplay);
    let template = template::with_layout(index_markup);
    try!(req.session().set(sessionid));
    resp.set_mut(template);
    Ok(resp)
}

fn action_youwon(req: &mut Request) -> IronResult<Response> {
    let sessionid = match try!(req.session().get::<SessionId>()) {
        Some(sessionid) => sessionid,
        None => SessionId(Uuid::new_v4().hyphenated().to_string().to_owned()),
    };
    let mut resp = Response::new();

    let mut t = req.get::<GamePoolMiddleware>();
    let mut arc : Arc<RwLock<GamePool>> = t.ok().unwrap();
    let mut gamepool = arc.write().ok().unwrap();
    let mut game = { gamepool.find_game(sessionid.clone().to_string()) };

    if game.gameplay != battleplanes::GamePlay::YouWon {
        resp.headers.set(iron::headers::Location("/".to_string()));
        resp.set_mut(status::Found);
        return Ok(resp);
    }

    let new_sessionid = SessionId(Uuid::new_v4().hyphenated().to_string().to_owned());
    match (std::env::var("PRIZE_TITLE"), std::env::var("PRIZE_LINK")) {
        (Ok(link_title), Ok(link_dest)) => {
            let won_markup = template::single_link_page(&link_title, &link_dest);
            let template = template::with_layout(won_markup);
            resp.set_mut(template);
        },
        _ => {
            resp.headers.set(iron::headers::Location("/".to_string()));
            resp.set_mut(status::Found);
        }
    }

    try!(req.session().set(new_sessionid));
    Ok(resp)
}

fn action_youlost(req: &mut Request) -> IronResult<Response> {
    let sessionid = match try!(req.session().get::<SessionId>()) {
        Some(sessionid) => sessionid,
        None => SessionId(Uuid::new_v4().hyphenated().to_string().to_owned()),
    };

    let mut t = req.get::<GamePoolMiddleware>();
    let mut arc : Arc<RwLock<GamePool>> = t.ok().unwrap();
    let mut gamepool = arc.write().ok().unwrap();

    //TODO: better handling, without clone possible?
    try!(req.session().set(sessionid));
    Ok(Response::with((status::Ok, format!("youlost: {}", *gamepool))))
}

fn action_env(req: &mut Request) -> IronResult<Response> {
    let mut stringified_env = String::new();
    for (var, val) in std::env::vars() {
        stringified_env.push_str(var.as_str());
        stringified_env.push_str("=");
        stringified_env.push_str(val.as_str());
        stringified_env.push_str("\n");
    }
    Ok(Response::with((status::Ok, stringified_env)))
}

fn main() {
    //TODO: load secret from env
    let my_secret = b"verysecret".to_vec();
    env_logger::init().unwrap();

    let mut router = Router::new();
    router.get("/", action_index);
    router.get("/randomgrid", action_randomgrid);
    router.get("/youwon", action_youwon);
    router.get("/youlost", action_youlost);
    router.get("/env", action_env);

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
