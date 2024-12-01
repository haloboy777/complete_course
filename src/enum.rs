enum Pets {
    Dog,
    Cat,
    Fish,
}

#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IPAddrKind,
    address: String,
}

impl Pets {
    fn what_am_i(&self) -> &str {
        match self {
            Pets::Cat => "I am a Cat",
            Pets::Dog => "I am a Dog",
            Pets::Fish => "I am a Fish",
        }
    }
}

fn main() {
    let dog = Pets::Dog;
    let home = IPAddr {
        kind: IPAddrKind::V4,
        address: "10.10.10.10".to_string(),
    };

    println!("{:?}", home);


    let loopback = IPAddr {
      kind: IPAddrKind::V6,
      address: "::1".to_string(),
    };

    println!("{:?}", loopback);

    println!("{}", dog.what_am_i());
}
