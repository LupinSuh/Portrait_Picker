extern "C" {
	const cv::CompressedRectilinearWarper* cv_PtrLcv_CompressedRectilinearWarperG_getInnerPtr_const(const cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			return instance->get();
	}
	
	cv::CompressedRectilinearWarper* cv_PtrLcv_CompressedRectilinearWarperG_getInnerPtrMut(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CompressedRectilinearWarperG_delete(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CompressedRectilinearWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::CompressedRectilinearWarper>* cv_PtrLcv_CompressedRectilinearWarperG_new_const_CompressedRectilinearWarper(cv::CompressedRectilinearWarper* val) {
			return new cv::Ptr<cv::CompressedRectilinearWarper>(val);
	}
	
}

