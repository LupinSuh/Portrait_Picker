pub type VectorOfVec3d = core::Vector<core::Vec3d>;

impl core::Vector<core::Vec3d> {
	pub fn as_raw_VectorOfVec3d(&self) -> extern_send!(Self) { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec3d(&mut self) -> extern_send!(mut Self) { self.as_raw_mut() }
}

vector_extern! { core::Vec3d,
	std_vectorLcv_Vec3dG_new_const, std_vectorLcv_Vec3dG_delete,
	std_vectorLcv_Vec3dG_len_const, std_vectorLcv_Vec3dG_isEmpty_const,
	std_vectorLcv_Vec3dG_capacity_const, std_vectorLcv_Vec3dG_shrinkToFit,
	std_vectorLcv_Vec3dG_reserve_size_t, std_vectorLcv_Vec3dG_remove_size_t,
	std_vectorLcv_Vec3dG_swap_size_t_size_t, std_vectorLcv_Vec3dG_clear,
	std_vectorLcv_Vec3dG_get_const_size_t, std_vectorLcv_Vec3dG_set_size_t_const_Vec3d,
	std_vectorLcv_Vec3dG_push_const_Vec3d, std_vectorLcv_Vec3dG_insert_size_t_const_Vec3d,
}
vector_copy_non_bool! { core::Vec3d,
	std_vectorLcv_Vec3dG_data_const, std_vectorLcv_Vec3dG_dataMut, cv_fromSlice_const_const_Vec3dX_size_t,
	std_vectorLcv_Vec3dG_clone_const,
}

impl core::ToInputArray for core::Vector<core::Vec3d> {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec3dG_inputArray_const(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

input_array_ref_forward! { core::Vector<core::Vec3d> }

impl core::ToOutputArray for core::Vector<core::Vec3d> {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec3dG_outputArray(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_OutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl core::ToInputOutputArray for core::Vector<core::Vec3d> {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		return_send!(via ocvrs_return);
		unsafe { sys::std_vectorLcv_Vec3dG_inputOutputArray(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::_InputOutputArray::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

output_array_ref_forward! { core::Vector<core::Vec3d> }

