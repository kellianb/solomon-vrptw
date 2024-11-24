use crate::location::Location;
use crate::route::Route;

#[derive(Debug, Clone)]
pub struct Vrp<'a> {
    pub customers: Vec<Location>,
    pub warehouse: Location,
    pub n_vehicles: u16,
    pub vehicle_capacity: u16,
    pub routes: Vec<Route<'a>>,
}

impl<'a> Vrp<'a> {
    pub fn nearest_neighbour_heuristic(&mut self) {
        let mut customers: Vec<&Location> = self.customers.iter().collect();

        while !customers.is_empty() {
            let mut route = Route {
                warehouse: &self.warehouse,
                customers: Vec::with_capacity(1),
            };

            let mut current = route.warehouse;

            let mut cost = 0f32;
            let mut demand = current.demand;
            let mut additional_cost: f32;

            loop {
                (current, additional_cost, customers) = if let Some(a) = current
                    .find_cheapest_deliverable(
                        customers.clone(),
                        cost,
                        self.vehicle_capacity.saturating_sub(demand),
                    ) {
                    a
                } else {
                    break;
                };

                cost += additional_cost;
                demand += current.demand;
                route.customers.push(current);
            }

            let route = route;

            self.routes.push(route);
        }
    }

    pub fn plot(&self) {
        for i in 0..self.routes.len() - 1 {
            self.routes[i].plot(&format! {"Route {i}"})
        }
    }
}
