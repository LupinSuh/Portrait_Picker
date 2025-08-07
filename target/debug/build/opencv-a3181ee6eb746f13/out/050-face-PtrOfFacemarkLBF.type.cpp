extern "C" {
	const cv::face::FacemarkLBF* cv_PtrLcv_face_FacemarkLBFG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return instance->get();
	}
	
	cv::face::FacemarkLBF* cv_PtrLcv_face_FacemarkLBFG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_FacemarkLBFG_delete(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkLBFG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkLBFG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}
	
	cv::Ptr<cv::face::FacemarkTrain>* cv_PtrLcv_face_FacemarkLBFG_to_PtrOfFacemarkTrain(cv::Ptr<cv::face::FacemarkLBF>* instance) {
			return new cv::Ptr<cv::face::FacemarkTrain>(instance->dynamicCast<cv::face::FacemarkTrain>());
	}
	
}

