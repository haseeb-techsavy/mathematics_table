pub mod table {
    pub fn integer () {
        println!("Please enter a number to generate its table");
        let mut user_input = String::new();   
        std::io::stdin().read_line(&mut user_input).unwrap(); //Reading the user input
        println!("You entered: {}", user_input);
        let user_input:u32 = user_input.trim().parse().     //Converting string in an integer
        expect("Enter a valid number");
        println!("The table of {} is as follow:\n", user_input);
        for count in 1..=10 {
            println!("{} X {} = {}", user_input, count, user_input * count);
        }

    }
}
