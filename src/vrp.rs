use crate::location::Location;
use crate::vrp_result::VrpResult;

use plotters::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Vrp {
    pub customers: Vec<Location>,
    pub warehouse: Location,
    pub n_vehicles: u16,
    pub vehicle_capacity: u16,
}

impl Vrp {
    pub fn new(
        warehouse: Location,
        customers: Vec<Location>,
        n_vehicles: u16,
        vehicle_capacity: u16,
    ) -> Vrp {
        Vrp {
            warehouse,
            customers,
            n_vehicles,
            vehicle_capacity,
        }
    }

    pub fn to_result(&self) -> VrpResult {
        VrpResult {
            n_vehicles: self.n_vehicles,
            vehicle_capacity: self.vehicle_capacity,
            coord_bounds: self.get_coord_bounds(),
            ..Default::default()
        }
    }

    pub fn get_coord_bounds(&self) -> (i32, i32, i32, i32) {
        let x_coords = std::iter::once(self.warehouse.x as i32).chain(
            self.customers
                .iter()
                .map(|c| c.x as i32)
                .chain(std::iter::once(self.warehouse.x as i32)),
        );

        let y_coords = std::iter::once(self.warehouse.y as i32).chain(
            self.customers
                .iter()
                .map(|c| c.y as i32)
                .chain(std::iter::once(self.warehouse.y as i32)),
        );
        (
            x_coords.clone().min().unwrap() - 10,
            x_coords.max().unwrap() + 10,
            y_coords.clone().min().unwrap() - 10,
            y_coords.max().unwrap() + 10,
        )
    }

    /// Print this VRP problem to a Markdown string
    pub fn print_to_md_string(&self) -> String {
        let mut output = String::new();
        output.push_str("# Vrp problem\n");
        output.push_str("## Details\n\n");
        output.push_str(&format! {"- N° of customers: {}\n", self.customers.len()});
        output.push_str(&format! {"- N° of vehicles: {}\n", self.n_vehicles});
        output.push_str(&format! {"- Vehicle capacity: {}\n", self.vehicle_capacity});

        output.push_str("\n## Display\n\n");

        output.push_str(&self.plot());

        output
    }

    pub fn plot(&self) -> String {
        let mut svg_data: String = String::new();
        {
            let root = SVGBackend::with_string(&mut svg_data, (800, 480)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let coord_bounds = self.get_coord_bounds();

            let mut chart = ChartBuilder::on(&root)
                .margin(5)
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
}
