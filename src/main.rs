mod file_parser;
mod location;
mod route;
mod vrp;

use file_parser::parse_solomon_vrp_file;
use vrp::Vrp;

fn main() {
    let (warehouse, customers) = parse_solomon_vrp_file(&String::from("sample.txt"));
    println!("N° of customers : {}", &customers.len());

    let mut vrp = Vrp {
        customers,
        warehouse,
        n_vehicles: 25,
        vehicle_capacity: 200,
        routes: Vec::with_capacity(0),
    };


    vrp.nearest_neighbour_heuristic();

    // vrp.plot();
}
