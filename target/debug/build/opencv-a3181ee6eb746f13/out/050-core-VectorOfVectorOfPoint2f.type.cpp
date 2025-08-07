extern "C" {
	std::vector<std::vector<cv::Point2f>>* std_vectorLstd_vectorLcv_Point2fGG_new_const() {
			std::vector<std::vector<cv::Point2f>>* ret = new std::vector<std::vector<cv::Point2f>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_delete(std::vector<std::vector<cv::Point2f>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_Point2fGG_len_const(const std::vector<std::vector<cv::Point2f>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_Point2fGG_isEmpty_const(const std::vector<std::vector<cv::Point2f>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_Point2fGG_capacity_const(const std::vector<std::vector<cv::Point2f>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_shrinkToFit(std::vector<std::vector<cv::Point2f>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_reserve_size_t(std::vector<std::vector<cv::Point2f>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_remove_size_t(std::vector<std::vector<cv::Point2f>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_swap_size_t_size_t(std::vector<std::vector<cv::Point2f>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_clear(std::vector<std::vector<cv::Point2f>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_push_const_vectorLPoint2fG(std::vector<std::vector<cv::Point2f>>* instance, const std::vector<cv::Point2f>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_insert_size_t_const_vectorLPoint2fG(std::vector<std::vector<cv::Point2f>>* instance, size_t index, const std::vector<cv::Point2f>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_get_const_size_t(const std::vector<std::vector<cv::Point2f>>* instance, size_t index, std::vector<cv::Point2f>** ocvrs_return) {
			std::vector<cv::Point2f> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Point2f>(ret);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_set_size_t_const_vectorLPoint2fG(std::vector<std::vector<cv::Point2f>>* instance, size_t index, const std::vector<cv::Point2f>* val) {
			(*instance)[index] = *val;
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_inputArray_const(const std::vector<std::vector<cv::Point2f>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_outputArray(std::vector<std::vector<cv::Point2f>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLcv_Point2fGG_inputOutputArray(std::vector<std::vector<cv::Point2f>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


