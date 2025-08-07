extern "C" {
	const cv::videostab::MotionStabilizationPipeline* cv_PtrLcv_videostab_MotionStabilizationPipelineG_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			return instance->get();
	}
	
	cv::videostab::MotionStabilizationPipeline* cv_PtrLcv_videostab_MotionStabilizationPipelineG_getInnerPtrMut(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_MotionStabilizationPipelineG_delete(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_MotionStabilizationPipelineG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
	
	cv::Ptr<cv::videostab::MotionStabilizationPipeline>* cv_PtrLcv_videostab_MotionStabilizationPipelineG_new_const_MotionStabilizationPipeline(cv::videostab::MotionStabilizationPipeline* val) {
			return new cv::Ptr<cv::videostab::MotionStabilizationPipeline>(val);
	}
	
}

