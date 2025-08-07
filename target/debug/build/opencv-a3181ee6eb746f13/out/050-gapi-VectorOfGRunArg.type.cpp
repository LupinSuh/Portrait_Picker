extern "C" {
	std::vector<cv::GRunArg>* std_vectorLcv_GRunArgG_new_const() {
			std::vector<cv::GRunArg>* ret = new std::vector<cv::GRunArg>();
			return ret;
	}
	
	void std_vectorLcv_GRunArgG_delete(std::vector<cv::GRunArg>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GRunArgG_len_const(const std::vector<cv::GRunArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GRunArgG_isEmpty_const(const std::vector<cv::GRunArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GRunArgG_capacity_const(const std::vector<cv::GRunArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GRunArgG_shrinkToFit(std::vector<cv::GRunArg>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GRunArgG_reserve_size_t(std::vector<cv::GRunArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GRunArgG_remove_size_t(std::vector<cv::GRunArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GRunArgG_swap_size_t_size_t(std::vector<cv::GRunArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GRunArgG_clear(std::vector<cv::GRunArg>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GRunArgG_push_const_GRunArg(std::vector<cv::GRunArg>* instance, const cv::GRunArg* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_GRunArgG_insert_size_t_const_GRunArg(std::vector<cv::GRunArg>* instance, size_t index, const cv::GRunArg* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_GRunArgG_get_const_size_t(const std::vector<cv::GRunArg>* instance, size_t index, cv::GRunArg** ocvrs_return) {
			cv::GRunArg ret = (*instance)[index];
			*ocvrs_return = new cv::GRunArg(ret);
	}
	
	void std_vectorLcv_GRunArgG_set_size_t_const_GRunArg(std::vector<cv::GRunArg>* instance, size_t index, const cv::GRunArg* val) {
			(*instance)[index] = *val;
	}
	
}


