
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "HTTPS SERVER", version = "0.0.1", about = "RUST HTTPS SERVER EDUCATIONAL")]

pub struct Args{
    #[arg(long,short)]
    pub server: bool = false,
    #[arg(long,short)]
    pub user:bool = false,

    pub username: String,
    pub pass_word: String,
}