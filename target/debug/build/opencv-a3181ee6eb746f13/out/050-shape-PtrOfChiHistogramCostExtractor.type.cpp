extern "C" {
	const cv::ChiHistogramCostExtractor* cv_PtrLcv_ChiHistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	cv::ChiHistogramCostExtractor* cv_PtrLcv_ChiHistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_ChiHistogramCostExtractorG_delete(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ChiHistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_ChiHistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
	
}

