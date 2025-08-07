extern "C" {
	const cv::xfeatures2d::TBMR* cv_PtrLcv_xfeatures2d_TBMRG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::TBMR* cv_PtrLcv_xfeatures2d_TBMRG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_TBMRG_delete(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
	cv::Ptr<cv::xfeatures2d::AffineFeature2D>* cv_PtrLcv_xfeatures2d_TBMRG_to_PtrOfAffineFeature2D(cv::Ptr<cv::xfeatures2d::TBMR>* instance) {
			return new cv::Ptr<cv::xfeatures2d::AffineFeature2D>(instance->dynamicCast<cv::xfeatures2d::AffineFeature2D>());
	}
	
}

