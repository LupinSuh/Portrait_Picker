//! # The module brings implementations of intensity transformation algorithms to adjust image contrast.
//! 
//! Namespace for all functions is `cv::intensity_transform`.
//! 
//! ### Supported Algorithms
//! - Autoscaling
//! - Log Transformations
//! - Power-Law (Gamma) Transformations
//! - Contrast Stretching
//! - BIMEF, A Bio-Inspired Multi-Exposure Fusion Framework for Low-light Image Enhancement [ying2017bio](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_ying2017bio) [ying2017new](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_ying2017new)
//! 
//! References from following book and websites:
//! - Digital Image Processing 4th Edition Chapter 3 [Rafael C. Gonzalez, Richard E. Woods] [Gonzalez2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Gonzalez2018)
//! - <http://www.cs.uregina.ca/Links/class-info/425/Lab3/> [lcs435lab](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_lcs435lab)
//! - <https://theailearner.com/2019/01/30/contrast-stretching/> [theailearner](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_theailearner)
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use {  };
}
