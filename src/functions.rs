fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}

fn control_flow() {
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is: {}", number);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number)
    }
    
    // Line comment
    
    /*
       Block comment
     */
}

pub fn run() {
    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);

    control_flow();
}
