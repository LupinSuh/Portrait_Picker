extern "C" {
	const cv::mcc::CCheckerDetector* cv_PtrLcv_mcc_CCheckerDetectorG_getInnerPtr_const(const cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			return instance->get();
	}
	
	cv::mcc::CCheckerDetector* cv_PtrLcv_mcc_CCheckerDetectorG_getInnerPtrMut(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_mcc_CCheckerDetectorG_delete(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_mcc_CCheckerDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::mcc::CCheckerDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

