# Mathematics Table:
A rust library that will generate a math table of an integer given by the user.
## Usage:
To use this library in your code, you just need to add following line in the dependencies section of your cargo.toml file
```
[dependencies]
mathematics_table = "0.1.0"
```
Your cargo.toml file will look like this:
```
[package]
name = "mathematics_table"
version = "0.1.0"
authors = ["Haseeb ul Hassan <haseeb.ee12@gmail.com>"]
edition = "2018"

[dependencies]
mathematics_table = "0.1.0"
```
Now, just come in `src/main.rs` to use this library crate. 
Write the following lines and it will generate a math table of integer '9', we have taken '9' for example, you can add any integer in place of '9':
```
use mathematics_table;
fn main () {
   mathematics_table::table::integer(9);
}
```
Another way to use this is as follow:
```
use mathematics_table::table;
fn main () {
   table::integer(9);
}
```
Now in the end, just use cargo run to get a math table of integer '9'.

### Syntax:
Here the `mathematics_table` is name of crate, `table` is the name of module and `integer` is the name of function.

#### Note:
You can pass only integers as an argument in the `integer` function.

### Output:
The output will be like this:
```
The table of 9 is as follow:

9 X 1 = 9
9 X 2 = 18
9 X 3 = 27
9 X 4 = 36
9 X 5 = 45
9 X 6 = 54
9 X 7 = 63
9 X 8 = 72
9 X 9 = 81
9 X 10 = 90
```
