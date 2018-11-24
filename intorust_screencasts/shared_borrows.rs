/*
- back to book analogy: lending a book to someone <-> 'take ownership of this string, print it,
and give it back to me':

fn main () {
    let name = format!("..."); // as per, allocate string buffer
    let r = &name; // '&' denotes that we're borrowing the variable 'name' to create a REFERENCE which is stored in 'r'
    helper(r); // 'r' is a reference to the string 'name': fn main actually retains ownership of the string
    helper(r);
} // at end of main - as fn main retains ownership - data is freed, not at end of fn helper

fn helper (name: &String) { // doesnt expect to take ownership of a string merely a reference to a string called 'name' as denoted by '&String'
    println!(..);
}

SHARED REFERENCES ARE COPY TYPES.

- n.b. shared == immutable (99% of the time):

fn helper(name: &String) {
    println!("{}", name); // <-- this is fine as it just reads
}

fn helper(name:&String) {
    name.push('x'); // <-- error as this would mutate 'name'
}

there are occasions when borrowed buffers can be mutated but they need to go via an API like
Lock so there is an ordering involved to avoid issues.

mutability is somewhat time-based: before a buffer is borrowed it is mutable (if you declare it
as such obviously) - an error will b thrown if another function tries to mutate it AFTER a reference to it has been
created and/or this reference exists within scope:

fn main() {
    let mut name = format!("fellow Rustaceans");
    name.push('x');

    {
        let r: &String = &name; // <-- ' declare buffer called 'r' of type string ref... '
        helper(r);
        helper(r);
    } <-- borrow ends here...

    name.push('x'); // <-- ...so we can mutate here
}

fn helper(name: &String) {
    println!("Hello, {}", name);
}

- all above examples are to an entire string buffer: sometimes you dont need to do this, just
operate with a subset of that buffer. This is easy and efficient in Rust.

At runtime, a string buffer is comprised of these three bits of data:
data - reference to the buffer itself in memory, which (e.g.) contains the word 'Rustaceans'
length - amount of data we're actually using in our buffer
capacity - total space available; usually there is a greater capacity than length so we can mutate the buffer's data

fn helper(name: &str){ <-- '&str' is a string slice
    // function
}

- A string slice is a type that is a subset of a string buffer. This is almost always BORROWED from a string buffer

You can denote the scope of the subsets via something like this:

fn main() {
    let name = format!("...");
    helper(&name[1..]);
}

This lends fn helper some of the string buffer 'name': from char 1 onwards (the first byte)
This is a reference directly into the string buffer skipping the first letter. No allocation no copying.

Substrings are comprised of two bits of data at runtime:
data - reference to the buffer it is a substring of skipping the first letter (in this case)
length - amount of data in the string buffer (here minus the first letter)

*it is a borrowed reference but instead of referring to the whole buffer it refers to a subet of its data*

*/

// Goal #1: Convert `greet` to use borrowing, not ownership, so that
// this program executes without doing any cloning.
/*
pub fn main() {
    let string = format!("my friend");
    greet(&string);
    greet(&string);
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}
*/

// Goal #2: Use a subslice so that it prints "Hello, friend" instead of
// "Hello, my friend".
pub fn main() {
    let string = format!("my friend");
    greet(&string[3..]);
    greet(&string);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}
