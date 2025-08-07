extern "C" {
	const cv::flann::LinearIndexParams* cv_PtrLcv_flann_LinearIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::LinearIndexParams* cv_PtrLcv_flann_LinearIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_LinearIndexParamsG_delete(cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_LinearIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::LinearIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::LinearIndexParams>* cv_PtrLcv_flann_LinearIndexParamsG_new_const_LinearIndexParams(cv::flann::LinearIndexParams* val) {
			return new cv::Ptr<cv::flann::LinearIndexParams>(val);
	}
	
}

