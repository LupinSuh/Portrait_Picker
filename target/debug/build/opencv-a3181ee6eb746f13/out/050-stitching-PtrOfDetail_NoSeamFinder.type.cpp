extern "C" {
	const cv::detail::NoSeamFinder* cv_PtrLcv_detail_NoSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			return instance->get();
	}
	
	cv::detail::NoSeamFinder* cv_PtrLcv_detail_NoSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_detail_NoSeamFinderG_delete(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_NoSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}
	
	cv::Ptr<cv::detail::NoSeamFinder>* cv_PtrLcv_detail_NoSeamFinderG_new_const_NoSeamFinder(cv::detail::NoSeamFinder* val) {
			return new cv::Ptr<cv::detail::NoSeamFinder>(val);
	}
	
}

