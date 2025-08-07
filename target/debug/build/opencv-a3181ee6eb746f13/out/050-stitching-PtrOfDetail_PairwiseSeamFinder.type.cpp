extern "C" {
	const cv::detail::PairwiseSeamFinder* cv_PtrLcv_detail_PairwiseSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			return instance->get();
	}
	
	cv::detail::PairwiseSeamFinder* cv_PtrLcv_detail_PairwiseSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_PairwiseSeamFinderG_delete(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_PairwiseSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
	
}

