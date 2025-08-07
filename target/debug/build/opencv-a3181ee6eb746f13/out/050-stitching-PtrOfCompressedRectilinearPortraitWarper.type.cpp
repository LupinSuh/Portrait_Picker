extern "C" {
	const cv::CompressedRectilinearPortraitWarper* cv_PtrLcv_CompressedRectilinearPortraitWarperG_getInnerPtr_const(const cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			return instance->get();
	}
	
	cv::CompressedRectilinearPortraitWarper* cv_PtrLcv_CompressedRectilinearPortraitWarperG_getInnerPtrMut(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_CompressedRectilinearPortraitWarperG_delete(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_CompressedRectilinearPortraitWarperG_to_PtrOfWarperCreator(cv::Ptr<cv::CompressedRectilinearPortraitWarper>* instance) {
			return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
	
	cv::Ptr<cv::CompressedRectilinearPortraitWarper>* cv_PtrLcv_CompressedRectilinearPortraitWarperG_new_const_CompressedRectilinearPortraitWarper(cv::CompressedRectilinearPortraitWarper* val) {
			return new cv::Ptr<cv::CompressedRectilinearPortraitWarper>(val);
	}
	
}

