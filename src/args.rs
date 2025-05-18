use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "HTTPS SERVER",
    version = "0.0.1",
    about = "RUST HTTPS SERVER EDUCATIONAL"
)]
pub struct Args {
    #[arg(long, short, default_value_t = false)]
    pub server: bool,
    #[arg(long, short, default_value_t = false)]
    pub user: bool,

    pub username: String,
    pub pass_word: String,
}
