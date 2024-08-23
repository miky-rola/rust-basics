
#[allow(unused_imports)] 
use std::fs::File;  
use std::io::Write;
use std::fs::OpenOptions;

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
    let user1_email: String = String::from("test1@example.com");
    let user1_username: String = String::from("test");
    let user1 = build_user(user1_email, user1_username);
    let user2_email: String = String::from("test1@example2.com");
    let user2_username: String = String::from("test2");
    let user2 = build_user(user2_email, user2_username);
    let user3_email: String = String::from("test1@example3.com");
    let user3_username: String = String::from("test3");
    let user3 = build_user(user3_email, user3_username );

    let mut file = OpenOptions::new()
        .append(true)
        .open("data.txt")
        .expect("unable to create file");
    writeln!(file, "{:?}", user1).expect("couldnt write file");
    writeln!(file, "{:?}", user2).expect("couldnt write file");
    writeln!(file, "{:?}", user3).expect("couldnt write file");
    println!("User has successfully been created");

}
