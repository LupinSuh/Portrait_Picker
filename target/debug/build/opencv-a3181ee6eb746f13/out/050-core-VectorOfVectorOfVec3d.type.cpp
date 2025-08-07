extern "C" {
	std::vector<std::vector<cv::Vec3d>>* std_vectorLstd_vectorLcv_Vec3dGG_new_const() {
			std::vector<std::vector<cv::Vec3d>>* ret = new std::vector<std::vector<cv::Vec3d>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_delete(std::vector<std::vector<cv::Vec3d>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_Vec3dGG_len_const(const std::vector<std::vector<cv::Vec3d>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_Vec3dGG_isEmpty_const(const std::vector<std::vector<cv::Vec3d>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_Vec3dGG_capacity_const(const std::vector<std::vector<cv::Vec3d>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_shrinkToFit(std::vector<std::vector<cv::Vec3d>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_reserve_size_t(std::vector<std::vector<cv::Vec3d>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_remove_size_t(std::vector<std::vector<cv::Vec3d>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_swap_size_t_size_t(std::vector<std::vector<cv::Vec3d>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_clear(std::vector<std::vector<cv::Vec3d>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_push_const_vectorLVec3dG(std::vector<std::vector<cv::Vec3d>>* instance, const std::vector<cv::Vec3d>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_insert_size_t_const_vectorLVec3dG(std::vector<std::vector<cv::Vec3d>>* instance, size_t index, const std::vector<cv::Vec3d>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_get_const_size_t(const std::vector<std::vector<cv::Vec3d>>* instance, size_t index, std::vector<cv::Vec3d>** ocvrs_return) {
			std::vector<cv::Vec3d> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Vec3d>(ret);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_set_size_t_const_vectorLVec3dG(std::vector<std::vector<cv::Vec3d>>* instance, size_t index, const std::vector<cv::Vec3d>* val) {
			(*instance)[index] = *val;
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_inputArray_const(const std::vector<std::vector<cv::Vec3d>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_outputArray(std::vector<std::vector<cv::Vec3d>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLcv_Vec3dGG_inputOutputArray(std::vector<std::vector<cv::Vec3d>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


