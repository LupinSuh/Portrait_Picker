extern "C" {
	const cv::BRISK* cv_PtrLcv_BRISKG_getInnerPtr_const(const cv::Ptr<cv::BRISK>* instance) {
			return instance->get();
	}
	
	cv::BRISK* cv_PtrLcv_BRISKG_getInnerPtrMut(cv::Ptr<cv::BRISK>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_BRISKG_delete(cv::Ptr<cv::BRISK>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BRISKG_to_PtrOfAlgorithm(cv::Ptr<cv::BRISK>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_BRISKG_to_PtrOfFeature2D(cv::Ptr<cv::BRISK>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
	
}

