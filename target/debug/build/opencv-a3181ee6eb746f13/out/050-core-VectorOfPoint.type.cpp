extern "C" {
	std::vector<cv::Point>* std_vectorLcv_PointG_new_const() {
			std::vector<cv::Point>* ret = new std::vector<cv::Point>();
			return ret;
	}
	
	void std_vectorLcv_PointG_delete(std::vector<cv::Point>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_PointG_len_const(const std::vector<cv::Point>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_PointG_isEmpty_const(const std::vector<cv::Point>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_PointG_capacity_const(const std::vector<cv::Point>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_PointG_shrinkToFit(std::vector<cv::Point>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_PointG_reserve_size_t(std::vector<cv::Point>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_PointG_remove_size_t(std::vector<cv::Point>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_PointG_swap_size_t_size_t(std::vector<cv::Point>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_PointG_clear(std::vector<cv::Point>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_PointG_push_const_Point(std::vector<cv::Point>* instance, const cv::Point* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_PointG_insert_size_t_const_Point(std::vector<cv::Point>* instance, size_t index, const cv::Point* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_PointG_get_const_size_t(const std::vector<cv::Point>* instance, size_t index, cv::Point* ocvrs_return) {
			cv::Point ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_PointG_set_size_t_const_Point(std::vector<cv::Point>* instance, size_t index, const cv::Point* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Point>* std_vectorLcv_PointG_clone_const(const std::vector<cv::Point>* instance) {
			std::vector<cv::Point> ret = std::vector<cv::Point>(*instance);
			return new std::vector<cv::Point>(ret);
	}
	
	const cv::Point* std_vectorLcv_PointG_data_const(const std::vector<cv::Point>* instance) {
			const cv::Point* ret = instance->data();
			return ret;
	}
	
	cv::Point* std_vectorLcv_PointG_dataMut(std::vector<cv::Point>* instance) {
			cv::Point* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Point>* cv_fromSlice_const_const_PointX_size_t(const cv::Point* data, size_t len) {
			return new std::vector<cv::Point>(data, data + len);
	}
	
	void std_vectorLcv_PointG_inputArray_const(const std::vector<cv::Point>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_PointG_outputArray(std::vector<cv::Point>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_PointG_inputOutputArray(std::vector<cv::Point>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


