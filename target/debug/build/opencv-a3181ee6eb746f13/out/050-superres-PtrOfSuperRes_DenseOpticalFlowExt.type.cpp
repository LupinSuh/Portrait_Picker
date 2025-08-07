extern "C" {
	const cv::superres::DenseOpticalFlowExt* cv_PtrLcv_superres_DenseOpticalFlowExtG_getInnerPtr_const(const cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			return instance->get();
	}
	
	cv::superres::DenseOpticalFlowExt* cv_PtrLcv_superres_DenseOpticalFlowExtG_getInnerPtrMut(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_superres_DenseOpticalFlowExtG_delete(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_DenseOpticalFlowExtG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::DenseOpticalFlowExt>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

