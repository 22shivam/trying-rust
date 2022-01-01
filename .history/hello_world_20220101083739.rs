// remember to save the file if you update program
// $ rustc hello_world.rs
// $ ./hello_world
// for variable declaration, use underscore NOT hyphens (hyphens result in weird behaviour). 
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

    println!("{}", fahrenheit_to_celsius(212.0));
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0/9.0)
}

// imp:
// for simple data types (stored on stack): always COPY. when they go out of scope nothing special happens
// for complex data types (stored on heap): move (default behaviour: shallow copy, rendering orignal variable unusable) and clone (deep copy). when they go out of scope, drop is called
// variables own values, and these values are copied, moved, or cloned. moving transfers ownership. 


