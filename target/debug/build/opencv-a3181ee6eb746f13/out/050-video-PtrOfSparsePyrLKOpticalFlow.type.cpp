extern "C" {
	const cv::SparsePyrLKOpticalFlow* cv_PtrLcv_SparsePyrLKOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::SparsePyrLKOpticalFlow* cv_PtrLcv_SparsePyrLKOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_SparsePyrLKOpticalFlowG_delete(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SparsePyrLKOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::SparseOpticalFlow>* cv_PtrLcv_SparsePyrLKOpticalFlowG_to_PtrOfSparseOpticalFlow(cv::Ptr<cv::SparsePyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::SparseOpticalFlow>(instance->dynamicCast<cv::SparseOpticalFlow>());
	}
	
}

