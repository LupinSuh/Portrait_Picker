extern "C" {
	const cv::optflow::GPCTrainingSamples* cv_PtrLcv_optflow_GPCTrainingSamplesG_getInnerPtr_const(const cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
			return instance->get();
	}
	
	cv::optflow::GPCTrainingSamples* cv_PtrLcv_optflow_GPCTrainingSamplesG_getInnerPtrMut(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_GPCTrainingSamplesG_delete(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::optflow::GPCTrainingSamples>* cv_PtrLcv_optflow_GPCTrainingSamplesG_new_const_GPCTrainingSamples(cv::optflow::GPCTrainingSamples* val) {
			return new cv::Ptr<cv::optflow::GPCTrainingSamples>(val);
	}
	
}

