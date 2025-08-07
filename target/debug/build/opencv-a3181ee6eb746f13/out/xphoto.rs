//! # Additional photo processing algorithms
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::WhiteBalancerTraitConst, super::WhiteBalancerTrait, super::SimpleWBTraitConst, super::SimpleWBTrait, super::GrayworldWBTraitConst, super::GrayworldWBTrait, super::LearningBasedWBTraitConst, super::LearningBasedWBTrait, super::TonemapDurandTraitConst, super::TonemapDurandTrait };
}

/// Execute only first step of the algorithm
pub const BM3D_STEP1: i32 = 1;
/// Execute only second step of the algorithm
pub const BM3D_STEP2: i32 = 2;
/// Execute all steps of the algorithm
pub const BM3D_STEPALL: i32 = 0;
/// Un-normalized Haar transform
pub const HAAR: i32 = 0;
/// Performs Frequency Selective Reconstruction (FSR).
/// One of the two quality profiles BEST and FAST can be chosen, depending on the time available for reconstruction.
/// See [GenserPCS2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_GenserPCS2018) and [SeilerTIP2015](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_SeilerTIP2015) for details.
/// 
/// The algorithm may be utilized for the following areas of application:
/// 1. %Error Concealment (Inpainting).
///    The sampling mask indicates the missing pixels of the distorted input
///    image to be reconstructed.
/// 2. Non-Regular Sampling.
///    For more information on how to choose a good sampling mask, please review
///    [GroscheICIP2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_GroscheICIP2018) and [GroscheIST2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_GroscheIST2018).
/// 
/// 1-channel grayscale or 3-channel BGR image are accepted.
/// 
/// Conventional accepted ranges:
/// - 0-255 for CV_8U
/// - 0-65535 for CV_16U
/// - 0-1 for CV_32F/CV_64F.
pub const INPAINT_FSR_BEST: i32 = 1;
/// See #INPAINT_FSR_BEST
pub const INPAINT_FSR_FAST: i32 = 2;
/// This algorithm searches for dominant correspondences (transformations) of
/// image patches and tries to seamlessly fill-in the area to be inpainted using this
/// transformations
pub const INPAINT_SHIFTMAP: i32 = 0;
/// BM3D algorithm steps
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Bm3dSteps {
	/// Execute all steps of the algorithm
	BM3D_STEPALL = 0,
	/// Execute only first step of the algorithm
	BM3D_STEP1 = 1,
	/// Execute only second step of the algorithm
	BM3D_STEP2 = 2,
}

opencv_type_enum! { crate::xphoto::Bm3dSteps }

/// Various inpainting algorithms
/// ## See also
/// inpaint
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InpaintTypes {
	/// This algorithm searches for dominant correspondences (transformations) of
	/// image patches and tries to seamlessly fill-in the area to be inpainted using this
	/// transformations
	INPAINT_SHIFTMAP = 0,
	/// Performs Frequency Selective Reconstruction (FSR).
	/// One of the two quality profiles BEST and FAST can be chosen, depending on the time available for reconstruction.
	/// See [GenserPCS2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_GenserPCS2018) and [SeilerTIP2015](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_SeilerTIP2015) for details.
	/// 
	/// The algorithm may be utilized for the following areas of application:
	/// 1. %Error Concealment (Inpainting).
	///    The sampling mask indicates the missing pixels of the distorted input
	///    image to be reconstructed.
	/// 2. Non-Regular Sampling.
	///    For more information on how to choose a good sampling mask, please review
	///    [GroscheICIP2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_GroscheICIP2018) and [GroscheIST2018](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_GroscheIST2018).
	/// 
	/// 1-channel grayscale or 3-channel BGR image are accepted.
	/// 
	/// Conventional accepted ranges:
	/// - 0-255 for CV_8U
	/// - 0-65535 for CV_16U
	/// - 0-1 for CV_32F/CV_64F.
	INPAINT_FSR_BEST = 1,
	/// See #INPAINT_FSR_BEST
	INPAINT_FSR_FAST = 2,
}

opencv_type_enum! { crate::xphoto::InpaintTypes }

/// BM3D transform types
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransformTypes {
	/// Un-normalized Haar transform
	HAAR = 0,
}

opencv_type_enum! { crate::xphoto::TransformTypes }

/// Constant methods for [crate::xphoto::GrayworldWB]
pub trait GrayworldWBTraitConst: crate::xphoto::WhiteBalancerTraitConst {
	fn as_raw_GrayworldWB(&self) -> *const c_void;

