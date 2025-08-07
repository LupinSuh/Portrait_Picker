//! # Alpha Matting
//! Alpha matting is used to extract a foreground object with soft boundaries from a background image.
//! 
//! This module is dedicated to computing alpha matte of objects in images from a given input image and a greyscale trimap image that contains information about the foreground, background and unknown pixels. The unknown pixels are assumed to be a combination of foreground and background pixels. The algorithm uses a combination of multiple carefully defined pixels affinities to estimate the opacity of the foreground pixels in the unkown region.
//! 
//! The implementation is based on [aksoy2017designing](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_aksoy2017designing).
//! 
//! This module was developed by Muskaan Kularia and Sunita Nayak as a project
//! for Google Summer of Code 2019 (GSoC 19).
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}
