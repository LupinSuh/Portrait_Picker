//! # Aruco markers, module functionality was moved to objdetect module
//! ArUco Marker Detection, module functionality was moved to objdetect module
//! ## See also
//! ArucoDetector, CharucoDetector, Board, GridBoard, CharucoBoard
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::EstimateParametersTraitConst, super::EstimateParametersTrait };
}

/// The marker coordinate system is centered on the middle of the marker.
/// 
/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
pub const ARUCO_CCW_CENTER: i32 = 0;
/// The marker coordinate system is centered on the top-left corner of the marker.
/// 
/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
/// (0, 0, 0), (markerLength, 0, 0),
/// (markerLength, markerLength, 0), (0, markerLength, 0).
/// 
/// These pattern dots are convenient to use with a chessboard/ChArUco board.
pub const ARUCO_CW_TOP_LEFT_CORNER: i32 = 1;
/// rvec/tvec define the right handed coordinate system of the marker.
/// 
/// PatternPositionType defines center this system and axes direction.
/// Axis X (red color) - first coordinate, axis Y (green color) - second coordinate,
/// axis Z (blue color) - third coordinate.
/// 
/// 
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
/// ## See also
/// estimatePoseSingleMarkers()
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PatternPositionType {
	/// The marker coordinate system is centered on the middle of the marker.
	/// 
	/// The coordinates of the four corners (CCW order) of the marker in its own coordinate system are:
	/// (-markerLength/2, markerLength/2, 0), (markerLength/2, markerLength/2, 0),
	/// (markerLength/2, -markerLength/2, 0), (-markerLength/2, -markerLength/2, 0).
	ARUCO_CCW_CENTER = 0,
	/// The marker coordinate system is centered on the top-left corner of the marker.
	/// 
	/// The coordinates of the four corners (CW order) of the marker in its own coordinate system are:
	/// (0, 0, 0), (markerLength, 0, 0),
	/// (markerLength, markerLength, 0), (0, markerLength, 0).
	/// 
	/// These pattern dots are convenient to use with a chessboard/ChArUco board.
	ARUCO_CW_TOP_LEFT_CORNER = 1,
}

opencv_type_enum! { crate::aruco::PatternPositionType }

/// Constant methods for [crate::aruco::EstimateParameters]
pub trait EstimateParametersTraitConst {
	fn as_raw_EstimateParameters(&self) -> *const c_void;

	#[inline]
	fn pattern(&self) -> crate::aruco::PatternPositionType {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_propPattern_const(self.as_raw_EstimateParameters(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn use_extrinsic_guess(&self) -> bool {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const(self.as_raw_EstimateParameters()) };
		ret
	}
	
	#[inline]
	fn solve_pnp_method(&self) -> i32 {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const(self.as_raw_EstimateParameters()) };
		ret
	}
	
}

/// Mutable methods for [crate::aruco::EstimateParameters]
pub trait EstimateParametersTrait: crate::aruco::EstimateParametersTraitConst {
	fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void;

	#[inline]
	fn set_pattern(&mut self, val: crate::aruco::PatternPositionType) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propPattern_const_PatternPositionType(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_use_extrinsic_guess(&mut self, val: bool) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propUseExtrinsicGuess_const_bool(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
	#[inline]
	fn set_solve_pnp_method(&mut self, val: i32) {
		let ret = unsafe { sys::cv_aruco_EstimateParameters_propSolvePnPMethod_const_int(self.as_raw_mut_EstimateParameters(), val) };
		ret
	}
	
}

/// Pose estimation parameters
/// 
/// ## Parameters
/// * pattern: Defines center this system and axes direction (default PatternPositionType::ARUCO_CCW_CENTER).
/// * useExtrinsicGuess: Parameter used for SOLVEPNP_ITERATIVE. If true (1), the function uses the provided
/// rvec and tvec values as initial approximations of the rotation and translation vectors, respectively, and further
/// optimizes them (default false).
/// * solvePnPMethod: Method for solving a PnP problem: see [calib3d_solvePnP_flags] (default SOLVEPNP_ITERATIVE).
/// 
/// 
/// **Deprecated**: Use Board::matchImagePoints and cv::solvePnP
/// ## See also
/// PatternPositionType, solvePnP()
#[deprecated = "Use Board::matchImagePoints and cv::solvePnP"]
pub struct EstimateParameters {
	ptr: *mut c_void
}

opencv_type_boxed! { EstimateParameters }

impl Drop for EstimateParameters {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_aruco_EstimateParameters_delete(self.as_raw_mut_EstimateParameters()) };
	}
}

unsafe impl Send for EstimateParameters {}

impl crate::aruco::EstimateParametersTraitConst for EstimateParameters {
	#[inline] fn as_raw_EstimateParameters(&self) -> *const c_void { self.as_raw() }
}

impl crate::aruco::EstimateParametersTrait for EstimateParameters {
	#[inline] fn as_raw_mut_EstimateParameters(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl EstimateParameters {
	#[inline]
	pub fn default() -> Result<crate::aruco::EstimateParameters> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_aruco_EstimateParameters_EstimateParameters(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::aruco::EstimateParameters::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for EstimateParameters {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_aruco_EstimateParameters_implicitClone_const(self.as_raw_EstimateParameters())) }
	}
}

impl std::fmt::Debug for EstimateParameters {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("EstimateParameters")
			.field("pattern", &crate::aruco::EstimateParametersTraitConst::pattern(self))
			.field("use_extrinsic_guess", &crate::aruco::EstimateParametersTraitConst::use_extrinsic_guess(self))
			.field("solve_pnp_method", &crate::aruco::EstimateParametersTraitConst::solve_pnp_method(self))
			.finish()
	}
}
