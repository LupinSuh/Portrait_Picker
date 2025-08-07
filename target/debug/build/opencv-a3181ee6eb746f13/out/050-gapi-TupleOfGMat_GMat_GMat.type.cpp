extern "C" {
	std::tuple<cv::GMat, cv::GMat, cv::GMat>* std_tupleLcv_GMat__cv_GMat__cv_GMatG_new_const_GMat_GMat_GMat(cv::GMat* arg, cv::GMat* arg_1, cv::GMat* arg_2) {
			std::tuple<cv::GMat, cv::GMat, cv::GMat>* ret = new std::tuple<cv::GMat, cv::GMat, cv::GMat>(*arg, *arg_1, *arg_2);
			return ret;
	}
	
	void std_tupleLcv_GMat__cv_GMat__cv_GMatG_get_0_const(const std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<0>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void std_tupleLcv_GMat__cv_GMat__cv_GMatG_get_1_const(const std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<1>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void std_tupleLcv_GMat__cv_GMat__cv_GMatG_get_2_const(const std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance, cv::GMat** ocvrs_return) {
			cv::GMat ret = std::get<2>(*instance);
			*ocvrs_return = new cv::GMat(ret);
	}
	
	void std_tupleLcv_GMat__cv_GMat__cv_GMatG_delete(std::tuple<cv::GMat, cv::GMat, cv::GMat>* instance) {
			delete instance;
	}
	
}

