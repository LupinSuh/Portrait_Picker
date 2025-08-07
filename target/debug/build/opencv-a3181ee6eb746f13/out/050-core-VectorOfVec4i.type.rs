pub type VectorOfVec4i = core::Vector<core::Vec4i>;

impl core::Vector<core::Vec4i> {
	pub fn as_raw_VectorOfVec4i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec4i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vec4i,
	std_vectorLcv_Vec4iG_new_const, std_vectorLcv_Vec4iG_delete,
	std_vectorLcv_Vec4iG_len_const, std_vectorLcv_Vec4iG_isEmpty_const,
	std_vectorLcv_Vec4iG_capacity_const, std_vectorLcv_Vec4iG_shrinkToFit,
	std_vectorLcv_Vec4iG_reserve_size_t, std_vectorLcv_Vec4iG_remove_size_t,
	std_vectorLcv_Vec4iG_swap_size_t_size_t, std_vectorLcv_Vec4iG_clear,
	std_vectorLcv_Vec4iG_get_const_size_t, std_vectorLcv_Vec4iG_set_size_t_const_Vec4i,
	std_vectorLcv_Vec4iG_push_const_Vec4i, std_vectorLcv_Vec4iG_insert_size_t_const_Vec4i,
}
vector_copy_non_bool! { core::Vec4i,
	std_vectorLcv_Vec4iG_data_const, std_vectorLcv_Vec4iG_dataMut, cv_fromSlice_const_const_Vec4iX_size_t,
	std_vectorLcv_Vec4iG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Vec4i> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec4iG_inputArray_const(self.as_raw_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vec4i> }

impl core::ToOutputArray for core::Vector<core::Vec4i> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec4iG_outputArray(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vec4i> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec4iG_inputOutputArray(self.as_raw_mut_VectorOfVec4i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vec4i> }

