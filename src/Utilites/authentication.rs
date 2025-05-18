use dotenvy::dotenv;
use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize)]
struct users_permit_check_json {
    // creates structure of checking json
    user_name_json: String,
    pass_json: String,
}
//turns mode to return to main to complete authentication
fn turn_mode_to_return_to_main(mode: (bool, bool, bool)) -> (bool, bool) {
    println!("{:?}", mode);
    (true, true)
}

/// checks for permit

fn check_if_permitted(name: &str, pass: &str) -> bool {
    println!("CALLEED"); //debug
    let path: &str = "user.json"; // set file location
    let file_path = path;
    let contents = fs::read_to_string(file_path); // take in the contents

    match contents {
        Ok(json_str) => {
            let deserialized_json: Result<users_permit_check_json, serde_json::Error> =
                serde_json::from_str(&json_str); //deserializes json and puts it into a var
            println!("deserialized_json {:?}", deserialized_json); //debug

            //if value in deserialized_json match with the input username and pass with name and pass in json if true passes to permit single bool

            match deserialized_json {
                Ok(user) => {
                    if user.user_name_json == name && user.pass_json == pass {
                        println!("PASSES Permit checks");
                        return true;
                    } else {
                        println!("INVALID");
                        return false;
                    }
                }
                Err(e) => {
                    println!("WRONG WITH JSON {e}");
                    //help create new user
                    return false;
                }
            }
        }

        Err(e) => {
            println!("False not found {e}");
            return false;
        }
    };
}

//i want this authentication to look throught files

pub fn authentication_main(
    server_mode: bool,
    user_mode: bool,
    username: &str,
    pass_word: &str,
) -> (bool, bool) {
    // (ACESS_PERMIT?, USER_TYPE)if returns (1,1) ADMIN ACCCESS if returns (1, 0), user access if (0,1), (0,0) no access but counts as login tries

    dotenv().ok();
    let mut permit: bool = false;

    if server_mode || user_mode {
        permit = check_if_permitted(username, pass_word);
    } else {
        permit = false;
    }

    let mode = (server_mode, user_mode, permit);
    let return_to_main: (bool, bool) = turn_mode_to_return_to_main(mode);

    //permit selection
    return_to_main
}
