pub mod table {
    pub fn integer (number:usize) {
        println!("The table of {} is as follow:\n", number);
        for count in 1..=10 {
            println!("{} X {} = {}", number, count, number * count);
        }
    }
}
