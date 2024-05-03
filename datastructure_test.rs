fn main() {
    //vector --> same type
    test_vector();
}

fn test_vector() {
    let my_numbers = vec![1, 2, 3]; //macro, more convenience

    for num in my_numbers {
        println!("{:?}", num);
    }
    //let mut my_numbers = Vec::new();
    //my_numbers.push(1);
    //my_numbers.push(2);
    //my_numbers.push(3);
    //my_numbers.pop();
    //my_numbers.len();

    //println!("{:?}", my_numbers);
}