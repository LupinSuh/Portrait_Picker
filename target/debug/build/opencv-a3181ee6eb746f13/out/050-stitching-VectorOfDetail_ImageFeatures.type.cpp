extern "C" {
	std::vector<cv::detail::ImageFeatures>* std_vectorLcv_detail_ImageFeaturesG_new_const() {
			std::vector<cv::detail::ImageFeatures>* ret = new std::vector<cv::detail::ImageFeatures>();
			return ret;
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_delete(std::vector<cv::detail::ImageFeatures>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_detail_ImageFeaturesG_len_const(const std::vector<cv::detail::ImageFeatures>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_detail_ImageFeaturesG_isEmpty_const(const std::vector<cv::detail::ImageFeatures>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_detail_ImageFeaturesG_capacity_const(const std::vector<cv::detail::ImageFeatures>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_shrinkToFit(std::vector<cv::detail::ImageFeatures>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_reserve_size_t(std::vector<cv::detail::ImageFeatures>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_remove_size_t(std::vector<cv::detail::ImageFeatures>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_swap_size_t_size_t(std::vector<cv::detail::ImageFeatures>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_clear(std::vector<cv::detail::ImageFeatures>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_push_const_ImageFeatures(std::vector<cv::detail::ImageFeatures>* instance, const cv::detail::ImageFeatures* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_insert_size_t_const_ImageFeatures(std::vector<cv::detail::ImageFeatures>* instance, size_t index, const cv::detail::ImageFeatures* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_get_const_size_t(const std::vector<cv::detail::ImageFeatures>* instance, size_t index, cv::detail::ImageFeatures** ocvrs_return) {
			cv::detail::ImageFeatures ret = (*instance)[index];
			*ocvrs_return = new cv::detail::ImageFeatures(ret);
	}
	
	void std_vectorLcv_detail_ImageFeaturesG_set_size_t_const_ImageFeatures(std::vector<cv::detail::ImageFeatures>* instance, size_t index, const cv::detail::ImageFeatures* val) {
			(*instance)[index] = *val;
	}
	
}


