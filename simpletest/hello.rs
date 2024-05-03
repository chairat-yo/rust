
//cargo run --bin hello

fn main() {
    let two = 2;
    let hello = "hello";
    let j = 'j';
    let my_half = 0.5;
    let mut my_name = "Bill";
    let quit_program = false;
    let your_half = my_half;

    //if
    if two > 1 {
      println!(">1");
    }
    else if two < 1{
      println!("<1");
    }
    println!("result: {:?}", add(2,3)); //macro standard lib, :? debug mode
    println!("Rust says Hello to TutorialsPoint !!");

    //arithmetic


    
    //loop
    let mut a = 0;
    loop {
      if a == 5 {
         break;
      }
      println!("{:?}", a);
      a += 1;
    }

    //while
    let mut a = 0;
    while a != 5 {
      println!("{:?}", a);
      a += 1;
    }
}

fn add(_a: i32, b: i32) -> i32 {
   _a+b
}