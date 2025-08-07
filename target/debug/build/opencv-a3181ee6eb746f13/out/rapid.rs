//! # silhouette based 3D object tracking
//! 
//! implements "RAPID-a video rate object tracker" [harris1990rapid](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_harris1990rapid) with the dynamic control point extraction of [drummond2002real](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_drummond2002real)
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::Rapid_TrackerTraitConst, super::Rapid_TrackerTrait, super::Rapid_RapidTraitConst, super::Rapid_RapidTrait, super::Rapid_OLSTrackerTraitConst, super::Rapid_OLSTrackerTrait, super::Rapid_GOSTrackerTraitConst, super::Rapid_GOSTrackerTrait };
}

/// Constant methods for [crate::rapid::Rapid_GOSTracker]
pub trait Rapid_GOSTrackerTraitConst: crate::rapid::Rapid_TrackerTraitConst {
	fn as_raw_Rapid_GOSTracker(&self) -> *const c_void;

}

/// Mutable methods for [crate::rapid::Rapid_GOSTracker]
pub trait Rapid_GOSTrackerTrait: crate::rapid::Rapid_GOSTrackerTraitConst + crate::rapid::Rapid_TrackerTrait {
	fn as_raw_mut_Rapid_GOSTracker(&mut self) -> *mut c_void;

}

/// implements "Global optimal searching for textureless 3D object tracking" [wang2015global](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_wang2015global)
pub struct Rapid_GOSTracker {
	ptr: *mut c_void
}

opencv_type_boxed! { Rapid_GOSTracker }

impl Drop for Rapid_GOSTracker {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rapid_GOSTracker_delete(self.as_raw_mut_Rapid_GOSTracker()) };
	}
}

unsafe impl Send for Rapid_GOSTracker {}

impl core::AlgorithmTraitConst for Rapid_GOSTracker {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Rapid_GOSTracker {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_TrackerTraitConst for Rapid_GOSTracker {
	#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_TrackerTrait for Rapid_GOSTracker {
	#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_GOSTrackerTraitConst for Rapid_GOSTracker {
	#[inline] fn as_raw_Rapid_GOSTracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_GOSTrackerTrait for Rapid_GOSTracker {
	#[inline] fn as_raw_mut_Rapid_GOSTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Rapid_GOSTracker {
	/// ## C++ default parameters
	/// * hist_bins: 4
	/// * sobel_thesh: 10
	#[inline]
	pub fn create(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray, hist_bins: i32, sobel_thesh: u8) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), hist_bins, sobel_thesh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [Rapid_GOSTracker::create] function uses the following default values for its arguments:
	/// * hist_bins: 4
	/// * sobel_thesh: 10
	#[inline]
	pub fn create_def(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_GOSTracker_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { Rapid_GOSTracker, core::Algorithm, cv_rapid_GOSTracker_to_Algorithm }

boxed_cast_base! { Rapid_GOSTracker, crate::rapid::Rapid_Tracker, cv_rapid_GOSTracker_to_Rapid_Tracker }

impl std::fmt::Debug for Rapid_GOSTracker {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Rapid_GOSTracker")
			.finish()
	}
}

/// Constant methods for [crate::rapid::Rapid_OLSTracker]
pub trait Rapid_OLSTrackerTraitConst: crate::rapid::Rapid_TrackerTraitConst {
	fn as_raw_Rapid_OLSTracker(&self) -> *const c_void;

}

/// Mutable methods for [crate::rapid::Rapid_OLSTracker]
pub trait Rapid_OLSTrackerTrait: crate::rapid::Rapid_OLSTrackerTraitConst + crate::rapid::Rapid_TrackerTrait {
	fn as_raw_mut_Rapid_OLSTracker(&mut self) -> *mut c_void;

}

/// implements "Optimal local searching for fast and robust textureless 3D object tracking in highly
/// cluttered backgrounds" [seo2013optimal](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_seo2013optimal)
pub struct Rapid_OLSTracker {
	ptr: *mut c_void
}

opencv_type_boxed! { Rapid_OLSTracker }

impl Drop for Rapid_OLSTracker {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rapid_OLSTracker_delete(self.as_raw_mut_Rapid_OLSTracker()) };
	}
}

unsafe impl Send for Rapid_OLSTracker {}

