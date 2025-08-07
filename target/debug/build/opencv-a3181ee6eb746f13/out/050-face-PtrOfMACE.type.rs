pub type PtrOfMACE = core::Ptr<crate::face::MACE>;

ptr_extern! { crate::face::MACE,
	cv_PtrLcv_face_MACEG_delete, cv_PtrLcv_face_MACEG_getInnerPtr_const, cv_PtrLcv_face_MACEG_getInnerPtrMut
}

impl core::Ptr<crate::face::MACE> {
	#[inline] pub fn as_raw_PtrOfMACE(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfMACE(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::face::MACETraitConst for core::Ptr<crate::face::MACE> {
	#[inline] fn as_raw_MACE(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::face::MACETrait for core::Ptr<crate::face::MACE> {
	#[inline] fn as_raw_mut_MACE(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::face::MACE> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::face::MACE> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfMACE, core::Ptr<core::Algorithm>, cv_PtrLcv_face_MACEG_to_PtrOfAlgorithm }

impl std::fmt::Debug for core::Ptr<crate::face::MACE> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfMACE")
			.finish()
	}
}

