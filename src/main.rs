fn main() {
    let origin = 120;

    let mut number = origin;
    let mut reverse = 0;

    while number != 0 {
        // get last digit using -> number % 10
        // append to reverse
        reverse = reverse * 10 + (number % 10);

        // remove last digit using -> number / 10
        number = number / 10;
    }

    println!("{}", reverse);
}