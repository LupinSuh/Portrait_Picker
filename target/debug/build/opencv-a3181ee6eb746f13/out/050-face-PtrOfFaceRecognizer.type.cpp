extern "C" {
	const cv::face::FaceRecognizer* cv_PtrLcv_face_FaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::FaceRecognizer>* instance) {
			return instance->get();
	}
	
	cv::face::FaceRecognizer* cv_PtrLcv_face_FaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::FaceRecognizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_FaceRecognizerG_delete(cv::Ptr<cv::face::FaceRecognizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

