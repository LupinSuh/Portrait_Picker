extern "C" {
	const cv::optflow::DenseRLOFOpticalFlow* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::optflow::DenseRLOFOpticalFlow* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_delete(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_DenseRLOFOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::DenseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}
	
}

