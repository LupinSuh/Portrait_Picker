extern "C" {
	const cv::detail::GraphCutSeamFinder* cv_PtrLcv_detail_GraphCutSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return instance->get();
	}
	
	cv::detail::GraphCutSeamFinder* cv_PtrLcv_detail_GraphCutSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_GraphCutSeamFinderG_delete(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::GraphCutSeamFinderBase>* cv_PtrLcv_detail_GraphCutSeamFinderG_to_PtrOfDetail_GraphCutSeamFinderBase(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::GraphCutSeamFinderBase>(instance->dynamicCast<cv::detail::GraphCutSeamFinderBase>());
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
	
	cv::Ptr<cv::detail::GraphCutSeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderG_new_const_GraphCutSeamFinder(cv::detail::GraphCutSeamFinder* val) {
			return new cv::Ptr<cv::detail::GraphCutSeamFinder>(val);
	}
	
}

