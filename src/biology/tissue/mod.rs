pub mod tissue_types;
pub mod extracellular_matrix;
pub mod organization;

pub use tissue_types::{Tissue, TissueType, TissueState};
pub use extracellular_matrix::{ExtracellularMatrix, ECMComponent, Fiber};
pub use organization::{TissueOrganization, CellArrangement};
