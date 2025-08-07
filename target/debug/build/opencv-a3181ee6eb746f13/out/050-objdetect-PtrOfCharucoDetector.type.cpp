extern "C" {
	const cv::aruco::CharucoDetector* cv_PtrLcv_aruco_CharucoDetectorG_getInnerPtr_const(const cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			return instance->get();
	}
	
	cv::aruco::CharucoDetector* cv_PtrLcv_aruco_CharucoDetectorG_getInnerPtrMut(cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_aruco_CharucoDetectorG_delete(cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_aruco_CharucoDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::aruco::CharucoDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::aruco::CharucoDetector>* cv_PtrLcv_aruco_CharucoDetectorG_new_const_CharucoDetector(cv::aruco::CharucoDetector* val) {
			return new cv::Ptr<cv::aruco::CharucoDetector>(val);
	}
	
}

