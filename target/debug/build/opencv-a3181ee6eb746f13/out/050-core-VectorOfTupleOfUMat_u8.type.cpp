extern "C" {
	std::vector<std::pair<cv::UMat, unsigned char>>* std_vectorLstd_pairLcv_UMat__unsigned_charGG_new_const() {
			std::vector<std::pair<cv::UMat, unsigned char>>* ret = new std::vector<std::pair<cv::UMat, unsigned char>>();
			return ret;
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_delete(std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_pairLcv_UMat__unsigned_charGG_len_const(const std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_pairLcv_UMat__unsigned_charGG_isEmpty_const(const std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_pairLcv_UMat__unsigned_charGG_capacity_const(const std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_shrinkToFit(std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_reserve_size_t(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_remove_size_t(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_swap_size_t_size_t(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_clear(std::vector<std::pair<cv::UMat, unsigned char>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_push_const_pairLcv_UMat__unsigned_charG(std::vector<std::pair<cv::UMat, unsigned char>>* instance, const std::pair<cv::UMat, unsigned char>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_insert_size_t_const_pairLcv_UMat__unsigned_charG(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index, const std::pair<cv::UMat, unsigned char>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_get_const_size_t(const std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index, std::pair<cv::UMat, unsigned char>** ocvrs_return) {
			std::pair<cv::UMat, unsigned char> ret = (*instance)[index];
			*ocvrs_return = new std::pair<cv::UMat, unsigned char>(ret);
	}
	
	void std_vectorLstd_pairLcv_UMat__unsigned_charGG_set_size_t_const_pairLcv_UMat__unsigned_charG(std::vector<std::pair<cv::UMat, unsigned char>>* instance, size_t index, const std::pair<cv::UMat, unsigned char>* val) {
			(*instance)[index] = *val;
	}
	
}


