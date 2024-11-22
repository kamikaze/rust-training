pub fn run() {
    fn b() {
        let x = String::from("world");
    }

    fn a() {
        let x = "hello";
        let y = 22;
        b();
    }

    {
        let s = "hello";
        let s = String::from("hello");
    }

    let x = 5;
    let y = x; // Copy

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);

    fn gives_ownership() -> String {
        let some_string = String::from("hello");

        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);
}
