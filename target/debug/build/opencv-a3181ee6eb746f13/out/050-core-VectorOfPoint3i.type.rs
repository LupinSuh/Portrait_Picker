pub type VectorOfPoint3i = core::Vector<core::Point3i>;

impl core::Vector<core::Point3i> {
	pub fn as_raw_VectorOfPoint3i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Point3i,
	std_vectorLcv_Point3iG_new_const, std_vectorLcv_Point3iG_delete,
	std_vectorLcv_Point3iG_len_const, std_vectorLcv_Point3iG_isEmpty_const,
	std_vectorLcv_Point3iG_capacity_const, std_vectorLcv_Point3iG_shrinkToFit,
	std_vectorLcv_Point3iG_reserve_size_t, std_vectorLcv_Point3iG_remove_size_t,
	std_vectorLcv_Point3iG_swap_size_t_size_t, std_vectorLcv_Point3iG_clear,
	std_vectorLcv_Point3iG_get_const_size_t, std_vectorLcv_Point3iG_set_size_t_const_Point3i,
	std_vectorLcv_Point3iG_push_const_Point3i, std_vectorLcv_Point3iG_insert_size_t_const_Point3i,
}
vector_copy_non_bool! { core::Point3i,
	std_vectorLcv_Point3iG_data_const, std_vectorLcv_Point3iG_dataMut, cv_fromSlice_const_const_Point3iX_size_t,
	std_vectorLcv_Point3iG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Point3i> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Point3iG_inputArray_const(self.as_raw_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Point3i> }

impl core::ToOutputArray for core::Vector<core::Point3i> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Point3iG_outputArray(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Point3i> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Point3iG_inputOutputArray(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Point3i> }

