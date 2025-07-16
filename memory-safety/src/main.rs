fn main() {
    let enigma: i32;

    // println!("{}". enigma); - Wont work as `enigma` is uninitialized
    // Uncommenting the line above will cause a compile-time error

    enigma = 42; // Now `enigma` is initialized
    println!("{}", enigma); // This will work now
}
