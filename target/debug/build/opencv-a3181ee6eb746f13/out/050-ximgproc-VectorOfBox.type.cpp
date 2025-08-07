extern "C" {
	std::vector<cv::ximgproc::Box>* std_vectorLcv_ximgproc_BoxG_new_const() {
			std::vector<cv::ximgproc::Box>* ret = new std::vector<cv::ximgproc::Box>();
			return ret;
	}
	
	void std_vectorLcv_ximgproc_BoxG_delete(std::vector<cv::ximgproc::Box>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_ximgproc_BoxG_len_const(const std::vector<cv::ximgproc::Box>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_ximgproc_BoxG_isEmpty_const(const std::vector<cv::ximgproc::Box>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_ximgproc_BoxG_capacity_const(const std::vector<cv::ximgproc::Box>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_ximgproc_BoxG_shrinkToFit(std::vector<cv::ximgproc::Box>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_ximgproc_BoxG_reserve_size_t(std::vector<cv::ximgproc::Box>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_ximgproc_BoxG_remove_size_t(std::vector<cv::ximgproc::Box>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_ximgproc_BoxG_swap_size_t_size_t(std::vector<cv::ximgproc::Box>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_ximgproc_BoxG_clear(std::vector<cv::ximgproc::Box>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_ximgproc_BoxG_push_const_Box(std::vector<cv::ximgproc::Box>* instance, const cv::ximgproc::Box* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_ximgproc_BoxG_insert_size_t_const_Box(std::vector<cv::ximgproc::Box>* instance, size_t index, const cv::ximgproc::Box* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_ximgproc_BoxG_get_const_size_t(const std::vector<cv::ximgproc::Box>* instance, size_t index, cv::ximgproc::Box* ocvrs_return) {
			cv::ximgproc::Box ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_ximgproc_BoxG_set_size_t_const_Box(std::vector<cv::ximgproc::Box>* instance, size_t index, const cv::ximgproc::Box* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::ximgproc::Box>* std_vectorLcv_ximgproc_BoxG_clone_const(const std::vector<cv::ximgproc::Box>* instance) {
			std::vector<cv::ximgproc::Box> ret = std::vector<cv::ximgproc::Box>(*instance);
			return new std::vector<cv::ximgproc::Box>(ret);
	}
	
	const cv::ximgproc::Box* std_vectorLcv_ximgproc_BoxG_data_const(const std::vector<cv::ximgproc::Box>* instance) {
			const cv::ximgproc::Box* ret = instance->data();
			return ret;
	}
	
	cv::ximgproc::Box* std_vectorLcv_ximgproc_BoxG_dataMut(std::vector<cv::ximgproc::Box>* instance) {
			cv::ximgproc::Box* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::ximgproc::Box>* cv_fromSlice_const_const_BoxX_size_t(const cv::ximgproc::Box* data, size_t len) {
			return new std::vector<cv::ximgproc::Box>(data, data + len);
	}
	
}


