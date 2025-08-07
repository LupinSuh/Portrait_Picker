pub type VectorOfVectorOfKeyLine = core::Vector<core::Vector<crate::line_descriptor::KeyLine>>;

impl core::Vector<core::Vector<crate::line_descriptor::KeyLine>> {
	pub fn as_raw_VectorOfVectorOfKeyLine(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfKeyLine(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<crate::line_descriptor::KeyLine>,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_new_const, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_delete,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_len_const, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_isEmpty_const,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_capacity_const, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_shrinkToFit,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_reserve_size_t, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_remove_size_t,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_clear,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_get_const_size_t, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_set_size_t_const_vectorLKeyLineG,
	std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_push_const_vectorLKeyLineG, std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_insert_size_t_const_vectorLKeyLineG,
}
vector_non_copy_or_bool! { clone core::Vector<crate::line_descriptor::KeyLine> }

