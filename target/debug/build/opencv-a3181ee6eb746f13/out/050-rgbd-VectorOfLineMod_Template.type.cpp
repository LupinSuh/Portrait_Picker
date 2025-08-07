extern "C" {
	std::vector<cv::linemod::Template>* std_vectorLcv_linemod_TemplateG_new_const() {
			std::vector<cv::linemod::Template>* ret = new std::vector<cv::linemod::Template>();
			return ret;
	}
	
	void std_vectorLcv_linemod_TemplateG_delete(std::vector<cv::linemod::Template>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_linemod_TemplateG_len_const(const std::vector<cv::linemod::Template>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_linemod_TemplateG_isEmpty_const(const std::vector<cv::linemod::Template>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_linemod_TemplateG_capacity_const(const std::vector<cv::linemod::Template>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_linemod_TemplateG_shrinkToFit(std::vector<cv::linemod::Template>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_linemod_TemplateG_reserve_size_t(std::vector<cv::linemod::Template>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_linemod_TemplateG_remove_size_t(std::vector<cv::linemod::Template>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_linemod_TemplateG_swap_size_t_size_t(std::vector<cv::linemod::Template>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_linemod_TemplateG_clear(std::vector<cv::linemod::Template>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_linemod_TemplateG_push_const_Template(std::vector<cv::linemod::Template>* instance, const cv::linemod::Template* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_linemod_TemplateG_insert_size_t_const_Template(std::vector<cv::linemod::Template>* instance, size_t index, const cv::linemod::Template* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_linemod_TemplateG_get_const_size_t(const std::vector<cv::linemod::Template>* instance, size_t index, cv::linemod::Template** ocvrs_return) {
			cv::linemod::Template ret = (*instance)[index];
			*ocvrs_return = new cv::linemod::Template(ret);
	}
	
	void std_vectorLcv_linemod_TemplateG_set_size_t_const_Template(std::vector<cv::linemod::Template>* instance, size_t index, const cv::linemod::Template* val) {
			(*instance)[index] = *val;
	}
	
}


