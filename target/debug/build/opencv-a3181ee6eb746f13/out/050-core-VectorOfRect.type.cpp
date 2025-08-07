extern "C" {
	std::vector<cv::Rect>* std_vectorLcv_RectG_new_const() {
			std::vector<cv::Rect>* ret = new std::vector<cv::Rect>();
			return ret;
	}
	
	void std_vectorLcv_RectG_delete(std::vector<cv::Rect>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_RectG_len_const(const std::vector<cv::Rect>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_RectG_isEmpty_const(const std::vector<cv::Rect>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_RectG_capacity_const(const std::vector<cv::Rect>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_RectG_shrinkToFit(std::vector<cv::Rect>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_RectG_reserve_size_t(std::vector<cv::Rect>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_RectG_remove_size_t(std::vector<cv::Rect>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_RectG_swap_size_t_size_t(std::vector<cv::Rect>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_RectG_clear(std::vector<cv::Rect>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_RectG_push_const_Rect(std::vector<cv::Rect>* instance, const cv::Rect* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_RectG_insert_size_t_const_Rect(std::vector<cv::Rect>* instance, size_t index, const cv::Rect* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_RectG_get_const_size_t(const std::vector<cv::Rect>* instance, size_t index, cv::Rect* ocvrs_return) {
			cv::Rect ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_RectG_set_size_t_const_Rect(std::vector<cv::Rect>* instance, size_t index, const cv::Rect* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Rect>* std_vectorLcv_RectG_clone_const(const std::vector<cv::Rect>* instance) {
			std::vector<cv::Rect> ret = std::vector<cv::Rect>(*instance);
			return new std::vector<cv::Rect>(ret);
	}
	
	const cv::Rect* std_vectorLcv_RectG_data_const(const std::vector<cv::Rect>* instance) {
			const cv::Rect* ret = instance->data();
			return ret;
	}
	
	cv::Rect* std_vectorLcv_RectG_dataMut(std::vector<cv::Rect>* instance) {
			cv::Rect* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Rect>* cv_fromSlice_const_const_RectX_size_t(const cv::Rect* data, size_t len) {
			return new std::vector<cv::Rect>(data, data + len);
	}
	
	void std_vectorLcv_RectG_inputArray_const(const std::vector<cv::Rect>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_RectG_outputArray(std::vector<cv::Rect>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_RectG_inputOutputArray(std::vector<cv::Rect>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


