extern "C" {
	const cv::ximgproc::EdgeDrawing* cv_PtrLcv_ximgproc_EdgeDrawingG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::EdgeDrawing* cv_PtrLcv_ximgproc_EdgeDrawingG_getInnerPtrMut(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_EdgeDrawingG_delete(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_EdgeDrawingG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::EdgeDrawing>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

