fn main() {
    println!("Hello, world!");

    another_function();
    another_another_function(5);
    type_annotation(7,8);
    return_five();
    let x = no_semicolon(5);
    println!("NO_SEMICOLON: x = {}", x)
}

fn another_function() {
    println!("Another function.");
}

fn another_another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn type_annotation (x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn no_semicolon(x: i32) -> i32 {
    x + 8
}

fn return_five() {
    let a = five();

    println!("The value of a is: {}", a);
}
