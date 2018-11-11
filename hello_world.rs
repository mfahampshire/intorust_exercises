// Goals:
// - change to greet you by name
// - introduce a variable `name` and insert this into the format string
// - try `println!("{}", name)` vs `println!("{:?}", name)` and see what the difference is

fn main() {
    let name = "world";
    let formattedname = format!("pre-formatted world"); //instead of using '.to_string()' we can preformat the var to be whatever string type buffer
    let formattedplaceholdername = format!("Max {}", "Hampshire");
    println!("Hello, world!");
    println!("Hello, {}!", "world");
    println!("Hello, {}!", name);
    greet(name.to_string()); //'to_string()' method allocates a buffer in memory and copies the string literal represented by 'name' to that, creating a capital string (the parameter type passed to our greet fn)
    greet(formattedname);
    greet(formattedplaceholdername);
}
// helper funciton called 'greet' with parameter of TYPE (denoted by the colon) string called 'name'
// if it was to return something (e.g. a 32bit integer) it would read: 'fn greet(name: String) -> i32' before continuing w the function body
fn greet(name: String) {
    println!("Hello from the Greeter fn, {}!", name);
}

/*
'!' signals that 'println' and 'format' are macros
'{}' denotes a placeholder for the following variable
*/
