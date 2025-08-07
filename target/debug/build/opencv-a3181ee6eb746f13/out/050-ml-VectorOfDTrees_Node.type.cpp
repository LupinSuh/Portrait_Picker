extern "C" {
	std::vector<cv::ml::DTrees::Node>* std_vectorLcv_ml_DTrees_NodeG_new_const() {
			std::vector<cv::ml::DTrees::Node>* ret = new std::vector<cv::ml::DTrees::Node>();
			return ret;
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_delete(std::vector<cv::ml::DTrees::Node>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_ml_DTrees_NodeG_len_const(const std::vector<cv::ml::DTrees::Node>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_ml_DTrees_NodeG_isEmpty_const(const std::vector<cv::ml::DTrees::Node>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_ml_DTrees_NodeG_capacity_const(const std::vector<cv::ml::DTrees::Node>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_shrinkToFit(std::vector<cv::ml::DTrees::Node>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_reserve_size_t(std::vector<cv::ml::DTrees::Node>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_remove_size_t(std::vector<cv::ml::DTrees::Node>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_swap_size_t_size_t(std::vector<cv::ml::DTrees::Node>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_clear(std::vector<cv::ml::DTrees::Node>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_push_const_Node(std::vector<cv::ml::DTrees::Node>* instance, const cv::ml::DTrees::Node* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_insert_size_t_const_Node(std::vector<cv::ml::DTrees::Node>* instance, size_t index, const cv::ml::DTrees::Node* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_get_const_size_t(const std::vector<cv::ml::DTrees::Node>* instance, size_t index, cv::ml::DTrees::Node** ocvrs_return) {
			cv::ml::DTrees::Node ret = (*instance)[index];
			*ocvrs_return = new cv::ml::DTrees::Node(ret);
	}
	
	void std_vectorLcv_ml_DTrees_NodeG_set_size_t_const_Node(std::vector<cv::ml::DTrees::Node>* instance, size_t index, const cv::ml::DTrees::Node* val) {
			(*instance)[index] = *val;
	}
	
}


