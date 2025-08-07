extern "C" {
	std::vector<cv::Vec3i>* std_vectorLcv_Vec3iG_new_const() {
			std::vector<cv::Vec3i>* ret = new std::vector<cv::Vec3i>();
			return ret;
	}
	
	void std_vectorLcv_Vec3iG_delete(std::vector<cv::Vec3i>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Vec3iG_len_const(const std::vector<cv::Vec3i>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Vec3iG_isEmpty_const(const std::vector<cv::Vec3i>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Vec3iG_capacity_const(const std::vector<cv::Vec3i>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Vec3iG_shrinkToFit(std::vector<cv::Vec3i>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Vec3iG_reserve_size_t(std::vector<cv::Vec3i>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Vec3iG_remove_size_t(std::vector<cv::Vec3i>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Vec3iG_swap_size_t_size_t(std::vector<cv::Vec3i>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Vec3iG_clear(std::vector<cv::Vec3i>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Vec3iG_push_const_Vec3i(std::vector<cv::Vec3i>* instance, const cv::Vec3i* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Vec3iG_insert_size_t_const_Vec3i(std::vector<cv::Vec3i>* instance, size_t index, const cv::Vec3i* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Vec3iG_get_const_size_t(const std::vector<cv::Vec3i>* instance, size_t index, cv::Vec3i* ocvrs_return) {
			cv::Vec3i ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Vec3iG_set_size_t_const_Vec3i(std::vector<cv::Vec3i>* instance, size_t index, const cv::Vec3i* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Vec3i>* std_vectorLcv_Vec3iG_clone_const(const std::vector<cv::Vec3i>* instance) {
			std::vector<cv::Vec3i> ret = std::vector<cv::Vec3i>(*instance);
			return new std::vector<cv::Vec3i>(ret);
	}
	
	const cv::Vec3i* std_vectorLcv_Vec3iG_data_const(const std::vector<cv::Vec3i>* instance) {
			const cv::Vec3i* ret = instance->data();
			return ret;
	}
	
	cv::Vec3i* std_vectorLcv_Vec3iG_dataMut(std::vector<cv::Vec3i>* instance) {
			cv::Vec3i* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Vec3i>* cv_fromSlice_const_const_Vec3iX_size_t(const cv::Vec3i* data, size_t len) {
			return new std::vector<cv::Vec3i>(data, data + len);
	}
	
	void std_vectorLcv_Vec3iG_inputArray_const(const std::vector<cv::Vec3i>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec3iG_outputArray(std::vector<cv::Vec3i>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Vec3iG_inputOutputArray(std::vector<cv::Vec3i>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


