extern "C" {
	const cv::rapid::OLSTracker* cv_PtrLcv_rapid_OLSTrackerG_getInnerPtr_const(const cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return instance->get();
	}
	
	cv::rapid::OLSTracker* cv_PtrLcv_rapid_OLSTrackerG_getInnerPtrMut(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rapid_OLSTrackerG_delete(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_OLSTrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_OLSTrackerG_to_PtrOfRapid_Tracker(cv::Ptr<cv::rapid::OLSTracker>* instance) {
			return new cv::Ptr<cv::rapid::Tracker>(instance->dynamicCast<cv::rapid::Tracker>());
	}
	
}

