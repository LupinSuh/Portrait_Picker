extern "C" {
	const cv::StereoSGBM* cv_PtrLcv_StereoSGBMG_getInnerPtr_const(const cv::Ptr<cv::StereoSGBM>* instance) {
			return instance->get();
	}
	
	cv::StereoSGBM* cv_PtrLcv_StereoSGBMG_getInnerPtrMut(cv::Ptr<cv::StereoSGBM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_StereoSGBMG_delete(cv::Ptr<cv::StereoSGBM>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_StereoSGBMG_to_PtrOfAlgorithm(cv::Ptr<cv::StereoSGBM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::StereoMatcher>* cv_PtrLcv_StereoSGBMG_to_PtrOfStereoMatcher(cv::Ptr<cv::StereoSGBM>* instance) {
			return new cv::Ptr<cv::StereoMatcher>(instance->dynamicCast<cv::StereoMatcher>());
	}
	
}

