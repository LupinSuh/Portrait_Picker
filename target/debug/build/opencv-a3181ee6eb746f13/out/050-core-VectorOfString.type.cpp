extern "C" {
	std::vector<cv::String>* std_vectorLcv_StringG_new_const() {
			std::vector<cv::String>* ret = new std::vector<cv::String>();
			return ret;
	}
	
	void std_vectorLcv_StringG_delete(std::vector<cv::String>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_StringG_len_const(const std::vector<cv::String>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_StringG_isEmpty_const(const std::vector<cv::String>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_StringG_capacity_const(const std::vector<cv::String>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_StringG_shrinkToFit(std::vector<cv::String>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_StringG_reserve_size_t(std::vector<cv::String>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_StringG_remove_size_t(std::vector<cv::String>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_StringG_swap_size_t_size_t(std::vector<cv::String>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_StringG_clear(std::vector<cv::String>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_StringG_push_const_String(std::vector<cv::String>* instance, const char* val) {
			instance->push_back(cv::String(val));
	}
	
	void std_vectorLcv_StringG_insert_size_t_const_String(std::vector<cv::String>* instance, size_t index, const char* val) {
			instance->insert(instance->begin() + index, cv::String(val));
	}
	
	void std_vectorLcv_StringG_get_const_size_t(const std::vector<cv::String>* instance, size_t index, void** ocvrs_return) {
			cv::String ret = (*instance)[index];
			*ocvrs_return = ocvrs_create_string(ret.c_str());
	}
	
	void std_vectorLcv_StringG_set_size_t_const_String(std::vector<cv::String>* instance, size_t index, const char* val) {
			(*instance)[index] = cv::String(val);
	}
	
}


