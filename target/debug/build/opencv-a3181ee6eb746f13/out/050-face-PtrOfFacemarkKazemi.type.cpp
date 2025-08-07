extern "C" {
	const cv::face::FacemarkKazemi* cv_PtrLcv_face_FacemarkKazemiG_getInnerPtr_const(const cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return instance->get();
	}
	
	cv::face::FacemarkKazemi* cv_PtrLcv_face_FacemarkKazemiG_getInnerPtrMut(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_face_FacemarkKazemiG_delete(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_face_FacemarkKazemiG_to_PtrOfAlgorithm(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::face::Facemark>* cv_PtrLcv_face_FacemarkKazemiG_to_PtrOfFacemark(cv::Ptr<cv::face::FacemarkKazemi>* instance) {
			return new cv::Ptr<cv::face::Facemark>(instance->dynamicCast<cv::face::Facemark>());
	}
	
}