	/// Maximum saturation for a pixel to be included in the
	///    gray-world assumption
	/// ## See also
	/// setSaturationThreshold
	#[inline]
	fn get_saturation_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_GrayworldWB_getSaturationThreshold_const(self.as_raw_GrayworldWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::xphoto::GrayworldWB]
pub trait GrayworldWBTrait: crate::xphoto::GrayworldWBTraitConst + crate::xphoto::WhiteBalancerTrait {
	fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void;

	/// Maximum saturation for a pixel to be included in the
	///    gray-world assumption
	/// ## See also
	/// setSaturationThreshold getSaturationThreshold
	#[inline]
	fn set_saturation_threshold(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_GrayworldWB_setSaturationThreshold_float(self.as_raw_mut_GrayworldWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Gray-world white balance algorithm
/// 
/// This algorithm scales the values of pixels based on a
/// gray-world assumption which states that the average of all channels
/// should result in a gray image.
/// 
/// It adds a modification which thresholds pixels based on their
/// saturation value and only uses pixels below the provided threshold in
/// finding average pixel values.
/// 
/// Saturation is calculated using the following for a 3-channel RGB image per
/// pixel I and is in the range [0, 1]:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Ctexttt%7BSaturation%7D%20%5BI%5D%20%3D%20%5Cfrac%7B%5Ctextrm%7Bmax%7D%28R%2CG%2CB%29%20%2D%20%5Ctextrm%7Bmin%7D%28R%2CG%2CB%29%0A%7D%7B%5Ctextrm%7Bmax%7D%28R%2CG%2CB%29%7D%20)
/// 
/// A threshold of 1 means that all pixels are used to white-balance, while a
/// threshold of 0 means no pixels are used. Lower thresholds are useful in
/// white-balancing saturated images.
/// 
/// Currently supports images of type [CV_8UC3] and [CV_16UC3].
pub struct GrayworldWB {
	ptr: *mut c_void
}

opencv_type_boxed! { GrayworldWB }

impl Drop for GrayworldWB {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xphoto_GrayworldWB_delete(self.as_raw_mut_GrayworldWB()) };
	}
}

unsafe impl Send for GrayworldWB {}

impl core::AlgorithmTraitConst for GrayworldWB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GrayworldWB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerTraitConst for GrayworldWB {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::WhiteBalancerTrait for GrayworldWB {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::GrayworldWBTraitConst for GrayworldWB {
	#[inline] fn as_raw_GrayworldWB(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::GrayworldWBTrait for GrayworldWB {
	#[inline] fn as_raw_mut_GrayworldWB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GrayworldWB {
}

boxed_cast_base! { GrayworldWB, core::Algorithm, cv_xphoto_GrayworldWB_to_Algorithm }

boxed_cast_base! { GrayworldWB, crate::xphoto::WhiteBalancer, cv_xphoto_GrayworldWB_to_WhiteBalancer }

impl std::fmt::Debug for GrayworldWB {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GrayworldWB")
			.finish()
	}
}

/// Constant methods for [crate::xphoto::LearningBasedWB]
pub trait LearningBasedWBTraitConst: crate::xphoto::WhiteBalancerTraitConst {
	fn as_raw_LearningBasedWB(&self) -> *const c_void;

