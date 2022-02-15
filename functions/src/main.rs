fn main() {
    println!("Hello, world!");

    another_function(3);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let w = five();
    println!("The value of w is: {}", w);

    let v = plus_one(5);
    println!("The value of v is: {}", v);
}

fn another_function (z: i32) {
    println!("The value of z is: {}", z);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
