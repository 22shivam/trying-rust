// remember to save the file before running the program
// $ rustc hello_world.rs
// $ ./hello_world
// for variable declaration, use underscore NOT hyphens. 
// lack of semi colons results in errors. 

fn main() {

    let y = if true {2} else {3}; // nice syntax
    println!("{}", y);

    println!("Hello, World!");
    let mut x = 1;
    'print_1_to_10: loop {
        println!("{}",x);
        x = x + 1;
        if x == 10 {
            break 'print_1_to_10;
        }
    }

    println!("{}", fahrenheit_to_celsius(212));
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

