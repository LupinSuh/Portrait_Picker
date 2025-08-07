extern "C" {
	const cv::aruco::ArucoDetector* cv_PtrLcv_aruco_ArucoDetectorG_getInnerPtr_const(const cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			return instance->get();
	}
	
	cv::aruco::ArucoDetector* cv_PtrLcv_aruco_ArucoDetectorG_getInnerPtrMut(cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_aruco_ArucoDetectorG_delete(cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_aruco_ArucoDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::aruco::ArucoDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::aruco::ArucoDetector>* cv_PtrLcv_aruco_ArucoDetectorG_new_const_ArucoDetector(cv::aruco::ArucoDetector* val) {
			return new cv::Ptr<cv::aruco::ArucoDetector>(val);
	}
	
}

