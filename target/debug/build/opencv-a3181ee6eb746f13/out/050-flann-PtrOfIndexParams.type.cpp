extern "C" {
	const cv::flann::IndexParams* cv_PtrLcv_flann_IndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::IndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::IndexParams* cv_PtrLcv_flann_IndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::IndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_IndexParamsG_delete(cv::Ptr<cv::flann::IndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_IndexParamsG_new_const_IndexParams(cv::flann::IndexParams* val) {
			return new cv::Ptr<cv::flann::IndexParams>(val);
	}
	
}

