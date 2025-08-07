pub type PtrOfBackgroundSubtractorKNN = core::Ptr<crate::video::BackgroundSubtractorKNN>;

ptr_extern! { crate::video::BackgroundSubtractorKNN,
	cv_PtrLcv_BackgroundSubtractorKNNG_delete, cv_PtrLcv_BackgroundSubtractorKNNG_getInnerPtr_const, cv_PtrLcv_BackgroundSubtractorKNNG_getInnerPtrMut
}

impl core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] pub fn as_raw_PtrOfBackgroundSubtractorKNN(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfBackgroundSubtractorKNN(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::BackgroundSubtractorKNNTraitConst for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] fn as_raw_BackgroundSubtractorKNN(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorKNNTrait for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] fn as_raw_mut_BackgroundSubtractorKNN(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfBackgroundSubtractorKNN, core::Ptr<core::Algorithm>, cv_PtrLcv_BackgroundSubtractorKNNG_to_PtrOfAlgorithm }

impl crate::video::BackgroundSubtractorTraitConst for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] fn as_raw_BackgroundSubtractor(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::BackgroundSubtractorTrait for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline] fn as_raw_mut_BackgroundSubtractor(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfBackgroundSubtractorKNN, core::Ptr<crate::video::BackgroundSubtractor>, cv_PtrLcv_BackgroundSubtractorKNNG_to_PtrOfBackgroundSubtractor }

impl std::fmt::Debug for core::Ptr<crate::video::BackgroundSubtractorKNN> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfBackgroundSubtractorKNN")
			.finish()
	}
}

