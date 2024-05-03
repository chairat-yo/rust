enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    //ownership model
    //owner (fn)--> cleanup
    //move or borrow
    let dull = Light::Dull;
    display_light(dull);
    display_light(dull); //dull is gone!!!
}