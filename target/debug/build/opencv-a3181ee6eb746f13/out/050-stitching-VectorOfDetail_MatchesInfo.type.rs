pub type VectorOfDetail_MatchesInfo = core::Vector<crate::stitching::Detail_MatchesInfo>;

impl core::Vector<crate::stitching::Detail_MatchesInfo> {
	pub fn as_raw_VectorOfDetail_MatchesInfo(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_MatchesInfo(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::stitching::Detail_MatchesInfo,
	std_vectorLcv_detail_MatchesInfoG_new_const, std_vectorLcv_detail_MatchesInfoG_delete,
	std_vectorLcv_detail_MatchesInfoG_len_const, std_vectorLcv_detail_MatchesInfoG_isEmpty_const,
	std_vectorLcv_detail_MatchesInfoG_capacity_const, std_vectorLcv_detail_MatchesInfoG_shrinkToFit,
	std_vectorLcv_detail_MatchesInfoG_reserve_size_t, std_vectorLcv_detail_MatchesInfoG_remove_size_t,
	std_vectorLcv_detail_MatchesInfoG_swap_size_t_size_t, std_vectorLcv_detail_MatchesInfoG_clear,
	std_vectorLcv_detail_MatchesInfoG_get_const_size_t, std_vectorLcv_detail_MatchesInfoG_set_size_t_const_MatchesInfo,
	std_vectorLcv_detail_MatchesInfoG_push_const_MatchesInfo, std_vectorLcv_detail_MatchesInfoG_insert_size_t_const_MatchesInfo,
}
vector_non_copy_or_bool! { clone crate::stitching::Detail_MatchesInfo }

