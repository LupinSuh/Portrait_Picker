extern "C" {
	std::vector<cv::DetectionBasedTracker::ExtObject>* std_vectorLcv_DetectionBasedTracker_ExtObjectG_new_const() {
			std::vector<cv::DetectionBasedTracker::ExtObject>* ret = new std::vector<cv::DetectionBasedTracker::ExtObject>();
			return ret;
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_delete(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_DetectionBasedTracker_ExtObjectG_len_const(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_DetectionBasedTracker_ExtObjectG_isEmpty_const(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_DetectionBasedTracker_ExtObjectG_capacity_const(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_shrinkToFit(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_reserve_size_t(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_remove_size_t(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_swap_size_t_size_t(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_clear(std::vector<cv::DetectionBasedTracker::ExtObject>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_push_const_ExtObject(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, const cv::DetectionBasedTracker::ExtObject* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_insert_size_t_const_ExtObject(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, const cv::DetectionBasedTracker::ExtObject* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_get_const_size_t(const std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, cv::DetectionBasedTracker::ExtObject** ocvrs_return) {
			cv::DetectionBasedTracker::ExtObject ret = (*instance)[index];
			*ocvrs_return = new cv::DetectionBasedTracker::ExtObject(ret);
	}
	
	void std_vectorLcv_DetectionBasedTracker_ExtObjectG_set_size_t_const_ExtObject(std::vector<cv::DetectionBasedTracker::ExtObject>* instance, size_t index, const cv::DetectionBasedTracker::ExtObject* val) {
			(*instance)[index] = *val;
	}
	
}


