//This is a struct I made
struct Car {
    color: String,
    make: String
}
//This is where the program starts
fn main() {
    let car: Car = Car { color: String::from("Green"), make: String::from("Toyota") };
    println!("The color of my car is: {}, and the make is: {}", car.color, car.make);
    let tommy = String::from("My Name is Tommy");
    println!("{}", tommy);
    println!("Hello, world!");
}
