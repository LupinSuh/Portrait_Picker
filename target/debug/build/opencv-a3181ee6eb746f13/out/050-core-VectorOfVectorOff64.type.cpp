extern "C" {
	std::vector<std::vector<double>>* std_vectorLstd_vectorLdoubleGG_new_const() {
			std::vector<std::vector<double>>* ret = new std::vector<std::vector<double>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLdoubleGG_delete(std::vector<std::vector<double>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLdoubleGG_len_const(const std::vector<std::vector<double>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLdoubleGG_isEmpty_const(const std::vector<std::vector<double>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLdoubleGG_capacity_const(const std::vector<std::vector<double>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLdoubleGG_shrinkToFit(std::vector<std::vector<double>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLdoubleGG_reserve_size_t(std::vector<std::vector<double>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLdoubleGG_remove_size_t(std::vector<std::vector<double>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLdoubleGG_swap_size_t_size_t(std::vector<std::vector<double>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLdoubleGG_clear(std::vector<std::vector<double>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLdoubleGG_push_const_vectorLdoubleG(std::vector<std::vector<double>>* instance, const std::vector<double>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLdoubleGG_insert_size_t_const_vectorLdoubleG(std::vector<std::vector<double>>* instance, size_t index, const std::vector<double>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLdoubleGG_get_const_size_t(const std::vector<std::vector<double>>* instance, size_t index, std::vector<double>** ocvrs_return) {
			std::vector<double> ret = (*instance)[index];
			*ocvrs_return = new std::vector<double>(ret);
	}
	
	void std_vectorLstd_vectorLdoubleGG_set_size_t_const_vectorLdoubleG(std::vector<std::vector<double>>* instance, size_t index, const std::vector<double>* val) {
			(*instance)[index] = *val;
	}
	
	void std_vectorLstd_vectorLdoubleGG_inputArray_const(const std::vector<std::vector<double>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLdoubleGG_outputArray(std::vector<std::vector<double>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLdoubleGG_inputOutputArray(std::vector<std::vector<double>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


