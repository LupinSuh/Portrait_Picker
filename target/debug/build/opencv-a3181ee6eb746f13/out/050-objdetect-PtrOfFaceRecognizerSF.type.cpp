extern "C" {
	const cv::FaceRecognizerSF* cv_PtrLcv_FaceRecognizerSFG_getInnerPtr_const(const cv::Ptr<cv::FaceRecognizerSF>* instance) {
			return instance->get();
	}
	
	cv::FaceRecognizerSF* cv_PtrLcv_FaceRecognizerSFG_getInnerPtrMut(cv::Ptr<cv::FaceRecognizerSF>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FaceRecognizerSFG_delete(cv::Ptr<cv::FaceRecognizerSF>* instance) {
			delete instance;
	}
	
}

