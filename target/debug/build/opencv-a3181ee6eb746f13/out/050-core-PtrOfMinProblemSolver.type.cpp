extern "C" {
	const cv::MinProblemSolver* cv_PtrLcv_MinProblemSolverG_getInnerPtr_const(const cv::Ptr<cv::MinProblemSolver>* instance) {
			return instance->get();
	}
	
	cv::MinProblemSolver* cv_PtrLcv_MinProblemSolverG_getInnerPtrMut(cv::Ptr<cv::MinProblemSolver>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_MinProblemSolverG_delete(cv::Ptr<cv::MinProblemSolver>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_MinProblemSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::MinProblemSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

