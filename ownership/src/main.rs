fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    let (s4, l) = calculate_length(s3);

    let l2 = calculate_length_2(&s4);

    println!("{}: {}", s4,  l);
    println!("{}: {}", s4, l2);

    let mut sm = String::from("hello-mutable");
    change(&mut sm);

    println!("{}", sm);

    let f = first_word(&sm);

    println!("first: {}", f);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}