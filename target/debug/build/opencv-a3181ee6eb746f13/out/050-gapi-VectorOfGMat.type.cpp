extern "C" {
	std::vector<cv::GMat>* std_vectorLcv_GMatG_new_const() {
			std::vector<cv::GMat>* ret = new std::vector<cv::GMat>();
			return ret;
	}
	
	void std_vectorLcv_GMatG_delete(std::vector<cv::GMat>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GMatG_len_const(const std::vector<cv::GMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GMatG_isEmpty_const(const std::vector<cv::GMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GMatG_capacity_const(const std::vector<cv::GMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GMatG_shrinkToFit(std::vector<cv::GMat>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GMatG_reserve_size_t(std::vector<cv::GMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GMatG_remove_size_t(std::vector<cv::GMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GMatG_swap_size_t_size_t(std::vector<cv::GMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GMatG_clear(std::vector<cv::GMat>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GMatG_push_const_GMat(std::vector<cv::GMat>* instance, const cv::GMat* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_GMatG_insert_size_t_const_GMat(std::vector<cv::GMat>* instance, size_t index, const cv::GMat* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_GMatG_get_const_size_t(const std::vector<cv::GMat>* instance, size_t index, cv::GMat** ocvrs_return) {
			cv::GMat ret = (*instance)[index];
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void std_vectorLcv_GMatG_set_size_t_const_GMat(std::vector<cv::GMat>* instance, size_t index, const cv::GMat* val) {
			(*instance)[index] = *val;
	}
	
}


