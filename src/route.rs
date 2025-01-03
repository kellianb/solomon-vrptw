use crate::location::Location;
use plotters::prelude::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Route {
    pub warehouse: Location,
    pub customers: Vec<Location>,
}

impl Route {
    pub fn len(&self) -> usize {
        self.customers.len() + 2
    }
    pub fn is_empty(&self) -> bool {
        self.customers.is_empty()
    }
    pub fn total_distance(&self) -> f32 {
        (0..self.len() - 1)
            .map(|i| self[i].distance_to(&self[i + 1]))
            .sum::<f32>()
    }

    // -- Calculate total route cost --
    // Get the cost for this route (distance + waiting time + service time)
    pub fn total_cost(&self) -> f32 {
        let mut cost = 0.0;

        for i in 0..self.len() - 1 {
            cost = self[i].cost_to_deliver(&self[i + 1], cost)
        }

        cost
    }

    // Get the cost for this route using a separate array of customers (distance + waiting time + service time)
    pub fn total_cost_with(&self, customers: &[&Location]) -> f32 {
        let mut cost = self.warehouse.cost_to_deliver(customers[0], 0f32);

        for i in 0..customers.len() - 1 {
            cost = customers[i].cost_to_deliver(customers[i + 1], cost)
        }

        self.customers[customers.len() - 1].cost_to_deliver(&self.warehouse, cost)
    }

    // -- Calculate total route cost without service time --
    // Get the cost for this route (distance + waiting time)
    pub fn total_cost_no_service_time(&self) -> f32 {
        let mut cost = 0.0;

        for i in 0..self.len() - 1 {
            cost = self[i].cost_to_delivery_window(&self[i + 1], cost)
        }

        cost
    }

    // Get the cost for this route using a separate array of customers (distance + waiting time)
    pub fn total_cost_no_service_time_with(&self, customers: &[&Location]) -> f32 {
        let mut cost = self.warehouse.cost_to_delivery_window(customers[0], 0f32);

        for i in 0..customers.len() - 1 {
            cost = customers[i].cost_to_delivery_window(customers[i + 1], cost)
        }

        self.customers[customers.len() - 1].cost_to_delivery_window(&self.warehouse, cost)
    }

    // -- Calculate the total demand of all customers in the route
    pub fn total_demand(&self) -> u16 {
        self.customers.iter().map(|c| c.demand).sum()
    }

    pub fn total_demand_with(customers: &[&Location]) -> u16 {
        customers.iter().map(|c| c.demand).sum()
    }

    // -- Check if route is valid --
    pub fn is_valid(&self, capacity: u16) -> bool {
        if self.total_demand() > capacity {
            return false;
        }

        if self.customers.is_empty() {
            return true;
        }

        let mut cost = self.warehouse.cost_to(&self.customers[0], 0f32);

        for (i, customer) in self.customers.iter().enumerate() {
            if cost > customer.due_date as f32 {
                return false;
            }

            cost += (customer.ready_time as f32 - cost).max(0f32); // Add potentital waiting time
            cost += customer.service_time as f32; // Add service time

            // If this is not the last customer, add the cost to the next customer
            if i < self.customers.len() - 1 {
                cost = customer.cost_to(&self.customers[i + 1], cost)
            }
        }

        cost = self
            .customers
            .last()
            .unwrap()
            .cost_to(&self.warehouse, cost);

        if cost > self.warehouse.due_date as f32 {
            return false;
        }

        true
    }

    pub fn is_valid_with(&self, customers: &[&Location], capacity: u16) -> bool {
        if Route::total_demand_with(customers) > capacity {
            return false;
        }

        if customers.is_empty() {
            return true;
        }

        let mut cost = self.warehouse.cost_to(customers[0], 0f32);

        for (i, customer) in customers.iter().enumerate() {
            if cost > customer.due_date as f32 {
                return false;
            }

            cost += (customer.ready_time as f32 - cost).max(0f32); // Add potentital waiting time
            cost += customer.service_time as f32; // Add service time

            // If this is not the last customer, add the cost to the next customer
            if i < customers.len() - 1 {
                cost = customer.cost_to(customers[i + 1], cost)
            }
        }

        cost = customers.last().unwrap().cost_to(&self.warehouse, cost);

        if cost > self.warehouse.due_date as f32 {
            return false;
        }

        true
    }

