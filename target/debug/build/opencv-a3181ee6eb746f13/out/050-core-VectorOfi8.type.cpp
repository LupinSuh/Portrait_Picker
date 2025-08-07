extern "C" {
	std::vector<signed char>* std_vectorLsigned_charG_new_const() {
			std::vector<signed char>* ret = new std::vector<signed char>();
			return ret;
	}
	
	void std_vectorLsigned_charG_delete(std::vector<signed char>* instance) {
			delete instance;
	}
	
	size_t std_vectorLsigned_charG_len_const(const std::vector<signed char>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLsigned_charG_isEmpty_const(const std::vector<signed char>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLsigned_charG_capacity_const(const std::vector<signed char>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLsigned_charG_shrinkToFit(std::vector<signed char>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLsigned_charG_reserve_size_t(std::vector<signed char>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLsigned_charG_remove_size_t(std::vector<signed char>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLsigned_charG_swap_size_t_size_t(std::vector<signed char>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLsigned_charG_clear(std::vector<signed char>* instance) {
			instance->clear();
	}
	
	void std_vectorLsigned_charG_push_const_signed_char(std::vector<signed char>* instance, const signed char val) {
			instance->push_back(val);
	}
	
	void std_vectorLsigned_charG_insert_size_t_const_signed_char(std::vector<signed char>* instance, size_t index, const signed char val) {
			instance->insert(instance->begin() + index, val);
	}
	
	void std_vectorLsigned_charG_get_const_size_t(const std::vector<signed char>* instance, size_t index, signed char* ocvrs_return) {
			signed char ret = (*instance)[index];
			*ocvrs_return = ret;
	}
	
	void std_vectorLsigned_charG_set_size_t_const_signed_char(std::vector<signed char>* instance, size_t index, const signed char val) {
			(*instance)[index] = val;
	}
	
	std::vector<signed char>* std_vectorLsigned_charG_clone_const(const std::vector<signed char>* instance) {
			std::vector<signed char> ret = std::vector<signed char>(*instance);
			return new std::vector<signed char>(ret);
	}
	
	const signed char* std_vectorLsigned_charG_data_const(const std::vector<signed char>* instance) {
			const signed char* ret = instance->data();
			return ret;
	}
	
	signed char* std_vectorLsigned_charG_dataMut(std::vector<signed char>* instance) {
			signed char* ret = instance->data();
			return ret;
	}
	
	std::vector<signed char>* cv_fromSlice_const_const_signed_charX_size_t(const signed char* data, size_t len) {
			return new std::vector<signed char>(data, data + len);
	}
	
}


