extern "C" {
	const cv::EMDL1HistogramCostExtractor* cv_PtrLcv_EMDL1HistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	cv::EMDL1HistogramCostExtractor* cv_PtrLcv_EMDL1HistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_EMDL1HistogramCostExtractorG_delete(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_EMDL1HistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}
	
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_EMDL1HistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}
	
}

