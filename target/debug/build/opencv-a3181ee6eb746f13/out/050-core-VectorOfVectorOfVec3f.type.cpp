extern "C" {
	std::vector<std::vector<cv::Vec3f>>* std_vectorLstd_vectorLcv_Vec3fGG_new_const() {
			std::vector<std::vector<cv::Vec3f>>* ret = new std::vector<std::vector<cv::Vec3f>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_delete(std::vector<std::vector<cv::Vec3f>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_Vec3fGG_len_const(const std::vector<std::vector<cv::Vec3f>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_Vec3fGG_isEmpty_const(const std::vector<std::vector<cv::Vec3f>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_Vec3fGG_capacity_const(const std::vector<std::vector<cv::Vec3f>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_shrinkToFit(std::vector<std::vector<cv::Vec3f>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_reserve_size_t(std::vector<std::vector<cv::Vec3f>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_remove_size_t(std::vector<std::vector<cv::Vec3f>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_swap_size_t_size_t(std::vector<std::vector<cv::Vec3f>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_clear(std::vector<std::vector<cv::Vec3f>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_push_const_vectorLVec3fG(std::vector<std::vector<cv::Vec3f>>* instance, const std::vector<cv::Vec3f>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_insert_size_t_const_vectorLVec3fG(std::vector<std::vector<cv::Vec3f>>* instance, size_t index, const std::vector<cv::Vec3f>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_get_const_size_t(const std::vector<std::vector<cv::Vec3f>>* instance, size_t index, std::vector<cv::Vec3f>** ocvrs_return) {
			std::vector<cv::Vec3f> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Vec3f>(ret);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_set_size_t_const_vectorLVec3fG(std::vector<std::vector<cv::Vec3f>>* instance, size_t index, const std::vector<cv::Vec3f>* val) {
			(*instance)[index] = *val;
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_inputArray_const(const std::vector<std::vector<cv::Vec3f>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_outputArray(std::vector<std::vector<cv::Vec3f>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLcv_Vec3fGG_inputOutputArray(std::vector<std::vector<cv::Vec3f>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


