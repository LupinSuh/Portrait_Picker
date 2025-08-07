extern "C" {
	const cv::AKAZE* cv_PtrLcv_AKAZEG_getInnerPtr_const(const cv::Ptr<cv::AKAZE>* instance) {
			return instance->get();
	}
	
	cv::AKAZE* cv_PtrLcv_AKAZEG_getInnerPtrMut(cv::Ptr<cv::AKAZE>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AKAZEG_delete(cv::Ptr<cv::AKAZE>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AKAZEG_to_PtrOfAlgorithm(cv::Ptr<cv::AKAZE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_AKAZEG_to_PtrOfFeature2D(cv::Ptr<cv::AKAZE>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

