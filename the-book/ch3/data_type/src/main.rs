// fn main() {
//     let guess: u32 = "42".parse().expect("Not a number!");
//     println!("{}", guess);
// }

fn main() {
    let sum = 5 + 10;
    println!("{}", sum);

    let difference = 95.5 - 4.3;
    println!("{}", difference);

    let product = 4 * 30;
    println!("{}", product);

    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    let floored = 2 / 3;
    println!("{}", floored);

    let remainder = 43 % 5;
    println!("{}", remainder);

    let c = 'z';
    println!("{}", c);

    let z = 'ℤ';
    println!("{}", z);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    println!("{:#?}", tup);

    let (x, y, _) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    println!("{}", tup.0);
    println!("{}", tup.1);
}
