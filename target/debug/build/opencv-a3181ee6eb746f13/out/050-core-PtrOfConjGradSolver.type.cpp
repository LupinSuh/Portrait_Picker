extern "C" {
	const cv::ConjGradSolver* cv_PtrLcv_ConjGradSolverG_getInnerPtr_const(const cv::Ptr<cv::ConjGradSolver>* instance) {
			return instance->get();
	}
	
	cv::ConjGradSolver* cv_PtrLcv_ConjGradSolverG_getInnerPtrMut(cv::Ptr<cv::ConjGradSolver>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ConjGradSolverG_delete(cv::Ptr<cv::ConjGradSolver>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ConjGradSolverG_to_PtrOfAlgorithm(cv::Ptr<cv::ConjGradSolver>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::MinProblemSolver>* cv_PtrLcv_ConjGradSolverG_to_PtrOfMinProblemSolver(cv::Ptr<cv::ConjGradSolver>* instance) {
			return new cv::Ptr<cv::MinProblemSolver>(instance->dynamicCast<cv::MinProblemSolver>());
	}
	
}

