fn main() {
    let sum = 5 + 10;
    println!("Addition, int32: {}", sum);

    let difference = 95.5 - 4.3; 
    println!("Subtraction, f64: {}", difference);

    let product = 4 * 30;
    println!("Multiplication, i32: {}", product);

    let quotient = 56.7 / 32.2;
    println!("Division, f64: {}", quotient);

    let remainder = 43 % 5;
    println!("Remainder, i32: {}", remainder);

    let t = true;
    println!("Boolean is: {}", t);

    let f: bool = false;
    println!("Boolean is: {}", f);

    let character = 'c';
    println!("Character is: {}", character);

    let character = 'ℤ';
    println!("Unicode character: {}", character);
    
    let character = '🤯';
    println!("Emoji character: {}", character);

    let tuple: (i16, f32, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tuple;
    println!("Tuple at y: {}", y);
    println!("Tuple at 0: {}", tuple.0);
    println!("Tuple at 1: {}", tuple.1);
    println!("Tuple at 2: {}", tuple.2);

    let array = [1, 2, 3, 4];
    println!("Inferred array: {}", array[1]);

    let array: [i8; 4] = [5, 6, 7, 8];
    println!("Typed array: {}", array[1]);

    let array = [3, 5];
    println!("Filled array: {}", array[1]);
}
