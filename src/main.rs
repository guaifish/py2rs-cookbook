use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    // choose
    let values = [1, 2, 3, 4, 5, 6];
    let mut rng = thread_rng();
    for _ in 0..5 {
        println!("{:?}", values.choose(&mut rng).unwrap());
    }

    // random
    let x: u8 = rand::random();
    println!("x = {}", x);

    let a: [u8; 3] = rand::random();
    println!("a = {:?}", a);

    // shuffle
    let mut values = [1, 2, 3, 4, 5, 6];
    values.shuffle(&mut rng);
    println!("{:?}", values);
}
