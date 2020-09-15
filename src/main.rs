fn main() {
    let text = "Hello World";
    let x = 1.2345;
    println!("{:<20}", text);
    println!("{:>20}", text);
    println!("{:^20}", text);
    println!("{:=>20}", text);
    println!("{:*^20}", text);
    println!("{:>10}", x);
    println!("{:.2}", x);
}
