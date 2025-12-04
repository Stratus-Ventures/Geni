// Clean Architecture Layers
// Dependency Rule: Domain <- Application <- Infrastructure <- API

pub mod domain;
pub mod application;
pub mod adapters;
pub mod infra;
