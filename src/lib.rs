//! This crate defines classes to model the VRPTW and provides a parser for Solomon VRP instances
//! It also implements various heuristics that can be used on the VRPTW

/// Parse solomon VRPTW txt files
pub mod file_parser;

/// Various heuristic that can be used on the [Vrp](vrp::Vrp) object
pub mod heuristics;

/// Represents individual locations in the VRP
pub mod location;

/// Represents vehicle routes in the VRP, contains [Location](location::Location) objects
pub mod route;

/// Represents a full VRPTW, contains [Location](location::Location) and [Route](route::Route)
/// objects
pub mod vrp;
