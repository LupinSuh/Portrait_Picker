extern "C" {
	std::vector<cv::Scalar>* std_vectorLcv_ScalarG_new_const() {
			std::vector<cv::Scalar>* ret = new std::vector<cv::Scalar>();
			return ret;
	}
	
	void std_vectorLcv_ScalarG_delete(std::vector<cv::Scalar>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_ScalarG_len_const(const std::vector<cv::Scalar>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_ScalarG_isEmpty_const(const std::vector<cv::Scalar>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_ScalarG_capacity_const(const std::vector<cv::Scalar>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_ScalarG_shrinkToFit(std::vector<cv::Scalar>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_ScalarG_reserve_size_t(std::vector<cv::Scalar>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_ScalarG_remove_size_t(std::vector<cv::Scalar>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_ScalarG_swap_size_t_size_t(std::vector<cv::Scalar>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_ScalarG_clear(std::vector<cv::Scalar>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_ScalarG_push_const_Scalar(std::vector<cv::Scalar>* instance, const cv::Scalar* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_ScalarG_insert_size_t_const_Scalar(std::vector<cv::Scalar>* instance, size_t index, const cv::Scalar* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_ScalarG_get_const_size_t(const std::vector<cv::Scalar>* instance, size_t index, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_ScalarG_set_size_t_const_Scalar(std::vector<cv::Scalar>* instance, size_t index, const cv::Scalar* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Scalar>* std_vectorLcv_ScalarG_clone_const(const std::vector<cv::Scalar>* instance) {
			std::vector<cv::Scalar> ret = std::vector<cv::Scalar>(*instance);
			return new std::vector<cv::Scalar>(ret);
	}
	
	const cv::Scalar* std_vectorLcv_ScalarG_data_const(const std::vector<cv::Scalar>* instance) {
			const cv::Scalar* ret = instance->data();
			return ret;
	}
	
	cv::Scalar* std_vectorLcv_ScalarG_dataMut(std::vector<cv::Scalar>* instance) {
			cv::Scalar* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Scalar>* cv_fromSlice_const_const_ScalarX_size_t(const cv::Scalar* data, size_t len) {
			return new std::vector<cv::Scalar>(data, data + len);
	}
	
	void std_vectorLcv_ScalarG_inputArray_const(const std::vector<cv::Scalar>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_ScalarG_outputArray(std::vector<cv::Scalar>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_ScalarG_inputOutputArray(std::vector<cv::Scalar>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


