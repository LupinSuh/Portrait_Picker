pub type VectorOfDictionary = core::Vector<crate::objdetect::Dictionary>;

impl core::Vector<crate::objdetect::Dictionary> {
	pub fn as_raw_VectorOfDictionary(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfDictionary(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { crate::objdetect::Dictionary,
	std_vectorLcv_aruco_DictionaryG_new_const, std_vectorLcv_aruco_DictionaryG_delete,
	std_vectorLcv_aruco_DictionaryG_len_const, std_vectorLcv_aruco_DictionaryG_isEmpty_const,
	std_vectorLcv_aruco_DictionaryG_capacity_const, std_vectorLcv_aruco_DictionaryG_shrinkToFit,
	std_vectorLcv_aruco_DictionaryG_reserve_size_t, std_vectorLcv_aruco_DictionaryG_remove_size_t,
	std_vectorLcv_aruco_DictionaryG_swap_size_t_size_t, std_vectorLcv_aruco_DictionaryG_clear,
	std_vectorLcv_aruco_DictionaryG_get_const_size_t, std_vectorLcv_aruco_DictionaryG_set_size_t_const_Dictionary,
	std_vectorLcv_aruco_DictionaryG_push_const_Dictionary, std_vectorLcv_aruco_DictionaryG_insert_size_t_const_Dictionary,
}
vector_non_copy_or_bool! { clone crate::objdetect::Dictionary }

