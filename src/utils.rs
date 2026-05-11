// Utility script written in order to organize the structure

/// Below, both modules from 'utils' subdirectory are called in 'utils.rs'
/// This makes followings possible:
/// * being able to call them inside the main
/// * more organized structure
/// * modification of 'server' and 'address' are now easier
///
pub mod client;
pub mod server;
