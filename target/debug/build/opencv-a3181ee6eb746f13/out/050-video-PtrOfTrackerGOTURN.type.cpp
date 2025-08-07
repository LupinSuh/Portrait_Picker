extern "C" {
	const cv::TrackerGOTURN* cv_PtrLcv_TrackerGOTURNG_getInnerPtr_const(const cv::Ptr<cv::TrackerGOTURN>* instance) {
			return instance->get();
	}
	
	cv::TrackerGOTURN* cv_PtrLcv_TrackerGOTURNG_getInnerPtrMut(cv::Ptr<cv::TrackerGOTURN>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TrackerGOTURNG_delete(cv::Ptr<cv::TrackerGOTURN>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerGOTURNG_to_PtrOfTracker(cv::Ptr<cv::TrackerGOTURN>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}
	
}

