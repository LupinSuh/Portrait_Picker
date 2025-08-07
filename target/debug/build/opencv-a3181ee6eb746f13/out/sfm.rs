//! # Structure From Motion
//! 
//! The opencv_sfm module contains algorithms to perform 3d reconstruction
//! from 2d images.
//! 
//! The core of the module is based on a light version of
//! [Libmv](https://developer.blender.org/project/profile/59) originally
//! developed by Sameer Agarwal and Keir Mierle.
//! 
//! __Whats is libmv?__ 
//! 
//! libmv, also known as the Library for Multiview Reconstruction (or LMV),
//! is the computer vision backend for Blender's motion tracking abilities.
//! Unlike other vision libraries with general ambitions, libmv is focused
//! on algorithms for match moving, specifically targeting [Blender](https://developer.blender.org) as the
//! primary customer. Dense reconstruction, reconstruction from unorganized
//! photo collections, image recognition, and other tasks are not a focus
//! of libmv.
//! 
//! __Development__ 
//! 
//! libmv is officially under the Blender umbrella, and so is developed
//! on developer.blender.org. The [source repository](https://developer.blender.org/diffusion/LMV) can get checked out
//! independently from Blender.
//! 
//! This module has been originally developed as a project for Google Summer of Code 2012-2015.
//! 
//! 
//! Note:
//!   - Notice that it is compiled only when Eigen, GLog and GFlags are correctly installed.
//! 
//!    Check installation instructions in the following tutorial: [tutorial_sfm_installation]
//!    # Conditioning
//!    # Fundamental
//!    # Input/Output
//!    # Numeric
//!    # Projection
//!    # Robust Estimation
//!    # Triangulation
//! 
//!    # Reconstruction
//! 
//!     
//! Note:
//!    - Notice that it is compiled only when Ceres Solver is correctly installed.
//! 
//!        Check installation instructions in the following tutorial: [tutorial_sfm_installation]
//! 
//!    # Simple Pipeline
//! 
//!     
//! Note:
//!        - Notice that it is compiled only when Ceres Solver is correctly installed.
//! 
//!        Check installation instructions in the following tutorial: [tutorial_sfm_installation]
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::BaseSFMTraitConst, super::BaseSFMTrait, super::SFMLibmvEuclideanReconstructionTraitConst, super::SFMLibmvEuclideanReconstructionTrait };
}

pub const SFM_DISTORTION_MODEL_DIVISION: i32 = 1;
pub const SFM_DISTORTION_MODEL_POLYNOMIAL: i32 = 0;
pub const SFM_IO_BUNDLER: i32 = 0;
pub const SFM_IO_OPENMVG: i32 = 3;
pub const SFM_IO_OPENSFM: i32 = 2;
pub const SFM_IO_THEIASFM: i32 = 4;
pub const SFM_IO_VISUALSFM: i32 = 1;
pub const SFM_REFINE_FOCAL_LENGTH: i32 = 1;
pub const SFM_REFINE_PRINCIPAL_POINT: i32 = 2;
pub const SFM_REFINE_RADIAL_DISTORTION_K1: i32 = 4;
pub const SFM_REFINE_RADIAL_DISTORTION_K2: i32 = 16;
/// Constant methods for [crate::sfm::BaseSFM]
pub trait BaseSFMTraitConst {
	fn as_raw_BaseSFM(&self) -> *const c_void;

