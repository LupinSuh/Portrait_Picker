extern "C" {
	std::vector<cv::gapi::GBackend>* std_vectorLcv_gapi_GBackendG_new_const() {
			std::vector<cv::gapi::GBackend>* ret = new std::vector<cv::gapi::GBackend>();
			return ret;
	}
	
	void std_vectorLcv_gapi_GBackendG_delete(std::vector<cv::gapi::GBackend>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_gapi_GBackendG_len_const(const std::vector<cv::gapi::GBackend>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_gapi_GBackendG_isEmpty_const(const std::vector<cv::gapi::GBackend>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_gapi_GBackendG_capacity_const(const std::vector<cv::gapi::GBackend>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_gapi_GBackendG_shrinkToFit(std::vector<cv::gapi::GBackend>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_gapi_GBackendG_reserve_size_t(std::vector<cv::gapi::GBackend>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_gapi_GBackendG_remove_size_t(std::vector<cv::gapi::GBackend>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_gapi_GBackendG_swap_size_t_size_t(std::vector<cv::gapi::GBackend>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_gapi_GBackendG_clear(std::vector<cv::gapi::GBackend>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_gapi_GBackendG_push_const_GBackend(std::vector<cv::gapi::GBackend>* instance, const cv::gapi::GBackend* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_gapi_GBackendG_insert_size_t_const_GBackend(std::vector<cv::gapi::GBackend>* instance, size_t index, const cv::gapi::GBackend* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_gapi_GBackendG_get_const_size_t(const std::vector<cv::gapi::GBackend>* instance, size_t index, cv::gapi::GBackend** ocvrs_return) {
			cv::gapi::GBackend ret = (*instance)[index];
			*ocvrs_return = new cv::gapi::GBackend(ret);
	}
	
	void std_vectorLcv_gapi_GBackendG_set_size_t_const_GBackend(std::vector<cv::gapi::GBackend>* instance, size_t index, const cv::gapi::GBackend* val) {
			(*instance)[index] = *val;
	}
	
}


