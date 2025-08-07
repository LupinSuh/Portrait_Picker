extern "C" {
	const cv::optflow::DualTVL1OpticalFlow* cv_PtrLcv_optflow_DualTVL1OpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::optflow::DualTVL1OpticalFlow* cv_PtrLcv_optflow_DualTVL1OpticalFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_DualTVL1OpticalFlowG_delete(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_DualTVL1OpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_optflow_DualTVL1OpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::optflow::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}
	
}

