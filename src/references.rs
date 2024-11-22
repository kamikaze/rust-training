pub fn run() {
    fn calculate_length(s: &String) -> usize {
        let length = s.len();
        length
    }

    fn change(s: &mut String) {
        s.push_str(", world");
    }

    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    change(&mut s1);
    println!("Changed mutable string to: {}", s1);

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // can only have one mutable reference

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // cannot mix with immutable references

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // once immutable references are out of the scope - it's possible
    println!("{}", r3);


    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }
    //
    // let ref_to_nothing = dangle();
}
