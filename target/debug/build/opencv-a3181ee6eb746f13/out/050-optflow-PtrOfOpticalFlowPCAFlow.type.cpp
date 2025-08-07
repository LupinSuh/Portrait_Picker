extern "C" {
	const cv::optflow::OpticalFlowPCAFlow* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return instance->get();
	}
	
	cv::optflow::OpticalFlowPCAFlow* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_OpticalFlowPCAFlowG_delete(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}
	
	cv::Ptr<cv::optflow::OpticalFlowPCAFlow>* cv_PtrLcv_optflow_OpticalFlowPCAFlowG_new_const_OpticalFlowPCAFlow(cv::optflow::OpticalFlowPCAFlow* val) {
			return new cv::Ptr<cv::optflow::OpticalFlowPCAFlow>(val);
	}
	
}

