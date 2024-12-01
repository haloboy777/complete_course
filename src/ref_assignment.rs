fn main() {
    // let val = vec![1, 3, 5, 7];
    let val = vec![];
    let result = check_val(&val);
    println!("{}", result);
    println!("{:?}", val);

    let mut x = 5;
    x = add_two(x);
    println!("{}", x);
}

fn check_val(val: &Vec<i32>) -> bool {
    match val.get(0) {
        Some(&1) => true,
        _ => false,
    }
}

fn add_two(val: i32) -> i32 {
    val + 2
}
