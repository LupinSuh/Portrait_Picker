extern "C" {
	std::pair<cv::Rect, int>* std_pairLcv_Rect__intG_new_const_Rect_int(cv::Rect* arg, int arg_1) {
			std::pair<cv::Rect, int>* ret = new std::pair<cv::Rect, int>(*arg, arg_1);
			return ret;
	}
	
	void std_pairLcv_Rect__intG_get_0_const(const std::pair<cv::Rect, int>* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = std::get<0>(*instance);
			*ocvrs_return = ret;
	}
	
	void std_pairLcv_Rect__intG_get_1_const(const std::pair<cv::Rect, int>* instance, int* ocvrs_return) {
			int ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}
	
	void std_pairLcv_Rect__intG_delete(std::pair<cv::Rect, int>* instance) {
			delete instance;
	}
	
}

