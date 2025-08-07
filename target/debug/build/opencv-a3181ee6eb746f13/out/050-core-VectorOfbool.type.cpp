extern "C" {
	std::vector<bool>* std_vectorLboolG_new_const() {
			std::vector<bool>* ret = new std::vector<bool>();
			return ret;
	}
	
	void std_vectorLboolG_delete(std::vector<bool>* instance) {
			delete instance;
	}
	
	size_t std_vectorLboolG_len_const(const std::vector<bool>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLboolG_isEmpty_const(const std::vector<bool>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLboolG_capacity_const(const std::vector<bool>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLboolG_shrinkToFit(std::vector<bool>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLboolG_reserve_size_t(std::vector<bool>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLboolG_remove_size_t(std::vector<bool>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLboolG_swap_size_t_size_t(std::vector<bool>* instance, size_t index1, size_t index2) {
			instance->swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLboolG_clear(std::vector<bool>* instance) {
			instance->clear();
	}
	
	void std_vectorLboolG_push_const_bool(std::vector<bool>* instance, const bool val) {
			instance->push_back(val);
	}
	
	void std_vectorLboolG_insert_size_t_const_bool(std::vector<bool>* instance, size_t index, const bool val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLboolG_get_const_size_t(const std::vector<bool>* instance, size_t index, bool* ocvrs_return) {
			bool ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLboolG_set_size_t_const_bool(std::vector<bool>* instance, size_t index, const bool val) {
			(*instance)[index] = val;
	}
	
}


