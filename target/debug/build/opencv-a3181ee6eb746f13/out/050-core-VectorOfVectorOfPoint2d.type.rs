pub type VectorOfVectorOfPoint2d = core::Vector<core::Vector<core::Point2d>>;

impl core::Vector<core::Vector<core::Point2d>> {
	pub fn as_raw_VectorOfVectorOfPoint2d(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint2d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point2d>,
	std_vectorLstd_vectorLcv_Point2dGG_new_const, std_vectorLstd_vectorLcv_Point2dGG_delete,
	std_vectorLstd_vectorLcv_Point2dGG_len_const, std_vectorLstd_vectorLcv_Point2dGG_isEmpty_const,
	std_vectorLstd_vectorLcv_Point2dGG_capacity_const, std_vectorLstd_vectorLcv_Point2dGG_shrinkToFit,
	std_vectorLstd_vectorLcv_Point2dGG_reserve_size_t, std_vectorLstd_vectorLcv_Point2dGG_remove_size_t,
	std_vectorLstd_vectorLcv_Point2dGG_swap_size_t_size_t, std_vectorLstd_vectorLcv_Point2dGG_clear,
	std_vectorLstd_vectorLcv_Point2dGG_get_const_size_t, std_vectorLstd_vectorLcv_Point2dGG_set_size_t_const_vectorLPoint2dG,
	std_vectorLstd_vectorLcv_Point2dGG_push_const_vectorLPoint2dG, std_vectorLstd_vectorLcv_Point2dGG_insert_size_t_const_vectorLPoint2dG,
}
vector_non_copy_or_bool! { clone core::Vector<core::Point2d> }

impl core::ToInputArray for core::Vector<core::Vector<core::Point2d>> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point2dGG_inputArray_const(self.as_raw_VectorOfVectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vector<core::Point2d>> }

impl core::ToOutputArray for core::Vector<core::Vector<core::Point2d>> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point2dGG_outputArray(self.as_raw_mut_VectorOfVectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vector<core::Point2d>> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLcv_Point2dGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOfPoint2d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vector<core::Point2d>> }

