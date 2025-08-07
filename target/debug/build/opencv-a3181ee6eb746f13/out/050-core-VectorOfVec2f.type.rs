pub type VectorOfVec2f = core::Vector<core::Vec2f>;

impl core::Vector<core::Vec2f> {
	pub fn as_raw_VectorOfVec2f(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec2f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vec2f,
	std_vectorLcv_Vec2fG_new_const, std_vectorLcv_Vec2fG_delete,
	std_vectorLcv_Vec2fG_len_const, std_vectorLcv_Vec2fG_isEmpty_const,
	std_vectorLcv_Vec2fG_capacity_const, std_vectorLcv_Vec2fG_shrinkToFit,
	std_vectorLcv_Vec2fG_reserve_size_t, std_vectorLcv_Vec2fG_remove_size_t,
	std_vectorLcv_Vec2fG_swap_size_t_size_t, std_vectorLcv_Vec2fG_clear,
	std_vectorLcv_Vec2fG_get_const_size_t, std_vectorLcv_Vec2fG_set_size_t_const_Vec2f,
	std_vectorLcv_Vec2fG_push_const_Vec2f, std_vectorLcv_Vec2fG_insert_size_t_const_Vec2f,
}
vector_copy_non_bool! { core::Vec2f,
	std_vectorLcv_Vec2fG_data_const, std_vectorLcv_Vec2fG_dataMut, cv_fromSlice_const_const_Vec2fX_size_t,
	std_vectorLcv_Vec2fG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Vec2f> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec2fG_inputArray_const(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vec2f> }

impl core::ToOutputArray for core::Vector<core::Vec2f> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec2fG_outputArray(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vec2f> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec2fG_inputOutputArray(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vec2f> }

