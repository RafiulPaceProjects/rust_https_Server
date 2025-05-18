use clap::Parser; //brings in parser of ARGS
mod args;//looks for the args.rs
use args::Args; //gets the Args pub struct out
use std::env;//turn on std Env file mode
use dotenvy::dotenv; // opens rust dotenvy to load in env values

mod Utilites;
use Utilites::authentication::authentication_main;


fn main() {
    let cli = Args::parse(); // parses cli arguments according to the Args Structure
    //in args.rs

    print!("This is the arg: {:?}", cli); //prints out cli arguments to check

    println!("Mode Server = {}, client = {}", cli.server, cli.user); // check if modes are correctly taken
    //this is if server mode was turned onn
        //dotenv().ok(); //loads in env files
        //let main_admin_name = env::var("ADMIN_NAME").expect("ADMIN_NAME not set in .env");//gets the env for ADMIN as only admin can get into Server

        //get authentication to a seperate file 
        //getting into server will open up various features like turning server onn or off blocking off a specific address

        authentication_main(cli.server, cli.user, &cli.username, &cli.pass_word); //passes to authentication will give back bool on either client on server and bool is accepted to give other specific features


}
