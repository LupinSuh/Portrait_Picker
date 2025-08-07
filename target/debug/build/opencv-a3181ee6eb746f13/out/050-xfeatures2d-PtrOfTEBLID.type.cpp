extern "C" {
	const cv::xfeatures2d::TEBLID* cv_PtrLcv_xfeatures2d_TEBLIDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::TEBLID* cv_PtrLcv_xfeatures2d_TEBLIDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_TEBLIDG_delete(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_TEBLIDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_TEBLIDG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::TEBLID>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
	cv::Ptr<cv::xfeatures2d::TEBLID>* cv_PtrLcv_xfeatures2d_TEBLIDG_new_const_TEBLID(cv::xfeatures2d::TEBLID* val) {
			return new cv::Ptr<cv::xfeatures2d::TEBLID>(val);
	}
	
}

