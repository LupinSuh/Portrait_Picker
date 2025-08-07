extern "C" {
	const cv::detail::BestOf2NearestMatcher* cv_PtrLcv_detail_BestOf2NearestMatcherG_getInnerPtr_const(const cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			return instance->get();
	}
	
	cv::detail::BestOf2NearestMatcher* cv_PtrLcv_detail_BestOf2NearestMatcherG_getInnerPtrMut(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_BestOf2NearestMatcherG_delete(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrLcv_detail_BestOf2NearestMatcherG_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}
	
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_BestOf2NearestMatcherG_new_const_BestOf2NearestMatcher(cv::detail::BestOf2NearestMatcher* val) {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(val);
	}
	
}

