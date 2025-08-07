pub type VectorOfRect = core::Vector<core::Rect>;

impl core::Vector<core::Rect> {
	pub fn as_raw_VectorOfRect(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfRect(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Rect,
	std_vectorLcv_RectG_new_const, std_vectorLcv_RectG_delete,
	std_vectorLcv_RectG_len_const, std_vectorLcv_RectG_isEmpty_const,
	std_vectorLcv_RectG_capacity_const, std_vectorLcv_RectG_shrinkToFit,
	std_vectorLcv_RectG_reserve_size_t, std_vectorLcv_RectG_remove_size_t,
	std_vectorLcv_RectG_swap_size_t_size_t, std_vectorLcv_RectG_clear,
	std_vectorLcv_RectG_get_const_size_t, std_vectorLcv_RectG_set_size_t_const_Rect,
	std_vectorLcv_RectG_push_const_Rect, std_vectorLcv_RectG_insert_size_t_const_Rect,
}
vector_copy_non_bool! { core::Rect,
	std_vectorLcv_RectG_data_const, std_vectorLcv_RectG_dataMut, cv_fromSlice_const_const_RectX_size_t,
	std_vectorLcv_RectG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Rect> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_RectG_inputArray_const(self.as_raw_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Rect> }

impl core::ToOutputArray for core::Vector<core::Rect> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_RectG_outputArray(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Rect> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_RectG_inputOutputArray(self.as_raw_mut_VectorOfRect(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Rect> }

