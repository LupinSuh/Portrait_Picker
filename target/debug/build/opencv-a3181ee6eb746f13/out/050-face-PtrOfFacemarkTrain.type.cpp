extern "C" {
	const cv::face::FacemarkTrain* cv_PtrLcv_face_FacemarkTrainG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return instance->get();
	}
	
	cv::face::FacemarkTrain* cv_PtrLcv_face_FacemarkTrainG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_FacemarkTrainG_delete(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkTrainG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkTrainG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkTrain>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}
	
}

