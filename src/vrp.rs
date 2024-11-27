use crate::location::Location;
use crate::route::Route;

#[derive(Debug, Clone)]
pub struct Vrp {
    pub customers: Vec<Location>,
    pub warehouse: Location,
    pub n_vehicles: u16,
    pub vehicle_capacity: u16,
    pub routes: Vec<Route>,
}

impl Vrp {
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

    pub fn plot(&self, dirname: &str) -> &Vrp {
        for i in 0..self.routes.len() {
            self.routes[i].plot(dirname, &format! {"Route {}", i+1});
        }
        self
    }

    /// Print this VRP problem
    pub fn print(&self) -> &Vrp {
        println!("{}", self.print_to_string());
        self
    }
    /// Print this VRP problem to a string
    pub fn print_to_string(&self) -> String {
        let mut output = String::new();
        output.push_str("Vrp problem\n");
        output.push_str(&format! {"Total cost: {}\n", self.total_cost()});
        output.push_str(&format! {"n_vehicles: {}\n", self.n_vehicles});
        output.push('\n');
        for (i, route) in self.routes.iter().enumerate() {
            output.push('\n');
            output.push_str(&format! {"Is valid: {}\n", route.is_valid(self.vehicle_capacity)});
            output.push_str(&route.print_to_string(Some(&format! {"Route {}", i+1})));
            output.push('\n');
        }
        output
    }

    /// Print this VRP problem to a Markdown string
    pub fn print_to_md_string(&self) -> String {
        let mut output = String::new();
        output.push_str("# Vrp problem\n");
        output.push_str("## Details\n");
        output.push_str(&format! {"- Total cost: {}\n", self.total_cost()});
        output.push_str(&format! {"- N° of customers: {}\n", self.customers.len()});
        output.push_str(&format! {"- N° of vehicles: {}\n", self.n_vehicles});
        output.push_str(&format! {"- Vehicle capacity: {}\n", self.vehicle_capacity});

        output.push_str("\n## Routes\n");
        for (i, route) in self.routes.iter().enumerate() {
            output.push_str(&format! {"\n### Route {}\n", i+1});
            output.push_str(
                &route.print_to_md_string(self.vehicle_capacity, &format!("Route {}.svg", i + 1)),
            );
        }
        output
    }
}
