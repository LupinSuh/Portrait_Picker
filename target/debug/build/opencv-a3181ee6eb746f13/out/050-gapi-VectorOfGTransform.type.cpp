extern "C" {
	std::vector<cv::GTransform>* std_vectorLcv_GTransformG_new_const() {
			std::vector<cv::GTransform>* ret = new std::vector<cv::GTransform>();
			return ret;
	}
	
	void std_vectorLcv_GTransformG_delete(std::vector<cv::GTransform>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GTransformG_len_const(const std::vector<cv::GTransform>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GTransformG_isEmpty_const(const std::vector<cv::GTransform>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GTransformG_capacity_const(const std::vector<cv::GTransform>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GTransformG_shrinkToFit(std::vector<cv::GTransform>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GTransformG_reserve_size_t(std::vector<cv::GTransform>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GTransformG_remove_size_t(std::vector<cv::GTransform>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GTransformG_swap_size_t_size_t(std::vector<cv::GTransform>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GTransformG_clear(std::vector<cv::GTransform>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GTransformG_push_const_GTransform(std::vector<cv::GTransform>* instance, const cv::GTransform* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_GTransformG_insert_size_t_const_GTransform(std::vector<cv::GTransform>* instance, size_t index, const cv::GTransform* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_GTransformG_get_const_size_t(const std::vector<cv::GTransform>* instance, size_t index, cv::GTransform** ocvrs_return) {
			cv::GTransform ret = (*instance)[index];
			*ocvrs_return = new cv::GTransform(ret);
	}
	
	void std_vectorLcv_GTransformG_set_size_t_const_GTransform(std::vector<cv::GTransform>* instance, size_t index, const cv::GTransform* val) {
			(*instance)[index] = *val;
	}
	
}


