use crate::{route::Route, vrp::Vrp};
use plotters::prelude::*;
use rand::Rng;

#[derive(Debug, Clone, Default)]
pub struct VrpResult {
    pub n_vehicles: u16,
    pub vehicle_capacity: u16,
    pub routes: Vec<Route>,
    pub coord_bounds: (i32, i32, i32, i32),
    pub heuristic_cost_history: Option<Vec<f32>>,
}

impl VrpResult {
    pub fn from_vrp(
        vrp: &Vrp,
        routes: Vec<Route>,
        heuristic_cost_history: Option<Vec<f32>>,
    ) -> VrpResult {
        VrpResult {
            routes,
            heuristic_cost_history,
            ..vrp.to_result()
        }
    }

    pub fn total_cost(&self) -> f32 {
        self.routes.iter().map(|x| x.total_cost()).sum()
    }

    pub fn total_cost_with(&self, routes: &[Route]) -> f32 {
        routes.iter().map(|x| x.total_cost()).sum()
    }

    pub fn total_cost_no_service_time(&self) -> f32 {
        self.routes
            .iter()
            .map(|x| x.total_cost_no_service_time())
            .sum()
    }

    pub fn total_cost_no_service_time_with(&self, routes: &[Route]) -> f32 {
        routes.iter().map(|x| x.total_cost_no_service_time()).sum()
    }

    /// Print this VRP problem
    pub fn print(&self) -> &VrpResult {
        println!("{}", self.as_string());
        self
    }
    /// Print this VRP problem to a string
    pub fn as_string(&self) -> String {
        let mut output = String::new();
        output.push_str("Vrp problem\n");
        output.push_str(&format! {"Total cost: {}\n", self.total_cost()});
        output.push_str(&format! {"n_vehicles: {}\n", self.n_vehicles});
        output.push('\n');
        for (i, route) in self.routes.iter().enumerate() {
            output.push('\n');
            output.push_str(&format! {"Is valid: {}\n", route.is_valid(self.vehicle_capacity)});
            output.push_str(&route.print_to_string(Some(&format! {"Route {}", i + 1})));
            output.push('\n');
        }
        output
    }

    /// Print this VRP problem to a Markdown string
    pub fn as_md_string(&self) -> String {
        let mut output = String::new();
        output.push_str("# Vrp problem\n");
        output.push_str("## Details\n\n");
        output.push_str(&format! {"- Total cost: {}\n", self.total_cost()});
        output.push_str(&format! {"- N° of vehicles: {}\n", self.n_vehicles});
        output.push_str(&format! {"- Vehicle capacity: {}\n", self.vehicle_capacity});

        if let Some(val) = self.plot_heuristic_cost_history() {
            output.push_str("\n## Heuristic Cost History\n\n");
            output.push_str(&val);
        }

        output.push_str("\n## Display\n\n");

        output.push_str(&self.plot());

        output.push_str("\n## Routes\n");
        for (i, route) in self.routes.iter().enumerate() {
            output.push_str(&format! {"\n### Route {}\n", i + 1});
            output.push_str(&route.print_to_md_string(self.vehicle_capacity, self.coord_bounds));
        }
        output
    }

    pub fn plot(&self) -> String {
        let mut svg_data: String = String::new();
        {
            let root = SVGBackend::with_string(&mut svg_data, (800, 480)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let coord_bounds = self.coord_bounds;

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
            if let Some(val) = self.routes.first() {
                chart
                    .draw_series(std::iter::once(Circle::new(
                        (val.warehouse.x as i32, val.warehouse.y as i32),
                        3,
                        RED.filled(),
                    )))
                    .unwrap();
            }

            // Plot the customers
            for route in &self.routes {
                let color = random_color();
                for customer in &route.customers {
                    chart
                        .draw_series(std::iter::once(Circle::new(
                            (customer.x as i32, customer.y as i32),
                            3,
                            color.filled(),
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
            }

            root.present().unwrap();
        }
        svg_data
    }

    pub fn plot_heuristic_cost_history(&self) -> Option<String> {
        let mut svg_data: String = String::new();
        if let Some(history) = &self.heuristic_cost_history {
            let root = SVGBackend::with_string(&mut svg_data, (800, 480)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let max_cost = history.iter().cloned().fold(f32::NAN, f32::max);
            let min_cost = history.iter().cloned().fold(f32::NAN, f32::min);

            let mut chart = ChartBuilder::on(&root)
                .margin(5)
                .x_label_area_size(35)
                .y_label_area_size(40)
                .build_cartesian_2d(0..history.len(), min_cost..max_cost)
                .unwrap();

            chart
                .configure_mesh()
                .x_desc("Iteration")
                .y_desc("Cost")
                .draw()
                .unwrap();

            chart
                .draw_series(LineSeries::new(
                    history.iter().enumerate().map(|(i, &cost)| (i, cost)),
                    &RED,
                ))
                .unwrap()
                .label("Cost")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

            chart
                .configure_series_labels()
                .background_style(WHITE.mix(0.8))
                .draw()
                .unwrap();

            root.present().unwrap();
        } else {
            return None;
        }
        Some(svg_data)
    }
}

fn random_color() -> RGBColor {
    let mut rng = rand::thread_rng();
    RGBColor(
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
        rng.gen_range(0..=255),
    )
}
