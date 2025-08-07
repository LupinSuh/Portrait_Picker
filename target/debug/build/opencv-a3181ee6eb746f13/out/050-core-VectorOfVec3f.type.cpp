extern "C" {
	std::vector<cv::Vec3f>* std_vectorLcv_Vec3fG_new_const() {
			std::vector<cv::Vec3f>* ret = new std::vector<cv::Vec3f>();
			return ret;
	}
	
	void std_vectorLcv_Vec3fG_delete(std::vector<cv::Vec3f>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Vec3fG_len_const(const std::vector<cv::Vec3f>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Vec3fG_isEmpty_const(const std::vector<cv::Vec3f>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Vec3fG_capacity_const(const std::vector<cv::Vec3f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Vec3fG_shrinkToFit(std::vector<cv::Vec3f>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Vec3fG_reserve_size_t(std::vector<cv::Vec3f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Vec3fG_remove_size_t(std::vector<cv::Vec3f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Vec3fG_swap_size_t_size_t(std::vector<cv::Vec3f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Vec3fG_clear(std::vector<cv::Vec3f>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Vec3fG_push_const_Vec3f(std::vector<cv::Vec3f>* instance, const cv::Vec3f* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Vec3fG_insert_size_t_const_Vec3f(std::vector<cv::Vec3f>* instance, size_t index, const cv::Vec3f* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Vec3fG_get_const_size_t(const std::vector<cv::Vec3f>* instance, size_t index, cv::Vec3f* ocvrs_return) {
			cv::Vec3f ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Vec3fG_set_size_t_const_Vec3f(std::vector<cv::Vec3f>* instance, size_t index, const cv::Vec3f* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Vec3f>* std_vectorLcv_Vec3fG_clone_const(const std::vector<cv::Vec3f>* instance) {
			std::vector<cv::Vec3f> ret = std::vector<cv::Vec3f>(*instance);
			return new std::vector<cv::Vec3f>(ret);
	}
	
	const cv::Vec3f* std_vectorLcv_Vec3fG_data_const(const std::vector<cv::Vec3f>* instance) {
			const cv::Vec3f* ret = instance->data();
			return ret;
	}
	
	cv::Vec3f* std_vectorLcv_Vec3fG_dataMut(std::vector<cv::Vec3f>* instance) {
			cv::Vec3f* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Vec3f>* cv_fromSlice_const_const_Vec3fX_size_t(const cv::Vec3f* data, size_t len) {
			return new std::vector<cv::Vec3f>(data, data + len);
	}
	
	void std_vectorLcv_Vec3fG_inputArray_const(const std::vector<cv::Vec3f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec3fG_outputArray(std::vector<cv::Vec3f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec3fG_inputOutputArray(std::vector<cv::Vec3f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


