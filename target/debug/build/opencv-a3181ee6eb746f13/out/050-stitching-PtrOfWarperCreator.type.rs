pub type PtrOfWarperCreator = core::Ptr<crate::stitching::WarperCreator>;

ptr_extern! { crate::stitching::WarperCreator,
	cv_PtrLcv_WarperCreatorG_delete, cv_PtrLcv_WarperCreatorG_getInnerPtr_const, cv_PtrLcv_WarperCreatorG_getInnerPtrMut
}

impl core::Ptr<crate::stitching::WarperCreator> {
	#[inline] pub fn as_raw_PtrOfWarperCreator(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfWarperCreator(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::stitching::WarperCreatorTraitConst for core::Ptr<crate::stitching::WarperCreator> {
	#[inline] fn as_raw_WarperCreator(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::stitching::WarperCreatorTrait for core::Ptr<crate::stitching::WarperCreator> {
	#[inline] fn as_raw_mut_WarperCreator(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl std::fmt::Debug for core::Ptr<crate::stitching::WarperCreator> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfWarperCreator")
			.finish()
	}
}

