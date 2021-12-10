pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
            pub fn nested_modules_2() {}
        }
    }
}

use a::series::of;
use a::series::of::nested_modules_2;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}



enum Signs {
    Stop,
    Caution,
    OneWay,
}

use TrafficLight::{Red, Yellow};
use Signs::*;

fn main() {
    of::nested_modules();
    nested_modules_2();

    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;

    let _stop = Stop;
    let _catuion = Caution;
    let _one_way = OneWay;
}