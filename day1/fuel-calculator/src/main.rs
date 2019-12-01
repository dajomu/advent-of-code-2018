use std::fs;

fn main() {
    let filename = "modules.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    second_part(contents);
}

fn first_part(contents: std::string::String) {
    let module_weights = contents.lines();

    let total_fuel: i32 = module_weights.map(|mass| calculate_fuel(mass.parse::<i32>().unwrap())).sum();

    println!("total fuel for modules: {}", total_fuel)
}

fn second_part(contents: std::string::String) {

    let module_weights = contents.lines();

    let total_fuel: i32 = module_weights.map(|mass| calculate_fuel_with_fuel(mass.parse::<i32>().unwrap())).sum();

    println!("total fuel including fuel fuel: {}", total_fuel);
}

fn calculate_fuel(weight: i32) -> (i32) {
    //  mass, divide by three, round down, and subtract 2
    let fuel = ((weight / 3) as f64).floor() as i32 - 2;
    fuel
}

fn calculate_fuel_with_fuel(weight: i32) -> (i32) {
    let initial_fuel = calculate_fuel(weight);
    recursive_fuel(initial_fuel, initial_fuel)
}

fn recursive_fuel(total_weight: i32, current_weight: i32) -> (i32) {
    let fuel_needed = calculate_fuel(current_weight);
    if fuel_needed > 0  
     {  
        recursive_fuel(total_weight + fuel_needed, fuel_needed)
     }
     else
     {
        total_weight
     }
}
