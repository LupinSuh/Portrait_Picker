extern "C" {
	std::vector<cv::Vec2f>* std_vectorLcv_Vec2fG_new_const() {
			std::vector<cv::Vec2f>* ret = new std::vector<cv::Vec2f>();
			return ret;
	}
	
	void std_vectorLcv_Vec2fG_delete(std::vector<cv::Vec2f>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Vec2fG_len_const(const std::vector<cv::Vec2f>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Vec2fG_isEmpty_const(const std::vector<cv::Vec2f>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Vec2fG_capacity_const(const std::vector<cv::Vec2f>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Vec2fG_shrinkToFit(std::vector<cv::Vec2f>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Vec2fG_reserve_size_t(std::vector<cv::Vec2f>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Vec2fG_remove_size_t(std::vector<cv::Vec2f>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Vec2fG_swap_size_t_size_t(std::vector<cv::Vec2f>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Vec2fG_clear(std::vector<cv::Vec2f>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Vec2fG_push_const_Vec2f(std::vector<cv::Vec2f>* instance, const cv::Vec2f* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Vec2fG_insert_size_t_const_Vec2f(std::vector<cv::Vec2f>* instance, size_t index, const cv::Vec2f* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Vec2fG_get_const_size_t(const std::vector<cv::Vec2f>* instance, size_t index, cv::Vec2f* ocvrs_return) {
			cv::Vec2f ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Vec2fG_set_size_t_const_Vec2f(std::vector<cv::Vec2f>* instance, size_t index, const cv::Vec2f* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Vec2f>* std_vectorLcv_Vec2fG_clone_const(const std::vector<cv::Vec2f>* instance) {
			std::vector<cv::Vec2f> ret = std::vector<cv::Vec2f>(*instance);
			return new std::vector<cv::Vec2f>(ret);
	}
	
	const cv::Vec2f* std_vectorLcv_Vec2fG_data_const(const std::vector<cv::Vec2f>* instance) {
			const cv::Vec2f* ret = instance->data();
			return ret;
	}
	
	cv::Vec2f* std_vectorLcv_Vec2fG_dataMut(std::vector<cv::Vec2f>* instance) {
			cv::Vec2f* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Vec2f>* cv_fromSlice_const_const_Vec2fX_size_t(const cv::Vec2f* data, size_t len) {
			return new std::vector<cv::Vec2f>(data, data + len);
	}
	
	void std_vectorLcv_Vec2fG_inputArray_const(const std::vector<cv::Vec2f>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec2fG_outputArray(std::vector<cv::Vec2f>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec2fG_inputOutputArray(std::vector<cv::Vec2f>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


