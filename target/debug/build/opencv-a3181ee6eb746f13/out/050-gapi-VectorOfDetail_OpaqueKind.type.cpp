extern "C" {
	std::vector<cv::detail::OpaqueKind>* std_vectorLcv_detail_OpaqueKindG_new_const() {
			std::vector<cv::detail::OpaqueKind>* ret = new std::vector<cv::detail::OpaqueKind>();
			return ret;
	}
	
	void std_vectorLcv_detail_OpaqueKindG_delete(std::vector<cv::detail::OpaqueKind>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_detail_OpaqueKindG_len_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_detail_OpaqueKindG_isEmpty_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_detail_OpaqueKindG_capacity_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_detail_OpaqueKindG_shrinkToFit(std::vector<cv::detail::OpaqueKind>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_detail_OpaqueKindG_reserve_size_t(std::vector<cv::detail::OpaqueKind>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_detail_OpaqueKindG_remove_size_t(std::vector<cv::detail::OpaqueKind>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_detail_OpaqueKindG_swap_size_t_size_t(std::vector<cv::detail::OpaqueKind>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_detail_OpaqueKindG_clear(std::vector<cv::detail::OpaqueKind>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_detail_OpaqueKindG_push_const_OpaqueKind(std::vector<cv::detail::OpaqueKind>* instance, const cv::detail::OpaqueKind val) {
			instance->push_back(val);
	}
	
	void std_vectorLcv_detail_OpaqueKindG_insert_size_t_const_OpaqueKind(std::vector<cv::detail::OpaqueKind>* instance, size_t index, const cv::detail::OpaqueKind val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLcv_detail_OpaqueKindG_get_const_size_t(const std::vector<cv::detail::OpaqueKind>* instance, size_t index, cv::detail::OpaqueKind* ocvrs_return) {
			cv::detail::OpaqueKind ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_detail_OpaqueKindG_set_size_t_const_OpaqueKind(std::vector<cv::detail::OpaqueKind>* instance, size_t index, const cv::detail::OpaqueKind val) {
			(*instance)[index] = val;
	}
	
	std::vector<cv::detail::OpaqueKind>* std_vectorLcv_detail_OpaqueKindG_clone_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			std::vector<cv::detail::OpaqueKind> ret = std::vector<cv::detail::OpaqueKind>(*instance);
			return new std::vector<cv::detail::OpaqueKind>(ret);
	}
	
	const cv::detail::OpaqueKind* std_vectorLcv_detail_OpaqueKindG_data_const(const std::vector<cv::detail::OpaqueKind>* instance) {
			const cv::detail::OpaqueKind* ret = instance->data();
			return ret;
	}
	
	cv::detail::OpaqueKind* std_vectorLcv_detail_OpaqueKindG_dataMut(std::vector<cv::detail::OpaqueKind>* instance) {
			cv::detail::OpaqueKind* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::detail::OpaqueKind>* cv_fromSlice_const_const_OpaqueKindX_size_t(const cv::detail::OpaqueKind* data, size_t len) {
			return new std::vector<cv::detail::OpaqueKind>(data, data + len);
	}
	
}


