pub type VectorOfFacemarkAAM_Config = core::Vector<crate::face::FacemarkAAM_Config>;

impl core::Vector<crate::face::FacemarkAAM_Config> {
	pub fn as_raw_VectorOfFacemarkAAM_Config(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfFacemarkAAM_Config(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::face::FacemarkAAM_Config,
	std_vectorLcv_face_FacemarkAAM_ConfigG_new_const, std_vectorLcv_face_FacemarkAAM_ConfigG_delete,
	std_vectorLcv_face_FacemarkAAM_ConfigG_len_const, std_vectorLcv_face_FacemarkAAM_ConfigG_isEmpty_const,
	std_vectorLcv_face_FacemarkAAM_ConfigG_capacity_const, std_vectorLcv_face_FacemarkAAM_ConfigG_shrinkToFit,
	std_vectorLcv_face_FacemarkAAM_ConfigG_reserve_size_t, std_vectorLcv_face_FacemarkAAM_ConfigG_remove_size_t,
	std_vectorLcv_face_FacemarkAAM_ConfigG_swap_size_t_size_t, std_vectorLcv_face_FacemarkAAM_ConfigG_clear,
	std_vectorLcv_face_FacemarkAAM_ConfigG_get_const_size_t, std_vectorLcv_face_FacemarkAAM_ConfigG_set_size_t_const_Config,
	std_vectorLcv_face_FacemarkAAM_ConfigG_push_const_Config, std_vectorLcv_face_FacemarkAAM_ConfigG_insert_size_t_const_Config,
}
vector_non_copy_or_bool! { crate::face::FacemarkAAM_Config }

