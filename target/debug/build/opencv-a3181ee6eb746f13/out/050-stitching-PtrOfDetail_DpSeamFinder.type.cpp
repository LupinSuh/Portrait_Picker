extern "C" {
	const cv::detail::DpSeamFinder* cv_PtrLcv_detail_DpSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			return instance->get();
	}
	
	cv::detail::DpSeamFinder* cv_PtrLcv_detail_DpSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_DpSeamFinderG_delete(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_DpSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
	
	cv::Ptr<cv::detail::DpSeamFinder>* cv_PtrLcv_detail_DpSeamFinderG_new_const_DpSeamFinder(cv::detail::DpSeamFinder* val) {
			return new cv::Ptr<cv::detail::DpSeamFinder>(val);
	}
	
}

