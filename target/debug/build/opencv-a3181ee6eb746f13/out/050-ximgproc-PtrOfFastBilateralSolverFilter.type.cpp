extern "C" {
	const cv::ximgproc::FastBilateralSolverFilter* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::FastBilateralSolverFilter* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_delete(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_FastBilateralSolverFilterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

