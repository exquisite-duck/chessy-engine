pub mod board;
pub mod moves;
pub mod game;




pub fn engine_name() -> &'static str {
    "chessy engine"
}

pub fn engine_author() -> &'static str {
    "Mohammad Sohail"
}

pub struct EngineInfo {
    pub name: &'static str,
    pub author: &'static str
}

pub fn get_info() -> EngineInfo {
    EngineInfo {
        name: engine_name(),
        author: engine_author()
    }
}

pub fn new_game() {
    println!("Starting a new game...");
}
