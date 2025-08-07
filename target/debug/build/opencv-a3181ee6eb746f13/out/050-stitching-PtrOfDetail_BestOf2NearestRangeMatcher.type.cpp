extern "C" {
	const cv::detail::BestOf2NearestRangeMatcher* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_getInnerPtr_const(const cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return instance->get();
	}
	
	cv::detail::BestOf2NearestRangeMatcher* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_getInnerPtrMut(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_delete(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_to_PtrOfDetail_BestOf2NearestMatcher(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(instance->dynamicCast<cv::detail::BestOf2NearestMatcher>());
	}
	
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}
	
	cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_new_const_BestOf2NearestRangeMatcher(cv::detail::BestOf2NearestRangeMatcher* val) {
			return new cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>(val);
	}
	
}

