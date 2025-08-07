extern "C" {
	const cv::superres::FarnebackOpticalFlow* cv_PtrLcv_superres_FarnebackOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::superres::FarnebackOpticalFlow* cv_PtrLcv_superres_FarnebackOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_superres_FarnebackOpticalFlowG_delete(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_FarnebackOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_FarnebackOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::FarnebackOpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}
	
}

