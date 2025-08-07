extern "C" {
	const cv::DISOpticalFlow* cv_PtrLcv_DISOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::DISOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::DISOpticalFlow* cv_PtrLcv_DISOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::DISOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_DISOpticalFlowG_delete(cv::Ptr<cv::DISOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DISOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::DISOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_DISOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::DISOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}
	
}

