fn main() {
    // let some_num = Some(5);
    // let some_str = Some(String::from("string"));
    // let nothing: Option<i32> = None;

    // let x: i32 = 5;
    // let y: Option<i32> = Some(5);

    // let sum = x + y;
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six: {:?}", six);
    println!("none: {:?}", none);

    let dog = Some("Dog");
    if let Some("Dog") = dog {
        println!("the animal is a dog");
    } else {
        println!("not a dog");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let x = 2;
    match x {
        1 | 2 => println!("one or two"),
        1..=5 => println!("matches"),
        _ => println!("not one or two"),
    }

    let k = Some(120);
    let l = 5;

    match k {
        Some(10) => println!("ten!"),
        Some(x) if x == l => println!("Matches"),
        _ => println!("dont know")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octogon,
}

impl Shape {
    fn get_corners(&self) -> i8 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octogon => 8,
        }
    }
}

fn main() {
    let shape = Shape::Octogon;

    println!("{}", shape.get_corners());
}
