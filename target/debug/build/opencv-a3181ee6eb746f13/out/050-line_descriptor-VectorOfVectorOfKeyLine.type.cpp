extern "C" {
	std::vector<std::vector<cv::line_descriptor::KeyLine>>* std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_new_const() {
			std::vector<std::vector<cv::line_descriptor::KeyLine>>* ret = new std::vector<std::vector<cv::line_descriptor::KeyLine>>();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_delete(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_len_const(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_isEmpty_const(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_capacity_const(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_shrinkToFit(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_reserve_size_t(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_remove_size_t(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_swap_size_t_size_t(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_clear(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_push_const_vectorLKeyLineG(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, const std::vector<cv::line_descriptor::KeyLine>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_insert_size_t_const_vectorLKeyLineG(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, const std::vector<cv::line_descriptor::KeyLine>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_get_const_size_t(const std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, std::vector<cv::line_descriptor::KeyLine>** ocvrs_return) {
			std::vector<cv::line_descriptor::KeyLine> ret = (*instance)[index];
			*ocvrs_return = new std::vector<cv::line_descriptor::KeyLine>(ret);
	}
	
	void std_vectorLstd_vectorLcv_line_descriptor_KeyLineGG_set_size_t_const_vectorLKeyLineG(std::vector<std::vector<cv::line_descriptor::KeyLine>>* instance, size_t index, const std::vector<cv::line_descriptor::KeyLine>* val) {
			(*instance)[index] = *val;
	}
	
}


