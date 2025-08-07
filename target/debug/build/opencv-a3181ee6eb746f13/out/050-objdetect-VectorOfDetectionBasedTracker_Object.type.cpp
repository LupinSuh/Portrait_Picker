extern "C" {
	std::vector<cv::DetectionBasedTracker::Object>* std_vectorLcv_DetectionBasedTracker_ObjectG_new_const() {
			std::vector<cv::DetectionBasedTracker::Object>* ret = new std::vector<cv::DetectionBasedTracker::Object>();
			return ret;
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_delete(std::vector<cv::DetectionBasedTracker::Object>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_DetectionBasedTracker_ObjectG_len_const(const std::vector<cv::DetectionBasedTracker::Object>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_DetectionBasedTracker_ObjectG_isEmpty_const(const std::vector<cv::DetectionBasedTracker::Object>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_DetectionBasedTracker_ObjectG_capacity_const(const std::vector<cv::DetectionBasedTracker::Object>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_shrinkToFit(std::vector<cv::DetectionBasedTracker::Object>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_reserve_size_t(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_remove_size_t(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_swap_size_t_size_t(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_clear(std::vector<cv::DetectionBasedTracker::Object>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_push_const_Object(std::vector<cv::DetectionBasedTracker::Object>* instance, const cv::DetectionBasedTracker::Object* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_insert_size_t_const_Object(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index, const cv::DetectionBasedTracker::Object* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_get_const_size_t(const std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index, cv::DetectionBasedTracker::Object** ocvrs_return) {
			cv::DetectionBasedTracker::Object ret = (*instance)[index];
			*ocvrs_return = new cv::DetectionBasedTracker::Object(ret);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ObjectG_set_size_t_const_Object(std::vector<cv::DetectionBasedTracker::Object>* instance, size_t index, const cv::DetectionBasedTracker::Object* val) {
			(*instance)[index] = *val;
	}
	
}


