use std::env;

fn main() {
    println!("crypto pals");
    let args: Vec<_> = env::args().collect();
    let set_number = args[1].parse::<i32>().unwrap_or(-1);
    let challenge_number = args[2].parse::<i32>().unwrap_or(-1);

    match set_number {
        1 => match challenge_number {
            1 => set_1_challenge_1(&args[2..]),
            _ => panic!("Unsolved or nonexistent problem specified"),
        },
        _ => panic!("Unsolved or nonexistent problem specified"),
    }
}

fn set_1_challenge_1(args: &[String]) {
    let input = &args[0];
    let hex_content = match hex::decode(input) {
        Ok(content) => content,
        Err(error) => panic!("Unable to decode input string as hex: {}", error)
    };

    
}
