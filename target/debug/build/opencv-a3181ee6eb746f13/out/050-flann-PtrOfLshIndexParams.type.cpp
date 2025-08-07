extern "C" {
	const cv::flann::LshIndexParams* cv_PtrLcv_flann_LshIndexParamsG_getInnerPtr_const(const cv::Ptr<cv::flann::LshIndexParams>* instance) {
			return instance->get();
	}
	
	cv::flann::LshIndexParams* cv_PtrLcv_flann_LshIndexParamsG_getInnerPtrMut(cv::Ptr<cv::flann::LshIndexParams>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_flann_LshIndexParamsG_delete(cv::Ptr<cv::flann::LshIndexParams>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::flann::IndexParams>* cv_PtrLcv_flann_LshIndexParamsG_to_PtrOfIndexParams(cv::Ptr<cv::flann::LshIndexParams>* instance) {
			return new cv::Ptr<cv::flann::IndexParams>(instance->dynamicCast<cv::flann::IndexParams>());
	}
	
	cv::Ptr<cv::flann::LshIndexParams>* cv_PtrLcv_flann_LshIndexParamsG_new_const_LshIndexParams(cv::flann::LshIndexParams* val) {
			return new cv::Ptr<cv::flann::LshIndexParams>(val);
	}
	
}

