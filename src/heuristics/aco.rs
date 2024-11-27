use std::collections::HashMap;

use crate::location::Location;
use crate::route::Route;
use crate::vrp::Vrp;
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};

/// Parameters for the aco heuristic
pub struct AcoParams {
    /// The number of ants in this aco
    pub n_ants: u16,
    /// The number of iterations that will be run in thos aco
    pub max_iter: u16,
    /// The number of iterations after which the pheromones will be reset
    pub max_r: u16,
    /// The importance of pheromones when deciding which [Location](crate::location::Location) to
    /// go to next
    pub alpha: u16,
    /// The importance of [cost](crate::location::Location::cost_to_delivery_window) when deciding which [Location](crate::location::Location) to
    /// go to next
    pub beta: u16,
    /// The evaporation factor for pheromone
    pub rho: f32,
    /// The chance to pick the route with the lowest [cost](crate::location::Location::cost_to_delivery_window)
    pub q0: f32,
}

impl Default for AcoParams {
    fn default() -> Self {
        AcoParams {
            n_ants: 100,
            max_iter: 1000,
            max_r: 100,
            alpha: 1,
            beta: 1,
            rho: 0.1,
            q0: 0.4,
        }
    }
}

impl Vrp {
    /// Run the aco heuritic on a Vrp instance
    pub fn aco_heuristic(&mut self, params: &AcoParams) -> &mut Vrp {
        let mut pheromones: HashMap<(Location, Location), f32> = HashMap::new();

        let pheromon_amt: f32 = if self.routes.is_empty() {
            1f32 / 4500f32
        } else {
            1f32 / self.total_cost() * self.customers.len() as f32
        };

        // Initialise pheromones
        self.set_pheromones(pheromon_amt, &mut pheromones, false);

        let mut best_solution: Vec<Route> = Vec::with_capacity(1);
        let mut best_cost = f32::INFINITY;

        // Count iterations since an improvement was made, if it reaches max_r, the pheromones are
        // reset
        let mut iter_counter: u16 = 0;

        let unvisited: Vec<&Location> = self.customers.iter().collect();

        for _ in 0..params.max_iter {
            iter_counter += 1;
            let solutions: Vec<Vec<Route>> = (0..params.n_ants)
                .map(|_| self.construct_routes(unvisited.clone(), params, &pheromones))
                .collect();

            self.update_pheromones(
                pheromon_amt,
                &best_solution,
                &solutions,
                params,
                &mut pheromones,
            );

            for solution in solutions {
                let cost: f32 = self.total_cost_with(&solution);

                if cost < best_cost {
                    best_solution = solution;
                    best_cost = cost;
                    iter_counter = 0;
                }
            }

            if iter_counter >= params.max_r {
                iter_counter = 0;
                self.routes = best_solution.clone();
                self.set_pheromones(pheromon_amt, &mut pheromones, true);
                println!("Reset pheromones, best cost: {}", best_cost)
            }
        }

        self.routes = best_solution;

        self
    }

    /// Reset or set the pheromones
    fn set_pheromones(
        &self,
        pheromone_amt: f32,
        pheromones: &mut HashMap<(Location, Location), f32>,
        reward_existing_routes: bool,
    ) {
        let locations: Vec<&Location> = self
            .customers
            .iter()
            .chain(std::iter::once(&self.warehouse))
            .collect();

        for &a in &locations {
            for &b in &locations {
                if a != b {
                    pheromones.insert((a.clone(), b.clone()), pheromone_amt);
                }
            }
        }

        if reward_existing_routes && !self.routes.is_empty() {
            let deposit = 1f32 / self.total_cost();

            for route in self.routes.iter() {
                *pheromones
                    .get_mut(&(
                        route.warehouse.clone(),
                        route.customers.first().unwrap().clone(),
                    ))
                    .unwrap() += deposit;

                if route.customers.len() >= 2 {
                    for i in 0..route.customers.len() - 1 {
                        *pheromones
                            .get_mut(&(route.customers[i].clone(), route.customers[i + 1].clone()))
                            .unwrap() += deposit;
                    }
                }

                *pheromones
                    .get_mut(&(
                        route.customers.last().unwrap().clone(),
                        route.warehouse.clone(),
                    ))
                    .unwrap() += deposit;
            }
        }
    }

