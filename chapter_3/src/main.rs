fn main() {
    let fahrenheit = 50;
    let celsius = f_to_c(fahrenheit);
    println!("{fahrenheit} Fahrenheit is {celsius} Celsius.\n");

    let n = 7;
    let nth_fib = fib(n);
    println!("The {n}th fibonacci number is {nth_fib}.\n");

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["And a partridge in a pear tree", "Two turtle doves", "Three french hens", "Four calling birds",
    "Five golden rings", "Six geese a-laying", "Seven swans-a-swimming", "Eight maids a-milking", "Nine ladies dancing",
    "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];
    twelve_days(days, gifts);
}

// A function that converts Fahrenheit to Celsius.
fn f_to_c(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

// A function that prints out the nth number of the fibonacci sequence.
fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    let mut first = 0;
    let mut second = 1;
    let mut count = 2;
    while count <= n {
        let next = first + second;
        first = second;
        second = next;
        count += 1
    }
    second
}

// A function that prints out the lyrics of the Christmas carol, "The Twelve Days oF Christmas".
fn twelve_days(days: [&str; 12], gifts: [&str; 12]) {
    let mut first = true;
    for day_idx in 0..12 {
        println!("On the {} day of Christmas my true love gave to me", days[day_idx]);
        for gift_idx in (0..day_idx + 1).rev() {
            if gift_idx == 0 && first {
                print!("A partridge in a pear tree\n");
            }
            else {
                println!("{}", gifts[gift_idx]);
            }
            if first {
                first = false;
            }
        }
        print!("\n");
    }
}