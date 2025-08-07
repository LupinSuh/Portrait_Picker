extern "C" {
	const cv::face::BIF* cv_PtrLcv_face_BIFG_getInnerPtr_const(const cv::Ptr<cv::face::BIF>* instance) {
			return instance->get();
	}
	
	cv::face::BIF* cv_PtrLcv_face_BIFG_getInnerPtrMut(cv::Ptr<cv::face::BIF>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_BIFG_delete(cv::Ptr<cv::face::BIF>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_BIFG_to_PtrOfAlgorithm(cv::Ptr<cv::face::BIF>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

