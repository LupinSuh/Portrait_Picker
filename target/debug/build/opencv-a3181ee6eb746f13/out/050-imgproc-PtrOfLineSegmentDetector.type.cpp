extern "C" {
	const cv::LineSegmentDetector* cv_PtrLcv_LineSegmentDetectorG_getInnerPtr_const(const cv::Ptr<cv::LineSegmentDetector>* instance) {
			return instance->get();
	}
	
	cv::LineSegmentDetector* cv_PtrLcv_LineSegmentDetectorG_getInnerPtrMut(cv::Ptr<cv::LineSegmentDetector>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_LineSegmentDetectorG_delete(cv::Ptr<cv::LineSegmentDetector>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_LineSegmentDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::LineSegmentDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

