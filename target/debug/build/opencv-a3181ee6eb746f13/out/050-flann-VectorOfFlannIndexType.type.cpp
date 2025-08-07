extern "C" {
	std::vector<cv::flann::FlannIndexType>* std_vectorLcv_flann_FlannIndexTypeG_new_const() {
			std::vector<cv::flann::FlannIndexType>* ret = new std::vector<cv::flann::FlannIndexType>();
			return ret;
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_delete(std::vector<cv::flann::FlannIndexType>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_flann_FlannIndexTypeG_len_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_flann_FlannIndexTypeG_isEmpty_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_flann_FlannIndexTypeG_capacity_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_shrinkToFit(std::vector<cv::flann::FlannIndexType>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_reserve_size_t(std::vector<cv::flann::FlannIndexType>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_remove_size_t(std::vector<cv::flann::FlannIndexType>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_swap_size_t_size_t(std::vector<cv::flann::FlannIndexType>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_clear(std::vector<cv::flann::FlannIndexType>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_push_const_FlannIndexType(std::vector<cv::flann::FlannIndexType>* instance, const cv::flann::FlannIndexType val) {
			instance->push_back(val);
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_insert_size_t_const_FlannIndexType(std::vector<cv::flann::FlannIndexType>* instance, size_t index, const cv::flann::FlannIndexType val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_get_const_size_t(const std::vector<cv::flann::FlannIndexType>* instance, size_t index, cv::flann::FlannIndexType* ocvrs_return) {
			cv::flann::FlannIndexType ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_flann_FlannIndexTypeG_set_size_t_const_FlannIndexType(std::vector<cv::flann::FlannIndexType>* instance, size_t index, const cv::flann::FlannIndexType val) {
			(*instance)[index] = val;
	}
	
	std::vector<cv::flann::FlannIndexType>* std_vectorLcv_flann_FlannIndexTypeG_clone_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			std::vector<cv::flann::FlannIndexType> ret = std::vector<cv::flann::FlannIndexType>(*instance);
			return new std::vector<cv::flann::FlannIndexType>(ret);
	}
	
	const cv::flann::FlannIndexType* std_vectorLcv_flann_FlannIndexTypeG_data_const(const std::vector<cv::flann::FlannIndexType>* instance) {
			const cv::flann::FlannIndexType* ret = instance->data();
			return ret;
	}
	
	cv::flann::FlannIndexType* std_vectorLcv_flann_FlannIndexTypeG_dataMut(std::vector<cv::flann::FlannIndexType>* instance) {
			cv::flann::FlannIndexType* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::flann::FlannIndexType>* cv_fromSlice_const_const_FlannIndexTypeX_size_t(const cv::flann::FlannIndexType* data, size_t len) {
			return new std::vector<cv::flann::FlannIndexType>(data, data + len);
	}
	
}


