extern "C" {
	const cv::TrackerVit* cv_PtrLcv_TrackerVitG_getInnerPtr_const(const cv::Ptr<cv::TrackerVit>* instance) {
			return instance->get();
	}
	
	cv::TrackerVit* cv_PtrLcv_TrackerVitG_getInnerPtrMut(cv::Ptr<cv::TrackerVit>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TrackerVitG_delete(cv::Ptr<cv::TrackerVit>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Tracker>* cv_PtrLcv_TrackerVitG_to_PtrOfTracker(cv::Ptr<cv::TrackerVit>* instance) {
			return new cv::Ptr<cv::Tracker>(instance->dynamicCast<cv::Tracker>());
	}
	
}

