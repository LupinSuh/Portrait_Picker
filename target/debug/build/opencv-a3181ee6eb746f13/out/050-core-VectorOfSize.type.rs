pub type VectorOfSize = core::Vector<core::Size>;

impl core::Vector<core::Size> {
	pub fn as_raw_VectorOfSize(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfSize(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Size,
	std_vectorLcv_SizeG_new_const, std_vectorLcv_SizeG_delete,
	std_vectorLcv_SizeG_len_const, std_vectorLcv_SizeG_isEmpty_const,
	std_vectorLcv_SizeG_capacity_const, std_vectorLcv_SizeG_shrinkToFit,
	std_vectorLcv_SizeG_reserve_size_t, std_vectorLcv_SizeG_remove_size_t,
	std_vectorLcv_SizeG_swap_size_t_size_t, std_vectorLcv_SizeG_clear,
	std_vectorLcv_SizeG_get_const_size_t, std_vectorLcv_SizeG_set_size_t_const_Size,
	std_vectorLcv_SizeG_push_const_Size, std_vectorLcv_SizeG_insert_size_t_const_Size,
}
vector_copy_non_bool! { core::Size,
	std_vectorLcv_SizeG_data_const, std_vectorLcv_SizeG_dataMut, cv_fromSlice_const_const_SizeX_size_t,
	std_vectorLcv_SizeG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Size> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_SizeG_inputArray_const(self.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Size> }

impl core::ToOutputArray for core::Vector<core::Size> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_SizeG_outputArray(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Size> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_SizeG_inputOutputArray(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Size> }

