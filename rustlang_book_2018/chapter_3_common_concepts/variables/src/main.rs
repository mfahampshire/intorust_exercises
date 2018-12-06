/// Mutability
fn mutability() {
    let mut x = 5;
    println!("MUTABILITY: The value of x is: {}", x);
    x = 6;
    println!("MUTABILITY: The value of x is now: {}", x);
}

///Shadowing
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("SHADOWING: The value of x is: {}", x);

    mutability();
}
