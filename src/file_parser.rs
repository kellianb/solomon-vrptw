use crate::location::Location;
use crate::vrp::Vrp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn parse_solomon_vrp_file(path: &String) -> Option<(Location, Vec<Location>, u16, u16)> {
    // Specify the file path
    let path = Path::new(path);

    // Open the file in read-only mode
    let file = File::open(path).ok()?;

    // Create a buffered reader to read the file line by line
    let reader = BufReader::new(file);

    // Get lines with data, drop invalid lines
    let mut locations = reader.lines().skip(4).filter_map(|x| x.ok());

    let mut restrictions: Vec<u16> = locations
        .next()?
        .split_whitespace()
        .filter_map(|s| s.parse::<u16>().ok())
        .collect();

    let vehicle_capacity = restrictions.pop()?;
    let n_vehicles = restrictions.pop()?;

    let locations = locations.skip(4);
    let locations = locations
        .filter(|l| !l.trim().is_empty()) // Remove empty
        // lines
        .map(|line| {
            // Unwrap each line to handle any I/O errors
            let values: Vec<u16> = line
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

    let (warehouse, locations) = locations
        .split_first()
        .map(|(warehouse, customers)| (warehouse.clone(), customers.to_vec()))?;

    Some((warehouse, locations, n_vehicles, vehicle_capacity))
}

impl Vrp {
    pub fn from_file(path: &String) -> Option<Vrp> {
        let (warehouse, customers, n_vehicles, vehicle_capacity) = parse_solomon_vrp_file(path)?;

        Some(Vrp {
            warehouse,
            customers,
            n_vehicles,
            vehicle_capacity,
        })
    }
}
