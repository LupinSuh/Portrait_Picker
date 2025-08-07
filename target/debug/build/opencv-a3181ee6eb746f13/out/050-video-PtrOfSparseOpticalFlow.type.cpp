extern "C" {
	const cv::SparseOpticalFlow* cv_PtrLcv_SparseOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::SparseOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::SparseOpticalFlow* cv_PtrLcv_SparseOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::SparseOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_SparseOpticalFlowG_delete(cv::Ptr<cv::SparseOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SparseOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::SparseOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

