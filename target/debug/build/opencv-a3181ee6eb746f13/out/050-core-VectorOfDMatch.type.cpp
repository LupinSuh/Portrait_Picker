extern "C" {
	std::vector<cv::DMatch>* std_vectorLcv_DMatchG_new_const() {
			std::vector<cv::DMatch>* ret = new std::vector<cv::DMatch>();
			return ret;
	}
	
	void std_vectorLcv_DMatchG_delete(std::vector<cv::DMatch>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_DMatchG_len_const(const std::vector<cv::DMatch>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_DMatchG_isEmpty_const(const std::vector<cv::DMatch>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_DMatchG_capacity_const(const std::vector<cv::DMatch>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_DMatchG_shrinkToFit(std::vector<cv::DMatch>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_DMatchG_reserve_size_t(std::vector<cv::DMatch>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_DMatchG_remove_size_t(std::vector<cv::DMatch>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_DMatchG_swap_size_t_size_t(std::vector<cv::DMatch>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_DMatchG_clear(std::vector<cv::DMatch>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_DMatchG_push_const_DMatch(std::vector<cv::DMatch>* instance, const cv::DMatch* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_DMatchG_insert_size_t_const_DMatch(std::vector<cv::DMatch>* instance, size_t index, const cv::DMatch* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_DMatchG_get_const_size_t(const std::vector<cv::DMatch>* instance, size_t index, cv::DMatch* ocvrs_return) {
			cv::DMatch ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_DMatchG_set_size_t_const_DMatch(std::vector<cv::DMatch>* instance, size_t index, const cv::DMatch* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::DMatch>* std_vectorLcv_DMatchG_clone_const(const std::vector<cv::DMatch>* instance) {
			std::vector<cv::DMatch> ret = std::vector<cv::DMatch>(*instance);
			return new std::vector<cv::DMatch>(ret);
	}
	
	const cv::DMatch* std_vectorLcv_DMatchG_data_const(const std::vector<cv::DMatch>* instance) {
			const cv::DMatch* ret = instance->data();
			return ret;
	}
	
	cv::DMatch* std_vectorLcv_DMatchG_dataMut(std::vector<cv::DMatch>* instance) {
			cv::DMatch* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::DMatch>* cv_fromSlice_const_const_DMatchX_size_t(const cv::DMatch* data, size_t len) {
			return new std::vector<cv::DMatch>(data, data + len);
	}
	
}


