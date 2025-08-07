pub type PtrOfDetail_BundleAdjusterAffinePartial = core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial>;

ptr_extern! { crate::stitching::Detail_BundleAdjusterAffinePartial,
	cv_PtrLcv_detail_BundleAdjusterAffinePartialG_delete, cv_PtrLcv_detail_BundleAdjusterAffinePartialG_getInnerPtr_const, cv_PtrLcv_detail_BundleAdjusterAffinePartialG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffinePartial, cv_PtrLcv_detail_BundleAdjusterAffinePartialG_new_const_BundleAdjusterAffinePartial }
impl core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffinePartial(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffinePartial(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterAffinePartialTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] fn as_raw_Detail_BundleAdjusterAffinePartial(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterAffinePartialTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffinePartial(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrLcv_detail_BundleAdjusterAffinePartialG_to_PtrOfDetail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffinePartial, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_BundleAdjusterAffinePartialG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BundleAdjusterAffinePartial> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BundleAdjusterAffinePartial")
			.finish()
	}
}

