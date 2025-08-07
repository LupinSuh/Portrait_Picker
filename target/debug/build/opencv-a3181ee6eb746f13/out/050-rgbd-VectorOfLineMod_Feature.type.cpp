extern "C" {
	std::vector<cv::linemod::Feature>* std_vectorLcv_linemod_FeatureG_new_const() {
			std::vector<cv::linemod::Feature>* ret = new std::vector<cv::linemod::Feature>();
			return ret;
	}
	
	void std_vectorLcv_linemod_FeatureG_delete(std::vector<cv::linemod::Feature>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_linemod_FeatureG_len_const(const std::vector<cv::linemod::Feature>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_linemod_FeatureG_isEmpty_const(const std::vector<cv::linemod::Feature>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_linemod_FeatureG_capacity_const(const std::vector<cv::linemod::Feature>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_linemod_FeatureG_shrinkToFit(std::vector<cv::linemod::Feature>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_linemod_FeatureG_reserve_size_t(std::vector<cv::linemod::Feature>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_linemod_FeatureG_remove_size_t(std::vector<cv::linemod::Feature>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_linemod_FeatureG_swap_size_t_size_t(std::vector<cv::linemod::Feature>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_linemod_FeatureG_clear(std::vector<cv::linemod::Feature>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_linemod_FeatureG_push_const_Feature(std::vector<cv::linemod::Feature>* instance, const cv::linemod::Feature* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_linemod_FeatureG_insert_size_t_const_Feature(std::vector<cv::linemod::Feature>* instance, size_t index, const cv::linemod::Feature* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_linemod_FeatureG_get_const_size_t(const std::vector<cv::linemod::Feature>* instance, size_t index, cv::linemod::Feature* ocvrs_return) {
			cv::linemod::Feature ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_linemod_FeatureG_set_size_t_const_Feature(std::vector<cv::linemod::Feature>* instance, size_t index, const cv::linemod::Feature* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::linemod::Feature>* std_vectorLcv_linemod_FeatureG_clone_const(const std::vector<cv::linemod::Feature>* instance) {
			std::vector<cv::linemod::Feature> ret = std::vector<cv::linemod::Feature>(*instance);
			return new std::vector<cv::linemod::Feature>(ret);
	}
	
	const cv::linemod::Feature* std_vectorLcv_linemod_FeatureG_data_const(const std::vector<cv::linemod::Feature>* instance) {
			const cv::linemod::Feature* ret = instance->data();
			return ret;
	}
	
	cv::linemod::Feature* std_vectorLcv_linemod_FeatureG_dataMut(std::vector<cv::linemod::Feature>* instance) {
			cv::linemod::Feature* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::linemod::Feature>* cv_fromSlice_const_const_FeatureX_size_t(const cv::linemod::Feature* data, size_t len) {
			return new std::vector<cv::linemod::Feature>(data, data + len);
	}
	
}


