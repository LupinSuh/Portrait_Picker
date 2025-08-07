extern "C" {
	std::vector<cv::text::ERStat>* std_vectorLcv_text_ERStatG_new_const() {
			std::vector<cv::text::ERStat>* ret = new std::vector<cv::text::ERStat>();
			return ret;
	}
	
	void std_vectorLcv_text_ERStatG_delete(std::vector<cv::text::ERStat>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_text_ERStatG_len_const(const std::vector<cv::text::ERStat>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_text_ERStatG_isEmpty_const(const std::vector<cv::text::ERStat>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_text_ERStatG_capacity_const(const std::vector<cv::text::ERStat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_text_ERStatG_shrinkToFit(std::vector<cv::text::ERStat>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_text_ERStatG_reserve_size_t(std::vector<cv::text::ERStat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_text_ERStatG_remove_size_t(std::vector<cv::text::ERStat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_text_ERStatG_swap_size_t_size_t(std::vector<cv::text::ERStat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_text_ERStatG_clear(std::vector<cv::text::ERStat>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_text_ERStatG_push_const_ERStat(std::vector<cv::text::ERStat>* instance, const cv::text::ERStat* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_text_ERStatG_insert_size_t_const_ERStat(std::vector<cv::text::ERStat>* instance, size_t index, const cv::text::ERStat* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_text_ERStatG_get_const_size_t(const std::vector<cv::text::ERStat>* instance, size_t index, cv::text::ERStat** ocvrs_return) {
			cv::text::ERStat ret = (*instance)[index];
			*ocvrs_return = new cv::text::ERStat(ret);
	}
	
	void std_vectorLcv_text_ERStatG_set_size_t_const_ERStat(std::vector<cv::text::ERStat>* instance, size_t index, const cv::text::ERStat* val) {
			(*instance)[index] = *val;
	}
	
}


