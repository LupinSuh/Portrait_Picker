extern "C" {
	const cv::xfeatures2d::PCTSignaturesSQFD* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_getInnerPtr_const(const cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			return instance->get();
	}
	
	cv::xfeatures2d::PCTSignaturesSQFD* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_getInnerPtrMut(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_delete(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xfeatures2d_PCTSignaturesSQFDG_to_PtrOfAlgorithm(cv::Ptr<cv::xfeatures2d::PCTSignaturesSQFD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

