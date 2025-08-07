extern "C" {
	const cv::xfeatures2d::LUCID* cv_PtrLcv_xfeatures2d_LUCIDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::LUCID* cv_PtrLcv_xfeatures2d_LUCIDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_LUCIDG_delete(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_LUCIDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_LUCIDG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::LUCID>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

