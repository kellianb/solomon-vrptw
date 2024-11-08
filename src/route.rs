use crate::location::Location;
use plotters::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Route {
    pub customers: Vec<Location>,
    pub warehouse: Location,
}

impl Route {
    pub fn len(&self) -> f64 {
        (0..self.customers.len() - 2)
            .map(|i| self.customers[i].distance_to(&self.customers[i + 1]))
            .sum::<f64>()
            // Distance from warehouse to first customer
            + self.warehouse.distance_to(&self.customers[0])
            // Distance from last customer to warehouse
            + self.customers[self.customers.len() -1].distance_to(&self.warehouse)
    }
    fn permute(&self) -> Vec<Route> {
        let mut result: Vec<Vec<Location>> = Vec::new();
        permute(&mut self.customers.clone(), 0, &mut result);

        result
            .into_iter()
            .map(|x| Route {
                customers: x,
                warehouse: self.warehouse.clone(),
            })
            .collect()
    }

    pub fn brute_force(&self) -> (u32, Route) {
        let route_lenghts: HashMap<u32, Route> = self
            .permute()
            .into_iter()
            .map(|x| ((x.len() * 100f64) as u32, x))
            .collect();

        let min_len = route_lenghts
            .keys()
            .min()
            .expect("Failed to calculate min len")
            .clone();

        let shortest = route_lenghts
            .get(&min_len)
            .expect("Failed to get min len")
            .clone();

        (min_len, shortest)
    }

    pub fn plot(&self) {
        let root = SVGBackend::new("vrp_route.svg", (640, 480)).into_drawing_area();
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

fn permute<T: Clone>(vec: &mut Vec<T>, start: usize, result: &mut Vec<Vec<T>>) {
    if start == vec.len() {
        result.push(vec.clone());
    } else {
        for i in start..vec.len() {
            vec.swap(start, i);
            permute(vec, start + 1, result);
            vec.swap(start, i); // backtrack
        }
    }
}
