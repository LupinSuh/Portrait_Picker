extern "C" {
	const cv::optflow::GPCTree* cv_PtrLcv_optflow_GPCTreeG_getInnerPtr_const(const cv::Ptr<cv::optflow::GPCTree>* instance) {
			return instance->get();
	}
	
	cv::optflow::GPCTree* cv_PtrLcv_optflow_GPCTreeG_getInnerPtrMut(cv::Ptr<cv::optflow::GPCTree>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_GPCTreeG_delete(cv::Ptr<cv::optflow::GPCTree>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_GPCTreeG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::GPCTree>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::optflow::GPCTree>* cv_PtrLcv_optflow_GPCTreeG_new_const_GPCTree(cv::optflow::GPCTree* val) {
			return new cv::Ptr<cv::optflow::GPCTree>(val);
	}
	
}

