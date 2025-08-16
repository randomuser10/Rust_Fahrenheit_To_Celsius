fn main() {
    let celsius = fahren_to_cel(100.0);

    println!("The temperature today is {:.0} degrees", celsius);
}

fn fahren_to_cel(f: f32) -> f32 {
    let celsius = (f - 32.0 ) * (5.0/9.0);
    return celsius;
}
