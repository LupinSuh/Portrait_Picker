extern "C" {
	const cv::TrackerNano* cv_PtrLcv_TrackerNanoG_getInnerPtr_const(const cv::Ptr<cv::TrackerNano>* instance) {
			return instance->get();
	}
	
	cv::TrackerNano* cv_PtrLcv_TrackerNanoG_getInnerPtrMut(cv::Ptr<cv::TrackerNano>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TrackerNanoG_delete(cv::Ptr<cv::TrackerNano>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerNanoG_to_PtrOfTracker(cv::Ptr<cv::TrackerNano>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}
	
}

