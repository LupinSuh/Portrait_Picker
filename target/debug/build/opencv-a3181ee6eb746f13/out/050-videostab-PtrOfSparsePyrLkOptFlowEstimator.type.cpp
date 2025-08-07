extern "C" {
	const cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_getInnerPtr_const(const cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return instance->get();
	}
	
	cv::videostab::SparsePyrLkOptFlowEstimator* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_getInnerPtrMut(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_delete(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::ISparseOptFlowEstimator>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_to_PtrOfISparseOptFlowEstimator(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return new cv::Ptr<cv::videostab::ISparseOptFlowEstimator>(instance->dynamicCast<cv::videostab::ISparseOptFlowEstimator>());
	}
	
	cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_to_PtrOfPyrLkOptFlowEstimatorBase(cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* instance) {
			return new cv::Ptr<cv::videostab::PyrLkOptFlowEstimatorBase>(instance->dynamicCast<cv::videostab::PyrLkOptFlowEstimatorBase>());
	}
	
	cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>* cv_PtrLcv_videostab_SparsePyrLkOptFlowEstimatorG_new_const_SparsePyrLkOptFlowEstimator(cv::videostab::SparsePyrLkOptFlowEstimator* val) {
			return new cv::Ptr<cv::videostab::SparsePyrLkOptFlowEstimator>(val);
	}
	
}

