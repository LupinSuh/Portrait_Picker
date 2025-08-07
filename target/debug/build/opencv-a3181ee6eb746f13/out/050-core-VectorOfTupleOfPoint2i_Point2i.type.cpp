extern "C" {
	std::vector<std::pair<cv::Point2i, cv::Point2i>>* std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_new_const() {
			std::vector<std::pair<cv::Point2i, cv::Point2i>>* ret = new std::vector<std::pair<cv::Point2i, cv::Point2i>>();
			return ret;
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_delete(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_len_const(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_isEmpty_const(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_capacity_const(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_shrinkToFit(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_reserve_size_t(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_remove_size_t(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_swap_size_t_size_t(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_clear(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_push_const_pairLcv_Point2i__cv_Point2iG(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, const std::pair<cv::Point2i, cv::Point2i>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_insert_size_t_const_pairLcv_Point2i__cv_Point2iG(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index, const std::pair<cv::Point2i, cv::Point2i>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_get_const_size_t(const std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index, std::pair<cv::Point2i, cv::Point2i>** ocvrs_return) {
			std::pair<cv::Point2i, cv::Point2i> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::Point2i, cv::Point2i>(ret);
	}
	
	void std_vectorLstd_pairLcv_Point2i__cv_Point2iGG_set_size_t_const_pairLcv_Point2i__cv_Point2iG(std::vector<std::pair<cv::Point2i, cv::Point2i>>* instance, size_t index, const std::pair<cv::Point2i, cv::Point2i>* val) {
			(*instance)[index] = *val;
	}
	
}


