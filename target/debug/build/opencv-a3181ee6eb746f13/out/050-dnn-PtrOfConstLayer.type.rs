pub type PtrOfConstLayer = core::Ptr<crate::dnn::ConstLayer>;

ptr_extern! { crate::dnn::ConstLayer,
	cv_PtrLcv_dnn_ConstLayerG_delete, cv_PtrLcv_dnn_ConstLayerG_getInnerPtr_const, cv_PtrLcv_dnn_ConstLayerG_getInnerPtrMut
}

ptr_extern_ctor! { crate::dnn::ConstLayer, cv_PtrLcv_dnn_ConstLayerG_new_const_ConstLayer }
impl core::Ptr<crate::dnn::ConstLayer> {
	#[inline] pub fn as_raw_PtrOfConstLayer(&self) -> extern_send!(Self) { self.as_raw() }
	#[inline] pub fn as_raw_mut_PtrOfConstLayer(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

impl crate::dnn::ConstLayerTraitConst for core::Ptr<crate::dnn::ConstLayer> {
	#[inline] fn as_raw_ConstLayer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::ConstLayerTrait for core::Ptr<crate::dnn::ConstLayer> {
	#[inline] fn as_raw_mut_ConstLayer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

impl core::AlgorithmTraitConst for core::Ptr<crate::dnn::ConstLayer> {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.inner_as_raw() }
}

impl core::AlgorithmTrait for core::Ptr<crate::dnn::ConstLayer> {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfConstLayer, core::Ptr<core::Algorithm>, cv_PtrLcv_dnn_ConstLayerG_to_PtrOfAlgorithm }

impl crate::dnn::LayerTraitConst for core::Ptr<crate::dnn::ConstLayer> {
	#[inline] fn as_raw_Layer(&self) -> *const c_void { self.inner_as_raw() }
}

impl crate::dnn::LayerTrait for core::Ptr<crate::dnn::ConstLayer> {
	#[inline] fn as_raw_mut_Layer(&mut self) -> *mut c_void { self.inner_as_raw_mut() }
}

ptr_cast_base! { PtrOfConstLayer, core::Ptr<crate::dnn::Layer>, cv_PtrLcv_dnn_ConstLayerG_to_PtrOfLayer }

impl std::fmt::Debug for core::Ptr<crate::dnn::ConstLayer> {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("PtrOfConstLayer")
			.field("blobs", &crate::dnn::LayerTraitConst::blobs(self))
			.field("name", &crate::dnn::LayerTraitConst::name(self))
			.field("typ", &crate::dnn::LayerTraitConst::typ(self))
			.field("preferable_target", &crate::dnn::LayerTraitConst::preferable_target(self))
			.finish()
	}
}

