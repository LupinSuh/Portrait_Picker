extern "C" {
	const cv::FarnebackOpticalFlow* cv_PtrLcv_FarnebackOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::FarnebackOpticalFlow* cv_PtrLcv_FarnebackOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_FarnebackOpticalFlowG_delete(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_FarnebackOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::DenseOpticalFlow>* cv_PtrLcv_FarnebackOpticalFlowG_to_PtrOfDenseOpticalFlow(cv::Ptr<cv::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::DenseOpticalFlow>(instance->dynamicCast<cv::DenseOpticalFlow>());
	}
	
}

