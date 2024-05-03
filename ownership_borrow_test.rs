enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) { //not allow to delete light (borrow), main fn is the owner
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
    display_light(&dull);
    display_light(&dull); 
}