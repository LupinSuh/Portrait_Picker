extern "C" {
	std::vector<std::vector<cv::DMatch>>* std_vectorLstd_vectorLcv_DMatchGG_new_const() {
			std::vector<std::vector<cv::DMatch>>* ret = new std::vector<std::vector<cv::DMatch>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_delete(std::vector<std::vector<cv::DMatch>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_DMatchGG_len_const(const std::vector<std::vector<cv::DMatch>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_DMatchGG_isEmpty_const(const std::vector<std::vector<cv::DMatch>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_DMatchGG_capacity_const(const std::vector<std::vector<cv::DMatch>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_shrinkToFit(std::vector<std::vector<cv::DMatch>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_reserve_size_t(std::vector<std::vector<cv::DMatch>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_remove_size_t(std::vector<std::vector<cv::DMatch>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_swap_size_t_size_t(std::vector<std::vector<cv::DMatch>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_clear(std::vector<std::vector<cv::DMatch>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_push_const_vectorLDMatchG(std::vector<std::vector<cv::DMatch>>* instance, const std::vector<cv::DMatch>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_insert_size_t_const_vectorLDMatchG(std::vector<std::vector<cv::DMatch>>* instance, size_t index, const std::vector<cv::DMatch>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_get_const_size_t(const std::vector<std::vector<cv::DMatch>>* instance, size_t index, std::vector<cv::DMatch>** ocvrs_return) {
			std::vector<cv::DMatch> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::DMatch>(ret);
	}
	
	void std_vectorLstd_vectorLcv_DMatchGG_set_size_t_const_vectorLDMatchG(std::vector<std::vector<cv::DMatch>>* instance, size_t index, const std::vector<cv::DMatch>* val) {
			(*instance)[index] = *val;
	}
	
}


