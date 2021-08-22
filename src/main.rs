struct Car {
    model: String,
    brand: String,
    price: f32,
    year: u16,
}

fn main() {
    let mut cars_list: Vec<Car> = Vec::new();

    let car = Car {
        model: String::from("sei la"),
        brand: String::from("sei la"),
        price: 2022.2,
        year: 2022,
    };

    cars_list.push(car);

    // let cars = cars_list.iter();

    for car in cars_list {
        println!("Model: {}", car.model);
    }
}
