// src/linalg/matrix/mod.rs

pub mod traits;
pub mod mat_impl;
pub mod matrix_builder;

pub use traits::Matrix;
pub use traits::MatrixOperations;
pub use traits::ExtendedMatrixOperations;

#[cfg(test)]
mod tests;