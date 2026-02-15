use std::io;
fn main(){
let mut x=String::new();
let mut y=String::new();
let mut operation=String::new();
let result: i32;
//let result=String::new();

println!("enter the first number");
io::stdin().read_line(&mut x).expect("Invalid input");
let x: i32 = match x.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input!");
        return;
    }
};

println!("enter the second number");
io::stdin().read_line(&mut y).expect("Invalid input");
let y: i32 = match y.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input!");
        return;
    }
};

println!("Operations List");
println!("press 1 to +");
println!("press 2 to -");
println!("press 3 to *");
println!("press 4 to %");
println!("Please Select ");
io::stdin().read_line(&mut operation).expect("Invalid operation");


let operation: i32 = match operation.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Invalid input!");
        return;
    }
};

match operation{

1 => result= x+y,
2 => result= x-y ,
3 => result= x*y,
4 =>result= x/y,
_ => {
    println!("Invalid selection");
    return;
}

}
println!("The Result Is {} ",result);


}