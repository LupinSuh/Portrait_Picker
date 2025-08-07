extern "C" {
	const cv::face::BasicFaceRecognizer* cv_PtrLcv_face_BasicFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return instance->get();
	}
	
	cv::face::BasicFaceRecognizer* cv_PtrLcv_face_BasicFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_BasicFaceRecognizerG_delete(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_BasicFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_BasicFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::BasicFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}
	
}

