extern "C" {
	const cv::cuda::GpuMat::Allocator* cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtr_const(const cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
			return instance->get();
	}
	
	cv::cuda::GpuMat::Allocator* cv_PtrLcv_cuda_GpuMat_AllocatorG_getInnerPtrMut(cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
			return instance->get();
	}
	
	void cv_PtrLcv_cuda_GpuMat_AllocatorG_delete(cv::Ptr<cv::cuda::GpuMat::Allocator>* instance) {
			delete instance;
	}
	
}

