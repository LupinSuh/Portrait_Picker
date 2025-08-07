extern "C" {
	std::vector<cvflann::lsh::FeatureIndex>* std_vectorLcvflann_lsh_FeatureIndexG_new_const() {
			std::vector<cvflann::lsh::FeatureIndex>* ret = new std::vector<cvflann::lsh::FeatureIndex>();
			return ret;
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_delete(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcvflann_lsh_FeatureIndexG_len_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcvflann_lsh_FeatureIndexG_isEmpty_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcvflann_lsh_FeatureIndexG_capacity_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_shrinkToFit(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_reserve_size_t(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_remove_size_t(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_swap_size_t_size_t(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_clear(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			instance->clear();
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_push_const_FeatureIndex(std::vector<cvflann::lsh::FeatureIndex>* instance, const cvflann::lsh::FeatureIndex val) {
			instance->push_back(val);
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_insert_size_t_const_FeatureIndex(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index, const cvflann::lsh::FeatureIndex val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_get_const_size_t(const std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index, cvflann::lsh::FeatureIndex* ocvrs_return) {
			cvflann::lsh::FeatureIndex ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLcvflann_lsh_FeatureIndexG_set_size_t_const_FeatureIndex(std::vector<cvflann::lsh::FeatureIndex>* instance, size_t index, const cvflann::lsh::FeatureIndex val) {
			(*instance)[index] = val;
	}
	
	std::vector<cvflann::lsh::FeatureIndex>* std_vectorLcvflann_lsh_FeatureIndexG_clone_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			std::vector<cvflann::lsh::FeatureIndex> ret = std::vector<cvflann::lsh::FeatureIndex>(*instance);
			return new std::vector<cvflann::lsh::FeatureIndex>(ret);
	}
	
	const cvflann::lsh::FeatureIndex* std_vectorLcvflann_lsh_FeatureIndexG_data_const(const std::vector<cvflann::lsh::FeatureIndex>* instance) {
			const cvflann::lsh::FeatureIndex* ret = instance->data();
			return ret;
	}
	
	cvflann::lsh::FeatureIndex* std_vectorLcvflann_lsh_FeatureIndexG_dataMut(std::vector<cvflann::lsh::FeatureIndex>* instance) {
			cvflann::lsh::FeatureIndex* ret = instance->data();
			return ret;
	}
	
	std::vector<cvflann::lsh::FeatureIndex>* cv_fromSlice_const_const_FeatureIndexX_size_t(const cvflann::lsh::FeatureIndex* data, size_t len) {
			return new std::vector<cvflann::lsh::FeatureIndex>(data, data + len);
	}
	
}


