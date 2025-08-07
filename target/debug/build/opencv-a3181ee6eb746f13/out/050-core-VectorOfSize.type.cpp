extern "C" {
	std::vector<cv::Size>* std_vectorLcv_SizeG_new_const() {
			std::vector<cv::Size>* ret = new std::vector<cv::Size>();
			return ret;
	}
	
	void std_vectorLcv_SizeG_delete(std::vector<cv::Size>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_SizeG_len_const(const std::vector<cv::Size>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_SizeG_isEmpty_const(const std::vector<cv::Size>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_SizeG_capacity_const(const std::vector<cv::Size>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_SizeG_shrinkToFit(std::vector<cv::Size>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_SizeG_reserve_size_t(std::vector<cv::Size>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_SizeG_remove_size_t(std::vector<cv::Size>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_SizeG_swap_size_t_size_t(std::vector<cv::Size>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_SizeG_clear(std::vector<cv::Size>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_SizeG_push_const_Size(std::vector<cv::Size>* instance, const cv::Size* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_SizeG_insert_size_t_const_Size(std::vector<cv::Size>* instance, size_t index, const cv::Size* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_SizeG_get_const_size_t(const std::vector<cv::Size>* instance, size_t index, cv::Size* ocvrs_return) {
			cv::Size ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_SizeG_set_size_t_const_Size(std::vector<cv::Size>* instance, size_t index, const cv::Size* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Size>* std_vectorLcv_SizeG_clone_const(const std::vector<cv::Size>* instance) {
			std::vector<cv::Size> ret = std::vector<cv::Size>(*instance);
			return new std::vector<cv::Size>(ret);
	}
	
	const cv::Size* std_vectorLcv_SizeG_data_const(const std::vector<cv::Size>* instance) {
			const cv::Size* ret = instance->data();
			return ret;
	}
	
	cv::Size* std_vectorLcv_SizeG_dataMut(std::vector<cv::Size>* instance) {
			cv::Size* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Size>* cv_fromSlice_const_const_SizeX_size_t(const cv::Size* data, size_t len) {
			return new std::vector<cv::Size>(data, data + len);
	}
	
	void std_vectorLcv_SizeG_inputArray_const(const std::vector<cv::Size>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_SizeG_outputArray(std::vector<cv::Size>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_SizeG_inputOutputArray(std::vector<cv::Size>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


