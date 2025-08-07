extern "C" {
	const cv::superres::PyrLKOpticalFlow* cv_PtrLcv_superres_PyrLKOpticalFlowG_getInnerPtr_const(const cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return instance->get();
	}
	
	cv::superres::PyrLKOpticalFlow* cv_PtrLcv_superres_PyrLKOpticalFlowG_getInnerPtrMut(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_superres_PyrLKOpticalFlowG_delete(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_superres_PyrLKOpticalFlowG_to_PtrOfAlgorithm(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::superres::DenseOpticalFlowExt>* cv_PtrLcv_superres_PyrLKOpticalFlowG_to_PtrOfSuperRes_DenseOpticalFlowExt(cv::Ptr<cv::superres::PyrLKOpticalFlow>* instance) {
			return new cv::Ptr<cv::superres::DenseOpticalFlowExt>(instance->dynamicCast<cv::superres::DenseOpticalFlowExt>());
	}
	
}

