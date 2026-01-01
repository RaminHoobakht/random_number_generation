use rand::Rng;

fn main() {
    println!("\n");

    let mut rnd_num_vec: Vec<i32> = Vec::new();

    for _item in 0..20 {
        rnd_num_vec.push(rand::rng().random_range(100..=1000));
    }

    println!("Random Generated Number:");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~");
    for item in &rnd_num_vec {
        println!(" -> {item}");
    }

    println!("\n The End ...\n");
}
