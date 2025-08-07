extern "C" {
	const cv::superres::BroxOpticalFlow* cv_PtrLcv_superres_BroxOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::superres::BroxOpticalFlow* cv_PtrLcv_superres_BroxOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_superres_BroxOpticalFlowG_delete(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_BroxOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_BroxOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::BroxOpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}
	
}

