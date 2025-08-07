extern "C" {
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyTextureG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}
	
}

