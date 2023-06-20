#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

mod restaurant;
use crate::restaurant::order_food;

fn read_file_tut() {
  let path = "lines.txt";
  let output = File::create(path);
 
  // read file
  let mut output = match output {
    Ok(f) => f,
    Err(e) => {
      panic!("Error reading file: {:?}", e);
    }
  };

  // write to file
  write!(output, "Just some\nRandom words").expect("Failed to write to file");

  let input = File::open(path).unwrap();
  let buffered = BufReader::new(input);

  for line in buffered.lines() {
    println!("{}", line.unwrap());
  }

  let output2 = File::create("rand.txt");
  let output2 = match output2 {
    Ok(f) => f,
    Err(e) => match e.kind() {
      ErrorKind::NotFound => match File::create("rand.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Unable to create file {:?}", e),
      },
      _other_error => panic!("Problem opening file"),
    },
  };

}

fn main() {
  read_file_tut()
}

fn hashmap_examples() {
  let mut heros = HashMap::new();
  heros.insert("Superman", "Clark Kent");
  heros.insert("Batman", "Bruce Wayne");
  heros.insert("The Flash", "Barry Allen");

  for (k, v) in heros.iter() {
    println!("{} : {}", k, v)
  }

  println!("Length: {}", heros.len());

  if heros.contains_key(&"Batman") {
    let the_batman = heros.get(&"Batman");
    match the_batman {
      Some(x) => println!("Batman is a hero"),
      None => println!("Not a hero"),
    }
  }
}

fn structs_example() {
  struct Customer {
    name: String,
    address: String,
    balance: f32,
  }

  let mut bob = Customer{
    name: String::from("Bob Smith"),
    address: String::from("555 main street"),
    balance: 234.54,
  };

  bob.address = String::from("505 Main street");

  println!("{}", bob.address);
}

fn generics_example() {
  // Kinda interface
  trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
  }

  struct Rectangle {
    length: f32,
    width: f32,
  }

  struct Circle {
    length: f32,
    width: f32,
  }

  impl Shape for Rectangle {
    fn new(length: f32, width: f32) -> Rectangle {
      return Rectangle { length, width }
    }
    fn area(&self) -> f32 {
      return self.length * self.width;
    }
  }

  const PI: f32 = 3.14;

  impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
      return Circle { length, width }
    }
    fn area(&self) -> f32 {
      return (self.length / 2.0).powf(2.0) * PI;
    }
  }

  let rec: Rectangle = Shape::new(10.0, 10.0);
  let circ: Circle = Shape::new(10.0, 10.0);

  println!("Rec area {}", rec.area());
  println!("Circ area {}", circ.area());
}

fn main3() {
  order_food();
  hashmap_examples();
  structs_example();
  generics_example();
}

fn main2() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't receive input");
    println!("Hello, {}, {}", name.trim_end(), greeting);

    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.14;
    let age: &str = "27";
    // No shaddowing for web script
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

    println!("Max f32 : {}", f32::MAX);

    let is_true = true;

    println!("So {}", is_true);

    let my_grade = 'A';

    // f32 precision = 6
    // f64 precision = 14
    let num_1: f32 = 1.1111111111;
    println!("f32 : {}", num_1 + 0.1111111111111111111);

    let num_2 = 5;
    let num_3 = 6;

    println!("num 2 + num 3 = {}", num_2 + num_3);

    let random_num = rand::thread_rng().gen_range(1..101);

    println!("Random: {}", random_num);


    let new_age = 8;
    
    if (age <= 18) {
      println!("party time")
    } else if (age == 21) || (age == 50) {
      println!("Special")
    } else if (age >= 65) {
      println!("Nice")
    } else {
      println!("who cares")
    }

    let mut my_age_1 = 27;

    let can_vote = if my_age_1 >= 18 {
      true
    } else {
      false
    };

    println!("Can vote {}", can_vote);

    let age_2 = 8;

    match age_2 {
      1..=18 => println!("Party time"),
      21 | 50 => println!("Important"),
      65..=i32::MAX => println!("Nice"),
      _ => println!("Who cares lol")
    };

    let my_age_27 = 27;
    let voting_age = 18;

    match my_age_1.cmp(&voting_age) {
      Ordering::Less => println!("Cant vote"),
      Ordering::Greater => println!("Can vote"),
      Ordering::Equal => println!("You gained the right to vote"),
      _ => println!("Yolo")
    }

    let arr_1 = [1, 2,3,4,5,6,7,8,9];

    println!("1st elem : {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let mut loop_idx = 0;
    loop {
      if arr_1[loop_idx] % 2 == 0 {
        loop_idx+=1;
        continue;
      }

      if arr_1[loop_idx] == 9 {
        break;
      }

      println!("Odd Val: {}", arr_1[loop_idx]);
      loop_idx+=1;
    }

    loop_idx = 0;

    while loop_idx < arr_1.len() {
      println!("Arr: {}", arr_1[loop_idx]);
      loop_idx += 1;
    }

    for val in arr_1.iter() {
      println!("Val : {}", val);
    }

    let my_tuple = (27, "Tony".to_string(), 50_000.00);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", my_tuple.0);


    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
      println!("Word: {}", word)
    }

    let st2: String = st1.replace("A", "another");

    let st3 = String::from("x r t b h k k a m c");

    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
      println!("Char: {}", char);
    }

    let st4: &str = "Random String...";
    // heap allocated?...
    let mut st5: String = st4.to_string();

    println!("ST5: {}", st5);

    let byte_arr_1 = st5.as_bytes();

    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7: String = String::from(" words");
    // & sign pulls value and kills reference...?
    let st8 = st6 + &st7;

    for char in st8.bytes() {
      println!("char : {}", char);
    }

    let int_u8 = 5;
    let int2_u8 = 4;
    let int4_u32 = (int_u8 as u32);

    enum Day {
      Monday,
      Tuesday,
      Wednesday,
      Thursday,
      Friday,
      Staturday,
      Sunday,
    }

    impl Day {
      fn is_weekend(&self) -> bool {
        match self {
          Day::Staturday | Day::Sunday => true,
          _ => false,
        }
      }
    }

    let today:Day = Day::Monday;

    match today {
      Day::Monday => println!("Boo monday"),
      _ => println!("Just another day"),
    }
    
    println!("Is today the weekend, {}", today.is_weekend());

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];

    match vec2.get(1) {
      Some(second) => println!("2nd : {}", second),
      None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
      *i *=2;
    }

    for i in &vec2 {
      println!("Mutated: {}", i);
    }

    println!("Vec2 len: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());

    say_hello();
    println!("Sum: {}", get_sum(1, 2));

    // hasmap_examples();
}

fn get_sum(x: i32, y:i32) -> i32 {
  x + y
}

fn say_hello() {
  println!("Hello");
}