    // -- Try and insert a customer into the route, find the best index --
    pub fn try_insert(&self, customer: &Location, capacity: u16) -> Option<(f32, u16)> {
        let mut min_cost = f32::INFINITY;
        let mut min_index = 0;

        let customers: Vec<&Location> = self.customers.iter().collect();

        for i in 0..self.customers.len() {
            let new_customers: Vec<&Location> = customers[0..i]
                .iter()
                .chain(std::iter::once(&customer))
                .chain(customers[i..].iter())
                .cloned()
                .collect();

            if !self.is_valid_with(&new_customers, capacity) {
                continue;
            }

            let cost = self.total_cost_with(&new_customers);
            if cost < min_cost {
                min_cost = cost;
                min_index = i;
            }
        }

        // If no suitable insertion index was found, return None
        if min_cost == f32::INFINITY {
            return None;
        }

        Some((min_cost, min_index as u16))
    }

    // -- Print the route --
    pub fn print(&self, name: Option<&str>) -> &Route {
        print!("{}", self.print_to_string(name));
        self
    }

    pub fn print_to_string(&self, name: Option<&str>) -> String {
        let name = name.unwrap_or("Route");

        let mut output = String::new();

        output.push_str(&format!("==== {} =====\n", name));
        output.push_str(&format!("Total demand: {}\n", self.total_demand()));
        output.push_str(&format!("Total distance: {}\n", self.total_distance()));
        output.push_str(&format!("Total cost: {}\n", self.total_cost()));
        output.push_str(&format!("Total customers: {}\n", self.customers.len()));
        output.push('\n');

        let mut cost = 0.0;

        output.push_str(&format!(
            "{:<30} ID: {}  TW: {} - {}\n",
            "■ Warehouse", self.warehouse.id, self.warehouse.ready_time, self.warehouse.due_date
        ));
        output.push_str(&format!("|   Departure: {}\n", cost));
        output.push_str("|\n");

        cost += if !self.customers.is_empty() {
            self.warehouse.distance_to(&self.customers[0])
        } else {
            0.0
        };

        for (i, customer) in self.customers.iter().enumerate() {
            output.push_str("|\n");
            output.push_str(&format!("▼   Arrival: {}\n", cost));
            output.push_str(&format!(
                "{:<30} ID: {}  TW: {} - {}\n",
                format!("⌂ Customer {}/{}", i + 1, self.customers.len()),
                customer.id,
                customer.ready_time,
                customer.due_date
            ));

            let waiting_time = (customer.ready_time as f32 - cost).max(0.0);
            output.push_str(&format!("… Waiting Time: {}\n", waiting_time));
            cost += waiting_time;
            output.push_str(&format!("… Service Time: {}\n", customer.service_time));
            cost += customer.service_time as f32;
            output.push_str(&format!("|   Departure: {}\n", cost));
            output.push_str("|\n");

            if i < self.customers.len() - 1 {
                cost += customer.distance_to(&self.customers[i + 1]);
            }
        }

        cost += if !self.customers.is_empty() {
            self.customers.last().unwrap().distance_to(&self.warehouse)
        } else {
            0.0
        };

        output.push_str("|\n");
        output.push_str(&format!("▼   Arrival: {}\n", cost));
        output.push_str(&format!(
            "{:<30} ID: {}  TW: {} - {}\n",
            "■ Warehouse", self.warehouse.id, self.warehouse.ready_time, self.warehouse.due_date
        ));

        output
    }

