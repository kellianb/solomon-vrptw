use crate::location::Location;
use crate::route::Route;
use crate::vrp::Vrp;
use crate::vrp_result::VrpResult;

impl Vrp {
    pub fn nearest_neighbour_heuristic(&self) -> VrpResult {
        let mut customers: Vec<&Location> = self.customers.iter().collect();

        let mut routes: Vec<Route> = Vec::new();

        while !customers.is_empty() {
            let mut route = Route {
                warehouse: self.warehouse.clone(),
                ..Default::default()
            };

            let mut current = &route.warehouse;

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
                route.customers.push(current.clone());
            }

            let route = route;

            routes.push(route);
        }
        VrpResult::from_vrp(self, routes, None)
    }
}
