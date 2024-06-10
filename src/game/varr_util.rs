use std::io::stdin;

// std readline to get a raw input string and attempt to parse to integer, then return it
pub fn get_integer_input() -> i32 {
    let mut rawinput = String::new();
    let input_int;

    loop {
        stdin().read_line(&mut rawinput).expect("Failed to read user input");
        let parse_result = match rawinput.trim().parse::<i32>() {
            Ok(num) => Ok(num),
            Err(_) => Err(String::from("Invalid input. Please enter a number.")),
        };

        input_int = match parse_result {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        break;
    }
    return input_int;
}