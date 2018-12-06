fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if_else();
    let_if();
    // inf_loop();
    loop_break();
    while_loop();
    for_loop();
    loop_rev();
}

fn if_else() {
    let numbertwo = 6;

    if numbertwo % 4 == 0 {
        println!("numbertwo is divisible by 4");
    } else if numbertwo % 3 == 0 {
        println!("numbertwo is divisible by 3");
    } else if numbertwo % 2 == 0 {
        println!("numbertwo is divisible by 2");
    } else {
        println!("numbertwo is not divisible by 4, 3, or 2");
    }
}

fn let_if() {
    let condition = true;
    let if_number = if condition {
        5
    } else {
        6
    };

    println!("The value of if_number is: {}", if_number);
}

// fn inf_loop() {
//     loop {
//         println!("again!");
//     }
// }

fn loop_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn loop_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
