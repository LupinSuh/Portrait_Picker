extern "C" {
	const cv::TrackerDaSiamRPN* cv_PtrLcv_TrackerDaSiamRPNG_getInnerPtr_const(const cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			return instance->get();
	}
	
	cv::TrackerDaSiamRPN* cv_PtrLcv_TrackerDaSiamRPNG_getInnerPtrMut(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TrackerDaSiamRPNG_delete(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerDaSiamRPNG_to_PtrOfTracker(cv::Ptr<cv::TrackerDaSiamRPN>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}
	
}

