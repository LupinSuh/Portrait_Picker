extern "C" {
	std::vector<cv::Point3i>* std_vectorLcv_Point3iG_new_const() {
			std::vector<cv::Point3i>* ret = new std::vector<cv::Point3i>();
			return ret;
	}
	
	void std_vectorLcv_Point3iG_delete(std::vector<cv::Point3i>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Point3iG_len_const(const std::vector<cv::Point3i>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Point3iG_isEmpty_const(const std::vector<cv::Point3i>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Point3iG_capacity_const(const std::vector<cv::Point3i>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Point3iG_shrinkToFit(std::vector<cv::Point3i>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Point3iG_reserve_size_t(std::vector<cv::Point3i>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Point3iG_remove_size_t(std::vector<cv::Point3i>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Point3iG_swap_size_t_size_t(std::vector<cv::Point3i>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Point3iG_clear(std::vector<cv::Point3i>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Point3iG_push_const_Point3i(std::vector<cv::Point3i>* instance, const cv::Point3i* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Point3iG_insert_size_t_const_Point3i(std::vector<cv::Point3i>* instance, size_t index, const cv::Point3i* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Point3iG_get_const_size_t(const std::vector<cv::Point3i>* instance, size_t index, cv::Point3i* ocvrs_return) {
			cv::Point3i ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Point3iG_set_size_t_const_Point3i(std::vector<cv::Point3i>* instance, size_t index, const cv::Point3i* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Point3i>* std_vectorLcv_Point3iG_clone_const(const std::vector<cv::Point3i>* instance) {
			std::vector<cv::Point3i> ret = std::vector<cv::Point3i>(*instance);
			return new std::vector<cv::Point3i>(ret);
	}
	
	const cv::Point3i* std_vectorLcv_Point3iG_data_const(const std::vector<cv::Point3i>* instance) {
			const cv::Point3i* ret = instance->data();
			return ret;
	}
	
	cv::Point3i* std_vectorLcv_Point3iG_dataMut(std::vector<cv::Point3i>* instance) {
			cv::Point3i* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Point3i>* cv_fromSlice_const_const_Point3iX_size_t(const cv::Point3i* data, size_t len) {
			return new std::vector<cv::Point3i>(data, data + len);
	}
	
	void std_vectorLcv_Point3iG_inputArray_const(const std::vector<cv::Point3i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Point3iG_outputArray(std::vector<cv::Point3i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Point3iG_inputOutputArray(std::vector<cv::Point3i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


