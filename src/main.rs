use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Car {
    id: u8,
    model: String,
    brand: String,
    plate: String,
    price: f32,
    year: u16,
}

fn main() -> Result<()> {
    let conn = Connection::open("cars.db")?;

    conn.execute(
        "create table if not exists cars (
            id integer primary key autoincrement,
            model text not null,
            brand text not null,
            plate text not null,
            price real not null,
            year integer not null
        )",
        [],
    )?;

    let mut stmt = conn.prepare("SELECT * FROM cars;")?;

    let cars = stmt.query_map([], |row| {
        Ok(Car {
            id: row.get(0)?,
            model: row.get(1)?,
            brand: row.get(2)?,
            plate: row.get(3)?,
            price: row.get(4)?,
            year: row.get(5)?,
        })
    })?;

    for car in cars {
        println!("Car specs: {:?}", car);
    }

    Ok(())
}

// fn list_all_cars() {}
// fn sum_all_cars_prices() {}
// fn list_car_by_plate() {}
// fn rent_car() {}
