mod file_parser;
mod location;
mod route;

use file_parser::parse_solomon_vrp_file;
use route::Route;

fn main() {
    let (warehouse, customers) = parse_solomon_vrp_file(&String::from("sample.txt"));
    println!("N° of customers : {}", &customers.len());

    let rt = Route {
        customers,
        warehouse,
    };

    let (min_len, shortest) = rt.brute_force();

    println!("Shortest route lenght : {}", min_len);
    println!("Shortest route order : {:?}", shortest);

    shortest.plot();
}
