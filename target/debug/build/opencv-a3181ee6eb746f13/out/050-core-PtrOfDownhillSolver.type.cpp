extern "C" {
	const cv::DownhillSolver* cv_PtrLcv_DownhillSolverG_getInnerPtr_const(const cv::Ptr<cv::DownhillSolver>* instance) {
			return instance->get();
	}
	
	cv::DownhillSolver* cv_PtrLcv_DownhillSolverG_getInnerPtrMut(cv::Ptr<cv::DownhillSolver>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_DownhillSolverG_delete(cv::Ptr<cv::DownhillSolver>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DownhillSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::DownhillSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::MinProblemSolver>* cv_PtrLcv_DownhillSolverG_to_PtrOfMinProblemSolver(cv::Ptr<cv::DownhillSolver>* instance) {
			return new cv::Ptr<cv::MinProblemSolver>(instance->dynamicCast<cv::MinProblemSolver>());
	}
	
}

