extern "C" {
	const cv::ORB* cv_PtrLcv_ORBG_getInnerPtr_const(const cv::Ptr<cv::ORB>* instance) {
			return instance->get();
	}
	
	cv::ORB* cv_PtrLcv_ORBG_getInnerPtrMut(cv::Ptr<cv::ORB>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ORBG_delete(cv::Ptr<cv::ORB>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ORBG_to_PtrOfAlgorithm(cv::Ptr<cv::ORB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_ORBG_to_PtrOfFeature2D(cv::Ptr<cv::ORB>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

