extern "C" {
	std::vector<cv::face::FacemarkAAM::Model::Texture>* std_vectorLcv_face_FacemarkAAM_Model_TextureG_new_const() {
			std::vector<cv::face::FacemarkAAM::Model::Texture>* ret = new std::vector<cv::face::FacemarkAAM::Model::Texture>();
			return ret;
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_delete(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_face_FacemarkAAM_Model_TextureG_len_const(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_face_FacemarkAAM_Model_TextureG_isEmpty_const(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_face_FacemarkAAM_Model_TextureG_capacity_const(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_shrinkToFit(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_reserve_size_t(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_remove_size_t(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_swap_size_t_size_t(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_clear(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_push_const_Texture(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, const cv::face::FacemarkAAM::Model::Texture* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_insert_size_t_const_Texture(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, const cv::face::FacemarkAAM::Model::Texture* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_get_const_size_t(const std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, cv::face::FacemarkAAM::Model::Texture** ocvrs_return) {
			cv::face::FacemarkAAM::Model::Texture ret = (*instance)[index];
			*ocvrs_return = new cv::face::FacemarkAAM::Model::Texture(ret);
	}
	
	void std_vectorLcv_face_FacemarkAAM_Model_TextureG_set_size_t_const_Texture(std::vector<cv::face::FacemarkAAM::Model::Texture>* instance, size_t index, const cv::face::FacemarkAAM::Model::Texture* val) {
			(*instance)[index] = *val;
	}
	
}


