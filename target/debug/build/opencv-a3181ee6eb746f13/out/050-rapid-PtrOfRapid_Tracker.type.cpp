extern "C" {
	const cv::rapid::Tracker* cv_PtrLcv_rapid_TrackerG_getInnerPtr_const(const cv::Ptr<cv::rapid::Tracker>* instance) {
			return instance->get();
	}
	
	cv::rapid::Tracker* cv_PtrLcv_rapid_TrackerG_getInnerPtrMut(cv::Ptr<cv::rapid::Tracker>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rapid_TrackerG_delete(cv::Ptr<cv::rapid::Tracker>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_TrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::Tracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