	#[inline]
	fn get_error(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getError_const(self.as_raw_BaseSFM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_intrinsics(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getIntrinsics_const(self.as_raw_BaseSFM(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::sfm::BaseSFM]
pub trait BaseSFMTrait: crate::sfm::BaseSFMTraitConst {
	fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void;

	#[inline]
	fn run(&mut self, points2d: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(points2d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const__InputArrayR(self.as_raw_mut_BaseSFM(), points2d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn run_1(&mut self, points2d: &impl core::ToInputArray, k: &mut impl core::ToInputOutputArray, rs: &mut impl core::ToOutputArray, ts: &mut impl core::ToOutputArray, points3d: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(points2d);
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_BaseSFM(), points2d.as_raw__InputArray(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn run_2(&mut self, images: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const_vectorLStringGR(self.as_raw_mut_BaseSFM(), images.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn run_3(&mut self, images: &core::Vector<String>, k: &mut impl core::ToInputOutputArray, rs: &mut impl core::ToOutputArray, ts: &mut impl core::ToOutputArray, points3d: &mut impl core::ToOutputArray) -> Result<()> {
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_BaseSFM(), images.as_raw_VectorOfString(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_points(&mut self, points3d: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getPoints_const__OutputArrayR(self.as_raw_mut_BaseSFM(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_cameras(&mut self, rs: &mut impl core::ToOutputArray, ts: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(rs);
		output_array_arg!(ts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_getCameras_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_BaseSFM(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_setReconstructionOptions_const_libmv_ReconstructionOptionsR(self.as_raw_mut_BaseSFM(), &libmv_reconstruction_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_BaseSFM_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(self.as_raw_mut_BaseSFM(), &libmv_camera_intrinsics_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// base class BaseSFM declares a common API that would be used in a typical scene reconstruction scenario
pub struct BaseSFM {
	ptr: *mut c_void
}

opencv_type_boxed! { BaseSFM }

impl Drop for BaseSFM {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_sfm_BaseSFM_delete(self.as_raw_mut_BaseSFM()) };
	}
}

unsafe impl Send for BaseSFM {}

impl crate::sfm::BaseSFMTraitConst for BaseSFM {
	#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.as_raw() }
}

impl crate::sfm::BaseSFMTrait for BaseSFM {
	#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BaseSFM {
}

boxed_cast_descendant! { BaseSFM, crate::sfm::SFMLibmvEuclideanReconstruction, cv_sfm_BaseSFM_to_SFMLibmvEuclideanReconstruction }

impl std::fmt::Debug for BaseSFM {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("BaseSFM")
			.finish()
	}
}

/// Constant methods for [crate::sfm::SFMLibmvEuclideanReconstruction]
pub trait SFMLibmvEuclideanReconstructionTraitConst: crate::sfm::BaseSFMTraitConst {
	fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void;

	/// Returns the computed reprojection error.
	#[inline]
	fn get_error(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getError_const(self.as_raw_SFMLibmvEuclideanReconstruction(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the refined camera calibration matrix.
	#[inline]
	fn get_intrinsics(&self) -> Result<core::Mat> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getIntrinsics_const(self.as_raw_SFMLibmvEuclideanReconstruction(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Mat::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::sfm::SFMLibmvEuclideanReconstruction]
pub trait SFMLibmvEuclideanReconstructionTrait: crate::sfm::BaseSFMTrait + crate::sfm::SFMLibmvEuclideanReconstructionTraitConst {
	fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void;

	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
	/// 
	/// 
	/// Note:
	///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
	#[inline]
	fn run(&mut self, points2d: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(points2d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), points2d.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * points2d: Input vector of vectors of 2d points (the inner vector is per image).
	/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
	/// * Rs: Output vector of 3x3 rotations of the camera.
	/// * Ts: Output vector of 3x1 translations of the camera.
	/// * points3d: Output array with estimated 3d points.
	/// 
	/// 
	/// Note:
	///   - Tracks must be as precise as possible. It does not handle outliers and is very sensible to them.
	#[inline]
	fn run_1(&mut self, points2d: &impl core::ToInputArray, k: &mut impl core::ToInputOutputArray, rs: &mut impl core::ToOutputArray, ts: &mut impl core::ToOutputArray, points3d: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(points2d);
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), points2d.as_raw__InputArray(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * images: a vector of string with the images paths.
	/// 
	/// 
	/// Note:
	///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
	///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
	#[inline]
	fn run_2(&mut self, images: &core::Vector<String>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Calls the pipeline in order to perform Eclidean reconstruction.
	/// ## Parameters
	/// * images: a vector of string with the images paths.
	/// * K: Input/Output camera matrix ![inline formula](https://latex.codecogs.com/png.latex?K%20%3D%20%5Cbegin%7Bbmatrix%7D%20f%5Fx%20%26%200%20%26%20c%5Fx%5C%5C%200%20%26%20f%5Fy%20%26%20c%5Fy%5C%5C%200%20%26%200%20%26%201%20%5Cend%7Bbmatrix%7D). Input parameters used as initial guess.
	/// * Rs: Output vector of 3x3 rotations of the camera.
	/// * Ts: Output vector of 3x1 translations of the camera.
	/// * points3d: Output array with estimated 3d points.
	/// 
	/// 
	/// Note:
	///   - The images must be ordered as they were an image sequence. Additionally, each frame should be as close as posible to the previous and posterior.
	///   - For now DAISY features are used in order to compute the 2d points tracks and it only works for 3-4 images.
	#[inline]
	fn run_3(&mut self, images: &core::Vector<String>, k: &mut impl core::ToInputOutputArray, rs: &mut impl core::ToOutputArray, ts: &mut impl core::ToOutputArray, points3d: &mut impl core::ToOutputArray) -> Result<()> {
		input_output_array_arg!(k);
		output_array_arg!(rs);
		output_array_arg!(ts);
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_run_const_vectorLStringGR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), images.as_raw_VectorOfString(), k.as_raw__InputOutputArray(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the estimated 3d points.
	/// ## Parameters
	/// * points3d: Output array with estimated 3d points.
	#[inline]
	fn get_points(&mut self, points3d: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(points3d);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getPoints_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), points3d.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the estimated camera extrinsic parameters.
	/// ## Parameters
	/// * Rs: Output vector of 3x3 rotations of the camera.
	/// * Ts: Output vector of 3x1 translations of the camera.
	#[inline]
	fn get_cameras(&mut self, rs: &mut impl core::ToOutputArray, ts: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(rs);
		output_array_arg!(ts);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_getCameras_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), rs.as_raw__OutputArray(), ts.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Setter method for reconstruction options.
	/// ## Parameters
	/// * libmv_reconstruction_options: struct with reconstruction options such as initial keyframes,
	///   automatic keyframe selection, parameters to refine and the verbosity level.
	#[inline]
	fn set_reconstruction_options(&mut self, libmv_reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setReconstructionOptions_const_libmv_ReconstructionOptionsR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), &libmv_reconstruction_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Setter method for camera intrinsic options.
	/// ## Parameters
	/// * libmv_camera_intrinsics_options: struct with camera intrinsic options such as camera model and
	///   the internal camera parameters.
	#[inline]
	fn set_camera_intrinsic_options(&mut self, libmv_camera_intrinsics_options: crate::sfm::libmv_CameraIntrinsicsOptions) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_setCameraIntrinsicOptions_const_libmv_CameraIntrinsicsOptionsR(self.as_raw_mut_SFMLibmvEuclideanReconstruction(), &libmv_camera_intrinsics_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// SFMLibmvEuclideanReconstruction class provides an interface with the Libmv Structure From Motion pipeline.
pub struct SFMLibmvEuclideanReconstruction {
	ptr: *mut c_void
}

opencv_type_boxed! { SFMLibmvEuclideanReconstruction }

impl Drop for SFMLibmvEuclideanReconstruction {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_delete(self.as_raw_mut_SFMLibmvEuclideanReconstruction()) };
	}
}

unsafe impl Send for SFMLibmvEuclideanReconstruction {}

impl crate::sfm::BaseSFMTraitConst for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_BaseSFM(&self) -> *const c_void { self.as_raw() }
}

impl crate::sfm::BaseSFMTrait for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_mut_BaseSFM(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::sfm::SFMLibmvEuclideanReconstructionTraitConst for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_SFMLibmvEuclideanReconstruction(&self) -> *const c_void { self.as_raw() }
}

impl crate::sfm::SFMLibmvEuclideanReconstructionTrait for SFMLibmvEuclideanReconstruction {
	#[inline] fn as_raw_mut_SFMLibmvEuclideanReconstruction(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl SFMLibmvEuclideanReconstruction {
	/// Creates an instance of the SFMLibmvEuclideanReconstruction class. Initializes Libmv.
	/// 
	/// ## C++ default parameters
	/// * camera_instrinsic_options: libmv_CameraIntrinsicsOptions()
	/// * reconstruction_options: libmv_ReconstructionOptions()
	#[inline]
	pub fn create(camera_instrinsic_options: crate::sfm::libmv_CameraIntrinsicsOptions, reconstruction_options: crate::sfm::libmv_ReconstructionOptions) -> Result<core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_create_const_libmv_CameraIntrinsicsOptionsR_const_libmv_ReconstructionOptionsR(&camera_instrinsic_options, &reconstruction_options, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::sfm::SFMLibmvEuclideanReconstruction>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Creates an instance of the SFMLibmvEuclideanReconstruction class. Initializes Libmv.
	/// 
	/// ## Note
	/// This alternative version of [SFMLibmvEuclideanReconstruction::create] function uses the following default values for its arguments:
	/// * camera_instrinsic_options: libmv_CameraIntrinsicsOptions()
	/// * reconstruction_options: libmv_ReconstructionOptions()
	#[inline]
	pub fn create_def() -> Result<core::Ptr<crate::sfm::SFMLibmvEuclideanReconstruction>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_SFMLibmvEuclideanReconstruction_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::sfm::SFMLibmvEuclideanReconstruction>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { SFMLibmvEuclideanReconstruction, crate::sfm::BaseSFM, cv_sfm_SFMLibmvEuclideanReconstruction_to_BaseSFM }

impl std::fmt::Debug for SFMLibmvEuclideanReconstruction {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("SFMLibmvEuclideanReconstruction")
			.finish()
	}
}

/// Data structure describing the camera model and its parameters.
/// ## Parameters
/// * _distortion_model: Type of camera model.
/// * _focal_length_x: focal length of the camera (in pixels).
/// * _focal_length_y: focal length of the camera (in pixels).
/// * _principal_point_x: principal point of the camera in the x direction (in pixels).
/// * _principal_point_y: principal point of the camera in the y direction (in pixels).
/// * _polynomial_k1: radial distortion parameter.
/// * _polynomial_k2: radial distortion parameter.
/// * _polynomial_k3: radial distortion parameter.
/// * _polynomial_p1: radial distortion parameter.
/// * _polynomial_p2: radial distortion parameter.
/// 
/// Is assumed that modern cameras have their principal point in the image center.
/// 
/// In case that the camera model was SFM_DISTORTION_MODEL_DIVISION, it's only needed to provide
/// _polynomial_k1 and _polynomial_k2 which will be assigned as division distortion parameters.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct libmv_CameraIntrinsicsOptions {
	pub distortion_model: i32,
	pub image_width: i32,
	pub image_height: i32,
	pub focal_length_x: f64,
	pub focal_length_y: f64,
	pub principal_point_x: f64,
	pub principal_point_y: f64,
	pub polynomial_k1: f64,
	pub polynomial_k2: f64,
	pub polynomial_k3: f64,
	pub polynomial_p1: f64,
	pub polynomial_p2: f64,
	pub division_k1: f64,
	pub division_k2: f64,
}

opencv_type_simple! { crate::sfm::libmv_CameraIntrinsicsOptions }

impl libmv_CameraIntrinsicsOptions {
	/// ## C++ default parameters
	/// * _distortion_model: 0
	/// * _focal_length_x: 0
	/// * _focal_length_y: 0
	/// * _principal_point_x: 0
	/// * _principal_point_y: 0
	/// * _polynomial_k1: 0
	/// * _polynomial_k2: 0
	/// * _polynomial_k3: 0
	/// * _polynomial_p1: 0
	/// * _polynomial_p2: 0
	#[inline]
	pub fn new(_distortion_model: i32, _focal_length_x: f64, _focal_length_y: f64, _principal_point_x: f64, _principal_point_y: f64, _polynomial_k1: f64, _polynomial_k2: f64, _polynomial_k3: f64, _polynomial_p1: f64, _polynomial_p2: f64) -> Result<crate::sfm::libmv_CameraIntrinsicsOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions_const_int_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double_const_double(_distortion_model, _focal_length_x, _focal_length_y, _principal_point_x, _principal_point_y, _polynomial_k1, _polynomial_k2, _polynomial_k3, _polynomial_p1, _polynomial_p2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _distortion_model: 0
	/// * _focal_length_x: 0
	/// * _focal_length_y: 0
	/// * _principal_point_x: 0
	/// * _principal_point_y: 0
	/// * _polynomial_k1: 0
	/// * _polynomial_k2: 0
	/// * _polynomial_k3: 0
	/// * _polynomial_p1: 0
	/// * _polynomial_p2: 0
	#[inline]
	pub fn new_def() -> Result<crate::sfm::libmv_CameraIntrinsicsOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_CameraIntrinsicsOptions_libmv_CameraIntrinsicsOptions(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Data structure describing the reconstruction options.
/// ## Parameters
/// * _keyframe1: first keyframe used in order to initialize the reconstruction.
/// * _keyframe2: second keyframe used in order to initialize the reconstruction.
/// * _refine_intrinsics: camera parameter or combination of parameters to refine.
/// * _select_keyframes: allows to select automatically the initial keyframes. If 1 then autoselection is enabled. If 0 then is disabled.
/// * _verbosity_level: verbosity logs level for Glog. If -1 then logs are disabled, otherwise the log level will be the input integer.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct libmv_ReconstructionOptions {
	pub keyframe1: i32,
	pub keyframe2: i32,
	pub refine_intrinsics: i32,
	pub select_keyframes: i32,
	pub verbosity_level: i32,
}

opencv_type_simple! { crate::sfm::libmv_ReconstructionOptions }

impl libmv_ReconstructionOptions {
	/// ## C++ default parameters
	/// * _keyframe1: 1
	/// * _keyframe2: 2
	/// * _refine_intrinsics: 1
	/// * _select_keyframes: 1
	/// * _verbosity_level: -1
	#[inline]
	pub fn new(_keyframe1: i32, _keyframe2: i32, _refine_intrinsics: i32, _select_keyframes: i32, _verbosity_level: i32) -> Result<crate::sfm::libmv_ReconstructionOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions_const_int_const_int_const_int_const_int_const_int(_keyframe1, _keyframe2, _refine_intrinsics, _select_keyframes, _verbosity_level, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * _keyframe1: 1
	/// * _keyframe2: 2
	/// * _refine_intrinsics: 1
	/// * _select_keyframes: 1
	/// * _verbosity_level: -1
	#[inline]
	pub fn new_def() -> Result<crate::sfm::libmv_ReconstructionOptions> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_sfm_libmv_ReconstructionOptions_libmv_ReconstructionOptions(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
