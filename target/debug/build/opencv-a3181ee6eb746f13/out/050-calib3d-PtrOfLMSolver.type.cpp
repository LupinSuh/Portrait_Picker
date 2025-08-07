extern "C" {
	const cv::LMSolver* cv_PtrLcv_LMSolverG_getInnerPtr_const(const cv::Ptr<cv::LMSolver>* instance) {
			return instance->get();
	}
	
	cv::LMSolver* cv_PtrLcv_LMSolverG_getInnerPtrMut(cv::Ptr<cv::LMSolver>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_LMSolverG_delete(cv::Ptr<cv::LMSolver>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_LMSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::LMSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

