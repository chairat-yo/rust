
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

    //arithmetic
    let rem1 = 6 % 3;
    let rem2 = 5 % 2;
    println!("{:?} {:?}", rem1, rem2);

    //match
    let my_int = 2;
    match my_int {
      1 => println!("1"),
      2 => println!("{:?}", my_int),
      _ => println!("something else"),
    }

    /* enum */
    let go = Direction::Up;
    match go {
      Direction::Up => println!("go up"),
      Direction::Down => println!("down"),
      Direction::Left => println!("go left"),
      Direction::Right => println!("go right"),
    }

    /* struct */
    let my_box = ShippingBox { 
      depth: 3,
      width: 2,
      height: 5,
      };
   println!("{:?}", my_box.depth);

   /* Tuples */
   // (x, y, z)
   let coord = (2, 3);
   let (x,y) = coord;
   println!("{:?} : {:?}", y, x);

   let (name, age) = ("Chairat", 43);
   println!("{:?} : {:?}", name, age);

}

struct ShippingBox {
   depth: i32,
   width: i32,
   height: i32,
}

enum Direction {
   Up,
   Down,
   Left,
   Right
}

fn add(_a: i32, b: i32) -> i32 {
   _a+b
}