extern "C" {
	std::vector<cv::line_descriptor::KeyLine>* std_vectorLcv_line_descriptor_KeyLineG_new_const() {
			std::vector<cv::line_descriptor::KeyLine>* ret = new std::vector<cv::line_descriptor::KeyLine>();
			return ret;
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_delete(std::vector<cv::line_descriptor::KeyLine>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_line_descriptor_KeyLineG_len_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_line_descriptor_KeyLineG_isEmpty_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_line_descriptor_KeyLineG_capacity_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_shrinkToFit(std::vector<cv::line_descriptor::KeyLine>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_reserve_size_t(std::vector<cv::line_descriptor::KeyLine>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_remove_size_t(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_swap_size_t_size_t(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_clear(std::vector<cv::line_descriptor::KeyLine>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_push_const_KeyLine(std::vector<cv::line_descriptor::KeyLine>* instance, const cv::line_descriptor::KeyLine* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_insert_size_t_const_KeyLine(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index, const cv::line_descriptor::KeyLine* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_get_const_size_t(const std::vector<cv::line_descriptor::KeyLine>* instance, size_t index, cv::line_descriptor::KeyLine* ocvrs_return) {
			cv::line_descriptor::KeyLine ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_line_descriptor_KeyLineG_set_size_t_const_KeyLine(std::vector<cv::line_descriptor::KeyLine>* instance, size_t index, const cv::line_descriptor::KeyLine* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::line_descriptor::KeyLine>* std_vectorLcv_line_descriptor_KeyLineG_clone_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			std::vector<cv::line_descriptor::KeyLine> ret = std::vector<cv::line_descriptor::KeyLine>(*instance);
			return new std::vector<cv::line_descriptor::KeyLine>(ret);
	}
	
	const cv::line_descriptor::KeyLine* std_vectorLcv_line_descriptor_KeyLineG_data_const(const std::vector<cv::line_descriptor::KeyLine>* instance) {
			const cv::line_descriptor::KeyLine* ret = instance->data();
			return ret;
	}
	
	cv::line_descriptor::KeyLine* std_vectorLcv_line_descriptor_KeyLineG_dataMut(std::vector<cv::line_descriptor::KeyLine>* instance) {
			cv::line_descriptor::KeyLine* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::line_descriptor::KeyLine>* cv_fromSlice_const_const_KeyLineX_size_t(const cv::line_descriptor::KeyLine* data, size_t len) {
			return new std::vector<cv::line_descriptor::KeyLine>(data, data + len);
	}
	
}


