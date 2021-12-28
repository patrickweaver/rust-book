fn main() {
    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    println!("s1: {},\ns2: {},\ns3: {}", s1, s2, s3);

    let mut s1 = String::from("foo");
    s1.push_str("bar");
    let s2 = "beep";

    println!("s4: {}", s1);

    s1.push_str(s2);
    println!("s4: {}", s1);

    s1.push('y');
    println!("s4: {}", s1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("s8: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("s: {}", s);

    let s1 = String::from("tic");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    let len = String::from("Hola").len();
    println!("len Hola: {}", len);

    let len = String::from("👩‍👩‍👦‍👦👨‍❤️‍💋‍👨").len();
    println!("len 👩‍👩‍👦‍👦👨‍❤️‍💋‍👨: {}", len);

    let e = "👩‍👩‍👦‍👦👨‍❤️‍💋‍👨";
    let es0 = &e[0..4];
    let es01 = &e[4..7];
    let es1 = &e[0..7];
    let es2 = &e[0..14];
    let es3 = &e[0..21];
    let es4 = &e[0..25];
    let es5 = &e[0..29];
    let es6 = &e[29..52];

    println!("es0: {}\nes01: {}\nes1: {}\nes2: {}\nes3: {}\nese4: {}\nes5: {}\nes6: {}", es0, es01, es1, es2, es3, es4, es5, es6);

    let mut index = 0;
    for c in e.chars() {
        println!("{}: {}", index, c);
        index += 1;
    }

    let mut index = 0;
    for b in e.bytes() {
        println!("{}: {}", index, b);
        index += 1;
    }
}

