extern "C" {
	std::vector<std::pair<int, double>>* std_vectorLstd_pairLint__doubleGG_new_const() {
			std::vector<std::pair<int, double>>* ret = new std::vector<std::pair<int, double>>();
			return ret;
	}
	
	void std_vectorLstd_pairLint__doubleGG_delete(std::vector<std::pair<int, double>>* instance) {
			delete instance;
	}
	
	size_t std_vectorLstd_pairLint__doubleGG_len_const(const std::vector<std::pair<int, double>>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLstd_pairLint__doubleGG_isEmpty_const(const std::vector<std::pair<int, double>>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLstd_pairLint__doubleGG_capacity_const(const std::vector<std::pair<int, double>>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLstd_pairLint__doubleGG_shrinkToFit(std::vector<std::pair<int, double>>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLstd_pairLint__doubleGG_reserve_size_t(std::vector<std::pair<int, double>>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLstd_pairLint__doubleGG_remove_size_t(std::vector<std::pair<int, double>>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLstd_pairLint__doubleGG_swap_size_t_size_t(std::vector<std::pair<int, double>>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLstd_pairLint__doubleGG_clear(std::vector<std::pair<int, double>>* instance) {
			instance->clear();
	}
	
	void std_vectorLstd_pairLint__doubleGG_push_const_pairLint__doubleG(std::vector<std::pair<int, double>>* instance, const std::pair<int, double>* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLstd_pairLint__doubleGG_insert_size_t_const_pairLint__doubleG(std::vector<std::pair<int, double>>* instance, size_t index, const std::pair<int, double>* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLstd_pairLint__doubleGG_get_const_size_t(const std::vector<std::pair<int, double>>* instance, size_t index, std::pair<int, double>** ocvrs_return) {
			std::pair<int, double> ret = (*instance)[index];
			*ocvrs_return = new std::pair<int, double>(ret);
	}
	
	void std_vectorLstd_pairLint__doubleGG_set_size_t_const_pairLint__doubleG(std::vector<std::pair<int, double>>* instance, size_t index, const std::pair<int, double>* val) {
			(*instance)[index] = *val;
	}
	
}


