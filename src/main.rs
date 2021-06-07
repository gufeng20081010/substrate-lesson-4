use std::f32::consts::PI;

fn main() {
    // question 1
    let light = TrafficLight::Red;
    println!("light time is: {}", light.time());

    // question 2
    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    let sumVal = cal_sum(&v);
    match sumVal {
        Some(x) => println!("Some: x={}", x),
        None => println!("None") // None
    }

    // question 3
    let square = Square {
        width: 4.0,
        height: 4.0,
    };
    print_area(square);

    let triangle = Triangle {
        base: 6.0,
        height: 4.0,
    };
    print_area(triangle);

    let circle = Circle {
        radius: 5.0,
    };
    print_area(circle);
}


//------------------------ question 1 start ------------------------

pub trait ShowTime {
    fn time(&self) -> u8;
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl ShowTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 10,
            TrafficLight::Green => 20,
            TrafficLight::Yellow => 5,
        }
    }
}
// ------------------------ question 1 end ------------------------

// ------------------------ question 2 start ------------------------
fn cal_sum(vec: & Vec<u32>) -> Option<u32> {
    let mut tmp: u32 = 0;

    for &item in vec.iter() {
        tmp += item;
    }

    Some(tmp)
}
// ------------------------ question 2 end ------------------------

// ------------------------ question 3 start ------------------------
pub trait Shape {
    fn area(&self) -> f32;
}

// 正方形
struct Square {
    width: f32,
    height: f32,
}

impl  Shape for Square  {
    fn area(&self) -> f32 {
        (self.width * self.height) as f32
    }
}

// 三角形
struct Triangle {
    base: f32,
    height: f32,
}

impl  Shape for Triangle  {
    fn area(&self) -> f32 {
        (self.base * self.height * 0.5) as f32
    }
}

// 圆形
struct Circle {
    radius: f32,
}

impl  Shape for Circle  {
    fn area(&self) -> f32 {
        PI * self.radius.powf(2.0)
    }
}

// 打印
pub fn print_area<T: Shape >(item: T) {
    println!("area is {}", item.area());
}
// ------------------------ question 3 end ------------------------
