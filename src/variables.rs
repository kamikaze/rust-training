pub fn run() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100000;

    let a = 99_222;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b'A';

    let f: u8 = 255;

    let f = 2.0;
    let g: f32 = 3.0;

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let reminder = 43 % 5;

    let t = true;
    let f = false;

    let c = 'z';
    let heart_eyed_cat = '😻';

    let tup = ("Magic", 100_000);
    let (name, count) = tup;
    let count = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];

}
