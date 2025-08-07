pub type PtrOfAKAZE = core::Ptr<crate::features2d::AKAZE>;

ptr_extern! { crate::features2d::AKAZE,
	cv_PtrLcv_AKAZEG_delete, cv_PtrLcv_AKAZEG_getInnerPtr_const, cv_PtrLcv_AKAZEG_getInnerPtrMut
}

impl core::Ptr<crate::features2d::AKAZE> {
	#[inline] pub fn as_raw_PtrOfAKAZE(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfAKAZE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::AKAZETraitConst for core::Ptr<crate::features2d::AKAZE> {
	#[inline] fn as_raw_AKAZE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::AKAZETrait for core::Ptr<crate::features2d::AKAZE> {
	#[inline] fn as_raw_mut_AKAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::AKAZE> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features2d::AKAZE> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAKAZE, core::Ptr<core::Algorithm>, cv_PtrLcv_AKAZEG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::AKAZE> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::AKAZE> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfAKAZE, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_AKAZEG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features2d::AKAZE> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfAKAZE")
			.finish()
	}
}

