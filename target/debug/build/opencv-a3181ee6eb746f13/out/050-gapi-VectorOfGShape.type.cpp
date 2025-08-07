extern "C" {
	std::vector<cv::GShape>* std_vectorLcv_GShapeG_new_const() {
			std::vector<cv::GShape>* ret = new std::vector<cv::GShape>();
			return ret;
	}
	
	void std_vectorLcv_GShapeG_delete(std::vector<cv::GShape>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_GShapeG_len_const(const std::vector<cv::GShape>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_GShapeG_isEmpty_const(const std::vector<cv::GShape>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_GShapeG_capacity_const(const std::vector<cv::GShape>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_GShapeG_shrinkToFit(std::vector<cv::GShape>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_GShapeG_reserve_size_t(std::vector<cv::GShape>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_GShapeG_remove_size_t(std::vector<cv::GShape>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_GShapeG_swap_size_t_size_t(std::vector<cv::GShape>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_GShapeG_clear(std::vector<cv::GShape>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_GShapeG_push_const_GShape(std::vector<cv::GShape>* instance, const cv::GShape val) {
			instance->push_back(val);
	}
	
	void std_vectorLcv_GShapeG_insert_size_t_const_GShape(std::vector<cv::GShape>* instance, size_t index, const cv::GShape val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLcv_GShapeG_get_const_size_t(const std::vector<cv::GShape>* instance, size_t index, cv::GShape* ocvrs_return) {
			cv::GShape ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcv_GShapeG_set_size_t_const_GShape(std::vector<cv::GShape>* instance, size_t index, const cv::GShape val) {
			(*instance)[index] = val;
	}
	
	std::vector<cv::GShape>* std_vectorLcv_GShapeG_clone_const(const std::vector<cv::GShape>* instance) {
			std::vector<cv::GShape> ret = std::vector<cv::GShape>(*instance);
			return new std::vector<cv::GShape>(ret);
	}
	
	const cv::GShape* std_vectorLcv_GShapeG_data_const(const std::vector<cv::GShape>* instance) {
			const cv::GShape* ret = instance->data();
			return ret;
	}
	
	cv::GShape* std_vectorLcv_GShapeG_dataMut(std::vector<cv::GShape>* instance) {
			cv::GShape* ret = instance->data();
			return ret;
	}
	
	std::vector<cv::GShape>* cv_fromSlice_const_const_GShapeX_size_t(const cv::GShape* data, size_t len) {
			return new std::vector<cv::GShape>(data, data + len);
	}
	
}


