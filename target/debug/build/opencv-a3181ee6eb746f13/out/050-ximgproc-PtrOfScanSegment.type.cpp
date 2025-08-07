extern "C" {
	const cv::ximgproc::ScanSegment* cv_PtrLcv_ximgproc_ScanSegmentG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::ScanSegment* cv_PtrLcv_ximgproc_ScanSegmentG_getInnerPtrMut(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_ScanSegmentG_delete(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_ScanSegmentG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

