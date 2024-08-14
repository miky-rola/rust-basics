fn change(string: &mut String){
    string.push_str("world")
}

fn main(){
    let mut s = String::from("Hello");

    let result = change(&mut s);
}