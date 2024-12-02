//! This crate uses the solomon_vrp_solver library to solve a VRP

use solomon_vrptw::file_parser::parse_solomon_vrp_file;
use solomon_vrptw::heuristics::aco::AcoParams;
use solomon_vrptw::vrp::Vrp;
use std::fs;
use std::io;

fn delete_all_files_in_directory(directory: &str) -> io::Result<()> {
    // Read the directory
    for entry in fs::read_dir(directory)? {
        let entry = entry?; // Handle Result from read_dir
        let path = entry.path();

        // If the entry is a file, delete it
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn write_to_file(filepath: &str, contents: &str) -> io::Result<()> {
    // Write the string to the file
    fs::write(filepath, contents)
}

/// Let the user pick a solomon VRPTW .txt problem definition
fn pick_file() -> Option<String> {
    let path = rfd::FileDialog::new()
        .add_filter("Text Files", &["txt"])
        .pick_file()?;

    path.to_str().map(String::from)
}

fn main() {
    let path = match pick_file() {
        Some(val) => val,
        None => return,
    };
    {
        let split: Vec<String> = path.split("/").map(String::from).collect();
        println!("Selected {}", split[split.len() - 2..split.len()].join("/"));
    }

    let target_dir = "routes";

    _ = delete_all_files_in_directory(target_dir);

    // -- Create an instance of VRP from the .txt problem definition --
    let (warehouse, customers) = parse_solomon_vrp_file(&path);
    println!("N° of customers : {}", &customers.len());

    let mut vrp = Vrp::new(warehouse, customers, 25, 200);

    // -- Run nearest neighbour heuristic --
    vrp.nearest_neighbour_heuristic();
    println!(
        "Total cost (nearest_neighbour_heuristic): {}",
        vrp.total_cost()
    );
    println!(
        "N° of routes (nearest_neighbour_heuristic): {}",
        vrp.routes.len()
    );

    _ = write_to_file(
        &format! {"{target_dir}/nearest_neighbour_heuristic.md"},
        &vrp.print_to_md_string(),
    );

    // -- Run Ant Colony Optimization heuristic
    let aco_params = AcoParams::default();
    vrp.aco_heuristic(&aco_params);

    println!("Total cost (aco_heuristic): {}", vrp.total_cost());
    println!("N° of routes (aco_heuristic): {}", vrp.routes.len());

    _ = write_to_file(
        &format! {"{target_dir}/aco_heuristic.md"},
        &vrp.print_to_md_string(),
    );
}
