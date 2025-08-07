pub type VectorOfbool = core::Vector<bool>;

impl core::Vector<bool> {
	pub fn as_raw_VectorOfbool(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfbool(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { bool,
	std_vectorLboolG_new_const, std_vectorLboolG_delete,
	std_vectorLboolG_len_const, std_vectorLboolG_isEmpty_const,
	std_vectorLboolG_capacity_const, std_vectorLboolG_shrinkToFit,
	std_vectorLboolG_reserve_size_t, std_vectorLboolG_remove_size_t,
	std_vectorLboolG_swap_size_t_size_t, std_vectorLboolG_clear,
	std_vectorLboolG_get_const_size_t, std_vectorLboolG_set_size_t_const_bool,
	std_vectorLboolG_push_const_bool, std_vectorLboolG_insert_size_t_const_bool,
}
vector_non_copy_or_bool! { clone bool }

