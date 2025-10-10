pub mod extracellular_matrix;
pub mod organization;
pub mod tissue_types;

pub use extracellular_matrix::{ECMComponent, ExtracellularMatrix, Fiber};
pub use organization::{CellArrangement, TissueOrganization};
pub use tissue_types::{Tissue, TissueState, TissueType};
