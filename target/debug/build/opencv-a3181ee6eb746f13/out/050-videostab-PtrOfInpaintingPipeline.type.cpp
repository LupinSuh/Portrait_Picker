extern "C" {
	const cv::videostab::InpaintingPipeline* cv_PtrLcv_videostab_InpaintingPipelineG_getInnerPtr_const(const cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			return instance->get();
	}
	
	cv::videostab::InpaintingPipeline* cv_PtrLcv_videostab_InpaintingPipelineG_getInnerPtrMut(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_videostab_InpaintingPipelineG_delete(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrLcv_videostab_InpaintingPipelineG_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::InpaintingPipeline>* instance) {
			return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
	
	cv::Ptr<cv::videostab::InpaintingPipeline>* cv_PtrLcv_videostab_InpaintingPipelineG_new_const_InpaintingPipeline(cv::videostab::InpaintingPipeline* val) {
			return new cv::Ptr<cv::videostab::InpaintingPipeline>(val);
	}
	
}

