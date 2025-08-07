extern "C" {
	std::vector<cv::RotatedRect>* std_vectorLcv_RotatedRectG_new_const() {
			std::vector<cv::RotatedRect>* ret = new std::vector<cv::RotatedRect>();
			return ret;
	}
	
	void std_vectorLcv_RotatedRectG_delete(std::vector<cv::RotatedRect>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_RotatedRectG_len_const(const std::vector<cv::RotatedRect>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_RotatedRectG_isEmpty_const(const std::vector<cv::RotatedRect>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_RotatedRectG_capacity_const(const std::vector<cv::RotatedRect>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_RotatedRectG_shrinkToFit(std::vector<cv::RotatedRect>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_RotatedRectG_reserve_size_t(std::vector<cv::RotatedRect>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_RotatedRectG_remove_size_t(std::vector<cv::RotatedRect>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_RotatedRectG_swap_size_t_size_t(std::vector<cv::RotatedRect>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_RotatedRectG_clear(std::vector<cv::RotatedRect>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_RotatedRectG_push_const_RotatedRect(std::vector<cv::RotatedRect>* instance, const cv::RotatedRect* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_RotatedRectG_insert_size_t_const_RotatedRect(std::vector<cv::RotatedRect>* instance, size_t index, const cv::RotatedRect* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_RotatedRectG_get_const_size_t(const std::vector<cv::RotatedRect>* instance, size_t index, cv::RotatedRect* ocvrs_return) {
			cv::RotatedRect ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_RotatedRectG_set_size_t_const_RotatedRect(std::vector<cv::RotatedRect>* instance, size_t index, const cv::RotatedRect* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::RotatedRect>* std_vectorLcv_RotatedRectG_clone_const(const std::vector<cv::RotatedRect>* instance) {
			std::vector<cv::RotatedRect> ret = std::vector<cv::RotatedRect>(*instance);
			return new std::vector<cv::RotatedRect>(ret);
	}
	
	const cv::RotatedRect* std_vectorLcv_RotatedRectG_data_const(const std::vector<cv::RotatedRect>* instance) {
			const cv::RotatedRect* ret = instance->data();
			return ret;
	}
	
	cv::RotatedRect* std_vectorLcv_RotatedRectG_dataMut(std::vector<cv::RotatedRect>* instance) {
			cv::RotatedRect* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::RotatedRect>* cv_fromSlice_const_const_RotatedRectX_size_t(const cv::RotatedRect* data, size_t len) {
			return new std::vector<cv::RotatedRect>(data, data + len);
	}
	
}


