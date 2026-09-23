fn main() {
    println!("Hello, world!");

    another_function(32);

    print_labeled_mesurement(5, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_mesurement(value: i32, unit_lable: char){
    println!("The measuremnt is: {value}{unit_lable}");
}
