fn main() {
    // scope
    // let var = 5;
    // let mut s = "hello to ".to_string();
    // s.push_str("ayush");

    // println!("var: {}", s);

    // move example
    // let x = vec!["tyler".to_string()];
    // let y = x;
    // let z = y;
    // println!("{:?}", z);

    // clone
    // let x = vec!["tyler".to_string()];
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?} {:?} {:?}", x, y, z);

    // copy
    // let x = 1;
    // let y = x;
    // println!("x: {}, y: {}", x, y);

    // let s = String::from("hello"); // create a variable with string "hello"
    // takes_ownership(s); // give ownership to takes_ownership
    //                     // println!("{}", s); // error: value borrowed here after move

    // let x = 5;
    // takes_ownership_int(x);
    // println!("{}", x);

    // // let s1 = String::from("hello");
    // let s2 = give_ownership();
    // println!("{}", s2);

    // let s3 = take_and_give(s2);
    // println!("{}", s3);

    // if (true) {
    //     let s4 = s3;
    // } else {
    //     let s5 = s3;
    // }
    // println!("{}", s4);

    // let mut str1 = String::from("ayush");
    // let mut str2: String;

    // loop {
    //     str2 = str1;
    // }

    let mut s = String::from("hello");
    chage_string(&mut s);
    println!("{}", s);
}

fn chage_string(s: &mut String) {
    s.push_str(", ayush");
}

// fn takes_ownership(some_string: String) {
//     let another_string = some_string;
//     println!("{}", another_string);
// }

// fn takes_ownership_int(some_int: u32) {
//     let another_int = some_int;
//     println!("a: {}", another_int);
// }

// fn give_ownership() -> String {
//     "given to string".to_string()
// }

// fn take_and_give(s: String) -> String {
//     // let s1 = String::from("hello");
//     s
// }

// var is dropped
// s is dropped
