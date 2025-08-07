pub type VectorOfDetail_CameraParams = core::Vector<crate::stitching::Detail_CameraParams>;

impl core::Vector<crate::stitching::Detail_CameraParams> {
	pub fn as_raw_VectorOfDetail_CameraParams(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDetail_CameraParams(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::stitching::Detail_CameraParams,
	std_vectorLcv_detail_CameraParamsG_new_const, std_vectorLcv_detail_CameraParamsG_delete,
	std_vectorLcv_detail_CameraParamsG_len_const, std_vectorLcv_detail_CameraParamsG_isEmpty_const,
	std_vectorLcv_detail_CameraParamsG_capacity_const, std_vectorLcv_detail_CameraParamsG_shrinkToFit,
	std_vectorLcv_detail_CameraParamsG_reserve_size_t, std_vectorLcv_detail_CameraParamsG_remove_size_t,
	std_vectorLcv_detail_CameraParamsG_swap_size_t_size_t, std_vectorLcv_detail_CameraParamsG_clear,
	std_vectorLcv_detail_CameraParamsG_get_const_size_t, std_vectorLcv_detail_CameraParamsG_set_size_t_const_CameraParams,
	std_vectorLcv_detail_CameraParamsG_push_const_CameraParams, std_vectorLcv_detail_CameraParamsG_insert_size_t_const_CameraParams,
}
vector_non_copy_or_bool! { clone crate::stitching::Detail_CameraParams }

