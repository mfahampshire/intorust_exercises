/*
- sometimes we dont just want to delegate the ability to read a data
structure but also to mutate it. The helper function below will not
just read a string that fn main owns but write to it also:

fn main() {
    let mut name = ...; // <-- allocate a mutable string buffer
    update(&mut name); // <-- lend the string mutably / lend a mutable reference
    println!("{}", name);
}

fn update(name: &mut String) { // <-- take a MUTABLE reference to a string
    name.push_str("..."); // -- mutate name in place
}

cf. shared borrows / reference (which you can lend infinite numbers of) you can only
have one copy at a time & when it is active (when it is IN SCOPE - altho you can of course
create a subscope like in the 'shared_borrows' notes) you lose access to the original
variable to avoid stuff like data races.

when fn helper mutates the reference it actually changes the variable that fn main
OWNS.

- three primary ways to interact with data in rust:
'name: String' : ownership - controll all access, will free when done
'name: &String' : shared reference - many readers no writers (in 99% of cases)
'name: &mut String' : mutable reference - no other readers one writer

- re scope: these things are synonymous re scope:
1.
let mut name = ...;
{
    let r = &mut name;
    helper(r);
}
2.
helper(&mut name);

number 2 is basically shorthand for 1 and is declaring a var with a very narrow scope - the scope of that function call

- key rules that allow rust to avoid using a garbage collector whilst also avoiding segmentation faults &
threads without dataraces: all based on ownership and borrowing rules:

Compile-time read-write-lock model enforced at compilation time:
Creating a shared ref to X 'read locks' X
    - other readers ok
    - no writers
    - lock lasts until ref goes out of scope
Creating a mutable ref to X "write locks" X
    - no other readers or writers
    - lock lasts until ref goes out of scope
*Never have a reader/writer at the same time*

- When we're referring to borrowing, we talk about the *lifetime* of borrows.
The lifetime = the space of code where the reference is used (synonymous(?) with scope of borrow).
Locks will occur only during lifetime of reference (for instance).

Lifetimes are referred to by a name following a tick (') e.g. 'l

- cheat sheet:
&String // type of shared reference
&mut String // type of mutable reference
&str // type of string slice

fn greet(name: &String) {..}
fn adjust(name: &mut String) {..}

&name // shared borrow
&mut name // mutable borrow
&name[x..y] // slice expression
*/

// PROBLEM: 
// pub fn main() {
//     let (mut str1, str2) = two_words(); // only str1 is declared as mutable...
//     str1 = join_words(str1, str2); // ...because we're reassigning it here
//     println!("concatenated string is {:?}", str1);
// }
//
// fn two_words() -> (String, String) {
//     (format!("fellow"), format!("Rustaceans"))
// }
//
// /// Concatenate `suffix` onto the end of `prefix`.
// fn join_words(mut prefix: String, suffix: String) -> String { // takes ownership of both strings. Argument prefix is also declared as mutable: sort of a note to self for dn join_words
//     prefix.push(' '); // separate the words with a space
//     for ch in suffix.chars() { //pushes each char of suffix and pushes onto the string
//         prefix.push(ch);
//     }
//     prefix // returns ownership to caller
// }

// Challenge: Convert `join_words` to use borrowing, not ownership.
// The new function should mutate `prefix` in place, and should not
// take ownership of `suffix`. <-- there should be not return in join_words for this to occur
//
// Hint: If you'd like a hint as to how to proceed, open
// <http://intorust.com/hint/mutable_borrow_1/>.

// Question: Now that you've converted `join_words`, what happens if you
// call `join_words` using the same string for `prefix` and `suffix`?
// Why?

// ANSWER:
pub fn main() {
    let (mut str1, str2) = two_words();
    join_words(&mut str1, &str2);
    println!("concatenated string is {:?}", str1);
}

fn two_words() -> (String, String) {
    (format!("fellow"), format!("Rustaceans"))
}

/// Concatenate `suffix` onto the end of `prefix`.
fn join_words(prefix: &mut String, suffix: &String) {
    prefix.push(' '); // separate the words with a space
    // prefix.push_str(suffix); <-- this can be used instead of the below for loop as we're pushing an entire string buffer
    for ch in suffix.chars() { //pushes each char of suffix and pushes onto the string
        prefix.push(ch);
    }
}
