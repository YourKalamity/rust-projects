fn main() {
    println!("What's up!");
    other_function(999, 69);
    println!("{}",plus_one(9));
}

fn other_function(x: i32, y: i32) {
    println!("{} {}",x, y);
}

fn plus_one(x:i32) -> i32 {
    x + 1
}