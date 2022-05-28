/*
struct Point<T, U> {
    x: T,
    y: U
}
*/

fn main() {
    let number_list = vec![34, 50, 25, 100, 55];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 222, 34, 28, 88, 2, 32, 98];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    /*
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    */

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
