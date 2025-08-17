#[allow(unused_imports)]
use clap::Parser;

#[derive(Parser)]
#[command(name = "chessy",version = "0.1.0")]
struct Cli {}

fn main() {
    let _cli = Cli::parse();
    println!("================================================");
    println!("             Chessy engine ready                ");
    println!("================================================");

    let engine_info = chessy_engine::get_info();
    println!("================================================");
    println!("        Engine Name: {}", engine_info.name);
    println!("        Engine Author: {}", engine_info.author);
    println!("================================================");

    chessy_engine::new_game();
    println!("================================================");
    println!("             Game started successfully           ");
    println!("================================================");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn does_name_fn_work() {
        assert_eq!(chessy_engine::engine_name(), "chessy engine");
    }

    #[test]
    fn test_new_game() {
        chessy_engine::new_game();
    }
}
