/*
- "data structures in rust work pretty much the same way as books in the
real world"
- when you create a variable - e.g. create a string - you allocate some resources
and ownership of said resources is transferred to the creator (e.g. the function
that declares the var).
- "for any given resource that you've allocated, there's always an owner"
- you can move resources between owners, or you can 'drop' them and free them.
- when another function has an argument a parameter of which is an already declared
variable, it essentially states that it would like to take ownership of said
variable to use. ownership = "unique access... and control"
- when a function returns, all the data it owns (e.g. local vars, parameters, etc)
gets 'freed' or 'dropped'. "The resources get returned to the system... more concretely
something called a destructor will run...".
- It frees the memory previously allocated to that declared variable.

this is why trying to do something like this will fail (cf. 'hello_world.rs),
as 'name' has been dropped after the first execution of 'greet' in fn main.

fn main() {
    let name = format!("world");
    greet(name);
    greet(name);
}
fn greet(name)
    println!("Hello {}", name)

- this is the key to letting Rust do what it does vs garbage-collected languages /
languages that ask for refernce or access to a declared var instead of ownership. This
is also where data races stem from in these languages and why they dont occur in
Rust.

- could use .clone method (works on many data types) but this is often not the best thing to do
as you're doubling memory allocation per cloned var

- some types are 'copy types' - automatic clone every time you use the var, instead of
giving ownership of the var. Stuff like ints & floating points are copy types: simple types

- three categories of values:
1. non-copyable: values move from place to place (e.g. structs) cannot be cloned
2. Clone: run code to make a copy (e.g. strings)
3. copy: type is implicitly copied when referenced (e.g. ints or floating points)
*/

fn main() {
    let (adjective, name) = two_words(); // after ownership passed from fn two_words, pair is broken up and ownership of each is given to vars 'adjective' and 'name'
    let name = format!("{} {}", adjective, name); // glues them together to get combined string 'name'
    print_out(name);
}

/*
fn main() { // the same as above can be done as so:
    let pair = two_words(); // after ownership passed from fn two_words, ownership of each is given to vars 'adjective' and 'name'
    let name = format!("{} {}", pair.0, pair.1);
    print_out(name);
}
*/

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans")) //returns a pair that owns two strings - the pair is understood as one thing - gives back ownership of two different strings
} // if you leave off the semicolon then the above line will be returned - ownership of the pair is given to main - without having to use the 'return' keyword

/// Given a string, returns a new string that
/// is the same as the input but does not contain
/// any vowels
///
/// # Examples
///
/// ```
/// assert_eq!(remove_vowels(format!("Hello!")) == format!("Hll!"));
/// ```

/*
fn remove_vowels(name: String) -> String {
    // Goal #1: What is needed here to make this compile?
    let mut output = String::new(); // 'mut' denotes that the empty string buffer allocated here is a mutable one - buffers are immutable by default
    for c in name.chars() { //returns an iterator over characters driven by forword
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {  // if its any of these...
                // ...skip vowels
            }
            _ => { // otherwise - '_' denotes 'match any other value'...
                output.push(c); //... call this. '.push' is a string method appending to string. Sometimes you might have to append more memory to buffer if it gets too big
            }
        }
    }
    output // returns 'output' as there's no semicolon
}

fn print_out(name: String) { // takes ownership of 'name' & calls 'removes_vowels' function
    let devowelized_name = remove_vowels(name.clone()); // get string back
    println!("Removing vowels yields {:?}", devowelized_name);

    // Goal #2: What happens when you uncomment the `println` below?
    // Can you change the code above so that the code below compiles
    // successfully?

    println!("Removing vowels from {:?} yields {:?}",
             name, devowelized_name);

}
*/

// Extra credit: Can you do it without copying any data?
// (Using only ownership transfer)
// fn helper(name: String) //takes ownership
// ANSWER edit fn remove_vowels to give back a pair:

fn remove_vowels(name: String) -> (String, String) {
    // Goal #1: What is needed here to make this compile?
    let mut output = String::new(); // 'mut' denotes that the empty string buffer allocated here is a mutable one - buffers are immutable by default
    for c in name.chars() { //returns an iterator over characters driven by forword
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {  // if its any of these...
                // ...skip vowels
            }
            _ => { // otherwise - '_' denotes 'match any other value'...
                output.push(c); //... call this. '.push' is a string method appending to string. Sometimes you might have to append more memory to buffer if it gets too big
            }
        }
    }
    (output, name) // returns 'output' & 'name' as a pair as there's no semicolon
}

fn print_out(name: String) {
let (devowelized_name, name1) = remove_vowels(name); // get string back
println!("Removing vowels yields {:?}", devowelized_name);

println!("Removing vowels from {:?} yields {:?}",
     name1, devowelized_name);

}
