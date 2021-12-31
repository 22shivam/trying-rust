fn main() {
    println!("Hello, World!");
    let mut x = 1;
    'print_1_to_10: loop {
        println!("{}",x);
        x = x + 1;
        if x == 10 {
            break 'print-1-10;
        }
    }
}

