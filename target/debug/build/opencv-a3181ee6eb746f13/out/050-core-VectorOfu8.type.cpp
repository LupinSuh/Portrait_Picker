extern "C" {
	std::vector<unsigned char>* std_vectorLunsigned_charG_new_const() {
			std::vector<unsigned char>* ret = new std::vector<unsigned char>();
			return ret;
	}
	
	void std_vectorLunsigned_charG_delete(std::vector<unsigned char>* instance) {
			delete instance;
	}
	
	size_t std_vectorLunsigned_charG_len_const(const std::vector<unsigned char>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLunsigned_charG_isEmpty_const(const std::vector<unsigned char>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLunsigned_charG_capacity_const(const std::vector<unsigned char>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLunsigned_charG_shrinkToFit(std::vector<unsigned char>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLunsigned_charG_reserve_size_t(std::vector<unsigned char>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLunsigned_charG_remove_size_t(std::vector<unsigned char>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLunsigned_charG_swap_size_t_size_t(std::vector<unsigned char>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLunsigned_charG_clear(std::vector<unsigned char>* instance) {
			instance->clear();
	}
	
	void std_vectorLunsigned_charG_push_const_unsigned_char(std::vector<unsigned char>* instance, const unsigned char val) {
			instance->push_back(val);
	}
	
	void std_vectorLunsigned_charG_insert_size_t_const_unsigned_char(std::vector<unsigned char>* instance, size_t index, const unsigned char val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLunsigned_charG_get_const_size_t(const std::vector<unsigned char>* instance, size_t index, unsigned char* ocvrs_return) {
			unsigned char ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLunsigned_charG_set_size_t_const_unsigned_char(std::vector<unsigned char>* instance, size_t index, const unsigned char val) {
			(*instance)[index] = val;
	}
	
	std::vector<unsigned char>* std_vectorLunsigned_charG_clone_const(const std::vector<unsigned char>* instance) {
			std::vector<unsigned char> ret = std::vector<unsigned char>(*instance);
			return new std::vector<unsigned char>(ret);
	}
	
	const unsigned char* std_vectorLunsigned_charG_data_const(const std::vector<unsigned char>* instance) {
			const unsigned char* ret = instance->data();
			return ret;
	}
	
	unsigned char* std_vectorLunsigned_charG_dataMut(std::vector<unsigned char>* instance) {
			unsigned char* ret = instance->data();
			return ret;
	}
	
	std::vector<unsigned char>* cv_fromSlice_const_const_unsigned_charX_size_t(const unsigned char* data, size_t len) {
			return new std::vector<unsigned char>(data, data + len);
	}
	
	void std_vectorLunsigned_charG_inputArray_const(const std::vector<unsigned char>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLunsigned_charG_outputArray(std::vector<unsigned char>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLunsigned_charG_inputOutputArray(std::vector<unsigned char>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


