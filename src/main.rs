fn calculate_length(s: &String) -> usize{
    s.len()
}

fn main(){
    let s: String = String::from("Hello world");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len)
}