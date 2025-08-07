extern "C" {
	const cv::EMDHistogramCostExtractor* cv_PtrLcv_EMDHistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	cv::EMDHistogramCostExtractor* cv_PtrLcv_EMDHistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_EMDHistogramCostExtractorG_delete(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_EMDHistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_EMDHistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
	
}

