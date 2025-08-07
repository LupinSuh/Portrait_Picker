extern "C" {
	std::vector<cv::linemod::Match>* std_vectorLcv_linemod_MatchG_new_const() {
			std::vector<cv::linemod::Match>* ret = new std::vector<cv::linemod::Match>();
			return ret;
	}
	
	void std_vectorLcv_linemod_MatchG_delete(std::vector<cv::linemod::Match>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_linemod_MatchG_len_const(const std::vector<cv::linemod::Match>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_linemod_MatchG_isEmpty_const(const std::vector<cv::linemod::Match>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_linemod_MatchG_capacity_const(const std::vector<cv::linemod::Match>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_linemod_MatchG_shrinkToFit(std::vector<cv::linemod::Match>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_linemod_MatchG_reserve_size_t(std::vector<cv::linemod::Match>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_linemod_MatchG_remove_size_t(std::vector<cv::linemod::Match>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_linemod_MatchG_swap_size_t_size_t(std::vector<cv::linemod::Match>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_linemod_MatchG_clear(std::vector<cv::linemod::Match>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_linemod_MatchG_push_const_Match(std::vector<cv::linemod::Match>* instance, const cv::linemod::Match* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_linemod_MatchG_insert_size_t_const_Match(std::vector<cv::linemod::Match>* instance, size_t index, const cv::linemod::Match* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_linemod_MatchG_get_const_size_t(const std::vector<cv::linemod::Match>* instance, size_t index, cv::linemod::Match** ocvrs_return) {
			cv::linemod::Match ret = (*instance)[index];
			*ocvrs_return = new cv::linemod::Match(ret);
	}
	
	void std_vectorLcv_linemod_MatchG_set_size_t_const_Match(std::vector<cv::linemod::Match>* instance, size_t index, const cv::linemod::Match* val) {
			(*instance)[index] = *val;
	}
	
}


