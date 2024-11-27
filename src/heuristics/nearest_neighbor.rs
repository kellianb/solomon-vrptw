use crate::location::Location;
use crate::route::Route;
use crate::vrp::Vrp;

impl Vrp {
    pub fn nearest_neighbour_heuristic(&mut self) -> &mut Vrp {
        let mut customers: Vec<&Location> = self.customers.iter().collect();

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

            self.routes.push(route);
        }
        self
    }
}
