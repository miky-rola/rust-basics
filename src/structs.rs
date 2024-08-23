

use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct User {
    email: String,
    username: String,
    verified: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        verified: false,
    }
}

fn main(){
    let user1_email: String = String::from("mikyrola8@gmail.com");
    let user1_username: String = String::from("mikyrola");
    let user1 = build_user(user1_email, user1_username);
    let user2_email: String = String::from("mikyrola@gmail.com");
    let user2_username: String = String::from("miky");
    let user2 = build_user(user2_email, user2_username);
    let user3_email: String = String::from("miky@gmail.com");
    let user3_username: String = String::from("rola");
    let user3 = build_user(user3_email, user3_username );

    let mut file = File::create("data.txt").expect("unable to create file");
    writeln!(file, "{:?}", user1).expect("couldnt write file");
    writeln!(file, "{:?}", user2).expect("couldnt write file");
    writeln!(file, "{:?}", user3).expect("couldnt write file");
    println!("User has successfully been created");

}
