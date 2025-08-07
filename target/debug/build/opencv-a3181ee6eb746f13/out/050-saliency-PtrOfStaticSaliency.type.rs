pub type PtrOfStaticSaliency = core::Ptr<crate::saliency::StaticSaliency>;

ptr_extern! { crate::saliency::StaticSaliency,
	cv_PtrLcv_saliency_StaticSaliencyG_delete, cv_PtrLcv_saliency_StaticSaliencyG_getInnerPtr_const, cv_PtrLcv_saliency_StaticSaliencyG_getInnerPtrMut
}

impl core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] pub fn as_raw_PtrOfStaticSaliency(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfStaticSaliency(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::StaticSaliencyTrait for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStaticSaliency, core::Ptr<core::Algorithm>, cv_PtrLcv_saliency_StaticSaliencyG_to_PtrOfAlgorithm }

impl crate::saliency::SaliencyTraitConst for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::saliency::SaliencyTrait for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfStaticSaliency, core::Ptr<crate::saliency::Saliency>, cv_PtrLcv_saliency_StaticSaliencyG_to_PtrOfSaliency }

impl std::fmt::Debug for core::Ptr<crate::saliency::StaticSaliency> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfStaticSaliency")
			.finish()
	}
}

