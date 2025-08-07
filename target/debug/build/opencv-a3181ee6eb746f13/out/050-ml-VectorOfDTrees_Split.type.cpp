extern "C" {
	std::vector<cv::ml::DTrees::Split>* std_vectorLcv_ml_DTrees_SplitG_new_const() {
			std::vector<cv::ml::DTrees::Split>* ret = new std::vector<cv::ml::DTrees::Split>();
			return ret;
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_delete(std::vector<cv::ml::DTrees::Split>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_ml_DTrees_SplitG_len_const(const std::vector<cv::ml::DTrees::Split>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_ml_DTrees_SplitG_isEmpty_const(const std::vector<cv::ml::DTrees::Split>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_ml_DTrees_SplitG_capacity_const(const std::vector<cv::ml::DTrees::Split>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_shrinkToFit(std::vector<cv::ml::DTrees::Split>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_reserve_size_t(std::vector<cv::ml::DTrees::Split>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_remove_size_t(std::vector<cv::ml::DTrees::Split>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_swap_size_t_size_t(std::vector<cv::ml::DTrees::Split>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_clear(std::vector<cv::ml::DTrees::Split>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_push_const_Split(std::vector<cv::ml::DTrees::Split>* instance, const cv::ml::DTrees::Split* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_insert_size_t_const_Split(std::vector<cv::ml::DTrees::Split>* instance, size_t index, const cv::ml::DTrees::Split* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_get_const_size_t(const std::vector<cv::ml::DTrees::Split>* instance, size_t index, cv::ml::DTrees::Split** ocvrs_return) {
			cv::ml::DTrees::Split ret = (*instance)[index];
			*ocvrs_return = new cv::ml::DTrees::Split(ret);
	}
	
	void std_vectorLcv_ml_DTrees_SplitG_set_size_t_const_Split(std::vector<cv::ml::DTrees::Split>* instance, size_t index, const cv::ml::DTrees::Split* val) {
			(*instance)[index] = *val;
	}
	
}


