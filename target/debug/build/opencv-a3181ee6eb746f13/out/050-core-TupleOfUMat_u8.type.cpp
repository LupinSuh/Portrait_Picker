extern "C" {
	std::pair<cv::UMat, unsigned char>* std_pairLcv_UMat__unsigned_charG_new_const_UMat_unsigned_char(cv::UMat* arg, unsigned char arg_1) {
			std::pair<cv::UMat, unsigned char>* ret = new std::pair<cv::UMat, unsigned char>(*arg, arg_1);
			return ret;
	}
	
	void std_pairLcv_UMat__unsigned_charG_get_0_const(const std::pair<cv::UMat, unsigned char>* instance, cv::UMat** ocvrs_return) {
			cv::UMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::UMat(ret);
	}
	
	void std_pairLcv_UMat__unsigned_charG_get_1_const(const std::pair<cv::UMat, unsigned char>* instance, unsigned char* ocvrs_return) {
			unsigned char ret = std::get<1>(*instance);
			*ocvrs_return = ret;
	}
	
	void std_pairLcv_UMat__unsigned_charG_delete(std::pair<cv::UMat, unsigned char>* instance) {
			delete instance;
	}
	
}

