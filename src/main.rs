//! This crate uses the solomon_vrp_solver library to solve a VRP
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
    let vrp = Vrp::from_file(&path).expect("Failed to parse file");

    dbg!(&vrp);

    // -- Run nearest neighbour heuristic --
    let nn_result = vrp.nearest_neighbour_heuristic();
    println!(
        "Total cost (nearest_neighbour_heuristic): {}",
        nn_result.total_cost()
    );
    println!(
        "N° of routes (nearest_neighbour_heuristic): {}",
        nn_result.routes.len()
    );

    fs::write(
        format! {"{target_dir}/nearest_neighbour_heuristic.md"},
        nn_result.as_md_string(),
    )
    .expect("Failed to write nearest_neighbour_heuristic results");

    // -- Run Ant Colony Optimization heuristic
    let aco_params = AcoParams {
        pheromone_amt: 1.0 / nn_result.total_cost(),
        ..AcoParams::default()
    };

    let aco_result = vrp.aco_heuristic(&aco_params);

    println!("Total cost (aco_heuristic): {}", aco_result.total_cost());
    println!("N° of routes (aco_heuristic): {}", aco_result.routes.len());

    fs::write(
        format! {"{target_dir}/aco_heuristic.md"},
        aco_result.as_md_string(),
    )
    .expect("Failed to write aco_heuristic results");
}
