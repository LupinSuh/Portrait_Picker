extern "C" {
	const cv::xfeatures2d::PCTSignatures* cv_PtrLcv_xfeatures2d_PCTSignaturesG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::PCTSignatures* cv_PtrLcv_xfeatures2d_PCTSignaturesG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_PCTSignaturesG_delete(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_PCTSignaturesG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

