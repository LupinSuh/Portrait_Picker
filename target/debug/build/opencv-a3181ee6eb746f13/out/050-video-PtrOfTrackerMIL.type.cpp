extern "C" {
	const cv::TrackerMIL* cv_PtrLcv_TrackerMILG_getInnerPtr_const(const cv::Ptr<cv::TrackerMIL>* instance) {
			return instance->get();
	}
	
	cv::TrackerMIL* cv_PtrLcv_TrackerMILG_getInnerPtrMut(cv::Ptr<cv::TrackerMIL>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TrackerMILG_delete(cv::Ptr<cv::TrackerMIL>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerMILG_to_PtrOfTracker(cv::Ptr<cv::TrackerMIL>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}
	
}

