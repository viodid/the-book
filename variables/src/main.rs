const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + THREE_HOURS_IN_SECONDS;
    println!("The value of x is: {x}");
}