impl core::AlgorithmTraitConst for Rapid_OLSTracker {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Rapid_OLSTracker {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_TrackerTraitConst for Rapid_OLSTracker {
	#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_TrackerTrait for Rapid_OLSTracker {
	#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_OLSTrackerTraitConst for Rapid_OLSTracker {
	#[inline] fn as_raw_Rapid_OLSTracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_OLSTrackerTrait for Rapid_OLSTracker {
	#[inline] fn as_raw_mut_Rapid_OLSTracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Rapid_OLSTracker {
	/// ## C++ default parameters
	/// * hist_bins: 8
	/// * sobel_thesh: 10
	#[inline]
	pub fn create(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray, hist_bins: i32, sobel_thesh: u8) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR_int_unsigned_char(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), hist_bins, sobel_thesh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [Rapid_OLSTracker::create] function uses the following default values for its arguments:
	/// * hist_bins: 8
	/// * sobel_thesh: 10
	#[inline]
	pub fn create_def(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray) -> Result<core::Ptr<crate::rapid::Rapid_OLSTracker>> {
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_OLSTracker_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rapid::Rapid_OLSTracker>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { Rapid_OLSTracker, core::Algorithm, cv_rapid_OLSTracker_to_Algorithm }

boxed_cast_base! { Rapid_OLSTracker, crate::rapid::Rapid_Tracker, cv_rapid_OLSTracker_to_Rapid_Tracker }

impl std::fmt::Debug for Rapid_OLSTracker {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Rapid_OLSTracker")
			.finish()
	}
}

/// Constant methods for [crate::rapid::Rapid_Rapid]
pub trait Rapid_RapidTraitConst: crate::rapid::Rapid_TrackerTraitConst {
	fn as_raw_Rapid_Rapid(&self) -> *const c_void;

}

/// Mutable methods for [crate::rapid::Rapid_Rapid]
pub trait Rapid_RapidTrait: crate::rapid::Rapid_RapidTraitConst + crate::rapid::Rapid_TrackerTrait {
	fn as_raw_mut_Rapid_Rapid(&mut self) -> *mut c_void;

}

/// wrapper around [rapid] function for uniform access
pub struct Rapid_Rapid {
	ptr: *mut c_void
}

opencv_type_boxed! { Rapid_Rapid }

impl Drop for Rapid_Rapid {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rapid_Rapid_delete(self.as_raw_mut_Rapid_Rapid()) };
	}
}

unsafe impl Send for Rapid_Rapid {}

impl core::AlgorithmTraitConst for Rapid_Rapid {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Rapid_Rapid {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_TrackerTraitConst for Rapid_Rapid {
	#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_TrackerTrait for Rapid_Rapid {
	#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_RapidTraitConst for Rapid_Rapid {
	#[inline] fn as_raw_Rapid_Rapid(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_RapidTrait for Rapid_Rapid {
	#[inline] fn as_raw_mut_Rapid_Rapid(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Rapid_Rapid {
	#[inline]
	pub fn create(pts3d: &impl core::ToInputArray, tris: &impl core::ToInputArray) -> Result<core::Ptr<crate::rapid::Rapid_Rapid>> {
		input_array_arg!(pts3d);
		input_array_arg!(tris);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_Rapid_create_const__InputArrayR_const__InputArrayR(pts3d.as_raw__InputArray(), tris.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::rapid::Rapid_Rapid>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { Rapid_Rapid, core::Algorithm, cv_rapid_Rapid_to_Algorithm }

boxed_cast_base! { Rapid_Rapid, crate::rapid::Rapid_Tracker, cv_rapid_Rapid_to_Rapid_Tracker }

impl std::fmt::Debug for Rapid_Rapid {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Rapid_Rapid")
			.finish()
	}
}

/// Constant methods for [crate::rapid::Rapid_Tracker]
pub trait Rapid_TrackerTraitConst: core::AlgorithmTraitConst {
	fn as_raw_Rapid_Tracker(&self) -> *const c_void;

}

/// Mutable methods for [crate::rapid::Rapid_Tracker]
pub trait Rapid_TrackerTrait: core::AlgorithmTrait + crate::rapid::Rapid_TrackerTraitConst {
	fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * termcrit: TermCriteria(TermCriteria::MAX_ITER|TermCriteria::EPS,5,1.5)
	#[inline]
	fn compute(&mut self, img: &impl core::ToInputArray, num: i32, len: i32, k: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray, termcrit: core::TermCriteria) -> Result<f32> {
		input_array_arg!(img);
		input_array_arg!(k);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_const_TermCriteriaR(self.as_raw_mut_Rapid_Tracker(), img.as_raw__InputArray(), num, len, k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), &termcrit, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [Rapid_TrackerTrait::compute] function uses the following default values for its arguments:
	/// * termcrit: TermCriteria(TermCriteria::MAX_ITER|TermCriteria::EPS,5,1.5)
	#[inline]
	fn compute_def(&mut self, img: &impl core::ToInputArray, num: i32, len: i32, k: &impl core::ToInputArray, rvec: &mut impl core::ToInputOutputArray, tvec: &mut impl core::ToInputOutputArray) -> Result<f32> {
		input_array_arg!(img);
		input_array_arg!(k);
		input_output_array_arg!(rvec);
		input_output_array_arg!(tvec);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_Tracker_compute_const__InputArrayR_int_int_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR(self.as_raw_mut_Rapid_Tracker(), img.as_raw__InputArray(), num, len, k.as_raw__InputArray(), rvec.as_raw__InputOutputArray(), tvec.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn clear_state(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_rapid_Tracker_clearState(self.as_raw_mut_Rapid_Tracker(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Abstract base class for stateful silhouette trackers
pub struct Rapid_Tracker {
	ptr: *mut c_void
}

opencv_type_boxed! { Rapid_Tracker }

impl Drop for Rapid_Tracker {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_rapid_Tracker_delete(self.as_raw_mut_Rapid_Tracker()) };
	}
}

unsafe impl Send for Rapid_Tracker {}

impl core::AlgorithmTraitConst for Rapid_Tracker {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for Rapid_Tracker {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::rapid::Rapid_TrackerTraitConst for Rapid_Tracker {
	#[inline] fn as_raw_Rapid_Tracker(&self) -> *const c_void { self.as_raw() }
}

impl crate::rapid::Rapid_TrackerTrait for Rapid_Tracker {
	#[inline] fn as_raw_mut_Rapid_Tracker(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Rapid_Tracker {
}

boxed_cast_descendant! { Rapid_Tracker, crate::rapid::Rapid_GOSTracker, cv_rapid_Tracker_to_Rapid_GOSTracker }

boxed_cast_descendant! { Rapid_Tracker, crate::rapid::Rapid_OLSTracker, cv_rapid_Tracker_to_Rapid_OLSTracker }

boxed_cast_descendant! { Rapid_Tracker, crate::rapid::Rapid_Rapid, cv_rapid_Tracker_to_Rapid_Rapid }

boxed_cast_base! { Rapid_Tracker, core::Algorithm, cv_rapid_Tracker_to_Algorithm }

impl std::fmt::Debug for Rapid_Tracker {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Rapid_Tracker")
			.finish()
	}
}
