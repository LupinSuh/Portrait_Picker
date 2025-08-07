extern "C" {
	std::vector<cv::aruco::Dictionary>* std_vectorLcv_aruco_DictionaryG_new_const() {
			std::vector<cv::aruco::Dictionary>* ret = new std::vector<cv::aruco::Dictionary>();
			return ret;
	}
	
	void std_vectorLcv_aruco_DictionaryG_delete(std::vector<cv::aruco::Dictionary>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_aruco_DictionaryG_len_const(const std::vector<cv::aruco::Dictionary>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_aruco_DictionaryG_isEmpty_const(const std::vector<cv::aruco::Dictionary>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_aruco_DictionaryG_capacity_const(const std::vector<cv::aruco::Dictionary>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_aruco_DictionaryG_shrinkToFit(std::vector<cv::aruco::Dictionary>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_aruco_DictionaryG_reserve_size_t(std::vector<cv::aruco::Dictionary>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_aruco_DictionaryG_remove_size_t(std::vector<cv::aruco::Dictionary>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_aruco_DictionaryG_swap_size_t_size_t(std::vector<cv::aruco::Dictionary>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_aruco_DictionaryG_clear(std::vector<cv::aruco::Dictionary>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_aruco_DictionaryG_push_const_Dictionary(std::vector<cv::aruco::Dictionary>* instance, const cv::aruco::Dictionary* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_aruco_DictionaryG_insert_size_t_const_Dictionary(std::vector<cv::aruco::Dictionary>* instance, size_t index, const cv::aruco::Dictionary* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_aruco_DictionaryG_get_const_size_t(const std::vector<cv::aruco::Dictionary>* instance, size_t index, cv::aruco::Dictionary** ocvrs_return) {
			cv::aruco::Dictionary ret = (*instance)[index];
			*ocvrs_return = new cv::aruco::Dictionary(ret);
	}
	
	void std_vectorLcv_aruco_DictionaryG_set_size_t_const_Dictionary(std::vector<cv::aruco::Dictionary>* instance, size_t index, const cv::aruco::Dictionary* val) {
			(*instance)[index] = *val;
	}
	
}


