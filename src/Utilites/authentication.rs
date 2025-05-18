use dotenvy::dotenv;
use std::env;


pub fn authentication_main(mut server_mode:bool,mut user_mode:bool, username: &str, pass_word: &str){
    dotenv().ok();
    let ADMIN_USERNAME = env::var("ADMIN_NAME").expect("Not in Workspace .env");
    let ADMIN_PASSWORD = env::var("ADMIN_PASS").expect("Not in workspace password .env");
    match server_mode{
        true => {
            user_mode =false;
            if username == ADMIN_USERNAME && pass_word == ADMIN_PASSWORD{
                println!("NO WAY YOU ARE THE ADMIN!!!");
            }
            else{
                server_mode =false;
            }
        
        }
        _=> {
            user_mode = true;
            
        }
 }

 if user_mode == true{
    println!("HI {}", username);
 } 


}

