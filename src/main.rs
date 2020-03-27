const MAX_POINTS: u32 = 100_000;     // constant declaration at
                                     // the global level.
const MAX_FLOATS: f32 = 285_351.95;  // NOTE: With f32 -> The amount of max floats is: 285351.94
const MAX_FLOAT: f64 = 285_351.95;  // NOTE: With f64 -> The amount of max float is: 285351.95
                                    // Correct size of the dataype makes a difference.

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;  // This is an error and the program
            // cannot run without first mutating the
            // value of x.
    println!("The value of x is: {}", x);
    println!("The amount of max points is: {}", MAX_POINTS); // The amount of max points is: 100000
    println!("The amount of max floats is: {}", MAX_FLOATS); // The amount of max floats is: 285351.94
    println!("The amount of max float is: {}", MAX_FLOAT);   // The amount of max floats is: 285351.95

    let x = 5;
    let x = x + 1;  // The program initially binds x to a value of 5 but then
                    // shadows it by repeating let x which takes the original 
                    // value and add's 1. 
    let x = x * 2;
    println!("The value of x is: {}", x);   // The value of x is: 12

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces); // The number of spaces is: 4
}
