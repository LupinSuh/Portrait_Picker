pub type VectorOfGPCPatchDescriptor = core::Vector<crate::optflow::GPCPatchDescriptor>;

impl core::Vector<crate::optflow::GPCPatchDescriptor> {
	pub fn as_raw_VectorOfGPCPatchDescriptor(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfGPCPatchDescriptor(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::optflow::GPCPatchDescriptor,
	std_vectorLcv_optflow_GPCPatchDescriptorG_new_const, std_vectorLcv_optflow_GPCPatchDescriptorG_delete,
	std_vectorLcv_optflow_GPCPatchDescriptorG_len_const, std_vectorLcv_optflow_GPCPatchDescriptorG_isEmpty_const,
	std_vectorLcv_optflow_GPCPatchDescriptorG_capacity_const, std_vectorLcv_optflow_GPCPatchDescriptorG_shrinkToFit,
	std_vectorLcv_optflow_GPCPatchDescriptorG_reserve_size_t, std_vectorLcv_optflow_GPCPatchDescriptorG_remove_size_t,
	std_vectorLcv_optflow_GPCPatchDescriptorG_swap_size_t_size_t, std_vectorLcv_optflow_GPCPatchDescriptorG_clear,
	std_vectorLcv_optflow_GPCPatchDescriptorG_get_const_size_t, std_vectorLcv_optflow_GPCPatchDescriptorG_set_size_t_const_GPCPatchDescriptor,
	std_vectorLcv_optflow_GPCPatchDescriptorG_push_const_GPCPatchDescriptor, std_vectorLcv_optflow_GPCPatchDescriptorG_insert_size_t_const_GPCPatchDescriptor,
}
vector_non_copy_or_bool! { crate::optflow::GPCPatchDescriptor }

