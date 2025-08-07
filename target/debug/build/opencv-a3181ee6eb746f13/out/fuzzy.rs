//! # Image processing based on fuzzy mathematics
//! 
//! Namespace for all functions is `ft`. The module brings implementation of the last image processing algorithms based on fuzzy mathematics. Method are named based on the pattern `FT`_degree_dimension`_`method.
//!    # Math with F0-transform support
//! 
//!    Fuzzy transform (![inline formula](https://latex.codecogs.com/png.latex?F%5E0)-transform) of the 0th degree transforms whole image to a matrix of its components. These components are used in latter computation where each of them represents average color of certain subarea.
//! 
//!    # Math with F1-transform support
//! 
//!    Fuzzy transform (![inline formula](https://latex.codecogs.com/png.latex?F%5E1)-transform) of the 1th degree transforms whole image to a matrix of its components. Each component is polynomial of the 1th degree carrying information about average color and average gradient of certain subarea.
//! 
//!    # Fuzzy image processing
//! 
//!    Image proceesing based on fuzzy mathematics namely F-transform.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}

/// processing in several iterations
pub const ITERATIVE: i32 = 3;
/// linear (triangular) shape
pub const LINEAR: i32 = 1;
/// processing in multiple step
pub const MULTI_STEP: i32 = 2;
/// processing in one step
pub const ONE_STEP: i32 = 1;
/// sinusoidal shape
pub const SINUS: i32 = 2;