use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Calling panic:
    //panic!("crash and burn");


    // Overflow panic
    // let v = vec![1,2,3];
    // v[99];


    //let f = File::open("goodbye.txt");
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("goodbye.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was a problem: {:?}", e)
                },
            }
        },
        Err(error) => {        
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    println!("{:?}", f);

    //let f2 = File::open("nofile.txt").unwrap();
    //let f2 = File::open("nofile.txt").expect("Failed to open nofile.txt");

    let u = read_username_from_file();
    match u {
        Ok(s) => println!("username: {}", s),
        Err(e) => println!("Error reading username: {:?}", e),
    }

}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    //let mut f = File::open("username.txt")?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
}
