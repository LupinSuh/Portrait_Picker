extern "C" {
	std::vector<cv::Range>* std_vectorLcv_RangeG_new_const() {
			std::vector<cv::Range>* ret = new std::vector<cv::Range>();
			return ret;
	}
	
	void std_vectorLcv_RangeG_delete(std::vector<cv::Range>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_RangeG_len_const(const std::vector<cv::Range>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_RangeG_isEmpty_const(const std::vector<cv::Range>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_RangeG_capacity_const(const std::vector<cv::Range>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_RangeG_shrinkToFit(std::vector<cv::Range>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_RangeG_reserve_size_t(std::vector<cv::Range>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_RangeG_remove_size_t(std::vector<cv::Range>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_RangeG_swap_size_t_size_t(std::vector<cv::Range>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_RangeG_clear(std::vector<cv::Range>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_RangeG_push_const_Range(std::vector<cv::Range>* instance, const cv::Range* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_RangeG_insert_size_t_const_Range(std::vector<cv::Range>* instance, size_t index, const cv::Range* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_RangeG_get_const_size_t(const std::vector<cv::Range>* instance, size_t index, cv::Range** ocvrs_return) {
			cv::Range ret = (*instance)[index];
			*ocvrs_return = new cv::Range(ret);
	}
	
	void std_vectorLcv_RangeG_set_size_t_const_Range(std::vector<cv::Range>* instance, size_t index, const cv::Range* val) {
			(*instance)[index] = *val;
	}
	
}


