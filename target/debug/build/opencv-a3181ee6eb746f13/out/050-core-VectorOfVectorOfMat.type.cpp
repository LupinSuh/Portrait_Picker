extern "C" {
	std::vector<std::vector<cv::Mat>>* std_vectorLstd_vectorLcv_MatGG_new_const() {
			std::vector<std::vector<cv::Mat>>* ret = new std::vector<std::vector<cv::Mat>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_MatGG_delete(std::vector<std::vector<cv::Mat>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_MatGG_len_const(const std::vector<std::vector<cv::Mat>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_MatGG_isEmpty_const(const std::vector<std::vector<cv::Mat>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_MatGG_capacity_const(const std::vector<std::vector<cv::Mat>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_MatGG_shrinkToFit(std::vector<std::vector<cv::Mat>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_MatGG_reserve_size_t(std::vector<std::vector<cv::Mat>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_MatGG_remove_size_t(std::vector<std::vector<cv::Mat>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_MatGG_swap_size_t_size_t(std::vector<std::vector<cv::Mat>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_MatGG_clear(std::vector<std::vector<cv::Mat>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_MatGG_push_const_vectorLMatG(std::vector<std::vector<cv::Mat>>* instance, const std::vector<cv::Mat>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_MatGG_insert_size_t_const_vectorLMatG(std::vector<std::vector<cv::Mat>>* instance, size_t index, const std::vector<cv::Mat>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_MatGG_get_const_size_t(const std::vector<std::vector<cv::Mat>>* instance, size_t index, std::vector<cv::Mat>** ocvrs_return) {
			std::vector<cv::Mat> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::Mat>(ret);
	}
	
	void std_vectorLstd_vectorLcv_MatGG_set_size_t_const_vectorLMatG(std::vector<std::vector<cv::Mat>>* instance, size_t index, const std::vector<cv::Mat>* val) {
			(*instance)[index] = *val;
	}
	
}


