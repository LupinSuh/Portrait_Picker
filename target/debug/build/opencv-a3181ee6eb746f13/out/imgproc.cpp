#include "ocvrs_common.hpp"
#include <opencv2/imgproc.hpp>
#include "imgproc_types.hpp"

extern "C" {
	void cv_morphologyDefaultBorderValue(Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::morphologyDefaultBorderValue();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(cv::CLAHE* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, ResultVoid* ocvrs_return) {
		try {
			instance->apply(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CLAHE_setClipLimit_double(cv::CLAHE* instance, double clipLimit, ResultVoid* ocvrs_return) {
		try {
			instance->setClipLimit(clipLimit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CLAHE_getClipLimit_const(const cv::CLAHE* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getClipLimit();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CLAHE_setTilesGridSize_Size(cv::CLAHE* instance, cv::Size* tileGridSize, ResultVoid* ocvrs_return) {
		try {
			instance->setTilesGridSize(*tileGridSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CLAHE_getTilesGridSize_const(const cv::CLAHE* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getTilesGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_CLAHE_collectGarbage(cv::CLAHE* instance, ResultVoid* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_CLAHE_to_Algorithm(cv::CLAHE* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_CLAHE_delete(cv::CLAHE* instance) {
			delete instance;
	}
	
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* templ, cv::Point* templCenter, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*templ, *templCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setTemplate_const__InputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* templ, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*templ);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, cv::Point* templCenter, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*edges, *dx, *dy, *templCenter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, ResultVoid* ocvrs_return) {
		try {
			instance->setTemplate(*edges, *dx, *dy);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, const cv::_OutputArray* votes, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *positions, *votes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* image, const cv::_OutputArray* positions, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *positions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, const cv::_OutputArray* votes, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*edges, *dx, *dy, *positions, *votes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::GeneralizedHough* instance, const cv::_InputArray* edges, const cv::_InputArray* dx, const cv::_InputArray* dy, const cv::_OutputArray* positions, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*edges, *dx, *dy, *positions);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setCannyLowThresh_int(cv::GeneralizedHough* instance, int cannyLowThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setCannyLowThresh(cannyLowThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_getCannyLowThresh_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyLowThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setCannyHighThresh_int(cv::GeneralizedHough* instance, int cannyHighThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setCannyHighThresh(cannyHighThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_getCannyHighThresh_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCannyHighThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setMinDist_double(cv::GeneralizedHough* instance, double minDist, ResultVoid* ocvrs_return) {
		try {
			instance->setMinDist(minDist);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_getMinDist_const(const cv::GeneralizedHough* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinDist();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setDp_double(cv::GeneralizedHough* instance, double dp, ResultVoid* ocvrs_return) {
		try {
			instance->setDp(dp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_getDp_const(const cv::GeneralizedHough* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getDp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_setMaxBufferSize_int(cv::GeneralizedHough* instance, int maxBufferSize, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxBufferSize(maxBufferSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHough_getMaxBufferSize_const(const cv::GeneralizedHough* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::GeneralizedHoughBallard* cv_GeneralizedHough_to_GeneralizedHoughBallard(cv::GeneralizedHough* instance) {
			return dynamic_cast<cv::GeneralizedHoughBallard*>(instance);
	}
	
	cv::GeneralizedHoughGuil* cv_GeneralizedHough_to_GeneralizedHoughGuil(cv::GeneralizedHough* instance) {
			return dynamic_cast<cv::GeneralizedHoughGuil*>(instance);
	}
	
	cv::Algorithm* cv_GeneralizedHough_to_Algorithm(cv::GeneralizedHough* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_GeneralizedHough_delete(cv::GeneralizedHough* instance) {
			delete instance;
	}
	
	void cv_GeneralizedHoughBallard_setLevels_int(cv::GeneralizedHoughBallard* instance, int levels, ResultVoid* ocvrs_return) {
		try {
			instance->setLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_getLevels_const(const cv::GeneralizedHoughBallard* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_setVotesThreshold_int(cv::GeneralizedHoughBallard* instance, int votesThreshold, ResultVoid* ocvrs_return) {
		try {
			instance->setVotesThreshold(votesThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughBallard_getVotesThreshold_const(const cv::GeneralizedHoughBallard* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getVotesThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_GeneralizedHoughBallard_to_Algorithm(cv::GeneralizedHoughBallard* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::GeneralizedHough* cv_GeneralizedHoughBallard_to_GeneralizedHough(cv::GeneralizedHoughBallard* instance) {
			return dynamic_cast<cv::GeneralizedHough*>(instance);
	}
	
	void cv_GeneralizedHoughBallard_delete(cv::GeneralizedHoughBallard* instance) {
			delete instance;
	}
	
	void cv_GeneralizedHoughGuil_setXi_double(cv::GeneralizedHoughGuil* instance, double xi, ResultVoid* ocvrs_return) {
		try {
			instance->setXi(xi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getXi_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getXi();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setLevels_int(cv::GeneralizedHoughGuil* instance, int levels, ResultVoid* ocvrs_return) {
		try {
			instance->setLevels(levels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getLevels_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLevels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setAngleEpsilon_double(cv::GeneralizedHoughGuil* instance, double angleEpsilon, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleEpsilon(angleEpsilon);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getAngleEpsilon_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAngleEpsilon();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMinAngle_double(cv::GeneralizedHoughGuil* instance, double minAngle, ResultVoid* ocvrs_return) {
		try {
			instance->setMinAngle(minAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMinAngle_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinAngle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMaxAngle_double(cv::GeneralizedHoughGuil* instance, double maxAngle, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxAngle(maxAngle);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMaxAngle_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxAngle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setAngleStep_double(cv::GeneralizedHoughGuil* instance, double angleStep, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleStep(angleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getAngleStep_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAngleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setAngleThresh_int(cv::GeneralizedHoughGuil* instance, int angleThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setAngleThresh(angleThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getAngleThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAngleThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMinScale_double(cv::GeneralizedHoughGuil* instance, double minScale, ResultVoid* ocvrs_return) {
		try {
			instance->setMinScale(minScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMinScale_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMinScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setMaxScale_double(cv::GeneralizedHoughGuil* instance, double maxScale, ResultVoid* ocvrs_return) {
		try {
			instance->setMaxScale(maxScale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getMaxScale_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMaxScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setScaleStep_double(cv::GeneralizedHoughGuil* instance, double scaleStep, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleStep(scaleStep);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getScaleStep_const(const cv::GeneralizedHoughGuil* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getScaleStep();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setScaleThresh_int(cv::GeneralizedHoughGuil* instance, int scaleThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setScaleThresh(scaleThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getScaleThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getScaleThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_setPosThresh_int(cv::GeneralizedHoughGuil* instance, int posThresh, ResultVoid* ocvrs_return) {
		try {
			instance->setPosThresh(posThresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_GeneralizedHoughGuil_getPosThresh_const(const cv::GeneralizedHoughGuil* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPosThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_GeneralizedHoughGuil_to_Algorithm(cv::GeneralizedHoughGuil* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	cv::GeneralizedHough* cv_GeneralizedHoughGuil_to_GeneralizedHough(cv::GeneralizedHoughGuil* instance) {
			return dynamic_cast<cv::GeneralizedHough*>(instance);
	}
	
	void cv_GeneralizedHoughGuil_delete(cv::GeneralizedHoughGuil* instance) {
			delete instance;
	}
	
	void cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(const cv::Mat* img, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_const_MatR_Point_Point(const cv::Mat* img, cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*img, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Point_Point_int_bool(cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Point_Point(cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Size_Point_Point_int_bool(cv::Size* boundingAreaSize, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaSize, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Size_Point_Point(cv::Size* boundingAreaSize, cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaSize, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_LineIterator_Rect_Point_Point(cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator* ret = new cv::LineIterator(*boundingAreaRect, *pt1, *pt2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(cv::LineIterator* instance, const cv::Mat* img, cv::Rect* boundingAreaRect, cv::Point* pt1, cv::Point* pt2, int connectivity, bool leftToRight, ResultVoid* ocvrs_return) {
		try {
			instance->init(img, *boundingAreaRect, *pt1, *pt2, connectivity, leftToRight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_operatorX(cv::LineIterator* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->operator*();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_operatorAA(cv::LineIterator* instance, Result<cv::LineIterator*>* ocvrs_return) {
		try {
			cv::LineIterator ret = instance->operator++();
			Ok(new cv::LineIterator(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineIterator_pos_const(const cv::LineIterator* instance, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->pos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	unsigned char* cv_LineIterator_propPtr_const(const cv::LineIterator* instance) {
			unsigned char* const ret = instance->ptr;
			return ret;
	}
	
	unsigned char* cv_LineIterator_propPtr(cv::LineIterator* instance) {
			unsigned char* ret = instance->ptr;
			return ret;
	}
	
	void cv_LineIterator_propPtr_unsigned_charX(cv::LineIterator* instance, unsigned char* const val) {
			instance->ptr = val;
	}
	
	const unsigned char* cv_LineIterator_propPtr0_const(const cv::LineIterator* instance) {
			const unsigned char* ret = instance->ptr0;
			return ret;
	}
	
	int cv_LineIterator_propStep_const(const cv::LineIterator* instance) {
			int ret = instance->step;
			return ret;
	}
	
	void cv_LineIterator_propStep_const_int(cv::LineIterator* instance, const int val) {
			instance->step = val;
	}
	
	int cv_LineIterator_propElemSize_const(const cv::LineIterator* instance) {
			int ret = instance->elemSize;
			return ret;
	}
	
	void cv_LineIterator_propElemSize_const_int(cv::LineIterator* instance, const int val) {
			instance->elemSize = val;
	}
	
	int cv_LineIterator_propErr_const(const cv::LineIterator* instance) {
			int ret = instance->err;
			return ret;
	}
	
	void cv_LineIterator_propErr_const_int(cv::LineIterator* instance, const int val) {
			instance->err = val;
	}
	
	int cv_LineIterator_propCount_const(const cv::LineIterator* instance) {
			int ret = instance->count;
			return ret;
	}
	
	void cv_LineIterator_propCount_const_int(cv::LineIterator* instance, const int val) {
			instance->count = val;
	}
	
	int cv_LineIterator_propMinusDelta_const(const cv::LineIterator* instance) {
			int ret = instance->minusDelta;
			return ret;
	}
	
	void cv_LineIterator_propMinusDelta_const_int(cv::LineIterator* instance, const int val) {
			instance->minusDelta = val;
	}
	
	int cv_LineIterator_propPlusDelta_const(const cv::LineIterator* instance) {
			int ret = instance->plusDelta;
			return ret;
	}
	
	void cv_LineIterator_propPlusDelta_const_int(cv::LineIterator* instance, const int val) {
			instance->plusDelta = val;
	}
	
	int cv_LineIterator_propMinusStep_const(const cv::LineIterator* instance) {
			int ret = instance->minusStep;
			return ret;
	}
	
	void cv_LineIterator_propMinusStep_const_int(cv::LineIterator* instance, const int val) {
			instance->minusStep = val;
	}
	
	int cv_LineIterator_propPlusStep_const(const cv::LineIterator* instance) {
			int ret = instance->plusStep;
			return ret;
	}
	
	void cv_LineIterator_propPlusStep_const_int(cv::LineIterator* instance, const int val) {
			instance->plusStep = val;
	}
	
	int cv_LineIterator_propMinusShift_const(const cv::LineIterator* instance) {
			int ret = instance->minusShift;
			return ret;
	}
	
	void cv_LineIterator_propMinusShift_const_int(cv::LineIterator* instance, const int val) {
			instance->minusShift = val;
	}
	
	int cv_LineIterator_propPlusShift_const(const cv::LineIterator* instance) {
			int ret = instance->plusShift;
			return ret;
	}
	
	void cv_LineIterator_propPlusShift_const_int(cv::LineIterator* instance, const int val) {
			instance->plusShift = val;
	}
	
	void cv_LineIterator_propP_const(const cv::LineIterator* instance, cv::Point* ocvrs_return) {
			cv::Point ret = instance->p;
			*ocvrs_return = ret;
	}
	
	void cv_LineIterator_propP_const_Point(cv::LineIterator* instance, const cv::Point* val) {
			instance->p = *val;
	}
	
	bool cv_LineIterator_propPtmode_const(const cv::LineIterator* instance) {
			bool ret = instance->ptmode;
			return ret;
	}
	
	void cv_LineIterator_propPtmode_const_bool(cv::LineIterator* instance, const bool val) {
			instance->ptmode = val;
	}
	
	void cv_LineIterator_delete(cv::LineIterator* instance) {
			delete instance;
	}
	
	void cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(cv::LineSegmentDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, const cv::_OutputArray* width, const cv::_OutputArray* prec, const cv::_OutputArray* nfa, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *lines, *width, *prec, *nfa);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(cv::LineSegmentDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->detect(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(cv::LineSegmentDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, ResultVoid* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, const cv::_InputOutputArray* image, Result<int>* ocvrs_return) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2, *image);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR(cv::LineSegmentDetector* instance, const cv::Size* size, const cv::_InputArray* lines1, const cv::_InputArray* lines2, Result<int>* ocvrs_return) {
		try {
			int ret = instance->compareSegments(*size, *lines1, *lines2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::Algorithm* cv_LineSegmentDetector_to_Algorithm(cv::LineSegmentDetector* instance) {
			return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_LineSegmentDetector_delete(cv::LineSegmentDetector* instance) {
			delete instance;
	}
	
	void cv_Subdiv2D_Subdiv2D(Result<cv::Subdiv2D*>* ocvrs_return) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_Subdiv2D_Rect(cv::Rect* rect, Result<cv::Subdiv2D*>* ocvrs_return) {
		try {
			cv::Subdiv2D* ret = new cv::Subdiv2D(*rect);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_initDelaunay_Rect(cv::Subdiv2D* instance, cv::Rect* rect, ResultVoid* ocvrs_return) {
		try {
			instance->initDelaunay(*rect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_insert_Point2f(cv::Subdiv2D* instance, cv::Point2f* pt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->insert(*pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_insert_const_vectorLPoint2fGR(cv::Subdiv2D* instance, const std::vector<cv::Point2f>* ptvec, ResultVoid* ocvrs_return) {
		try {
			instance->insert(*ptvec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_locate_Point2f_intR_intR(cv::Subdiv2D* instance, cv::Point2f* pt, int* edge, int* vertex, Result<int>* ocvrs_return) {
		try {
			int ret = instance->locate(*pt, *edge, *vertex);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_findNearest_Point2f_Point2fX(cv::Subdiv2D* instance, cv::Point2f* pt, cv::Point2f* nearestPt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findNearest(*pt, nearestPt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_findNearest_Point2f(cv::Subdiv2D* instance, cv::Point2f* pt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findNearest(*pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getEdgeList_const_vectorLVec4fGR(const cv::Subdiv2D* instance, std::vector<cv::Vec4f>* edgeList, ResultVoid* ocvrs_return) {
		try {
			instance->getEdgeList(*edgeList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getLeadingEdgeList_const_vectorLintGR(const cv::Subdiv2D* instance, std::vector<int>* leadingEdgeList, ResultVoid* ocvrs_return) {
		try {
			instance->getLeadingEdgeList(*leadingEdgeList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getTriangleList_const_vectorLVec6fGR(const cv::Subdiv2D* instance, std::vector<cv::Vec6f>* triangleList, ResultVoid* ocvrs_return) {
		try {
			instance->getTriangleList(*triangleList);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getVoronoiFacetList_const_vectorLintGR_vectorLvectorLPoint2fGGR_vectorLPoint2fGR(cv::Subdiv2D* instance, const std::vector<int>* idx, std::vector<std::vector<cv::Point2f>>* facetList, std::vector<cv::Point2f>* facetCenters, ResultVoid* ocvrs_return) {
		try {
			instance->getVoronoiFacetList(*idx, *facetList, *facetCenters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getVertex_const_int_intX(const cv::Subdiv2D* instance, int vertex, int* firstEdge, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getVertex(vertex, firstEdge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getVertex_const_int(const cv::Subdiv2D* instance, int vertex, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->getVertex(vertex);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_getEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int nextEdgeType, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getEdge(edge, nextEdgeType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_nextEdge_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nextEdge(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_rotateEdge_const_int_int(const cv::Subdiv2D* instance, int edge, int rotate, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rotateEdge(edge, rotate);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_symEdge_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->symEdge(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_edgeOrg_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* orgpt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeOrg(edge, orgpt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_edgeOrg_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeOrg(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_edgeDst_const_int_Point2fX(const cv::Subdiv2D* instance, int edge, cv::Point2f* dstpt, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeDst(edge, dstpt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_edgeDst_const_int(const cv::Subdiv2D* instance, int edge, Result<int>* ocvrs_return) {
		try {
			int ret = instance->edgeDst(edge);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_Subdiv2D_delete(cv::Subdiv2D* instance) {
			delete instance;
	}
	
	void cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB(Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB* ret = new cv::segmentation::IntelligentScissorsMB();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(cv::segmentation::IntelligentScissorsMB* instance, float weight_non_edge, float weight_gradient_direction, float weight_gradient_magnitude, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setWeights(weight_non_edge, weight_gradient_direction, weight_gradient_magnitude);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_threshold_max, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setGradientMagnitudeMaxLimit(gradient_magnitude_threshold_max);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit(cv::segmentation::IntelligentScissorsMB* instance, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setGradientMagnitudeMaxLimit();
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(cv::segmentation::IntelligentScissorsMB* instance, float gradient_magnitude_min_value, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureZeroCrossingParameters(gradient_magnitude_min_value);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters(cv::segmentation::IntelligentScissorsMB* instance, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureZeroCrossingParameters();
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(cv::segmentation::IntelligentScissorsMB* instance, double threshold1, double threshold2, int apertureSize, bool L2gradient, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureCannyParameters(threshold1, threshold2, apertureSize, L2gradient);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double(cv::segmentation::IntelligentScissorsMB* instance, double threshold1, double threshold2, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->setEdgeFeatureCannyParameters(threshold1, threshold2);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* image, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImage(*image);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* non_edge, const cv::_InputArray* gradient_direction, const cv::_InputArray* gradient_magnitude, const cv::_InputArray* image, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImageFeatures(*non_edge, *gradient_direction, *gradient_magnitude, *image);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::segmentation::IntelligentScissorsMB* instance, const cv::_InputArray* non_edge, const cv::_InputArray* gradient_direction, const cv::_InputArray* gradient_magnitude, Result<cv::segmentation::IntelligentScissorsMB*>* ocvrs_return) {
		try {
			cv::segmentation::IntelligentScissorsMB ret = instance->applyImageFeatures(*non_edge, *gradient_direction, *gradient_magnitude);
			Ok(new cv::segmentation::IntelligentScissorsMB(ret), ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* sourcePt, ResultVoid* ocvrs_return) {
		try {
			instance->buildMap(*sourcePt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(const cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* targetPt, const cv::_OutputArray* contour, bool backward, ResultVoid* ocvrs_return) {
		try {
			instance->getContour(*targetPt, *contour, backward);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	void cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR(const cv::segmentation::IntelligentScissorsMB* instance, const cv::Point* targetPt, const cv::_OutputArray* contour, ResultVoid* ocvrs_return) {
		try {
			instance->getContour(*targetPt, *contour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(ocvrs_return);
	}
	
	cv::segmentation::IntelligentScissorsMB* cv_segmentation_IntelligentScissorsMB_implicitClone_const(const cv::segmentation::IntelligentScissorsMB* instance) {
			return new cv::segmentation::IntelligentScissorsMB(*instance);
	}
	
	void cv_segmentation_IntelligentScissorsMB_delete(cv::segmentation::IntelligentScissorsMB* instance) {
			delete instance;
	}
	
}
