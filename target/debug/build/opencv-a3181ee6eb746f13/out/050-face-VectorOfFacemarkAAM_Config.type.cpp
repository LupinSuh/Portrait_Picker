extern "C" {
	std::vector<cv::face::FacemarkAAM::Config>* std_vectorLcv_face_FacemarkAAM_ConfigG_new_const() {
			std::vector<cv::face::FacemarkAAM::Config>* ret = new std::vector<cv::face::FacemarkAAM::Config>();
			return ret;
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_delete(std::vector<cv::face::FacemarkAAM::Config>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_face_FacemarkAAM_ConfigG_len_const(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_face_FacemarkAAM_ConfigG_isEmpty_const(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_face_FacemarkAAM_ConfigG_capacity_const(const std::vector<cv::face::FacemarkAAM::Config>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_shrinkToFit(std::vector<cv::face::FacemarkAAM::Config>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_reserve_size_t(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_remove_size_t(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_swap_size_t_size_t(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_clear(std::vector<cv::face::FacemarkAAM::Config>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_push_const_Config(std::vector<cv::face::FacemarkAAM::Config>* instance, const cv::face::FacemarkAAM::Config* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_insert_size_t_const_Config(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, const cv::face::FacemarkAAM::Config* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_get_const_size_t(const std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, cv::face::FacemarkAAM::Config** ocvrs_return) {
			cv::face::FacemarkAAM::Config ret = (*instance)[index];
			*ocvrs_return = new cv::face::FacemarkAAM::Config(ret);
	}
	
	void std_vectorLcv_face_FacemarkAAM_ConfigG_set_size_t_const_Config(std::vector<cv::face::FacemarkAAM::Config>* instance, size_t index, const cv::face::FacemarkAAM::Config* val) {
			(*instance)[index] = *val;
	}
	
}


