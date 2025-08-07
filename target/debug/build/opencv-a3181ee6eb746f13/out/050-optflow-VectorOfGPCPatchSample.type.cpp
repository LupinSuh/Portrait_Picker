extern "C" {
	std::vector<cv::optflow::GPCPatchSample>* std_vectorLcv_optflow_GPCPatchSampleG_new_const() {
			std::vector<cv::optflow::GPCPatchSample>* ret = new std::vector<cv::optflow::GPCPatchSample>();
			return ret;
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_delete(std::vector<cv::optflow::GPCPatchSample>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_optflow_GPCPatchSampleG_len_const(const std::vector<cv::optflow::GPCPatchSample>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_optflow_GPCPatchSampleG_isEmpty_const(const std::vector<cv::optflow::GPCPatchSample>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_optflow_GPCPatchSampleG_capacity_const(const std::vector<cv::optflow::GPCPatchSample>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_shrinkToFit(std::vector<cv::optflow::GPCPatchSample>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_reserve_size_t(std::vector<cv::optflow::GPCPatchSample>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_remove_size_t(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_swap_size_t_size_t(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_clear(std::vector<cv::optflow::GPCPatchSample>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_push_const_GPCPatchSample(std::vector<cv::optflow::GPCPatchSample>* instance, const cv::optflow::GPCPatchSample* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_insert_size_t_const_GPCPatchSample(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index, const cv::optflow::GPCPatchSample* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_get_const_size_t(const std::vector<cv::optflow::GPCPatchSample>* instance, size_t index, cv::optflow::GPCPatchSample** ocvrs_return) {
			cv::optflow::GPCPatchSample ret = (*instance)[index];
			*ocvrs_return = new cv::optflow::GPCPatchSample(ret);
	}
	
	void std_vectorLcv_optflow_GPCPatchSampleG_set_size_t_const_GPCPatchSample(std::vector<cv::optflow::GPCPatchSample>* instance, size_t index, const cv::optflow::GPCPatchSample* val) {
			(*instance)[index] = *val;
	}
	
}


