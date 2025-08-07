pub type VectorOfVectorOfPoint3i = core::Vector<core::Vector<core::Point3i>>;

impl core::Vector<core::Vector<core::Point3i>> {
	pub fn as_raw_VectorOfVectorOfPoint3i(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint3i(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point3i>,
	std_vectorLstd_vectorLcv_Point3iGG_new_const, std_vectorLstd_vectorLcv_Point3iGG_delete,
	std_vectorLstd_vectorLcv_Point3iGG_len_const, std_vectorLstd_vectorLcv_Point3iGG_isEmpty_const,
	std_vectorLstd_vectorLcv_Point3iGG_capacity_const, std_vectorLstd_vectorLcv_Point3iGG_shrinkToFit,
	std_vectorLstd_vectorLcv_Point3iGG_reserve_size_t, std_vectorLstd_vectorLcv_Point3iGG_remove_size_t,
	std_vectorLstd_vectorLcv_Point3iGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Point3iGG_clear,
	std_vectorLstd_vectorLcv_Point3iGG_get_const_size_t, std_vectorLstd_vectorLcv_Point3iGG_set_size_t_const_vectorLPoint3iG,
	std_vectorLstd_vectorLcv_Point3iGG_push_const_vectorLPoint3iG, std_vectorLstd_vectorLcv_Point3iGG_insert_size_t_const_vectorLPoint3iG,
}
vector_non_copy_or_bool! { clone core::Vector<core::Point3i> }

impl core::ToInputArray for core::Vector<core::Vector<core::Point3i>> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point3iGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vector<core::Point3i>> }

impl core::ToOutputArray for core::Vector<core::Vector<core::Point3i>> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point3iGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point3i>> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point3iGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint3i(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vector<core::Point3i>> }

