extern "C" {
	const cv::xfeatures2d::DAISY* cv_PtrLcv_xfeatures2d_DAISYG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::DAISY* cv_PtrLcv_xfeatures2d_DAISYG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_DAISYG_delete(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_DAISYG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_DAISYG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::DAISY>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

