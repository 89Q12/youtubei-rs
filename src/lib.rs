/// contains functions to generate the data to make an API call.
/// private because they return raw data and or errors
pub(crate) mod endpoints;
/// Contains all the structs used to represent the data returned by the API
pub mod types;
/// Contains utilities for various tasks
pub mod utils;
/// private because they are intended to be called by query functions only
pub(crate) mod extractors;
/// Contains the main functions of this library
/// Refer to the tests for usage information
pub mod query;
/// Contains test functions
mod tests;