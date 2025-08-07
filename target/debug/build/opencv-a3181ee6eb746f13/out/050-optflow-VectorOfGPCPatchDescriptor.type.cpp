extern "C" {
	std::vector<cv::optflow::GPCPatchDescriptor>* std_vectorLcv_optflow_GPCPatchDescriptorG_new_const() {
			std::vector<cv::optflow::GPCPatchDescriptor>* ret = new std::vector<cv::optflow::GPCPatchDescriptor>();
			return ret;
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_delete(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_optflow_GPCPatchDescriptorG_len_const(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_optflow_GPCPatchDescriptorG_isEmpty_const(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_optflow_GPCPatchDescriptorG_capacity_const(const std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_shrinkToFit(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_reserve_size_t(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_remove_size_t(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_swap_size_t_size_t(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_clear(std::vector<cv::optflow::GPCPatchDescriptor>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_push_const_GPCPatchDescriptor(std::vector<cv::optflow::GPCPatchDescriptor>* instance, const cv::optflow::GPCPatchDescriptor* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_insert_size_t_const_GPCPatchDescriptor(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, const cv::optflow::GPCPatchDescriptor* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_get_const_size_t(const std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, cv::optflow::GPCPatchDescriptor** ocvrs_return) {
			cv::optflow::GPCPatchDescriptor ret = (*instance)[index];
			*ocvrs_return = new cv::optflow::GPCPatchDescriptor(ret);
	}
	
	void std_vectorLcv_optflow_GPCPatchDescriptorG_set_size_t_const_GPCPatchDescriptor(std::vector<cv::optflow::GPCPatchDescriptor>* instance, size_t index, const cv::optflow::GPCPatchDescriptor* val) {
			(*instance)[index] = *val;
	}
	
}


