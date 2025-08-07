pub type VectorOffeature_index = core::Vector<crate::flann::feature_index>;

impl core::Vector<crate::flann::feature_index> {
	pub fn as_raw_VectorOffeature_index(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOffeature_index(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::flann::feature_index,
	std_vectorLcvflann_lsh_FeatureIndexG_new_const, std_vectorLcvflann_lsh_FeatureIndexG_delete,
	std_vectorLcvflann_lsh_FeatureIndexG_len_const, std_vectorLcvflann_lsh_FeatureIndexG_isEmpty_const,
	std_vectorLcvflann_lsh_FeatureIndexG_capacity_const, std_vectorLcvflann_lsh_FeatureIndexG_shrinkToFit,
	std_vectorLcvflann_lsh_FeatureIndexG_reserve_size_t, std_vectorLcvflann_lsh_FeatureIndexG_remove_size_t,
	std_vectorLcvflann_lsh_FeatureIndexG_swap_size_t_size_t, std_vectorLcvflann_lsh_FeatureIndexG_clear,
	std_vectorLcvflann_lsh_FeatureIndexG_get_const_size_t, std_vectorLcvflann_lsh_FeatureIndexG_set_size_t_const_FeatureIndex,
	std_vectorLcvflann_lsh_FeatureIndexG_push_const_FeatureIndex, std_vectorLcvflann_lsh_FeatureIndexG_insert_size_t_const_FeatureIndex,
}
vector_copy_non_bool! { crate::flann::feature_index,
	std_vectorLcvflann_lsh_FeatureIndexG_data_const, std_vectorLcvflann_lsh_FeatureIndexG_dataMut, cv_fromSlice_const_const_FeatureIndexX_size_t,
	std_vectorLcvflann_lsh_FeatureIndexG_clone_const,
}

