fn main() {
    let x = 5;

    {
        let y = 10;
        println!("Inner scope: x = {}, y = {}", x, y);
    }
    println!("Outer scope: x = {}", x);

    let shadowed_value = 5;

    {
        let shadowed_value = 10;
        println!("Inner scope: shadowed_value = {}", shadowed_value);
    }
    println!("Outer scope: shadowed_value = {}", shadowed_value);

    let override_value = "More cowbell";
    let override_value = make_image(override_value);

    println!("override_value: {}", override_value);
}

fn make_image(input: &str) -> String {
    format!("Image: {}", input)
}