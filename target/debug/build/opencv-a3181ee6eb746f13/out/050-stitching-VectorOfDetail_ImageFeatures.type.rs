pub type VectorOfDetail_ImageFeatures = core::Vector<crate::stitching::Detail_ImageFeatures>;

impl core::Vector<crate::stitching::Detail_ImageFeatures> {
	pub fn as_raw_VectorOfDetail_ImageFeatures(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_ImageFeatures(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::stitching::Detail_ImageFeatures,
	std_vectorLcv_detail_ImageFeaturesG_new_const, std_vectorLcv_detail_ImageFeaturesG_delete,
	std_vectorLcv_detail_ImageFeaturesG_len_const, std_vectorLcv_detail_ImageFeaturesG_isEmpty_const,
	std_vectorLcv_detail_ImageFeaturesG_capacity_const, std_vectorLcv_detail_ImageFeaturesG_shrinkToFit,
	std_vectorLcv_detail_ImageFeaturesG_reserve_size_t, std_vectorLcv_detail_ImageFeaturesG_remove_size_t,
	std_vectorLcv_detail_ImageFeaturesG_swap_size_t_size_t, std_vectorLcv_detail_ImageFeaturesG_clear,
	std_vectorLcv_detail_ImageFeaturesG_get_const_size_t, std_vectorLcv_detail_ImageFeaturesG_set_size_t_const_ImageFeatures,
	std_vectorLcv_detail_ImageFeaturesG_push_const_ImageFeatures, std_vectorLcv_detail_ImageFeaturesG_insert_size_t_const_ImageFeatures,
}
vector_non_copy_or_bool! { clone crate::stitching::Detail_ImageFeatures }

