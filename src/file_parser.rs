use crate::location::Location;
use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse_solomon_vrp_file(path: &String) -> (Location, Vec<Location>) {
    // Specify the file path
    let path = Path::new(path);

    // Open the file in read-only mode
    let file = File::open(path).expect("Failed to open : {path}");

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    let mut locations = reader
        .lines()
        .skip(10)
        .map(|line| {
            // Unwrap each line to handle any I/O errors
            let line = line.expect("Error on a line");

            let values: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Error on a line entry"))
                .collect();

            Location {
                id: values[0],
                x: values[1],
                y: values[2],
                demand: values[3],
                ready_time: values[4],
                due_date: values[5],
                service_time: values[6],
            }
        })
        .collect::<Vec<Location>>();

    match locations.split_first() {
        Some((warehouse, customers)) => (warehouse.clone(), customers.to_vec()),
        _ => panic!("Not enough locations found"),
    }
}
