extern "C" {
	const cv::detail::VoronoiSeamFinder* cv_PtrLcv_detail_VoronoiSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return instance->get();
	}
	
	cv::detail::VoronoiSeamFinder* cv_PtrLcv_detail_VoronoiSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_VoronoiSeamFinderG_delete(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::PairwiseSeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_to_PtrOfDetail_PairwiseSeamFinder(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::PairwiseSeamFinder>(instance->dynamicCast<cv::detail::PairwiseSeamFinder>());
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
	
	cv::Ptr<cv::detail::VoronoiSeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_new_const_VoronoiSeamFinder(cv::detail::VoronoiSeamFinder* val) {
			return new cv::Ptr<cv::detail::VoronoiSeamFinder>(val);
	}
	
}

