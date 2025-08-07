extern "C" {
	const cv::face::FacemarkAAM* cv_PtrLcv_face_FacemarkAAMG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return instance->get();
	}
	
	cv::face::FacemarkAAM* cv_PtrLcv_face_FacemarkAAMG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_FacemarkAAMG_delete(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkAAMG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkAAMG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}
	
	cv::Ptr<cv::face::FacemarkTrain>* cv_PtrLcv_face_FacemarkAAMG_to_PtrOfFacemarkTrain(cv::Ptr<cv::face::FacemarkAAM>* instance) {
			return new cv::Ptr<cv::face::FacemarkTrain>(instance->dynamicCast<cv::face::FacemarkTrain>());
	}
	
}

