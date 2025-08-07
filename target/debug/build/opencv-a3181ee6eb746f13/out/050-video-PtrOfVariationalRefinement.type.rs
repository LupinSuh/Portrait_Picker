pub type PtrOfVariationalRefinement = core::Ptr<crate::video::VariationalRefinement>;

ptr_extern! { crate::video::VariationalRefinement,
	cv_PtrLcv_VariationalRefinementG_delete, cv_PtrLcv_VariationalRefinementG_getInnerPtr_const, cv_PtrLcv_VariationalRefinementG_getInnerPtrMut
}

impl core::Ptr<crate::video::VariationalRefinement> {
	#[inline] pub fn as_raw_PtrOfVariationalRefinement(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfVariationalRefinement(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::video::VariationalRefinementTraitConst for core::Ptr<crate::video::VariationalRefinement> {
	#[inline] fn as_raw_VariationalRefinement(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::VariationalRefinementTrait for core::Ptr<crate::video::VariationalRefinement> {
	#[inline] fn as_raw_mut_VariationalRefinement(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::video::VariationalRefinement> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::video::VariationalRefinement> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfVariationalRefinement, core::Ptr<core::Algorithm>, cv_PtrLcv_VariationalRefinementG_to_PtrOfAlgorithm }

impl crate::video::DenseOpticalFlowTraitConst for core::Ptr<crate::video::VariationalRefinement> {
	#[inline] fn as_raw_DenseOpticalFlow(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::video::DenseOpticalFlowTrait for core::Ptr<crate::video::VariationalRefinement> {
	#[inline] fn as_raw_mut_DenseOpticalFlow(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfVariationalRefinement, core::Ptr<crate::video::DenseOpticalFlow>, cv_PtrLcv_VariationalRefinementG_to_PtrOfDenseOpticalFlow }

impl std::fmt::Debug for core::Ptr<crate::video::VariationalRefinement> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfVariationalRefinement")
			.finish()
	}
}

