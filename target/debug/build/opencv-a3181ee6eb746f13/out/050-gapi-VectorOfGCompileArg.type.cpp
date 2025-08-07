extern "C" {
	std::vector<cv::GCompileArg>* std_vectorLcv_GCompileArgG_new_const() {
			std::vector<cv::GCompileArg>* ret = new std::vector<cv::GCompileArg>();
			return ret;
	}
	
	void std_vectorLcv_GCompileArgG_delete(std::vector<cv::GCompileArg>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GCompileArgG_len_const(const std::vector<cv::GCompileArg>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GCompileArgG_isEmpty_const(const std::vector<cv::GCompileArg>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GCompileArgG_capacity_const(const std::vector<cv::GCompileArg>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GCompileArgG_shrinkToFit(std::vector<cv::GCompileArg>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GCompileArgG_reserve_size_t(std::vector<cv::GCompileArg>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GCompileArgG_remove_size_t(std::vector<cv::GCompileArg>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GCompileArgG_swap_size_t_size_t(std::vector<cv::GCompileArg>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GCompileArgG_clear(std::vector<cv::GCompileArg>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GCompileArgG_push_const_GCompileArg(std::vector<cv::GCompileArg>* instance, const cv::GCompileArg* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_GCompileArgG_insert_size_t_const_GCompileArg(std::vector<cv::GCompileArg>* instance, size_t index, const cv::GCompileArg* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_GCompileArgG_get_const_size_t(const std::vector<cv::GCompileArg>* instance, size_t index, cv::GCompileArg** ocvrs_return) {
			cv::GCompileArg ret = (*instance)[index];
			*ocvrs_return = new cv::GCompileArg(ret);
	}
	
	void std_vectorLcv_GCompileArgG_set_size_t_const_GCompileArg(std::vector<cv::GCompileArg>* instance, size_t index, const cv::GCompileArg* val) {
			(*instance)[index] = *val;
	}
	
}


