#[derive(Debug)]
struct Car {
    color: String,
    mpg: u16,
    top_speed: u16,
}

impl Car {
    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }
    fn set_top_speed(&mut self, new_top_speed: u32) {
        self.top_speed = u16::try_from(new_top_speed)
            .ok()
            .expect("new top speed is larger than u16");
    }
    fn set_mpg(&mut self, new_mpg: u16) {
      self.mpg = new_mpg;
    }
}

fn main() {
    let mut car1 = build_car("BLACK".to_string(), 100, 3113);

    println!("{:?}", car1);

    car1.set_color("BLUE".to_string());
    car1.set_top_speed(1233);
    car1.set_mpg(123);

    println!("{:?}", car1);
}

fn build_car(color: String, mpg: u16, top_speed: u64) -> Car {
    Car {
        color,
        mpg,
        top_speed: u16::try_from(top_speed)
            .ok()
            .expect("top speed out of bounds!!"),
    }
}

// #[derive(Debug)]
// struct MyString<'a> {
//   text: &'a str,
// }

// fn main() {
//   let s = String::from("my string");
//   let x = MyString{
//     text: s.as_str()
//   };
//   let y: &'static str = "i have a static lifetime";
//   println!("{:?}", x);

//   let k = MyString {
//     text: y
//   };

//   println!("{:?}", k);
// }
