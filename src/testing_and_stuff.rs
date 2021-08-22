struct Car {
    model: String,
    price: f32,
}

fn testing_and_stuff() {
    let car = Car {
        model: String::from("Honda"),
        price: 4099.99,
    };

    let yet_another_car = create_car(String::from("Model"), 1000.00);

    println!(
        "Model: {} | Price: {}",
        yet_another_car.model, yet_another_car.price
    );

    let another_car = Car {
        price: 20000.99,
        ..yet_another_car
    };

    println!(
        "Model: {} | Price: {}",
        another_car.model, another_car.price
    );
}

fn create_car(model: String, price: f32) -> Car {
    Car { model, price }
}
