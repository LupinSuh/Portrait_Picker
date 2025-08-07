extern "C" {
	std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_new_const() {
			std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* ret = new std::vector<cv::Ptr<cv::dnn::BackendWrapper>>();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_delete(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_len_const(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_isEmpty_const(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_capacity_const(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_shrinkToFit(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_reserve_size_t(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_remove_size_t(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_clear(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_push_const_PtrLBackendWrapperG(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, const cv::Ptr<cv::dnn::BackendWrapper>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_insert_size_t_const_PtrLBackendWrapperG(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendWrapper>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_get_const_size_t(const std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, cv::Ptr<cv::dnn::BackendWrapper>** ocvrs_return) {
			cv::Ptr<cv::dnn::BackendWrapper> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::dnn::BackendWrapper>(ret);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendWrapperGG_set_size_t_const_PtrLBackendWrapperG(std::vector<cv::Ptr<cv::dnn::BackendWrapper>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendWrapper>* val) {
			(*instance)[index] = *val;
	}
	
}


