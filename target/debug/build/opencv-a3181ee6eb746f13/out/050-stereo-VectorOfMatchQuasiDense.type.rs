pub type VectorOfMatchQuasiDense = core::Vector<crate::stereo::MatchQuasiDense>;

impl core::Vector<crate::stereo::MatchQuasiDense> {
	pub fn as_raw_VectorOfMatchQuasiDense(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfMatchQuasiDense(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::stereo::MatchQuasiDense,
	std_vectorLcv_stereo_MatchQuasiDenseG_new_const, std_vectorLcv_stereo_MatchQuasiDenseG_delete,
	std_vectorLcv_stereo_MatchQuasiDenseG_len_const, std_vectorLcv_stereo_MatchQuasiDenseG_isEmpty_const,
	std_vectorLcv_stereo_MatchQuasiDenseG_capacity_const, std_vectorLcv_stereo_MatchQuasiDenseG_shrinkToFit,
	std_vectorLcv_stereo_MatchQuasiDenseG_reserve_size_t, std_vectorLcv_stereo_MatchQuasiDenseG_remove_size_t,
	std_vectorLcv_stereo_MatchQuasiDenseG_swap_size_t_size_t, std_vectorLcv_stereo_MatchQuasiDenseG_clear,
	std_vectorLcv_stereo_MatchQuasiDenseG_get_const_size_t, std_vectorLcv_stereo_MatchQuasiDenseG_set_size_t_const_MatchQuasiDense,
	std_vectorLcv_stereo_MatchQuasiDenseG_push_const_MatchQuasiDense, std_vectorLcv_stereo_MatchQuasiDenseG_insert_size_t_const_MatchQuasiDense,
}
vector_copy_non_bool! { crate::stereo::MatchQuasiDense,
	std_vectorLcv_stereo_MatchQuasiDenseG_data_const, std_vectorLcv_stereo_MatchQuasiDenseG_dataMut, cv_fromSlice_const_const_MatchQuasiDenseX_size_t,
	std_vectorLcv_stereo_MatchQuasiDenseG_clone_const,
}

