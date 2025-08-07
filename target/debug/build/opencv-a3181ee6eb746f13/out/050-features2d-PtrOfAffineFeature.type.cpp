extern "C" {
	const cv::AffineFeature* cv_PtrLcv_AffineFeatureG_getInnerPtr_const(const cv::Ptr<cv::AffineFeature>* instance) {
			return instance->get();
	}
	
	cv::AffineFeature* cv_PtrLcv_AffineFeatureG_getInnerPtrMut(cv::Ptr<cv::AffineFeature>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_AffineFeatureG_delete(cv::Ptr<cv::AffineFeature>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_AffineFeatureG_to_PtrOfAlgorithm(cv::Ptr<cv::AffineFeature>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_AffineFeatureG_to_PtrOfFeature2D(cv::Ptr<cv::AffineFeature>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

