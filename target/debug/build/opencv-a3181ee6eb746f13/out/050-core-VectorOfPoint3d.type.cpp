extern "C" {
	std::vector<cv::Point3d>* std_vectorLcv_Point3dG_new_const() {
			std::vector<cv::Point3d>* ret = new std::vector<cv::Point3d>();
			return ret;
	}
	
	void std_vectorLcv_Point3dG_delete(std::vector<cv::Point3d>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_Point3dG_len_const(const std::vector<cv::Point3d>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_Point3dG_isEmpty_const(const std::vector<cv::Point3d>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_Point3dG_capacity_const(const std::vector<cv::Point3d>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_Point3dG_shrinkToFit(std::vector<cv::Point3d>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_Point3dG_reserve_size_t(std::vector<cv::Point3d>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_Point3dG_remove_size_t(std::vector<cv::Point3d>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_Point3dG_swap_size_t_size_t(std::vector<cv::Point3d>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_Point3dG_clear(std::vector<cv::Point3d>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_Point3dG_push_const_Point3d(std::vector<cv::Point3d>* instance, const cv::Point3d* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_Point3dG_insert_size_t_const_Point3d(std::vector<cv::Point3d>* instance, size_t index, const cv::Point3d* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_Point3dG_get_const_size_t(const std::vector<cv::Point3d>* instance, size_t index, cv::Point3d* ocvrs_return) {
			cv::Point3d ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_Point3dG_set_size_t_const_Point3d(std::vector<cv::Point3d>* instance, size_t index, const cv::Point3d* val) {
			(*instance)[index] = *val;
	}
	
	std::vector<cv::Point3d>* std_vectorLcv_Point3dG_clone_const(const std::vector<cv::Point3d>* instance) {
			std::vector<cv::Point3d> ret = std::vector<cv::Point3d>(*instance);
			return new std::vector<cv::Point3d>(ret);
	}
	
	const cv::Point3d* std_vectorLcv_Point3dG_data_const(const std::vector<cv::Point3d>* instance) {
			const cv::Point3d* ret = instance->data();
			return ret;
	}
	
	cv::Point3d* std_vectorLcv_Point3dG_dataMut(std::vector<cv::Point3d>* instance) {
			cv::Point3d* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::Point3d>* cv_fromSlice_const_const_Point3dX_size_t(const cv::Point3d* data, size_t len) {
			return new std::vector<cv::Point3d>(data, data + len);
	}
	
	void std_vectorLcv_Point3dG_inputArray_const(const std::vector<cv::Point3d>* instance, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray ret = cv::_InputArray(*instance);
			Ok(new cv::_InputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Point3dG_outputArray(std::vector<cv::Point3d>* instance, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray ret = cv::_OutputArray(*instance);
			Ok(new cv::_OutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void std_vectorLcv_Point3dG_inputOutputArray(std::vector<cv::Point3d>* instance, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray ret = cv::_InputOutputArray(*instance);
			Ok(new cv::_InputOutputArray(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
}


