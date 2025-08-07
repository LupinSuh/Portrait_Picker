extern "C" {
	const cv::GeneralizedHoughGuil* cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtr_const(const cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return instance->get();
	}
	
	cv::GeneralizedHoughGuil* cv_PtrLcv_GeneralizedHoughGuilG_getInnerPtrMut(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_GeneralizedHoughGuilG_delete(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfAlgorithm(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::GeneralizedHough>* cv_PtrLcv_GeneralizedHoughGuilG_to_PtrOfGeneralizedHough(cv::Ptr<cv::GeneralizedHoughGuil>* instance) {
			return new cv::Ptr<cv::GeneralizedHough>(instance->dynamicCast<cv::GeneralizedHough>());
	}
	
}

