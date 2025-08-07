extern "C" {
	const cv::NormHistogramCostExtractor* cv_PtrLcv_NormHistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	cv::NormHistogramCostExtractor* cv_PtrLcv_NormHistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_NormHistogramCostExtractorG_delete(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_NormHistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_NormHistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
	
}

