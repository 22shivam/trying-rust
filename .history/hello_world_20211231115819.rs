// remember to save the file before running the program
// $ rustc hello_world.rs
// $ ./hello_world
// for variable declaration, use underscore NOT hyphens. 

fn main() {
    println!("Hello, World!");
    let mut x = 1;
    'print_1_to_10: loop {
        println!("{}",x);
        println!(x)
        x = x + 1;
        if x == 10 {
            break 'print_1_to_10;
        }
    }
}

