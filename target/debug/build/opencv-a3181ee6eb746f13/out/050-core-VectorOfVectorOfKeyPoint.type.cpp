extern "C" {
	std::vector<std::vector<cv::KeyPoint>>* std_vectorLstd_vectorLcv_KeyPointGG_new_const() {
			std::vector<std::vector<cv::KeyPoint>>* ret = new std::vector<std::vector<cv::KeyPoint>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_delete(std::vector<std::vector<cv::KeyPoint>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_KeyPointGG_len_const(const std::vector<std::vector<cv::KeyPoint>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_KeyPointGG_isEmpty_const(const std::vector<std::vector<cv::KeyPoint>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_KeyPointGG_capacity_const(const std::vector<std::vector<cv::KeyPoint>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_shrinkToFit(std::vector<std::vector<cv::KeyPoint>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_reserve_size_t(std::vector<std::vector<cv::KeyPoint>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_remove_size_t(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_swap_size_t_size_t(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_clear(std::vector<std::vector<cv::KeyPoint>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_push_const_vectorLKeyPointG(std::vector<std::vector<cv::KeyPoint>>* instance, const std::vector<cv::KeyPoint>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_insert_size_t_const_vectorLKeyPointG(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, const std::vector<cv::KeyPoint>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_get_const_size_t(const std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, std::vector<cv::KeyPoint>** ocvrs_return) {
			std::vector<cv::KeyPoint> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::KeyPoint>(ret);
	}
	
	void std_vectorLstd_vectorLcv_KeyPointGG_set_size_t_const_vectorLKeyPointG(std::vector<std::vector<cv::KeyPoint>>* instance, size_t index, const std::vector<cv::KeyPoint>* val) {
			(*instance)[index] = *val;
	}
	
}


