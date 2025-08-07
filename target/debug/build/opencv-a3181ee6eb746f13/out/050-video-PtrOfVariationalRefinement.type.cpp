extern "C" {
	const cv::VariationalRefinement* cv_PtrLcv_VariationalRefinementG_getInnerPtr_const(const cv::Ptr<cv::VariationalRefinement>* instance) {
			return instance->get();
	}
	
	cv::VariationalRefinement* cv_PtrLcv_VariationalRefinementG_getInnerPtrMut(cv::Ptr<cv::VariationalRefinement>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_VariationalRefinementG_delete(cv::Ptr<cv::VariationalRefinement>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_VariationalRefinementG_to_PtrOfAlgorithm(cv::Ptr<cv::VariationalRefinement>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_VariationalRefinementG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::VariationalRefinement>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}
	
}

