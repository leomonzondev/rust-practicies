struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn car_factory(color:String, transmission: Transmission, convertible: bool) {
    let _car: Car = todo!("Create an instance of a `Car` struct");
}


fn main() {
    println!("Hello, world!");

    let a_number;
    let a_word = "Ten";
    
    a_number = 10;
    
    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage); 
}
