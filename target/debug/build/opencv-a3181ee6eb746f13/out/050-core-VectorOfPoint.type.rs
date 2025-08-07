pub type VectorOfPoint = core::Vector<core::Point>;

impl core::Vector<core::Point> {
	pub fn as_raw_VectorOfPoint(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Point,
	std_vectorLcv_PointG_new_const, std_vectorLcv_PointG_delete,
	std_vectorLcv_PointG_len_const, std_vectorLcv_PointG_isEmpty_const,
	std_vectorLcv_PointG_capacity_const, std_vectorLcv_PointG_shrinkToFit,
	std_vectorLcv_PointG_reserve_size_t, std_vectorLcv_PointG_remove_size_t,
	std_vectorLcv_PointG_swap_size_t_size_t, std_vectorLcv_PointG_clear,
	std_vectorLcv_PointG_get_const_size_t, std_vectorLcv_PointG_set_size_t_const_Point,
	std_vectorLcv_PointG_push_const_Point, std_vectorLcv_PointG_insert_size_t_const_Point,
}
vector_copy_non_bool! { core::Point,
	std_vectorLcv_PointG_data_const, std_vectorLcv_PointG_dataMut, cv_fromSlice_const_const_PointX_size_t,
	std_vectorLcv_PointG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Point> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_PointG_inputArray_const(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Point> }

impl core::ToOutputArray for core::Vector<core::Point> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_PointG_outputArray(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Point> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_PointG_inputOutputArray(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Point> }

