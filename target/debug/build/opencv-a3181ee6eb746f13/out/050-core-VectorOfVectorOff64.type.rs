pub type VectorOfVectorOff64 = core::Vector<core::Vector<f64>>;

impl core::Vector<core::Vector<f64>> {
	pub fn as_raw_VectorOfVectorOff64(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vector<f64>,
	std_vectorLstd_vectorLdoubleGG_new_const, std_vectorLstd_vectorLdoubleGG_delete,
	std_vectorLstd_vectorLdoubleGG_len_const, std_vectorLstd_vectorLdoubleGG_isEmpty_const,
	std_vectorLstd_vectorLdoubleGG_capacity_const, std_vectorLstd_vectorLdoubleGG_shrinkToFit,
	std_vectorLstd_vectorLdoubleGG_reserve_size_t, std_vectorLstd_vectorLdoubleGG_remove_size_t,
	std_vectorLstd_vectorLdoubleGG_swap_size_t_size_t, std_vectorLstd_vectorLdoubleGG_clear,
	std_vectorLstd_vectorLdoubleGG_get_const_size_t, std_vectorLstd_vectorLdoubleGG_set_size_t_const_vectorLdoubleG,
	std_vectorLstd_vectorLdoubleGG_push_const_vectorLdoubleG, std_vectorLstd_vectorLdoubleGG_insert_size_t_const_vectorLdoubleG,
}
vector_non_copy_or_bool! { clone core::Vector<f64> }

impl core::ToInputArray for core::Vector<core::Vector<f64>> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLdoubleGG_inputArray_const(self.as_raw_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vector<f64>> }

impl core::ToOutputArray for core::Vector<core::Vector<f64>> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLdoubleGG_outputArray(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vector<f64>> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLstd_vectorLdoubleGG_inputOutputArray(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vector<f64>> }

