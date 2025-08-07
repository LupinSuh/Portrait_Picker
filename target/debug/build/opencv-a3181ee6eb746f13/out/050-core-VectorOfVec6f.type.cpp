extern "C" {
	std::vector<cv::Vec6f>* std_vectorLcv_Vec6fG_new_const() {
			std::vector<cv::Vec6f>* ret = new std::vector<cv::Vec6f>();
			return ret;
	}
	
	void std_vectorLcv_Vec6fG_delete(std::vector<cv::Vec6f>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Vec6fG_len_const(const std::vector<cv::Vec6f>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Vec6fG_isEmpty_const(const std::vector<cv::Vec6f>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Vec6fG_capacity_const(const std::vector<cv::Vec6f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Vec6fG_shrinkToFit(std::vector<cv::Vec6f>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Vec6fG_reserve_size_t(std::vector<cv::Vec6f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Vec6fG_remove_size_t(std::vector<cv::Vec6f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Vec6fG_swap_size_t_size_t(std::vector<cv::Vec6f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Vec6fG_clear(std::vector<cv::Vec6f>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Vec6fG_push_const_Vec6f(std::vector<cv::Vec6f>* instance, const cv::Vec6f* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Vec6fG_insert_size_t_const_Vec6f(std::vector<cv::Vec6f>* instance, size_t index, const cv::Vec6f* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Vec6fG_get_const_size_t(const std::vector<cv::Vec6f>* instance, size_t index, cv::Vec6f* ocvrs_return) {
			cv::Vec6f ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Vec6fG_set_size_t_const_Vec6f(std::vector<cv::Vec6f>* instance, size_t index, const cv::Vec6f* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Vec6f>* std_vectorLcv_Vec6fG_clone_const(const std::vector<cv::Vec6f>* instance) {
			std::vector<cv::Vec6f> ret = std::vector<cv::Vec6f>(*instance);
			return new std::vector<cv::Vec6f>(ret);
	}
	
	const cv::Vec6f* std_vectorLcv_Vec6fG_data_const(const std::vector<cv::Vec6f>* instance) {
			const cv::Vec6f* ret = instance->data();
			return ret;
	}
	
	cv::Vec6f* std_vectorLcv_Vec6fG_dataMut(std::vector<cv::Vec6f>* instance) {
			cv::Vec6f* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Vec6f>* cv_fromSlice_const_const_Vec6fX_size_t(const cv::Vec6f* data, size_t len) {
			return new std::vector<cv::Vec6f>(data, data + len);
	}
	
	void std_vectorLcv_Vec6fG_inputArray_const(const std::vector<cv::Vec6f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec6fG_outputArray(std::vector<cv::Vec6f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec6fG_inputOutputArray(std::vector<cv::Vec6f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


