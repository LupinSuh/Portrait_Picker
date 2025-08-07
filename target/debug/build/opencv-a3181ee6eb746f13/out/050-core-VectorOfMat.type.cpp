extern "C" {
	std::vector<cv::Mat>* std_vectorLcv_MatG_new_const() {
			std::vector<cv::Mat>* ret = new std::vector<cv::Mat>();
			return ret;
	}
	
	void std_vectorLcv_MatG_delete(std::vector<cv::Mat>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_MatG_len_const(const std::vector<cv::Mat>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_MatG_isEmpty_const(const std::vector<cv::Mat>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_MatG_capacity_const(const std::vector<cv::Mat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_MatG_shrinkToFit(std::vector<cv::Mat>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_MatG_reserve_size_t(std::vector<cv::Mat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_MatG_remove_size_t(std::vector<cv::Mat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_MatG_swap_size_t_size_t(std::vector<cv::Mat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_MatG_clear(std::vector<cv::Mat>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_MatG_push_const_Mat(std::vector<cv::Mat>* instance, const cv::Mat* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_MatG_insert_size_t_const_Mat(std::vector<cv::Mat>* instance, size_t index, const cv::Mat* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_MatG_get_const_size_t(const std::vector<cv::Mat>* instance, size_t index, cv::Mat** ocvrs_return) {
			cv::Mat ret = (*instance)[index];
			*ocvrs_return = new cv::Mat(ret);
	}
	
	void std_vectorLcv_MatG_set_size_t_const_Mat(std::vector<cv::Mat>* instance, size_t index, const cv::Mat* val) {
			(*instance)[index] = *val;
	}
	
}


