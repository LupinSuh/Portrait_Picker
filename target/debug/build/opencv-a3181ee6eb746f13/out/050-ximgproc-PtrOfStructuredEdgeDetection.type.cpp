extern "C" {
	const cv::ximgproc::StructuredEdgeDetection* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::StructuredEdgeDetection* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_getInnerPtrMut(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_delete(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

