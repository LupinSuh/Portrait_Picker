extern "C" {
	const cv::rapid::GOSTracker* cv_PtrLcv_rapid_GOSTrackerG_getInnerPtr_const(const cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return instance->get();
	}
	
	cv::rapid::GOSTracker* cv_PtrLcv_rapid_GOSTrackerG_getInnerPtrMut(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rapid_GOSTrackerG_delete(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_GOSTrackerG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_GOSTrackerG_to_PtrOfRapid_Tracker(cv::Ptr<cv::rapid::GOSTracker>* instance) {
			return new cv::Ptr<cv::rapid::Tracker>(instance->dynamicCast<cv::rapid::Tracker>());
	}
	
}

