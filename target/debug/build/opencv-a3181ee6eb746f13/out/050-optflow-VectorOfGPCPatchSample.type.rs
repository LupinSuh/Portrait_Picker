pub type VectorOfGPCPatchSample = core::Vector<crate::optflow::GPCPatchSample>;

impl core::Vector<crate::optflow::GPCPatchSample> {
	pub fn as_raw_VectorOfGPCPatchSample(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGPCPatchSample(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::optflow::GPCPatchSample,
	std_vectorLcv_optflow_GPCPatchSampleG_new_const, std_vectorLcv_optflow_GPCPatchSampleG_delete,
	std_vectorLcv_optflow_GPCPatchSampleG_len_const, std_vectorLcv_optflow_GPCPatchSampleG_isEmpty_const,
	std_vectorLcv_optflow_GPCPatchSampleG_capacity_const, std_vectorLcv_optflow_GPCPatchSampleG_shrinkToFit,
	std_vectorLcv_optflow_GPCPatchSampleG_reserve_size_t, std_vectorLcv_optflow_GPCPatchSampleG_remove_size_t,
	std_vectorLcv_optflow_GPCPatchSampleG_swap_size_t_size_t, std_vectorLcv_optflow_GPCPatchSampleG_clear,
	std_vectorLcv_optflow_GPCPatchSampleG_get_const_size_t, std_vectorLcv_optflow_GPCPatchSampleG_set_size_t_const_GPCPatchSample,
	std_vectorLcv_optflow_GPCPatchSampleG_push_const_GPCPatchSample, std_vectorLcv_optflow_GPCPatchSampleG_insert_size_t_const_GPCPatchSample,
}
vector_non_copy_or_bool! { crate::optflow::GPCPatchSample }

