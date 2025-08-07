extern "C" {
	const cv::optflow::PCAPrior* cv_PtrLcv_optflow_PCAPriorG_getInnerPtr_const(const cv::Ptr<cv::optflow::PCAPrior>* instance) {
			return instance->get();
	}
	
	cv::optflow::PCAPrior* cv_PtrLcv_optflow_PCAPriorG_getInnerPtrMut(cv::Ptr<cv::optflow::PCAPrior>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_PCAPriorG_delete(cv::Ptr<cv::optflow::PCAPrior>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::optflow::PCAPrior>* cv_PtrLcv_optflow_PCAPriorG_new_const_PCAPrior(cv::optflow::PCAPrior* val) {
			return new cv::Ptr<cv::optflow::PCAPrior>(val);
	}
	
}

