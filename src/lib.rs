#[doc=include_str!("domain/docs/about_domain.md")]
pub mod domain;

#[doc=include_str!("geometry/docs/about_geometry.md")]
pub mod geometry;

#[doc=include_str!("boundary/docs/about_boundary.md")]
pub mod boundary;

pub mod solver;
pub mod time_stepping;
pub mod linalg;
pub mod input_output;
pub mod tests;

pub use geometry::{Geometry, CellShape, FaceShape};
pub use linalg::{Vector, Matrix};
pub use domain::{Arrow, MeshEntity, Section, Sieve, mesh::Mesh};