    pub fn print_to_md_string(
        &self,
        vehicle_capacity: u16,
        coord_bounds: (i32, i32, i32, i32),
    ) -> String {
        let mut output = String::new();

        output.push_str("\n#### Details\n\n");
        output.push_str(&format!("- Total demand: {}\n", self.total_demand()));
        output.push_str(&format!("- Total distance: {}\n", self.total_distance()));
        output.push_str(&format!("- Total cost: {}\n", self.total_cost()));
        output.push_str(&format!(
            "- Total cost without service time: {}\n",
            self.total_cost_no_service_time()
        ));
        output.push_str(&format!("- Total customers: {}\n", self.customers.len()));
        output.push_str(&format!(
            "- Is valid: {}\n",
            self.is_valid(vehicle_capacity)
        ));

        output.push_str("\n#### Display\n\n");

        output.push_str(&self.plot(coord_bounds));

        output.push_str("\n#### Locations\n\n");

        let mut cost = 0.0;

        output.push_str("```\n");

        output.push_str(&format!(
            "{:<30} ID: {}  TW: {} - {}\n",
            "■ Warehouse", self.warehouse.id, self.warehouse.ready_time, self.warehouse.due_date
        ));
        output.push_str(&format!("|   Departure: {}\n", cost));
        output.push_str("|\n");

        cost += if !self.customers.is_empty() {
            self.warehouse.distance_to(&self.customers[0])
        } else {
            0.0
        };

        for (i, customer) in self.customers.iter().enumerate() {
            output.push_str("|\n");
            output.push_str(&format!("▼   Arrival: {}\n", cost));
            output.push_str(&format!(
                "{:<30} ID: {}  TW: {} - {}\n",
                format!("⌂ Customer {}/{}", i + 1, self.customers.len()),
                customer.id,
                customer.ready_time,
                customer.due_date
            ));

            let waiting_time = (customer.ready_time as f32 - cost).max(0.0);
            output.push_str(&format!("… Waiting Time: {}\n", waiting_time));
            cost += waiting_time;
            output.push_str(&format!("… Service Time: {}\n", customer.service_time));
            cost += customer.service_time as f32;
            output.push_str(&format!("|   Departure: {}\n", cost));
            output.push_str("|\n");

            if i < self.customers.len() - 1 {
                cost += customer.distance_to(&self.customers[i + 1]);
            }
        }

        cost += if !self.customers.is_empty() {
            self.customers.last().unwrap().distance_to(&self.warehouse)
        } else {
            0.0
        };

        output.push_str("|\n");
        output.push_str(&format!("▼   Arrival: {}\n", cost));
        output.push_str(&format!(
            "{:<30} ID: {}  TW: {} - {}\n",
            "■ Warehouse", self.warehouse.id, self.warehouse.ready_time, self.warehouse.due_date
        ));

        output.push_str("```\n");

        output
    }
    pub fn plot(&self, coord_bounds: (i32, i32, i32, i32)) -> String {
        let mut svg_data: String = String::new();
        {
            let root = SVGBackend::with_string(&mut svg_data, (800, 480)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .build_cartesian_2d(
                    coord_bounds.0..coord_bounds.1,
                    coord_bounds.2..coord_bounds.3,
                )
                .unwrap();

            chart
                .configure_mesh()
                .x_desc("X")
                .y_desc("Y")
                .draw()
                .unwrap();

            // -- Plot the route --
            let route_iter = std::iter::once((self.warehouse.x as i32, self.warehouse.y as i32))
                .chain(
                    self.customers
                        .iter()
                        .map(|c| (c.x as i32, c.y as i32))
                        .chain(std::iter::once((
                            self.warehouse.x as i32,
                            self.warehouse.y as i32,
                        ))),
                );

            // Plot the route
            chart
                .draw_series(LineSeries::new(route_iter, &GREEN))
                .unwrap();

            // -- Plot the locations --
            // Plot the warehouse
            chart
                .draw_series(std::iter::once(Circle::new(
                    (self.warehouse.x as i32, self.warehouse.y as i32),
                    5,
                    RED.filled(),
                )))
                .unwrap();

            // Plot the customers
            for customer in &self.customers {
                chart
                    .draw_series(std::iter::once(Circle::new(
                        (customer.x as i32, customer.y as i32),
                        5,
                        BLUE.filled(),
                    )))
                    .unwrap();

                chart
                    .draw_series(std::iter::once(Text::new(
                        format!("{}", customer.id),
                        (customer.x as i32 + 1, customer.y as i32 + 1),
                        ("sans-serif", 15).into_font(),
                    )))
                    .unwrap();
            }

            root.present().unwrap();
        }
        svg_data
    }

    pub fn iter(&self) -> RouteIterator {
        RouteIterator {
            route: self,
            index: 0,
        }
    }
}

impl std::ops::Index<usize> for Route {
    type Output = Location;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.warehouse,
            x if x <= self.customers.len() => &self.customers[index - 1],
            x if x == self.customers.len() + 1 => &self.warehouse,
            _ => panic!("Index out of bounds"),
        }
    }
}

// -- Implement route iterator --
pub struct RouteIterator<'a> {
    route: &'a Route,
    index: usize,
}

impl<'a> Iterator for RouteIterator<'a> {
    type Item = &'a Location;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(&self.route.warehouse)
            }
            x if x <= self.route.customers.len() => {
                self.index += 1;
                Some(&self.route.customers[x - 1])
            }
            x if x == self.route.customers.len() + 1 => {
                self.index += 1;
                Some(&self.route.warehouse)
            }
            _ => None,
        }
    }
}
