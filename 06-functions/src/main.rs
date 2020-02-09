fn main() {
    another_function(5, 8);
    with_expression();
    let returned = returns_five();
    println!("Function returns: {}", returned);

    let plus_one = add_one(5);
    println!("5 plus one: {}", plus_one);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn with_expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("Value from expression: {}", y);
}

fn returns_five() -> i32 {
    5
}

fn add_one(x: i32) -> i32 {
    x + 1
}
