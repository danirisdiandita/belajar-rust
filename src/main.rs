// use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {guess}");


    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';


    println!("c: {c}, z: {z}, heart_eyed_cat: {heart_eyed_cat}");



    let tupple_ : (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tupple_;

    println!("The value of y is: {_y}");


    let gitu: u8 = 34 + _z; 

    println!("The value of gitu is: {gitu}");



    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}");
    println!("The value of second is: {second}");



    // println!("Please enter an array index.");

    // let mut index = String::new(); 

    // io::stdin().read_line(&mut index).expect("Failed to read line"); 

    // let index: usize = index.trim().parse().expect("Index entered was not a number"); // for indexing purpose use usize

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");


    nggak_bisa_jalan();
    another_function(5);

    print_labeled_measurement(5, 'h');


    let y = {
        let x = 3; 
        x + 1 
    }; 

    println!("The value of y is: {y}");

    println!("The value of five() is: {}", five());



    let number = 3; 


    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}


fn nggak_bisa_jalan() {
    println!("ngga bisa jalan");
}


fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn five() -> i32 {
    5
}