extern "C" {
	std::vector<cv::GTypeInfo>* std_vectorLcv_GTypeInfoG_new_const() {
			std::vector<cv::GTypeInfo>* ret = new std::vector<cv::GTypeInfo>();
			return ret;
	}
	
	void std_vectorLcv_GTypeInfoG_delete(std::vector<cv::GTypeInfo>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GTypeInfoG_len_const(const std::vector<cv::GTypeInfo>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GTypeInfoG_isEmpty_const(const std::vector<cv::GTypeInfo>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GTypeInfoG_capacity_const(const std::vector<cv::GTypeInfo>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GTypeInfoG_shrinkToFit(std::vector<cv::GTypeInfo>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GTypeInfoG_reserve_size_t(std::vector<cv::GTypeInfo>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GTypeInfoG_remove_size_t(std::vector<cv::GTypeInfo>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GTypeInfoG_swap_size_t_size_t(std::vector<cv::GTypeInfo>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GTypeInfoG_clear(std::vector<cv::GTypeInfo>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GTypeInfoG_push_const_GTypeInfo(std::vector<cv::GTypeInfo>* instance, const cv::GTypeInfo* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_GTypeInfoG_insert_size_t_const_GTypeInfo(std::vector<cv::GTypeInfo>* instance, size_t index, const cv::GTypeInfo* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_GTypeInfoG_get_const_size_t(const std::vector<cv::GTypeInfo>* instance, size_t index, cv::GTypeInfo** ocvrs_return) {
			cv::GTypeInfo ret = (*instance)[index];
			*ocvrs_return = new cv::GTypeInfo(ret);
	}
	
	void std_vectorLcv_GTypeInfoG_set_size_t_const_GTypeInfo(std::vector<cv::GTypeInfo>* instance, size_t index, const cv::GTypeInfo* val) {
			(*instance)[index] = *val;
	}
	
}


