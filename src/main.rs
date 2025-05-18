use clap::Parser; //brings in parser of ARGS
mod args; //looks for the args.rs
use args::Args; //gets the Args pub struct out
use std::{fs::File, io};
mod utilites;
use utilites::authentication::authentication_main;
mod gui_files;
use gui_files::admin_gui::admin_gui_main;
use gui_files::user_gui::user_gui_main;

//checks if json is created or not then create json
fn check_for_json_if_not_create() -> io::Result<()> {
    match File::open("user.json") {
        Ok(f) => f,
        Err(e) => {
            println!("File not found due to: {e}");
            println!("Creating new file");

            File::create("user.json")?
        }
    };
    Ok(())
}

fn main() {
    let cli = Args::parse(); // parses cli arguments according to the Args Structure
    //in args.rs

    // a seperate scope to take care of json
    {
        if let Err(e) = check_for_json_if_not_create() {
            println!("File could not be created because of {e}");
        }
    }

    print!("This is the arg: {:?}", cli); //prints out cli arguments to check

    println!("Mode Server = {}, client = {}", cli.server, cli.user); // check if modes are correctly taken
    //this is if server mode was turned onn
    //dotenv().ok(); //loads in env files
    //let main_admin_name = env::var("ADMIN_NAME").eJpect("ADMIN_NAME not set in .env");//gets the env for ADMIN as only admin can get into Server

    //get authentication to a seperate file
    //getting into server will open up various features like turning server onn or off blocking off a specific address

    let access_made = authentication_main(cli.server, cli.user, &cli.username, &cli.pass_word);
    println!("{:?}", access_made); //debug

    match access_made {
        (true, true) => {
            //open ADMIN GUI
            admin_gui_main();
        }
        (false, true) => {
            //open user GUI
            user_gui_main();
        }
        _ => {
            //DO NOTHING?(HELP USER OPEN NEW ACCOUNT?)
            println!("Sorry no access for you!~");
        }
    };

    pr
}
