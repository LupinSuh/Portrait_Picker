pub type PtrOfStarDetector = core::Ptr<crate::xfeatures2d::StarDetector>;

ptr_extern! { crate::xfeatures2d::StarDetector,
	cv_PtrLcv_xfeatures2d_StarDetectorG_delete, cv_PtrLcv_xfeatures2d_StarDetectorG_getInnerPtr_const, cv_PtrLcv_xfeatures2d_StarDetectorG_getInnerPtrMut
}

impl core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] pub fn as_raw_PtrOfStarDetector(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStarDetector(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::xfeatures2d::StarDetectorTraitConst for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] fn as_raw_StarDetector(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::xfeatures2d::StarDetectorTrait for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] fn as_raw_mut_StarDetector(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStarDetector, core::Ptr<core::Algorithm>, cv_PtrLcv_xfeatures2d_StarDetectorG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStarDetector, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_xfeatures2d_StarDetectorG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::xfeatures2d::StarDetector> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStarDetector")
			.finish()
	}
}

