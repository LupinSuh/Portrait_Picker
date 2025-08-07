extern "C" {
	const cv::GeneralizedHoughBallard* cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtr_const(const cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return instance->get();
	}
	
	cv::GeneralizedHoughBallard* cv_PtrLcv_GeneralizedHoughBallardG_getInnerPtrMut(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_GeneralizedHoughBallardG_delete(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfAlgorithm(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::GeneralizedHough>* cv_PtrLcv_GeneralizedHoughBallardG_to_PtrOfGeneralizedHough(cv::Ptr<cv::GeneralizedHoughBallard>* instance) {
			return new cv::Ptr<cv::GeneralizedHough>(instance->dynamicCast<cv::GeneralizedHough>());
	}
	
}

