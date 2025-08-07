extern "C" {
	std::vector<cv::Ptr<cv::linemod::Modality>>* std_vectorLcv_PtrLcv_linemod_ModalityGG_new_const() {
			std::vector<cv::Ptr<cv::linemod::Modality>>* ret = new std::vector<cv::Ptr<cv::linemod::Modality>>();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_delete(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_PtrLcv_linemod_ModalityGG_len_const(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_PtrLcv_linemod_ModalityGG_isEmpty_const(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_PtrLcv_linemod_ModalityGG_capacity_const(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_shrinkToFit(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_reserve_size_t(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_remove_size_t(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_clear(std::vector<cv::Ptr<cv::linemod::Modality>>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_push_const_PtrLModalityG(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, const cv::Ptr<cv::linemod::Modality>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_insert_size_t_const_PtrLModalityG(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, const cv::Ptr<cv::linemod::Modality>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_get_const_size_t(const std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, cv::Ptr<cv::linemod::Modality>** ocvrs_return) {
			cv::Ptr<cv::linemod::Modality> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::linemod::Modality>(ret);
	}
	
	void std_vectorLcv_PtrLcv_linemod_ModalityGG_set_size_t_const_PtrLModalityG(std::vector<cv::Ptr<cv::linemod::Modality>>* instance, size_t index, const cv::Ptr<cv::linemod::Modality>* val) {
			(*instance)[index] = *val;
	}
	
}


