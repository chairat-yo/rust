fn main() {
    test_expression_1();
}

fn test_expression_1() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 {
        true
    } else {
        false
    };

    let is_lt_5 = my_num < 5;

    println!("{:?}", is_lt_5);
}

fn test_expression_2() {
    let my_num = 3;
    let message = match my_num {
        1 => "hello",
        _ => "goodbye"
    };
}