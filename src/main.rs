use std::io;

fn main() {
    let mut in_car_state = false;
    let mut riding_state = false;
    let mut fuel: i8 = 10;
    let mut score: u128 = 0;

    loop {
        // Добавил список команд в скобках, чтобы было понятно, что вводить
        println!("Enter action (enter, exit, drive, stop, refuel, quit): \n");
        let mut choise: String = String::new();
        io::stdin().read_line(&mut choise).unwrap();

        match choise.trim().to_lowercase().trim() {
            "enter" => {
                if in_car_state == false {
                    println!("\nYou entered the car!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                    in_car_state = true;
                } else {
                    println!("\nYou are already in the car!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                }
            }
            "exit" => {
                if in_car_state == true && riding_state == false {
                    println!("\nYou left the car");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                    in_car_state = false;
                } else if in_car_state == false {
                    println!("\nYou are not in the car!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                } else if riding_state == true {
                    println!("\nStop the car first!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                }
            }
            "drive" => {
                if in_car_state == true && riding_state == false {
                    println!("\nYou started driving!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                    riding_state = true;
                    fuel -= 1;
                    score += 100;
                } else if in_car_state == false {
                    println!("\nEnter the car first!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                } else if riding_state == true {
                    println!("\nYou are driving!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                    fuel -= 1;
                    score += 100;
                }
            }
            "stop" => {
                if in_car_state == true && riding_state == true {
                    println!("\nYou stopped!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                    riding_state = false;
                    fuel -= 1;
                } else if in_car_state == false {
                    println!("\nEnter the car first!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                } else if riding_state == false {
                    println!("\nYou are not driving!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                }
            }
            "refuel" => {
                if in_car_state == false && riding_state == false && fuel <= 20 {
                    println!("\nYou refueled!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                    fuel += 10
                } else if in_car_state == true {
                    println!("\nYou must exit the car first!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                } else if fuel >= 20 {
                    println!("\nThe tank is full!");
                    println!("Fuel: {fuel}");
                    println!("Score: {score}");
                }
            }
            "quit" => break (),
            _ => println!("\nUnknown command!"),
        }

        if fuel <= 0 {
            println!("\nYou ran out of fuel!");
            break ();
        }
    }
    println!("\nGame over! Your score: {score} points")
}
