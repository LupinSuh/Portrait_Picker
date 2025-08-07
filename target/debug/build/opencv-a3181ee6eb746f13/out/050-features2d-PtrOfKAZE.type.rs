pub type PtrOfKAZE = core::Ptr<crate::features2d::KAZE>;

ptr_extern! { crate::features2d::KAZE,
	cv_PtrLcv_KAZEG_delete, cv_PtrLcv_KAZEG_getInnerPtr_const, cv_PtrLcv_KAZEG_getInnerPtrMut
}

impl core::Ptr<crate::features2d::KAZE> {
	#[inline] pub fn as_raw_PtrOfKAZE(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfKAZE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::features2d::KAZETraitConst for core::Ptr<crate::features2d::KAZE> {
	#[inline] fn as_raw_KAZE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::KAZETrait for core::Ptr<crate::features2d::KAZE> {
	#[inline] fn as_raw_mut_KAZE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::features2d::KAZE> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::features2d::KAZE> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfKAZE, core::Ptr<core::Algorithm>, cv_PtrLcv_KAZEG_to_PtrOfAlgorithm }

impl crate::features2d::Feature2DTraitConst for core::Ptr<crate::features2d::KAZE> {
	#[inline] fn as_raw_Feature2D(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::features2d::Feature2DTrait for core::Ptr<crate::features2d::KAZE> {
	#[inline] fn as_raw_mut_Feature2D(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfKAZE, core::Ptr<crate::features2d::Feature2D>, cv_PtrLcv_KAZEG_to_PtrOfFeature2D }

impl std::fmt::Debug for core::Ptr<crate::features2d::KAZE> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfKAZE")
			.finish()
	}
}

