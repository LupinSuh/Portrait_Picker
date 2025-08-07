extern "C" {
	const cv::ximgproc::segmentation::GraphSegmentation* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			return instance->get();
	}
	
	cv::ximgproc::segmentation::GraphSegmentation* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_delete(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

