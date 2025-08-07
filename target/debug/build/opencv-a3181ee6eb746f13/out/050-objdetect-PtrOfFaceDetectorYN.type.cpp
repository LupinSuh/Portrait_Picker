extern "C" {
	const cv::FaceDetectorYN* cv_PtrLcv_FaceDetectorYNG_getInnerPtr_const(const cv::Ptr<cv::FaceDetectorYN>* instance) {
			return instance->get();
	}
	
	cv::FaceDetectorYN* cv_PtrLcv_FaceDetectorYNG_getInnerPtrMut(cv::Ptr<cv::FaceDetectorYN>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FaceDetectorYNG_delete(cv::Ptr<cv::FaceDetectorYN>* instance) {
			delete instance;
	}
	
}

