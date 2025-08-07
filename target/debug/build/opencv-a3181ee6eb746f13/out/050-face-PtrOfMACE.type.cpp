extern "C" {
	const cv::face::MACE* cv_PtrLcv_face_MACEG_getInnerPtr_const(const cv::Ptr<cv::face::MACE>* instance) {
			return instance->get();
	}
	
	cv::face::MACE* cv_PtrLcv_face_MACEG_getInnerPtrMut(cv::Ptr<cv::face::MACE>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_MACEG_delete(cv::Ptr<cv::face::MACE>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_MACEG_to_PtrOfAlgorithm(cv::Ptr<cv::face::MACE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

