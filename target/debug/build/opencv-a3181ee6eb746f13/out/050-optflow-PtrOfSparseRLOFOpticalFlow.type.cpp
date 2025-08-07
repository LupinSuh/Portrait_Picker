extern "C" {
	const cv::optflow::SparseRLOFOpticalFlow* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::optflow::SparseRLOFOpticalFlow* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_delete(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::SparseOpticalFlow>* cv_PtrLcv_optflow_SparseRLOFOpticalFlowG_to_PtrOfSparseOpticalFlow(cv::Ptr<cv::optflow::SparseRLOFOpticalFlow>* instance) {
			return new cv::Ptr<cv::SparseOpticalFlow>(instance->dynamicCast<cv::SparseOpticalFlow>());
	}
	
}

