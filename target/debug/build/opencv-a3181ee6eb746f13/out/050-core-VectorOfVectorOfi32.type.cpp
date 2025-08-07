extern "C" {
	std::vector<std::vector<int>>* std_vectorLstd_vectorLintGG_new_const() {
			std::vector<std::vector<int>>* ret = new std::vector<std::vector<int>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLintGG_delete(std::vector<std::vector<int>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLintGG_len_const(const std::vector<std::vector<int>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLintGG_isEmpty_const(const std::vector<std::vector<int>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLintGG_capacity_const(const std::vector<std::vector<int>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLintGG_shrinkToFit(std::vector<std::vector<int>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLintGG_reserve_size_t(std::vector<std::vector<int>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLintGG_remove_size_t(std::vector<std::vector<int>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLintGG_swap_size_t_size_t(std::vector<std::vector<int>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLintGG_clear(std::vector<std::vector<int>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLintGG_push_const_vectorLintG(std::vector<std::vector<int>>* instance, const std::vector<int>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLintGG_insert_size_t_const_vectorLintG(std::vector<std::vector<int>>* instance, size_t index, const std::vector<int>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLintGG_get_const_size_t(const std::vector<std::vector<int>>* instance, size_t index, std::vector<int>** ocvrs_return) {
			std::vector<int> ret = (*instance)[index];
			*ocvrs_return = new std::vector<int>(ret);
	}
	
	void std_vectorLstd_vectorLintGG_set_size_t_const_vectorLintG(std::vector<std::vector<int>>* instance, size_t index, const std::vector<int>* val) {
			(*instance)[index] = *val;
	}
	
	void std_vectorLstd_vectorLintGG_inputArray_const(const std::vector<std::vector<int>>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLintGG_outputArray(std::vector<std::vector<int>>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLstd_vectorLintGG_inputOutputArray(std::vector<std::vector<int>>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


