use module_example::greet;
use rand::thread_rng;
use rand::Rng;

fn main() {
    greet();
    let x = thread_rng().gen_range(1, 100);
    println!("Random number: {}", x);
}
