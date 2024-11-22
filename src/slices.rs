pub fn run() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];

    let word = first_word(&s);
    // s.clear(); // this fails since immutable reference borrowed before

    println!("The first word is: {}", word);

    let mut s = String::from("hello world");
    let s2 = "hello world";
    let word = first_word(&s2);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..2];
}
