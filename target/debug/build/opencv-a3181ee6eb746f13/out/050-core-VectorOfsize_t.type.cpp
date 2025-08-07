extern "C" {
	std::vector<size_t>* std_vectorLsize_tG_new_const() {
			std::vector<size_t>* ret = new std::vector<size_t>();
			return ret;
	}
	
	void std_vectorLsize_tG_delete(std::vector<size_t>* instance) {
			delete instance;
	}
	
	size_t std_vectorLsize_tG_len_const(const std::vector<size_t>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLsize_tG_isEmpty_const(const std::vector<size_t>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLsize_tG_capacity_const(const std::vector<size_t>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLsize_tG_shrinkToFit(std::vector<size_t>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLsize_tG_reserve_size_t(std::vector<size_t>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLsize_tG_remove_size_t(std::vector<size_t>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLsize_tG_swap_size_t_size_t(std::vector<size_t>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLsize_tG_clear(std::vector<size_t>* instance) {
			instance->clear();
	}
	
	void std_vectorLsize_tG_push_const_size_t(std::vector<size_t>* instance, const size_t val) {
			instance->push_back(val);
	}
	
	void std_vectorLsize_tG_insert_size_t_const_size_t(std::vector<size_t>* instance, size_t index, const size_t val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLsize_tG_get_const_size_t(const std::vector<size_t>* instance, size_t index, size_t* ocvrs_return) {
			size_t ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLsize_tG_set_size_t_const_size_t(std::vector<size_t>* instance, size_t index, const size_t val) {
			(*instance)[index] = val;
	}
	
	std::vector<size_t>* std_vectorLsize_tG_clone_const(const std::vector<size_t>* instance) {
			std::vector<size_t> ret = std::vector<size_t>(*instance);
			return new std::vector<size_t>(ret);
	}
	
	const size_t* std_vectorLsize_tG_data_const(const std::vector<size_t>* instance) {
			const size_t* ret = instance->data();
			return ret;
	}
	
	size_t* std_vectorLsize_tG_dataMut(std::vector<size_t>* instance) {
			size_t* ret = instance->data();
			return ret;
	}
	
	std::vector<size_t>* cv_fromSlice_const_const_size_tX_size_t(const size_t* data, size_t len) {
			return new std::vector<size_t>(data, data + len);
	}
	
}


