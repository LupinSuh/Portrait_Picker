extern "C" {
	std::vector<double>* std_vectorLdoubleG_new_const() {
			std::vector<double>* ret = new std::vector<double>();
			return ret;
	}
	
	void std_vectorLdoubleG_delete(std::vector<double>* instance) {
			delete instance;
	}
	
	size_t std_vectorLdoubleG_len_const(const std::vector<double>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLdoubleG_isEmpty_const(const std::vector<double>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLdoubleG_capacity_const(const std::vector<double>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLdoubleG_shrinkToFit(std::vector<double>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLdoubleG_reserve_size_t(std::vector<double>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLdoubleG_remove_size_t(std::vector<double>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLdoubleG_swap_size_t_size_t(std::vector<double>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLdoubleG_clear(std::vector<double>* instance) {
			instance->clear();
	}
	
	void std_vectorLdoubleG_push_const_double(std::vector<double>* instance, const double val) {
			instance->push_back(val);
	}
	
	void std_vectorLdoubleG_insert_size_t_const_double(std::vector<double>* instance, size_t index, const double val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLdoubleG_get_const_size_t(const std::vector<double>* instance, size_t index, double* ocvrs_return) {
			double ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLdoubleG_set_size_t_const_double(std::vector<double>* instance, size_t index, const double val) {
			(*instance)[index] = val;
	}
	
	std::vector<double>* std_vectorLdoubleG_clone_const(const std::vector<double>* instance) {
			std::vector<double> ret = std::vector<double>(*instance);
			return new std::vector<double>(ret);
	}
	
	const double* std_vectorLdoubleG_data_const(const std::vector<double>* instance) {
			const double* ret = instance->data();
			return ret;
	}
	
	double* std_vectorLdoubleG_dataMut(std::vector<double>* instance) {
			double* ret = instance->data();
			return ret;
	}
	
	std::vector<double>* cv_fromSlice_const_const_doubleX_size_t(const double* data, size_t len) {
			return new std::vector<double>(data, data + len);
	}
	
	void std_vectorLdoubleG_inputArray_const(const std::vector<double>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLdoubleG_outputArray(std::vector<double>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLdoubleG_inputOutputArray(std::vector<double>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


