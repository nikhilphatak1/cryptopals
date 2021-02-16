fn main() {
    println!("crypto pals");
    let set_number = std::env::args().nth(1).expect("no set number given").parse::<i32>().unwrap();
    let problem_number = std::env::args().nth(2).expect("no problem number given").parse::<i32>().unwrap();
}

