#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointBoth<T, U> {
    x: T,
    y: U,
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let char_list = vec!['f', 'B', 'z', 's'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 4.0 };

    println!("{:?}", integer_point);
    println!("{:?}", float_point);

    println!("integer_point.x = {}", integer_point.x());
    println!("float_point.x = {}", float_point.x());

    //println!("distance integer_point: {}", integer_point.distance_from_origin());
    println!(
        "distance float_point: {}",
        float_point.distance_from_origin()
    );

    let both_integer = PointBoth { x: 5, y: 10 };
    let both_float = PointBoth { x: 1.0, y: 4.0 };
    let integer_and_float = PointBoth { x: 5, y: 4.0 };

    println!("{:?}", both_integer);
    println!("{:?}", both_float);
    println!("{:?}", integer_and_float);
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
