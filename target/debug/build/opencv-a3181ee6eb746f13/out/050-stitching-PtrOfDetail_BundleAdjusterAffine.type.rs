pub type PtrOfDetail_BundleAdjusterAffine = core::Ptr<crate::stitching::Detail_BundleAdjusterAffine>;

ptr_extern! { crate::stitching::Detail_BundleAdjusterAffine,
	cv_PtrLcv_detail_BundleAdjusterAffineG_delete, cv_PtrLcv_detail_BundleAdjusterAffineG_getInnerPtr_const, cv_PtrLcv_detail_BundleAdjusterAffineG_getInnerPtrMut
}

ptr_extern_ctor! { crate::stitching::Detail_BundleAdjusterAffine, cv_PtrLcv_detail_BundleAdjusterAffineG_new_const_BundleAdjusterAffine }
impl core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] pub fn as_raw_PtrOfDetail_BundleAdjusterAffine(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfDetail_BundleAdjusterAffine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterAffineTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] fn as_raw_Detail_BundleAdjusterAffine(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterAffineTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterAffine(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] fn as_raw_Detail_BundleAdjusterBase(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_BundleAdjusterBaseTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] fn as_raw_mut_Detail_BundleAdjusterBase(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<crate::stitching::Detail_BundleAdjusterBase>, cv_PtrLcv_detail_BundleAdjusterAffineG_to_PtrOfDetail_BundleAdjusterBase }

impl crate::stitching::Detail_EstimatorTraitConst for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] fn as_raw_Detail_Estimator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::Detail_EstimatorTrait for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline] fn as_raw_mut_Detail_Estimator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfDetail_BundleAdjusterAffine, core::Ptr<crate::stitching::Detail_Estimator>, cv_PtrLcv_detail_BundleAdjusterAffineG_to_PtrOfDetail_Estimator }

impl std::fmt::Debug for core::Ptr<crate::stitching::Detail_BundleAdjusterAffine> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfDetail_BundleAdjusterAffine")
			.finish()
	}
}

