extern "C" {
	const cv::cuda::SURF_CUDA* cv_PtrLcv_cuda_SURF_CUDAG_getInnerPtr_const(const cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
			return instance->get();
	}
	
	cv::cuda::SURF_CUDA* cv_PtrLcv_cuda_SURF_CUDAG_getInnerPtrMut(cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_cuda_SURF_CUDAG_delete(cv::Ptr<cv::cuda::SURF_CUDA>* instance) {
			delete instance;
	}
	
	cv::Ptr<cv::cuda::SURF_CUDA>* cv_PtrLcv_cuda_SURF_CUDAG_new_const_SURF_CUDA(cv::cuda::SURF_CUDA* val) {
			return new cv::Ptr<cv::cuda::SURF_CUDA>(val);
	}
	
}

