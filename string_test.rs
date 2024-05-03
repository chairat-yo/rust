
fn print_it(data: &str) {
    println!("{:?}", data);
}



fn main() {
    //String - owned (used in struct)
    //&str - borrowed String slice
    print_it("Helloooooooooooo!!!!");
}