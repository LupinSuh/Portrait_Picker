extern "C" {
	std::vector<cv::Vec4i>* std_vectorLcv_Vec4iG_new_const() {
			std::vector<cv::Vec4i>* ret = new std::vector<cv::Vec4i>();
			return ret;
	}
	
	void std_vectorLcv_Vec4iG_delete(std::vector<cv::Vec4i>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Vec4iG_len_const(const std::vector<cv::Vec4i>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Vec4iG_isEmpty_const(const std::vector<cv::Vec4i>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Vec4iG_capacity_const(const std::vector<cv::Vec4i>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Vec4iG_shrinkToFit(std::vector<cv::Vec4i>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Vec4iG_reserve_size_t(std::vector<cv::Vec4i>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Vec4iG_remove_size_t(std::vector<cv::Vec4i>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Vec4iG_swap_size_t_size_t(std::vector<cv::Vec4i>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Vec4iG_clear(std::vector<cv::Vec4i>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Vec4iG_push_const_Vec4i(std::vector<cv::Vec4i>* instance, const cv::Vec4i* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Vec4iG_insert_size_t_const_Vec4i(std::vector<cv::Vec4i>* instance, size_t index, const cv::Vec4i* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Vec4iG_get_const_size_t(const std::vector<cv::Vec4i>* instance, size_t index, cv::Vec4i* ocvrs_return) {
			cv::Vec4i ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Vec4iG_set_size_t_const_Vec4i(std::vector<cv::Vec4i>* instance, size_t index, const cv::Vec4i* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Vec4i>* std_vectorLcv_Vec4iG_clone_const(const std::vector<cv::Vec4i>* instance) {
			std::vector<cv::Vec4i> ret = std::vector<cv::Vec4i>(*instance);
			return new std::vector<cv::Vec4i>(ret);
	}
	
	const cv::Vec4i* std_vectorLcv_Vec4iG_data_const(const std::vector<cv::Vec4i>* instance) {
			const cv::Vec4i* ret = instance->data();
			return ret;
	}
	
	cv::Vec4i* std_vectorLcv_Vec4iG_dataMut(std::vector<cv::Vec4i>* instance) {
			cv::Vec4i* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Vec4i>* cv_fromSlice_const_const_Vec4iX_size_t(const cv::Vec4i* data, size_t len) {
			return new std::vector<cv::Vec4i>(data, data + len);
	}
	
	void std_vectorLcv_Vec4iG_inputArray_const(const std::vector<cv::Vec4i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec4iG_outputArray(std::vector<cv::Vec4i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec4iG_inputOutputArray(std::vector<cv::Vec4i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


