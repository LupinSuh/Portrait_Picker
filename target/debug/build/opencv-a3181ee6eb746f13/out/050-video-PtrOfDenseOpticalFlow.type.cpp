extern "C" {
	const cv::DenseOpticalFlow* cv_PtrLcv_DenseOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::DenseOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::DenseOpticalFlow* cv_PtrLcv_DenseOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::DenseOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_DenseOpticalFlowG_delete(cv::Ptr<cv::DenseOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_DenseOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::DenseOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

