extern "C" {
	std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_new_const() {
			std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* ret = new std::vector<cv::xfeatures2d::Elliptic_KeyPoint>();
			return ret;
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_delete(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_len_const(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_isEmpty_const(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_capacity_const(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_shrinkToFit(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_reserve_size_t(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_remove_size_t(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_swap_size_t_size_t(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_clear(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_push_const_Elliptic_KeyPoint(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, const cv::xfeatures2d::Elliptic_KeyPoint* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_insert_size_t_const_Elliptic_KeyPoint(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, const cv::xfeatures2d::Elliptic_KeyPoint* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_get_const_size_t(const std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, cv::xfeatures2d::Elliptic_KeyPoint** ocvrs_return) {
			cv::xfeatures2d::Elliptic_KeyPoint ret = (*instance)[index];
			*ocvrs_return = new cv::xfeatures2d::Elliptic_KeyPoint(ret);
	}
	
	void std_vectorLcv_xfeatures2d_Elliptic_KeyPointG_set_size_t_const_Elliptic_KeyPoint(std::vector<cv::xfeatures2d::Elliptic_KeyPoint>* instance, size_t index, const cv::xfeatures2d::Elliptic_KeyPoint* val) {
			(*instance)[index] = *val;
	}
	
}