	/// Maximum possible value of the input image (e.g. 255 for 8 bit images,
	///            4095 for 12 bit images)
	/// ## See also
	/// setRangeMaxVal
	#[inline]
	fn get_range_max_val(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_getRangeMaxVal_const(self.as_raw_LearningBasedWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Threshold that is used to determine saturated pixels, i.e. pixels where at least one of the
	///    channels exceeds ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsaturation%5Fthreshold%7D%5Ctimes%5Ctexttt%7Brange%5Fmax%5Fval%7D) are ignored.
	/// ## See also
	/// setSaturationThreshold
	#[inline]
	fn get_saturation_threshold(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_getSaturationThreshold_const(self.as_raw_LearningBasedWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Defines the size of one dimension of a three-dimensional RGB histogram that is used internally
	///    by the algorithm. It often makes sense to increase the number of bins for images with higher bit depth
	///    (e.g. 256 bins for a 12 bit image).
	/// ## See also
	/// setHistBinNum
	#[inline]
	fn get_hist_bin_num(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_getHistBinNum_const(self.as_raw_LearningBasedWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::xphoto::LearningBasedWB]
pub trait LearningBasedWBTrait: crate::xphoto::LearningBasedWBTraitConst + crate::xphoto::WhiteBalancerTrait {
	fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void;

	/// Implements the feature extraction part of the algorithm.
	/// 
	/// In accordance with [Cheng2015](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Cheng2015) , computes the following features for the input image:
	/// 1. Chromaticity of an average (R,G,B) tuple
	/// 2. Chromaticity of the brightest (R,G,B) tuple (while ignoring saturated pixels)
	/// 3. Chromaticity of the dominant (R,G,B) tuple (the one that has the highest value in the RGB histogram)
	/// 4. Mode of the chromaticity palette, that is constructed by taking 300 most common colors according to
	///    the RGB histogram and projecting them on the chromaticity plane. Mode is the most high-density point
	///    of the palette, which is computed by a straightforward fixed-bandwidth kernel density estimator with
	///    a Epanechnikov kernel function.
	/// 
	/// ## Parameters
	/// * src: Input three-channel image (BGR color space is assumed).
	/// * dst: An array of four (r,g) chromaticity tuples corresponding to the features listed above.
	#[inline]
	fn extract_simple_features(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_extractSimpleFeatures_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_LearningBasedWB(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Maximum possible value of the input image (e.g. 255 for 8 bit images,
	///            4095 for 12 bit images)
	/// ## See also
	/// setRangeMaxVal getRangeMaxVal
	#[inline]
	fn set_range_max_val(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_setRangeMaxVal_int(self.as_raw_mut_LearningBasedWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Threshold that is used to determine saturated pixels, i.e. pixels where at least one of the
	///    channels exceeds ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bsaturation%5Fthreshold%7D%5Ctimes%5Ctexttt%7Brange%5Fmax%5Fval%7D) are ignored.
	/// ## See also
	/// setSaturationThreshold getSaturationThreshold
	#[inline]
	fn set_saturation_threshold(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_setSaturationThreshold_float(self.as_raw_mut_LearningBasedWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Defines the size of one dimension of a three-dimensional RGB histogram that is used internally
	///    by the algorithm. It often makes sense to increase the number of bins for images with higher bit depth
	///    (e.g. 256 bins for a 12 bit image).
	/// ## See also
	/// setHistBinNum getHistBinNum
	#[inline]
	fn set_hist_bin_num(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_LearningBasedWB_setHistBinNum_int(self.as_raw_mut_LearningBasedWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// More sophisticated learning-based automatic white balance algorithm.
/// 
/// As [GrayworldWB], this algorithm works by applying different gains to the input
/// image channels, but their computation is a bit more involved compared to the
/// simple gray-world assumption. More details about the algorithm can be found in
/// [Cheng2015](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Cheng2015) .
/// 
/// To mask out saturated pixels this function uses only pixels that satisfy the
/// following condition:
/// 
/// ![block formula](https://latex.codecogs.com/png.latex?%20%5Cfrac%7B%5Ctextrm%7Bmax%7D%28R%2CG%2CB%29%7D%7B%5Ctexttt%7Brange%5Fmax%5Fval%7D%7D%20%3C%20%5Ctexttt%7Bsaturation%5Fthresh%7D%20)
/// 
/// Currently supports images of type [CV_8UC3] and [CV_16UC3].
pub struct LearningBasedWB {
	ptr: *mut c_void
}

opencv_type_boxed! { LearningBasedWB }

impl Drop for LearningBasedWB {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xphoto_LearningBasedWB_delete(self.as_raw_mut_LearningBasedWB()) };
	}
}

unsafe impl Send for LearningBasedWB {}

impl core::AlgorithmTraitConst for LearningBasedWB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LearningBasedWB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerTraitConst for LearningBasedWB {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::WhiteBalancerTrait for LearningBasedWB {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::LearningBasedWBTraitConst for LearningBasedWB {
	#[inline] fn as_raw_LearningBasedWB(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::LearningBasedWBTrait for LearningBasedWB {
	#[inline] fn as_raw_mut_LearningBasedWB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LearningBasedWB {
}

boxed_cast_base! { LearningBasedWB, core::Algorithm, cv_xphoto_LearningBasedWB_to_Algorithm }

boxed_cast_base! { LearningBasedWB, crate::xphoto::WhiteBalancer, cv_xphoto_LearningBasedWB_to_WhiteBalancer }

impl std::fmt::Debug for LearningBasedWB {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LearningBasedWB")
			.finish()
	}
}

/// Constant methods for [crate::xphoto::SimpleWB]
pub trait SimpleWBTraitConst: crate::xphoto::WhiteBalancerTraitConst {
	fn as_raw_SimpleWB(&self) -> *const c_void;

	/// Input image range minimum value
	/// ## See also
	/// setInputMin
	#[inline]
	fn get_input_min(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_getInputMin_const(self.as_raw_SimpleWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Input image range maximum value
	/// ## See also
	/// setInputMax
	#[inline]
	fn get_input_max(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_getInputMax_const(self.as_raw_SimpleWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Output image range minimum value
	/// ## See also
	/// setOutputMin
	#[inline]
	fn get_output_min(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_getOutputMin_const(self.as_raw_SimpleWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Output image range maximum value
	/// ## See also
	/// setOutputMax
	#[inline]
	fn get_output_max(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_getOutputMax_const(self.as_raw_SimpleWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Percent of top/bottom values to ignore
	/// ## See also
	/// setP
	#[inline]
	fn get_p(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_getP_const(self.as_raw_SimpleWB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::xphoto::SimpleWB]
pub trait SimpleWBTrait: crate::xphoto::SimpleWBTraitConst + crate::xphoto::WhiteBalancerTrait {
	fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void;

	/// Input image range minimum value
	/// ## See also
	/// setInputMin getInputMin
	#[inline]
	fn set_input_min(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_setInputMin_float(self.as_raw_mut_SimpleWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Input image range maximum value
	/// ## See also
	/// setInputMax getInputMax
	#[inline]
	fn set_input_max(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_setInputMax_float(self.as_raw_mut_SimpleWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Output image range minimum value
	/// ## See also
	/// setOutputMin getOutputMin
	#[inline]
	fn set_output_min(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_setOutputMin_float(self.as_raw_mut_SimpleWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Output image range maximum value
	/// ## See also
	/// setOutputMax getOutputMax
	#[inline]
	fn set_output_max(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_setOutputMax_float(self.as_raw_mut_SimpleWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Percent of top/bottom values to ignore
	/// ## See also
	/// setP getP
	#[inline]
	fn set_p(&mut self, val: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_SimpleWB_setP_float(self.as_raw_mut_SimpleWB(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// A simple white balance algorithm that works by independently stretching
/// each of the input image channels to the specified range. For increased robustness
/// it ignores the top and bottom ![inline formula](https://latex.codecogs.com/png.latex?p%5C%25) of pixel values.
pub struct SimpleWB {
	ptr: *mut c_void
}

opencv_type_boxed! { SimpleWB }

impl Drop for SimpleWB {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xphoto_SimpleWB_delete(self.as_raw_mut_SimpleWB()) };
	}
}

unsafe impl Send for SimpleWB {}

impl core::AlgorithmTraitConst for SimpleWB {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for SimpleWB {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerTraitConst for SimpleWB {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::WhiteBalancerTrait for SimpleWB {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::SimpleWBTraitConst for SimpleWB {
	#[inline] fn as_raw_SimpleWB(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::SimpleWBTrait for SimpleWB {
	#[inline] fn as_raw_mut_SimpleWB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SimpleWB {
}

boxed_cast_base! { SimpleWB, core::Algorithm, cv_xphoto_SimpleWB_to_Algorithm }

boxed_cast_base! { SimpleWB, crate::xphoto::WhiteBalancer, cv_xphoto_SimpleWB_to_WhiteBalancer }

impl std::fmt::Debug for SimpleWB {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SimpleWB")
			.finish()
	}
}

/// Constant methods for [crate::xphoto::TonemapDurand]
pub trait TonemapDurandTraitConst: crate::photo::TonemapTraitConst {
	fn as_raw_TonemapDurand(&self) -> *const c_void;

	#[inline]
	fn get_saturation(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_getSaturation_const(self.as_raw_TonemapDurand(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_contrast(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_getContrast_const(self.as_raw_TonemapDurand(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sigma_space(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_getSigmaSpace_const(self.as_raw_TonemapDurand(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_sigma_color(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_getSigmaColor_const(self.as_raw_TonemapDurand(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::xphoto::TonemapDurand]
pub trait TonemapDurandTrait: crate::photo::TonemapTrait + crate::xphoto::TonemapDurandTraitConst {
	fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void;

	#[inline]
	fn set_saturation(&mut self, saturation: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_setSaturation_float(self.as_raw_mut_TonemapDurand(), saturation, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_contrast(&mut self, contrast: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_setContrast_float(self.as_raw_mut_TonemapDurand(), contrast, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sigma_space(&mut self, sigma_space: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_setSigmaSpace_float(self.as_raw_mut_TonemapDurand(), sigma_space, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_sigma_color(&mut self, sigma_color: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_TonemapDurand_setSigmaColor_float(self.as_raw_mut_TonemapDurand(), sigma_color, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// This algorithm decomposes image into two layers: base layer and detail layer using bilateral filter
/// and compresses contrast of the base layer thus preserving all the details.
/// 
/// This implementation uses regular bilateral filter from OpenCV.
/// 
/// Saturation enhancement is possible as in cv::TonemapDrago.
/// 
/// For more information see [DD02](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_DD02) .
pub struct TonemapDurand {
	ptr: *mut c_void
}

opencv_type_boxed! { TonemapDurand }

impl Drop for TonemapDurand {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xphoto_TonemapDurand_delete(self.as_raw_mut_TonemapDurand()) };
	}
}

unsafe impl Send for TonemapDurand {}

impl core::AlgorithmTraitConst for TonemapDurand {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for TonemapDurand {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::photo::TonemapTraitConst for TonemapDurand {
	#[inline] fn as_raw_Tonemap(&self) -> *const c_void { self.as_raw() }
}

impl crate::photo::TonemapTrait for TonemapDurand {
	#[inline] fn as_raw_mut_Tonemap(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::TonemapDurandTraitConst for TonemapDurand {
	#[inline] fn as_raw_TonemapDurand(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::TonemapDurandTrait for TonemapDurand {
	#[inline] fn as_raw_mut_TonemapDurand(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl TonemapDurand {
}

boxed_cast_base! { TonemapDurand, core::Algorithm, cv_xphoto_TonemapDurand_to_Algorithm }

boxed_cast_base! { TonemapDurand, crate::photo::Tonemap, cv_xphoto_TonemapDurand_to_Tonemap }

impl std::fmt::Debug for TonemapDurand {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("TonemapDurand")
			.finish()
	}
}

/// Constant methods for [crate::xphoto::WhiteBalancer]
pub trait WhiteBalancerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_WhiteBalancer(&self) -> *const c_void;

}

/// Mutable methods for [crate::xphoto::WhiteBalancer]
pub trait WhiteBalancerTrait: core::AlgorithmTrait + crate::xphoto::WhiteBalancerTraitConst {
	fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void;

	/// Applies white balancing to the input image
	/// 
	/// ## Parameters
	/// * src: Input image
	/// * dst: White balancing result
	/// ## See also
	/// cvtColor, equalizeHist
	#[inline]
	fn balance_white(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_xphoto_WhiteBalancer_balanceWhite_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_WhiteBalancer(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// The base class for auto white balance algorithms.
pub struct WhiteBalancer {
	ptr: *mut c_void
}

opencv_type_boxed! { WhiteBalancer }

impl Drop for WhiteBalancer {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_xphoto_WhiteBalancer_delete(self.as_raw_mut_WhiteBalancer()) };
	}
}

unsafe impl Send for WhiteBalancer {}

impl core::AlgorithmTraitConst for WhiteBalancer {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for WhiteBalancer {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::xphoto::WhiteBalancerTraitConst for WhiteBalancer {
	#[inline] fn as_raw_WhiteBalancer(&self) -> *const c_void { self.as_raw() }
}

impl crate::xphoto::WhiteBalancerTrait for WhiteBalancer {
	#[inline] fn as_raw_mut_WhiteBalancer(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl WhiteBalancer {
}

boxed_cast_descendant! { WhiteBalancer, crate::xphoto::GrayworldWB, cv_xphoto_WhiteBalancer_to_GrayworldWB }

boxed_cast_descendant! { WhiteBalancer, crate::xphoto::LearningBasedWB, cv_xphoto_WhiteBalancer_to_LearningBasedWB }

boxed_cast_descendant! { WhiteBalancer, crate::xphoto::SimpleWB, cv_xphoto_WhiteBalancer_to_SimpleWB }

boxed_cast_base! { WhiteBalancer, core::Algorithm, cv_xphoto_WhiteBalancer_to_Algorithm }

impl std::fmt::Debug for WhiteBalancer {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("WhiteBalancer")
			.finish()
	}
}
