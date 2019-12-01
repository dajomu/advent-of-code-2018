use std::fs;

fn main() {
    let filename = "modules.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    first_part(contents);
}

fn first_part(contents: std::string::String) {
    let module_weights = contents.lines();

    let total_fuel: i32 = module_weights.map(|mass| calculate_fuel(mass.parse::<f64>().unwrap())).sum();

    println!("total fuel for modules: {}", total_fuel)
    // total_fuel
}

fn calculate_fuel(weight: f64) -> (i32) {
    //  mass, divide by three, round down, and subtract 2
    let fuel = (weight / 3.0).floor() as i32 - 2;
    fuel
}
