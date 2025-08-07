pub type VectorOfVec6f = core::Vector<core::Vec6f>;

impl core::Vector<core::Vec6f> {
	pub fn as_raw_VectorOfVec6f(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec6f(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vec6f,
	std_vectorLcv_Vec6fG_new_const, std_vectorLcv_Vec6fG_delete,
	std_vectorLcv_Vec6fG_len_const, std_vectorLcv_Vec6fG_isEmpty_const,
	std_vectorLcv_Vec6fG_capacity_const, std_vectorLcv_Vec6fG_shrinkToFit,
	std_vectorLcv_Vec6fG_reserve_size_t, std_vectorLcv_Vec6fG_remove_size_t,
	std_vectorLcv_Vec6fG_swap_size_t_size_t, std_vectorLcv_Vec6fG_clear,
	std_vectorLcv_Vec6fG_get_const_size_t, std_vectorLcv_Vec6fG_set_size_t_const_Vec6f,
	std_vectorLcv_Vec6fG_push_const_Vec6f, std_vectorLcv_Vec6fG_insert_size_t_const_Vec6f,
}
vector_copy_non_bool! { core::Vec6f,
	std_vectorLcv_Vec6fG_data_const, std_vectorLcv_Vec6fG_dataMut, cv_fromSlice_const_const_Vec6fX_size_t,
	std_vectorLcv_Vec6fG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Vec6f> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec6fG_inputArray_const(self.as_raw_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vec6f> }

impl core::ToOutputArray for core::Vector<core::Vec6f> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec6fG_outputArray(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vec6f> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec6fG_inputOutputArray(self.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vec6f> }

