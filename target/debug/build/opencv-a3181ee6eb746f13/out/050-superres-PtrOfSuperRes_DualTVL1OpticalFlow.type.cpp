extern "C" {
	const cv::superres::DualTVL1OpticalFlow* cv_PtrLcv_superres_DualTVL1OpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::superres::DualTVL1OpticalFlow* cv_PtrLcv_superres_DualTVL1OpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_superres_DualTVL1OpticalFlowG_delete(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_DualTVL1OpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_DualTVL1OpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::DualTVL1OpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}
	
}

