extern "C" {
	std::vector<int>* std_vectorLintG_new_const() {
			std::vector<int>* ret = new std::vector<int>();
			return ret;
	}
	
	void std_vectorLintG_delete(std::vector<int>* instance) {
			delete instance;
	}
	
	size_t std_vectorLintG_len_const(const std::vector<int>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLintG_isEmpty_const(const std::vector<int>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLintG_capacity_const(const std::vector<int>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLintG_shrinkToFit(std::vector<int>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLintG_reserve_size_t(std::vector<int>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLintG_remove_size_t(std::vector<int>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLintG_swap_size_t_size_t(std::vector<int>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLintG_clear(std::vector<int>* instance) {
			instance->clear();
	}
	
	void std_vectorLintG_push_const_int(std::vector<int>* instance, const int val) {
			instance->push_back(val);
	}
	
	void std_vectorLintG_insert_size_t_const_int(std::vector<int>* instance, size_t index, const int val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLintG_get_const_size_t(const std::vector<int>* instance, size_t index, int* ocvrs_return) {
			int ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLintG_set_size_t_const_int(std::vector<int>* instance, size_t index, const int val) {
			(*instance)[index] = val;
	}
	
	std::vector<int>* std_vectorLintG_clone_const(const std::vector<int>* instance) {
			std::vector<int> ret = std::vector<int>(*instance);
			return new std::vector<int>(ret);
	}
	
	const int* std_vectorLintG_data_const(const std::vector<int>* instance) {
			const int* ret = instance->data();
			return ret;
	}
	
	int* std_vectorLintG_dataMut(std::vector<int>* instance) {
			int* ret = instance->data();
			return ret;
	}
	
	std::vector<int>* cv_fromSlice_const_const_intX_size_t(const int* data, size_t len) {
			return new std::vector<int>(data, data + len);
	}
	
	void std_vectorLintG_inputArray_const(const std::vector<int>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLintG_outputArray(std::vector<int>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLintG_inputOutputArray(std::vector<int>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


