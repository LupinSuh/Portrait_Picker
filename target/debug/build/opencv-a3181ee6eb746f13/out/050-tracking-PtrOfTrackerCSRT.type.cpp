extern "C" {
	const cv::tracking::TrackerCSRT* cv_PtrLcv_tracking_TrackerCSRTG_getInnerPtr_const(const cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			return instance->get();
	}
	
	cv::tracking::TrackerCSRT* cv_PtrLcv_tracking_TrackerCSRTG_getInnerPtrMut(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_tracking_TrackerCSRTG_delete(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Tracker>* cv_PtrLcv_tracking_TrackerCSRTG_to_PtrOfTracker(cv::Ptr<cv::tracking::TrackerCSRT>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}
	
}

