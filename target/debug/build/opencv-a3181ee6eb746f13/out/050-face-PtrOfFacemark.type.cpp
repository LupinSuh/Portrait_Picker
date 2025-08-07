extern "C" {
	const cv::face::Facemark* cv_PtrLcv_face_FacemarkG_getInnerPtr_const(const cv::Ptr<cv::face::Facemark>* instance) {
			return instance->get();
	}
	
	cv::face::Facemark* cv_PtrLcv_face_FacemarkG_getInnerPtrMut(cv::Ptr<cv::face::Facemark>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_FacemarkG_delete(cv::Ptr<cv::face::Facemark>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkG_to_PtrOfAlgorithm(cv::Ptr<cv::face::Facemark>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

