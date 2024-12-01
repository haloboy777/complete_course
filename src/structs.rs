// #[derive(Debug)]
// struct User {
//     username: String,
//     sign_in_count: u64,
//     active: bool,
// }

#[derive(Debug)]
struct Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn what_is_my_width(&self) -> u32 {
        self.width
    }
    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let mut sq = Square {
        width: 10,
        height: 5,
    };

    println!("{}", sq.area());
    println!("{}", sq.what_is_my_width());

    sq.change_width(20);
    println!("{:?}", sq);

    // let user1 = build_user("Anushka".to_string());
    // println!("{:?}", user1);

    // let user2 = build_user(String::from("Ayush"));
    // println!("{:?}", user2);
}

// fn build_user(name: String) -> User {
//     User {
//         username: name,
//         sign_in_count: 1,
//         active: true,
//     }
// }
