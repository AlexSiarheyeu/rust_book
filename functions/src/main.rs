fn main() {
    another_function_1(1);
    another_function_2();
    five();
}

// Functions 

fn another_function_1(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function_2() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

// Loops