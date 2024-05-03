struct Temperature {
    degree_f: f64,
}

impl Temperature {
    fn freezing() -> Self { //Self is Temperature
        Self { degree_f: 32.0}
    }

    fn show_temp(&self) {
        println!("{:?} degree F", self.degree_f);
    }
}

fn main() {
    let hot = Temperature { degree_f: 99.9};
    // Temperature::show_temp(hot);
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
}