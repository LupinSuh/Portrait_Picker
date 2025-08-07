pub type VectorOfu8 = core::Vector<u8>;

impl core::Vector<u8> {
	pub fn as_raw_VectorOfu8(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfu8(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { u8,
	std_vectorLunsigned_charG_new_const, std_vectorLunsigned_charG_delete,
	std_vectorLunsigned_charG_len_const, std_vectorLunsigned_charG_isEmpty_const,
	std_vectorLunsigned_charG_capacity_const, std_vectorLunsigned_charG_shrinkToFit,
	std_vectorLunsigned_charG_reserve_size_t, std_vectorLunsigned_charG_remove_size_t,
	std_vectorLunsigned_charG_swap_size_t_size_t, std_vectorLunsigned_charG_clear,
	std_vectorLunsigned_charG_get_const_size_t, std_vectorLunsigned_charG_set_size_t_const_unsigned_char,
	std_vectorLunsigned_charG_push_const_unsigned_char, std_vectorLunsigned_charG_insert_size_t_const_unsigned_char,
}
vector_copy_non_bool! { u8,
	std_vectorLunsigned_charG_data_const, std_vectorLunsigned_charG_dataMut, cv_fromSlice_const_const_unsigned_charX_size_t,
	std_vectorLunsigned_charG_clone_const,
}

impl core::ToInputArray for core::Vector<u8> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLunsigned_charG_inputArray_const(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<u8> }

impl core::ToOutputArray for core::Vector<u8> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLunsigned_charG_outputArray(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<u8> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLunsigned_charG_inputOutputArray(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<u8> }

