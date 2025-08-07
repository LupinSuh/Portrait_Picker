extern "C" {
	const cv::ximgproc::EdgeBoxes* cv_PtrLcv_ximgproc_EdgeBoxesG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::EdgeBoxes* cv_PtrLcv_ximgproc_EdgeBoxesG_getInnerPtrMut(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_EdgeBoxesG_delete(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_EdgeBoxesG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::EdgeBoxes>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

