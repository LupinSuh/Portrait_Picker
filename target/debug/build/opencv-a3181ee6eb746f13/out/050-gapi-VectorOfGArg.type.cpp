extern "C" {
	std::vector<cv::GArg>* std_vectorLcv_GArgG_new_const() {
			std::vector<cv::GArg>* ret = new std::vector<cv::GArg>();
			return ret;
	}
	
	void std_vectorLcv_GArgG_delete(std::vector<cv::GArg>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GArgG_len_const(const std::vector<cv::GArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GArgG_isEmpty_const(const std::vector<cv::GArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GArgG_capacity_const(const std::vector<cv::GArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GArgG_shrinkToFit(std::vector<cv::GArg>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GArgG_reserve_size_t(std::vector<cv::GArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GArgG_remove_size_t(std::vector<cv::GArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GArgG_swap_size_t_size_t(std::vector<cv::GArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GArgG_clear(std::vector<cv::GArg>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GArgG_push_const_GArg(std::vector<cv::GArg>* instance, const cv::GArg* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_GArgG_insert_size_t_const_GArg(std::vector<cv::GArg>* instance, size_t index, const cv::GArg* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_GArgG_get_const_size_t(const std::vector<cv::GArg>* instance, size_t index, cv::GArg** ocvrs_return) {
			cv::GArg ret = (*instance)[index];
			*ocvrs_return = new cv::GArg(ret);
	}
	
	void std_vectorLcv_GArgG_set_size_t_const_GArg(std::vector<cv::GArg>* instance, size_t index, const cv::GArg* val) {
			(*instance)[index] = *val;
	}
	
}


