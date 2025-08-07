extern "C" {
	const cv::face::EigenFaceRecognizer* cv_PtrLcv_face_EigenFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return instance->get();
	}
	
	cv::face::EigenFaceRecognizer* cv_PtrLcv_face_EigenFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_EigenFaceRecognizerG_delete(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::BasicFaceRecognizer>* cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfBasicFaceRecognizer(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::BasicFaceRecognizer>(instance->dynamicCast<cv::face::BasicFaceRecognizer>());
	}
	
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_EigenFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}
	
}

