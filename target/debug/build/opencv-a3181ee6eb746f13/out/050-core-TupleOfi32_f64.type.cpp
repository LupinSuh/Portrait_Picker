extern "C" {
	std::pair<int, double>* std_pairLint__doubleG_new_const_int_double(int arg, double arg_1) {
			std::pair<int, double>* ret = new std::pair<int, double>(arg, arg_1);
			return ret;
	}
	
	void std_pairLint__doubleG_get_0_const(const std::pair<int, double>* instance, int* ocvrs_return) {
			int ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}
	
	void std_pairLint__doubleG_get_1_const(const std::pair<int, double>* instance, double* ocvrs_return) {
			double ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}
	
	void std_pairLint__doubleG_delete(std::pair<int, double>* instance) {
			delete instance;
	}
	
}

