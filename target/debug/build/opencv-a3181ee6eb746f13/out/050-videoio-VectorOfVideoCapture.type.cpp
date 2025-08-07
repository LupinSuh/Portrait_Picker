extern "C" {
	std::vector<cv::VideoCapture>* std_vectorLcv_VideoCaptureG_new_const() {
			std::vector<cv::VideoCapture>* ret = new std::vector<cv::VideoCapture>();
			return ret;
	}
	
	void std_vectorLcv_VideoCaptureG_delete(std::vector<cv::VideoCapture>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_VideoCaptureG_len_const(const std::vector<cv::VideoCapture>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_VideoCaptureG_isEmpty_const(const std::vector<cv::VideoCapture>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_VideoCaptureG_capacity_const(const std::vector<cv::VideoCapture>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_VideoCaptureG_shrinkToFit(std::vector<cv::VideoCapture>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_VideoCaptureG_reserve_size_t(std::vector<cv::VideoCapture>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_VideoCaptureG_remove_size_t(std::vector<cv::VideoCapture>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_VideoCaptureG_swap_size_t_size_t(std::vector<cv::VideoCapture>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_VideoCaptureG_clear(std::vector<cv::VideoCapture>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_VideoCaptureG_push_const_VideoCapture(std::vector<cv::VideoCapture>* instance, const cv::VideoCapture* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_VideoCaptureG_insert_size_t_const_VideoCapture(std::vector<cv::VideoCapture>* instance, size_t index, const cv::VideoCapture* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_VideoCaptureG_get_const_size_t(const std::vector<cv::VideoCapture>* instance, size_t index, cv::VideoCapture** ocvrs_return) {
			cv::VideoCapture ret = (*instance)[index];
			*ocvrs_return = new cv::VideoCapture(ret);
	}
	
	void std_vectorLcv_VideoCaptureG_set_size_t_const_VideoCapture(std::vector<cv::VideoCapture>* instance, size_t index, const cv::VideoCapture* val) {
			(*instance)[index] = *val;
	}
	
}


