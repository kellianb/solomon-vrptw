#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub id: u16,
    pub x: u16,
    pub y: u16,
    pub demand: u16,
    pub ready_time: u16,
    pub due_date: u16,
    pub service_time: u16,
}

impl Location {
    // Calculate distance from current customer to other customer
    pub fn distance_to(&self, other: &Location) -> f32 {
        f32::from(u16::pow(other.x.abs_diff(self.x), 2) + u16::pow(other.y.abs_diff(self.y), 2))
            .sqrt()
    }

    // Calculate cost to deliver to other customer from this customer
    pub fn cost_to(&self, other: &Location, current_cost: f32) -> f32 {
        // Add the distance to the other customer
        let current_cost = current_cost + self.distance_to(other);

        current_cost
            + (other.ready_time as f32 - current_cost).max(0f32) // Add potentital waiting time
            + other.service_time as f32 // Add service time
    }

    // Find all neighbors whose delivery windows are reachable from the current location, return them.
    pub fn find_reachable<'a>(
        &self,
        others: Vec<&'a Location>,
        current_cost: f32,
    ) -> Vec<&'a Location> {
        others
            .into_iter()
            .filter(|&customer| {
                customer.due_date as f32 >= (self.distance_to(customer) + current_cost)
            })
            .collect()
    }

    // Find all neighbors whose delivery windows are reachable from the current location and whose demand can be fulfilled with the remaining truck capacity, return them.
    pub fn find_deliverable<'a>(
        &self,
        others: Vec<&'a Location>,
        current_cost: f32,
        remaining_capacity: u16,
    ) -> Vec<&'a Location> {
        self.find_reachable(others, current_cost)
            .into_iter()
            .filter(|&customer| customer.demand <= remaining_capacity)
            .collect()
    }

    // Find the neighbor that is the cheapest to deliver to from the current location, return it and the remaining list.
    pub fn find_cheapest_deliverable<'a>(
        &self,
        others: Vec<&'a Location>,
        current_cost: f32,
        remaining_capacity: u16,
    ) -> Option<(&'a Location, f32, Vec<&'a Location>)> {
        let deliverable = self.find_deliverable(others.clone(), current_cost, remaining_capacity);

        let cheapest = deliverable.into_iter().min_by(|&a, &b| {
            self.cost_to(a, current_cost)
                .partial_cmp(&self.cost_to(b, current_cost))
                .unwrap_or(std::cmp::Ordering::Equal)
        })?;

        let cost = self.cost_to(cheapest, current_cost);

        let others: Vec<&Location> = others
            .into_iter()
            .filter(|&location| location != cheapest)
            .collect();

        Some((cheapest, cost, others))
    }
}
