extern "C" {
	std::vector<cv::cuda::GpuMat>* std_vectorLcv_cuda_GpuMatG_new_const() {
			std::vector<cv::cuda::GpuMat>* ret = new std::vector<cv::cuda::GpuMat>();
			return ret;
	}
	
	void std_vectorLcv_cuda_GpuMatG_delete(std::vector<cv::cuda::GpuMat>* instance) {
			delete instance;
	}
	
	size_t std_vectorLcv_cuda_GpuMatG_len_const(const std::vector<cv::cuda::GpuMat>* instance) {
			size_t ret = instance->size();
			return ret;
	}
	
	bool std_vectorLcv_cuda_GpuMatG_isEmpty_const(const std::vector<cv::cuda::GpuMat>* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	size_t std_vectorLcv_cuda_GpuMatG_capacity_const(const std::vector<cv::cuda::GpuMat>* instance) {
			size_t ret = instance->capacity();
			return ret;
	}
	
	void std_vectorLcv_cuda_GpuMatG_shrinkToFit(std::vector<cv::cuda::GpuMat>* instance) {
			instance->shrink_to_fit();
	}
	
	void std_vectorLcv_cuda_GpuMatG_reserve_size_t(std::vector<cv::cuda::GpuMat>* instance, size_t additional) {
			instance->reserve(instance->size() + additional);
	}
	
	void std_vectorLcv_cuda_GpuMatG_remove_size_t(std::vector<cv::cuda::GpuMat>* instance, size_t index) {
			instance->erase(instance->begin() + index);
	}
	
	void std_vectorLcv_cuda_GpuMatG_swap_size_t_size_t(std::vector<cv::cuda::GpuMat>* instance, size_t index1, size_t index2) {
			std::swap((*instance)[index1], (*instance)[index2]);
	}
	
	void std_vectorLcv_cuda_GpuMatG_clear(std::vector<cv::cuda::GpuMat>* instance) {
			instance->clear();
	}
	
	void std_vectorLcv_cuda_GpuMatG_push_const_GpuMat(std::vector<cv::cuda::GpuMat>* instance, const cv::cuda::GpuMat* val) {
			instance->push_back(*val);
	}
	
	void std_vectorLcv_cuda_GpuMatG_insert_size_t_const_GpuMat(std::vector<cv::cuda::GpuMat>* instance, size_t index, const cv::cuda::GpuMat* val) {
			instance->insert(instance->begin() + index, *val);
	}
	
	void std_vectorLcv_cuda_GpuMatG_get_const_size_t(const std::vector<cv::cuda::GpuMat>* instance, size_t index, cv::cuda::GpuMat** ocvrs_return) {
			cv::cuda::GpuMat ret = (*instance)[index];
			*ocvrs_return = new cv::cuda::GpuMat(ret);
	}
	
	void std_vectorLcv_cuda_GpuMatG_set_size_t_const_GpuMat(std::vector<cv::cuda::GpuMat>* instance, size_t index, const cv::cuda::GpuMat* val) {
			(*instance)[index] = *val;
	}
	
}


