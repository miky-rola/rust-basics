// finding the memory address 
fn main(){
    let number: i32 = 32;

    let p: &i32 = &number;

    println!("The address of the number is {:p}", p);
}