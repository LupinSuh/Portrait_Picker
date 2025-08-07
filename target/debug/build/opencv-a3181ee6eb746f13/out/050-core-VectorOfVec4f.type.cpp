extern "C" {
	std::vector<cv::Vec4f>* std_vectorLcv_Vec4fG_new_const() {
			std::vector<cv::Vec4f>* ret = new std::vector<cv::Vec4f>();
			return ret;
	}
	
	void std_vectorLcv_Vec4fG_delete(std::vector<cv::Vec4f>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Vec4fG_len_const(const std::vector<cv::Vec4f>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Vec4fG_isEmpty_const(const std::vector<cv::Vec4f>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Vec4fG_capacity_const(const std::vector<cv::Vec4f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Vec4fG_shrinkToFit(std::vector<cv::Vec4f>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Vec4fG_reserve_size_t(std::vector<cv::Vec4f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Vec4fG_remove_size_t(std::vector<cv::Vec4f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Vec4fG_swap_size_t_size_t(std::vector<cv::Vec4f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Vec4fG_clear(std::vector<cv::Vec4f>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Vec4fG_push_const_Vec4f(std::vector<cv::Vec4f>* instance, const cv::Vec4f* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Vec4fG_insert_size_t_const_Vec4f(std::vector<cv::Vec4f>* instance, size_t index, const cv::Vec4f* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Vec4fG_get_const_size_t(const std::vector<cv::Vec4f>* instance, size_t index, cv::Vec4f* ocvrs_return) {
			cv::Vec4f ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Vec4fG_set_size_t_const_Vec4f(std::vector<cv::Vec4f>* instance, size_t index, const cv::Vec4f* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Vec4f>* std_vectorLcv_Vec4fG_clone_const(const std::vector<cv::Vec4f>* instance) {
			std::vector<cv::Vec4f> ret = std::vector<cv::Vec4f>(*instance);
			return new std::vector<cv::Vec4f>(ret);
	}
	
	const cv::Vec4f* std_vectorLcv_Vec4fG_data_const(const std::vector<cv::Vec4f>* instance) {
			const cv::Vec4f* ret = instance->data();
			return ret;
	}
	
	cv::Vec4f* std_vectorLcv_Vec4fG_dataMut(std::vector<cv::Vec4f>* instance) {
			cv::Vec4f* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Vec4f>* cv_fromSlice_const_const_Vec4fX_size_t(const cv::Vec4f* data, size_t len) {
			return new std::vector<cv::Vec4f>(data, data + len);
	}
	
	void std_vectorLcv_Vec4fG_inputArray_const(const std::vector<cv::Vec4f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec4fG_outputArray(std::vector<cv::Vec4f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec4fG_inputOutputArray(std::vector<cv::Vec4f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


