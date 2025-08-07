extern "C" {
	const cv::detail::AffineBestOf2NearestMatcher* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_getInnerPtr_const(const cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return instance->get();
	}
	
	cv::detail::AffineBestOf2NearestMatcher* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_getInnerPtrMut(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_delete(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_to_PtrOfDetail_BestOf2NearestMatcher(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(instance->dynamicCast<cv::detail::BestOf2NearestMatcher>());
	}
	
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}
	
	cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_new_const_AffineBestOf2NearestMatcher(cv::detail::AffineBestOf2NearestMatcher* val) {
			return new cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>(val);
	}
	
}

