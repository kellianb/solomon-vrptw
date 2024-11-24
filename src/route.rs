use crate::location::Location;
use itertools::Itertools;
use plotters::prelude::*;

#[derive(Debug, Clone)]
pub struct Route<'a> {
    pub customers: Vec<&'a Location>,
    pub warehouse: &'a Location,
}

impl<'a> Route<'a> {
    pub fn total_distance(&self) -> f32 {
        (0..self.customers.len() - 2)
            .map(|i| self.customers[i].distance_to(self.customers[i + 1]))
            .sum::<f32>()
            // Distance from warehouse to first customer
            + self.warehouse.distance_to(self.customers[0])
            // Distance from last customer to warehouse
            + self.customers[self.customers.len() -1].distance_to(self.warehouse)
    }

    pub fn total_cost(&self) -> f32 {
        let mut cost = self.warehouse.cost_to(self.customers[0], 0f32);
        for i in 0..self.customers.len() - 2 {
            cost += self.customers[i].cost_to(self.customers[i + 1], cost)
        }
        cost + self.customers[self.customers.len() - 1].cost_to(self.warehouse, cost)
    }

    // Does not take into account any time window or capacity restrictions
    pub fn brute_force(&self) -> (f32, Route) {
        let mut shortest = self.clone();
        let mut min_len = shortest.total_distance();

        for i in self
            .customers
            .iter()
            .copied()
            .permutations(self.customers.len())
        {
            let route = Route {
                customers: i.clone(),
                warehouse: self.warehouse,
            };
            let len = route.total_distance();

            if len < min_len {
                min_len = len;
                shortest = route;
            }
        }

        (min_len, shortest)
    }

    pub fn plot(&self, title: &str) {
        let root = SVGBackend::new(title, (640, 480)).into_drawing_area();
        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .build_cartesian_2d(
                0..(self.customers.iter().map(|c| c.x as i32).max().unwrap_or(0) + 10),
                0..(self.customers.iter().map(|c| c.y as i32).max().unwrap_or(0) + 10),
            )
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("X")
            .y_desc("Y")
            .draw()
            .unwrap();

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
        }

        let route_iter = std::iter::once((self.warehouse.x as i32, self.warehouse.y as i32)).chain(
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

        root.present().unwrap();
        println!("SVG file saved as 'vrp_route.svg'");
    }
}
