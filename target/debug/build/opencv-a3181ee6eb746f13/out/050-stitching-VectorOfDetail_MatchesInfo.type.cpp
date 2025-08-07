extern "C" {
	std::vector<cv::detail::MatchesInfo>* std_vectorLcv_detail_MatchesInfoG_new_const() {
			std::vector<cv::detail::MatchesInfo>* ret = new std::vector<cv::detail::MatchesInfo>();
			return ret;
	}
	
	void std_vectorLcv_detail_MatchesInfoG_delete(std::vector<cv::detail::MatchesInfo>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_detail_MatchesInfoG_len_const(const std::vector<cv::detail::MatchesInfo>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_detail_MatchesInfoG_isEmpty_const(const std::vector<cv::detail::MatchesInfo>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_detail_MatchesInfoG_capacity_const(const std::vector<cv::detail::MatchesInfo>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_detail_MatchesInfoG_shrinkToFit(std::vector<cv::detail::MatchesInfo>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_detail_MatchesInfoG_reserve_size_t(std::vector<cv::detail::MatchesInfo>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_detail_MatchesInfoG_remove_size_t(std::vector<cv::detail::MatchesInfo>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_detail_MatchesInfoG_swap_size_t_size_t(std::vector<cv::detail::MatchesInfo>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_detail_MatchesInfoG_clear(std::vector<cv::detail::MatchesInfo>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_detail_MatchesInfoG_push_const_MatchesInfo(std::vector<cv::detail::MatchesInfo>* instance, const cv::detail::MatchesInfo* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_detail_MatchesInfoG_insert_size_t_const_MatchesInfo(std::vector<cv::detail::MatchesInfo>* instance, size_t index, const cv::detail::MatchesInfo* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_detail_MatchesInfoG_get_const_size_t(const std::vector<cv::detail::MatchesInfo>* instance, size_t index, cv::detail::MatchesInfo** ocvrs_return) {
			cv::detail::MatchesInfo ret = (*instance)[index];
			*ocvrs_return = new cv::detail::MatchesInfo(ret);
	}
	
	void std_vectorLcv_detail_MatchesInfoG_set_size_t_const_MatchesInfo(std::vector<cv::detail::MatchesInfo>* instance, size_t index, const cv::detail::MatchesInfo* val) {
			(*instance)[index] = *val;
	}
	
}


