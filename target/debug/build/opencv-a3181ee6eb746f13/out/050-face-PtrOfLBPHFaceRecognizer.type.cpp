extern "C" {
	const cv::face::LBPHFaceRecognizer* cv_PtrLcv_face_LBPHFaceRecognizerG_getInnerPtr_const(const cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return instance->get();
	}
	
	cv::face::LBPHFaceRecognizer* cv_PtrLcv_face_LBPHFaceRecognizerG_getInnerPtrMut(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_LBPHFaceRecognizerG_delete(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_LBPHFaceRecognizerG_to_PtrOfAlgorithm(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::FaceRecognizer>* cv_PtrLcv_face_LBPHFaceRecognizerG_to_PtrOfFaceRecognizer(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
			return new cv::Ptr<cv::face::FaceRecognizer>(instance->dynamicCast<cv::face::FaceRecognizer>());
	}
	
}

