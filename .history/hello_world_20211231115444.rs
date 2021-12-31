fn main() {
    println!("Hello, World!");
`   
let mut x: i32 = 1;
    'print-1-10: loop {
        println!("{}",x);
        x = x + 1;
        if x == 10 {
            break 'print-1-10;
        }
    }
}

