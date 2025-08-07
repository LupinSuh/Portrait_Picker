extern "C" {
	std::vector<cv::detail::CameraParams>* std_vectorLcv_detail_CameraParamsG_new_const() {
			std::vector<cv::detail::CameraParams>* ret = new std::vector<cv::detail::CameraParams>();
			return ret;
	}
	
	void std_vectorLcv_detail_CameraParamsG_delete(std::vector<cv::detail::CameraParams>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_detail_CameraParamsG_len_const(const std::vector<cv::detail::CameraParams>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_detail_CameraParamsG_isEmpty_const(const std::vector<cv::detail::CameraParams>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_detail_CameraParamsG_capacity_const(const std::vector<cv::detail::CameraParams>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_detail_CameraParamsG_shrinkToFit(std::vector<cv::detail::CameraParams>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_detail_CameraParamsG_reserve_size_t(std::vector<cv::detail::CameraParams>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_detail_CameraParamsG_remove_size_t(std::vector<cv::detail::CameraParams>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_detail_CameraParamsG_swap_size_t_size_t(std::vector<cv::detail::CameraParams>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_detail_CameraParamsG_clear(std::vector<cv::detail::CameraParams>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_detail_CameraParamsG_push_const_CameraParams(std::vector<cv::detail::CameraParams>* instance, const cv::detail::CameraParams* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_detail_CameraParamsG_insert_size_t_const_CameraParams(std::vector<cv::detail::CameraParams>* instance, size_t index, const cv::detail::CameraParams* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_detail_CameraParamsG_get_const_size_t(const std::vector<cv::detail::CameraParams>* instance, size_t index, cv::detail::CameraParams** ocvrs_return) {
			cv::detail::CameraParams ret = (*instance)[index];
			*ocvrs_return = new cv::detail::CameraParams(ret);
	}
	
	void std_vectorLcv_detail_CameraParamsG_set_size_t_const_CameraParams(std::vector<cv::detail::CameraParams>* instance, size_t index, const cv::detail::CameraParams* val) {
			(*instance)[index] = *val;
	}
	
}


