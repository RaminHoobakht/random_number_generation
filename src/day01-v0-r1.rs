use rand::Rng;

fn main() {
    println!("\n");

    let rnd: i32 = rand::rng().random_range(0..100);
    println!("random number is: {}", rnd);

    println!("\n The End ...\n");
}