    /// Update the pheromones to reward the best routes
    fn update_pheromones(
        &self,
        pheromone_amt: f32,
        best_solution: &Vec<Route>,
        solutions: &Vec<Vec<Route>>,
        params: &AcoParams,
        pheromones: &mut HashMap<(Location, Location), f32>,
    ) {
        let reward_deposit = params.rho / self.total_cost_no_service_time_with(best_solution);

        // Reward best solution
        for route in best_solution {
            let pheromone = pheromones
                .get_mut(&(
                    route.warehouse.clone(),
                    route.customers.first().unwrap().clone(),
                ))
                .unwrap();

            *pheromone = (1.0 - params.rho) * *pheromone + reward_deposit;

            if route.customers.len() >= 2 {
                for i in 0..route.customers.len() - 1 {
                    let pheromone = pheromones
                        .get_mut(&(route.customers[i].clone(), route.customers[i + 1].clone()))
                        .unwrap();

                    *pheromone = (1.0 - params.rho) * *pheromone + reward_deposit;
                }
            }

            let pheromone = pheromones
                .get_mut(&(
                    route.customers.last().unwrap().clone(),
                    route.warehouse.clone(),
                ))
                .unwrap();

            *pheromone = (1.0 - params.rho) * *pheromone + reward_deposit;
        }

        // Remove pheromones on each edge where ants passed
        for solution in solutions {
            let penalty_deposit = params.rho * pheromone_amt;

            for route in solution {
                let pheromone = pheromones
                    .get_mut(&(
                        route.warehouse.clone(),
                        route.customers.first().unwrap().clone(),
                    ))
                    .unwrap();

                *pheromone = (1.0 - params.rho) * *pheromone + penalty_deposit;

                if route.customers.len() >= 2 {
                    for i in 0..route.customers.len() - 1 {
                        let pheromone = pheromones
                            .get_mut(&(route.customers[i].clone(), route.customers[i + 1].clone()))
                            .unwrap();

                        *pheromone = (1.0 - params.rho) * *pheromone + penalty_deposit;
                    }
                }

                let pheromone = pheromones
                    .get_mut(&(
                        route.customers.last().unwrap().clone(),
                        route.warehouse.clone(),
                    ))
                    .unwrap();

                *pheromone = (1.0 - params.rho) * *pheromone + penalty_deposit;
            }
        }
    }

    fn construct_routes(
        &self,
        mut unvisited: Vec<&Location>,
        params: &AcoParams,
        pheromones: &HashMap<(Location, Location), f32>,
    ) -> Vec<Route> {
        let mut solution: Vec<Route> = Vec::with_capacity(1);

        while !unvisited.is_empty() && solution.len() < self.n_vehicles as usize {
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

                if let Some(next_loc) = next_loc {
                    new_route.customers.push(next_loc.clone());

                    // Add to total cost
                    current_cost += current.cost_to_deliver(next_loc, current_cost);

                    // Add demand to total route demand
                    total_demand += next_loc.demand;

                    // Remove next_loc from unvisited
                    if let Some(index) = unvisited.iter().position(|&x| x == next_loc) {
                        // Remove the element at the found index
                        unvisited.remove(index);
                    }

                    // Set current to next customer
                    current = next_loc;
                } else {
                    break;
                };
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

    // Return the best solution with q0 probability
    if params.q0 >= rng.gen() {
        // Get index of the highest probability
        let (best, _, _) =
            current.find_cheapest_deliverable(unvisited, current_cost, remaining_capacity)?;

        return Some(best);
    }

    let reachable_customers = current.find_deliverable(unvisited, current_cost, remaining_capacity);

    if reachable_customers.is_empty() {
        return None;
    }

    let probabilities: Vec<f32> = reachable_customers
        .iter()
        .map(|&next| {
            let &pheromone = pheromones
                .get(&(current.clone(), next.clone()))
                .expect("Failed to get pheromone value");

            let cost = current.cost_to_delivery_window(next, current_cost);

            let desirability = 1f32 / cost;

            f32::powi(pheromone, params.alpha as i32) * f32::powi(desirability, params.beta as i32)
                + 1e-6
        })
        .collect();

    let total: f32 = probabilities.iter().sum();

    let probabilities: Vec<f32> = probabilities.into_iter().map(|p| (p / total)).collect();

    // Create a WeightedIndex using the probabilities
    let dist = WeightedIndex::new(&probabilities).expect("Failed to generate WeightedIndex");

    // Select a random element based on the weighted distribution
    reachable_customers.get(dist.sample(&mut rng)).copied()
}
