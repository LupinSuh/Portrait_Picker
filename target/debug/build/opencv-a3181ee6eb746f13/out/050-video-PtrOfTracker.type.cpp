extern "C" {
	const cv::Tracker* cv_PtrLcv_TrackerG_getInnerPtr_const(const cv::Ptr<cv::Tracker>* instance) {
			return instance->get();
	}
	
	cv::Tracker* cv_PtrLcv_TrackerG_getInnerPtrMut(cv::Ptr<cv::Tracker>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_TrackerG_delete(cv::Ptr<cv::Tracker>* instance) {
			delete instance;
	}
	
}

