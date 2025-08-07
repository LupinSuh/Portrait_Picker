extern "C" {
	const cv::xfeatures2d::FREAK* cv_PtrLcv_xfeatures2d_FREAKG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::FREAK* cv_PtrLcv_xfeatures2d_FREAKG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_FREAKG_delete(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_FREAKG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_xfeatures2d_FREAKG_to_PtrOfFeature2D(cv::Ptr<cv::xfeatures2d::FREAK>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

