extern "C" {
	std::vector<cv::Ptr<cv::dnn::BackendNode>>* std_vectorLcv_PtrLcv_dnn_BackendNodeGG_new_const() {
			std::vector<cv::Ptr<cv::dnn::BackendNode>>* ret = new std::vector<cv::Ptr<cv::dnn::BackendNode>>();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_delete(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_PtrLcv_dnn_BackendNodeGG_len_const(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_PtrLcv_dnn_BackendNodeGG_isEmpty_const(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_PtrLcv_dnn_BackendNodeGG_capacity_const(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_shrinkToFit(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_reserve_size_t(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_remove_size_t(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_clear(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_push_const_PtrLBackendNodeG(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, const cv::Ptr<cv::dnn::BackendNode>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_insert_size_t_const_PtrLBackendNodeG(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendNode>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_get_const_size_t(const std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, cv::Ptr<cv::dnn::BackendNode>** ocvrs_return) {
			cv::Ptr<cv::dnn::BackendNode> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::dnn::BackendNode>(ret);
	}
	
	void std_vectorLcv_PtrLcv_dnn_BackendNodeGG_set_size_t_const_PtrLBackendNodeG(std::vector<cv::Ptr<cv::dnn::BackendNode>>* instance, size_t index, const cv::Ptr<cv::dnn::BackendNode>* val) {
			(*instance)[index] = *val;
	}
	
}


