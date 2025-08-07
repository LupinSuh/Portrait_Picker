pub type VectorOfi32 = core::Vector<i32>;

impl core::Vector<i32> {
	pub fn as_raw_VectorOfi32(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfi32(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { i32,
	std_vectorLintG_new_const, std_vectorLintG_delete,
	std_vectorLintG_len_const, std_vectorLintG_isEmpty_const,
	std_vectorLintG_capacity_const, std_vectorLintG_shrinkToFit,
	std_vectorLintG_reserve_size_t, std_vectorLintG_remove_size_t,
	std_vectorLintG_swap_size_t_size_t, std_vectorLintG_clear,
	std_vectorLintG_get_const_size_t, std_vectorLintG_set_size_t_const_int,
	std_vectorLintG_push_const_int, std_vectorLintG_insert_size_t_const_int,
}
vector_copy_non_bool! { i32,
	std_vectorLintG_data_const, std_vectorLintG_dataMut, cv_fromSlice_const_const_intX_size_t,
	std_vectorLintG_clone_const,
}

impl core::ToInputArray for core::Vector<i32> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLintG_inputArray_const(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<i32> }

impl core::ToOutputArray for core::Vector<i32> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLintG_outputArray(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<i32> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLintG_inputOutputArray(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<i32> }

