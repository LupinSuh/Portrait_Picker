extern "C" {
	const cv::HistogramCostExtractor* cv_PtrLcv_HistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::HistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	cv::HistogramCostExtractor* cv_PtrLcv_HistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::HistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_HistogramCostExtractorG_delete(cv::Ptr<cv::HistogramCostExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_HistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::HistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
}

