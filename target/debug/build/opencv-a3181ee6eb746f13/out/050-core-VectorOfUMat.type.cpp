extern "C" {
	std::vector<cv::UMat>* std_vectorLcv_UMatG_new_const() {
			std::vector<cv::UMat>* ret = new std::vector<cv::UMat>();
			return ret;
	}
	
	void std_vectorLcv_UMatG_delete(std::vector<cv::UMat>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_UMatG_len_const(const std::vector<cv::UMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_UMatG_isEmpty_const(const std::vector<cv::UMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_UMatG_capacity_const(const std::vector<cv::UMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_UMatG_shrinkToFit(std::vector<cv::UMat>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_UMatG_reserve_size_t(std::vector<cv::UMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_UMatG_remove_size_t(std::vector<cv::UMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_UMatG_swap_size_t_size_t(std::vector<cv::UMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_UMatG_clear(std::vector<cv::UMat>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_UMatG_push_const_UMat(std::vector<cv::UMat>* instance, const cv::UMat* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_UMatG_insert_size_t_const_UMat(std::vector<cv::UMat>* instance, size_t index, const cv::UMat* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_UMatG_get_const_size_t(const std::vector<cv::UMat>* instance, size_t index, cv::UMat** ocvrs_return) {
			cv::UMat ret = (*instance)[index];
			*ocvrs_return = new cv::UMat(ret);
	}
	
	void std_vectorLcv_UMatG_set_size_t_const_UMat(std::vector<cv::UMat>* instance, size_t index, const cv::UMat* val) {
			(*instance)[index] = *val;
	}
	
}


