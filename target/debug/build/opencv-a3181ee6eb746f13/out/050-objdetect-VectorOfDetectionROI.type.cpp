extern "C" {
	std::vector<cv::DetectionROI>* std_vectorLcv_DetectionROIG_new_const() {
			std::vector<cv::DetectionROI>* ret = new std::vector<cv::DetectionROI>();
			return ret;
	}
	
	void std_vectorLcv_DetectionROIG_delete(std::vector<cv::DetectionROI>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_DetectionROIG_len_const(const std::vector<cv::DetectionROI>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_DetectionROIG_isEmpty_const(const std::vector<cv::DetectionROI>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_DetectionROIG_capacity_const(const std::vector<cv::DetectionROI>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_DetectionROIG_shrinkToFit(std::vector<cv::DetectionROI>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_DetectionROIG_reserve_size_t(std::vector<cv::DetectionROI>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_DetectionROIG_remove_size_t(std::vector<cv::DetectionROI>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_DetectionROIG_swap_size_t_size_t(std::vector<cv::DetectionROI>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_DetectionROIG_clear(std::vector<cv::DetectionROI>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_DetectionROIG_push_const_DetectionROI(std::vector<cv::DetectionROI>* instance, const cv::DetectionROI* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_DetectionROIG_insert_size_t_const_DetectionROI(std::vector<cv::DetectionROI>* instance, size_t index, const cv::DetectionROI* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_DetectionROIG_get_const_size_t(const std::vector<cv::DetectionROI>* instance, size_t index, cv::DetectionROI** ocvrs_return) {
			cv::DetectionROI ret = (*instance)[index];
			*ocvrs_return = new cv::DetectionROI(ret);
	}
	
	void std_vectorLcv_DetectionROIG_set_size_t_const_DetectionROI(std::vector<cv::DetectionROI>* instance, size_t index, const cv::DetectionROI* val) {
			(*instance)[index] = *val;
	}
	
}


