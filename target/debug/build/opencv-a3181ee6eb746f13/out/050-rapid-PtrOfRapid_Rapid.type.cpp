extern "C" {
	const cv::rapid::Rapid* cv_PtrLcv_rapid_RapidG_getInnerPtr_const(const cv::Ptr<cv::rapid::Rapid>* instance) {
			return instance->get();
	}
	
	cv::rapid::Rapid* cv_PtrLcv_rapid_RapidG_getInnerPtrMut(cv::Ptr<cv::rapid::Rapid>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_rapid_RapidG_delete(cv::Ptr<cv::rapid::Rapid>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rapid_RapidG_to_PtrOfAlgorithm(cv::Ptr<cv::rapid::Rapid>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::rapid::Tracker>* cv_PtrLcv_rapid_RapidG_to_PtrOfRapid_Tracker(cv::Ptr<cv::rapid::Rapid>* instance) {
			return new cv::Ptr<cv::rapid::Tracker>(instance->dynamicCast<cv::rapid::Tracker>());
	}
	
}

