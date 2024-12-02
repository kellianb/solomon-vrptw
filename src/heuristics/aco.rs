use crate::location::Location;
use crate::route::Route;
use crate::vrp::Vrp;
use crate::vrp_result::VrpResult;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use std::collections::HashMap;

/// Parameters for the aco heuristic
#[derive(Debug)]
pub struct AcoParams {
    /// The number of ants in this aco
    pub n_ants: u16,
    /// The number of iterations that will be run in this aco
    pub max_iter: u16,
    /// The importance of pheromones when deciding which [Location](crate::location::Location) to
    /// go to next
    pub alpha: u16,
    /// The importance of [cost](crate::location::Location::cost_to_deliver) when deciding which [Location](crate::location::Location) to
    /// go to next
    pub beta: u16,
    /// The evaporation factor for pheromone
    pub rho: f32,
    /// The initial pheromone value, a good value for this is 1 / total cost of nearest neighbor
    /// for this dataset
    pub pheromone_amt: f32,
}

impl Default for AcoParams {
    fn default() -> Self {
        AcoParams {
            n_ants: 50,
            max_iter: 200,
            alpha: 1,
            beta: 1,
            rho: 0.1,
            pheromone_amt: 1.0 / 8000.0,
        }
    }
}

impl Vrp {
    /// Run the aco heuritic on a Vrp instance
    pub fn aco_heuristic(&self, params: &AcoParams) -> VrpResult {
        let mut pheromones: HashMap<(Location, Location), f32> = HashMap::new();

        // Initialise pheromones
        self.set_pheromones(params, &mut pheromones);

        // Store best results
        let mut best_solution = VrpResult::from_vrp(self, Vec::default(), None);
        let mut best_cost = f32::INFINITY;
        let mut best_cost_history: Vec<f32> = Vec::default();

        for _ in 0..params.max_iter {
            let solutions: Vec<Vec<Route>> = (0..params.n_ants)
                .map(|_| self.construct_routes(params, &pheromones))
                .collect();

            self.update_pheromones(&solutions, params, &mut pheromones);

            for solution in solutions {
                let solution = VrpResult::from_vrp(self, solution, None);
                let cost: f32 = solution.total_cost();

                if cost < best_cost {
                    best_solution = solution;
                    best_cost = cost;
                }
            }
            best_cost_history.push(best_cost);
        }

        VrpResult {
            heuristic_cost_history: Some(best_cost_history),
            ..best_solution
        }
    }

    /// Reset or set the pheromones
    fn set_pheromones(
        &self,
        params: &AcoParams,
        pheromones: &mut HashMap<(Location, Location), f32>,
    ) {
        let locations: Vec<&Location> = self
            .customers
            .iter()
            .chain(std::iter::once(&self.warehouse))
            .collect();

        for &a in &locations {
            for &b in &locations {
                if *a != *b {
                    pheromones.insert((a.clone(), b.clone()), params.pheromone_amt);
                }
            }
        }
    }

    /// Update the pheromones to reward the best routes
    fn update_pheromones(
        &self,
        solutions: &Vec<Vec<Route>>,
        params: &AcoParams,
        pheromones: &mut HashMap<(Location, Location), f32>,
    ) {
        for value in pheromones.values_mut() {
            *value *= 1.0 - params.rho;
        }

        // Remove pheromones on each edge where ants passed
        for solution in solutions {
            let solution = VrpResult::from_vrp(self, solution.clone(), None);
            let deposit = params.rho / solution.total_cost();

            for route in solution.routes {
                for i in 0..route.len() - 1 {
                    let pheromone = pheromones
                        .get_mut(&(route[i].clone(), route[i + 1].clone()))
                        .unwrap();

                    *pheromone += deposit;
                }
            }
        }
    }

    fn construct_routes(
        &self,
        params: &AcoParams,
        pheromones: &HashMap<(Location, Location), f32>,
    ) -> Vec<Route> {
        let mut solution: Vec<Route> = Vec::with_capacity(1);
        let mut unvisited: Vec<&Location> = self.customers.iter().collect();

        while !unvisited.is_empty() {
            let mut total_demand = 0;
            let mut current_cost: f32 = 0f32;

            let mut current = &self.warehouse;

            let mut new_route = Route {
                warehouse: self.warehouse.clone(),
                ..Default::default()
            };

            loop {
                let next_loc = select_next_location(
                    current,
                    unvisited.clone(),
                    current_cost,
                    self.vehicle_capacity - total_demand,
                    params,
                    pheromones,
                );

                let next_loc = if let Some(val) = next_loc {
                    val
                } else {
                    break;
                };
                new_route.customers.push(next_loc.clone());

                // Add to total cost
                current_cost += current.cost_to_deliver(next_loc, current_cost);

                // Add demand to total route demand
                total_demand += next_loc.demand;

                // Remove next_loc from unvisited
                if let Some(index) = unvisited.iter().position(|&x| x == next_loc) {
                    // Remove the element at the found index
                    unvisited.remove(index);
                } else {
                    panic!("Unable to remove customer")
                }

                // Set current to next customer
                current = next_loc;
            }
            solution.push(new_route);
        }
        solution
    }
}

fn select_next_location<'a>(
    current: &Location,
    unvisited: Vec<&'a Location>,
    current_cost: f32,
    remaining_capacity: u16,
    params: &AcoParams,
    pheromones: &HashMap<(Location, Location), f32>,
) -> Option<&'a Location> {
    // Create a random number generator
    let mut rng = thread_rng();

    let reachable_customers = current.find_deliverable(unvisited, current_cost, remaining_capacity);

    if reachable_customers.is_empty() {
        return None;
    }

    let probabilities: Vec<f32> = reachable_customers
        .iter()
        .map(|&next| {
            let pheromone = pheromones
                .get(&(current.clone(), next.clone()))
                .copied()
                .expect("Failed to get pheromone value");

            let cost = current.cost_to_deliver(next, current_cost) - current_cost;

            let desirability = 1f32 / cost;

            f32::powi(pheromone, params.alpha as i32) * f32::powi(desirability, params.beta as i32)
                + 1e-6
        })
        .collect();

    let total: f32 = probabilities.iter().sum();

    let normalized_probabilities: Vec<f32> = probabilities.iter().map(|&p| (p / total)).collect();

    // Create a WeightedIndex using the probabilities
    let dist =
        WeightedIndex::new(&normalized_probabilities).expect("Failed to generate WeightedIndex");

    // Select a random element based on the weighted distribution
    reachable_customers.get(dist.sample(&mut rng)).copied()
}
