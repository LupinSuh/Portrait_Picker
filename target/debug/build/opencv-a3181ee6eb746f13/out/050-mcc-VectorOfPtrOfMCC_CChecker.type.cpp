extern "C" {
	std::vector<cv::Ptr<cv::mcc::CChecker>>* std_vectorLcv_PtrLcv_mcc_CCheckerGG_new_const() {
			std::vector<cv::Ptr<cv::mcc::CChecker>>* ret = new std::vector<cv::Ptr<cv::mcc::CChecker>>();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_delete(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_PtrLcv_mcc_CCheckerGG_len_const(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_PtrLcv_mcc_CCheckerGG_isEmpty_const(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_PtrLcv_mcc_CCheckerGG_capacity_const(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_shrinkToFit(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_reserve_size_t(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_remove_size_t(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_swap_size_t_size_t(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_clear(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_push_const_PtrLCCheckerG(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, const cv::Ptr<cv::mcc::CChecker>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_insert_size_t_const_PtrLCCheckerG(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, const cv::Ptr<cv::mcc::CChecker>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_get_const_size_t(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>** ocvrs_return) {
			cv::Ptr<cv::mcc::CChecker> ret = (*instance)[index];
			*ocvrs_return = new cv::Ptr<cv::mcc::CChecker>(ret);
	}
	
	void std_vectorLcv_PtrLcv_mcc_CCheckerGG_set_size_t_const_PtrLCCheckerG(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, const cv::Ptr<cv::mcc::CChecker>* val) {
			(*instance)[index] = *val;
	}
	
}


