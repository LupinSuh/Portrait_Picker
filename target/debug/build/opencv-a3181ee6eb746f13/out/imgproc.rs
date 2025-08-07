//! # Image Processing
//! 
//! This module offers a comprehensive suite of image processing functions, enabling tasks such as those listed above.
//!    # Image Filtering
//! 
//!    Functions and classes described in this section are used to perform various linear or non-linear
//!    filtering operations on 2D images (represented as Mat's). It means that for each pixel location
//!    ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29) in the source image (normally, rectangular), its neighborhood is considered and used to
//!    compute the response. In case of a linear filter, it is a weighted sum of pixel values. In case of
//!    morphological operations, it is the minimum or maximum values, and so on. The computed response is
//!    stored in the destination image at the same location ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29). It means that the output image
//!    will be of the same size as the input image. Normally, the functions support multi-channel arrays,
//!    in which case every channel is processed independently. Therefore, the output image will also have
//!    the same number of channels as the input one.
//! 
//!    Another common feature of the functions and classes described in this section is that, unlike
//!    simple arithmetic functions, they need to extrapolate values of some non-existing pixels. For
//!    example, if you want to smooth an image using a Gaussian ![inline formula](https://latex.codecogs.com/png.latex?3%20%5Ctimes%203) filter, then, when
//!    processing the left-most pixels in each row, you need pixels to the left of them, that is, outside
//!    of the image. You can let these pixels be the same as the left-most image pixels ("replicated
//!    border" extrapolation method), or assume that all the non-existing pixels are zeros ("constant
//!    border" extrapolation method), and so on. OpenCV enables you to specify the extrapolation method.
//!    For details, see [border_types]
//! 
//!    @anchor filter_depths
//!    ### Depth combinations
//!    Input depth (src.depth()) | Output depth (ddepth)
//!    --------------------------|----------------------
//!    CV_8U                     | -1/CV_16S/CV_32F/CV_64F
//!    CV_16U/CV_16S             | -1/CV_32F/CV_64F
//!    CV_32F                    | -1/CV_32F
//!    CV_64F                    | -1/CV_64F
//! 
//!     
//! Note: when ddepth=-1, the output image will have the same depth as the source.
//! 
//!     
//! Note: if you need double floating-point accuracy and using single floating-point input data
//!    (CV_32F input and CV_64F output depth combination), you can use [Mat].convertTo to convert
//!    the input data to the desired precision.
//! 
//!    # Geometric Image Transformations
//! 
//!    The functions in this section perform various geometrical transformations of 2D images. They do not
//!    change the image content but deform the pixel grid and map this deformed grid to the destination
//!    image. In fact, to avoid sampling artifacts, the mapping is done in the reverse order, from
//!    destination to the source. That is, for each pixel ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) of the destination image, the
//!    functions compute coordinates of the corresponding "donor" pixel in the source image and copy the
//!    pixel value:
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%3D%20%5Ctexttt%7Bsrc%7D%20%28f%5Fx%28x%2Cy%29%2C%20f%5Fy%28x%2Cy%29%29)
//! 
//!    In case when you specify the forward mapping ![inline formula](https://latex.codecogs.com/png.latex?%5Cleft%3Cg%5Fx%2C%20g%5Fy%5Cright%3E%3A%20%5Ctexttt%7Bsrc%7D%20%5Crightarrow%0A%20%20%20%20%5Ctexttt%7Bdst%7D), the OpenCV functions first compute the corresponding inverse mapping
//!    ![inline formula](https://latex.codecogs.com/png.latex?%5Cleft%3Cf%5Fx%2C%20f%5Fy%5Cright%3E%3A%20%5Ctexttt%7Bdst%7D%20%5Crightarrow%20%5Ctexttt%7Bsrc%7D) and then use the above formula.
//! 
//!    The actual implementations of the geometrical transformations, from the most generic remap and to
//!    the simplest and the fastest resize, need to solve two main problems with the above formula:
//! 
//!    - Extrapolation of non-existing pixels. Similarly to the filtering functions described in the
//!    previous section, for some ![inline formula](https://latex.codecogs.com/png.latex?%28x%2Cy%29), either one of ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%28x%2Cy%29), or ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy%28x%2Cy%29), or both
//!    of them may fall outside of the image. In this case, an extrapolation method needs to be used.
//!    OpenCV provides the same selection of extrapolation methods as in the filtering functions. In
//!    addition, it provides the method #BORDER_TRANSPARENT. This means that the corresponding pixels in
//!    the destination image will not be modified at all.
//! 
//!    - Interpolation of pixel values. Usually ![inline formula](https://latex.codecogs.com/png.latex?f%5Fx%28x%2Cy%29) and ![inline formula](https://latex.codecogs.com/png.latex?f%5Fy%28x%2Cy%29) are floating-point
//!    numbers. This means that ![inline formula](https://latex.codecogs.com/png.latex?%5Cleft%3Cf%5Fx%2C%20f%5Fy%5Cright%3E) can be either an affine or perspective
//!    transformation, or radial lens distortion correction, and so on. So, a pixel value at fractional
//!    coordinates needs to be retrieved. In the simplest case, the coordinates can be just rounded to the
//!    nearest integer coordinates and the corresponding pixel can be used. This is called a
//!    nearest-neighbor interpolation. However, a better result can be achieved by using more
//!    sophisticated [interpolation methods](https://en.wikipedia.org/wiki/Multivariate_interpolation) ,
//!    where a polynomial function is fit into some neighborhood of the computed pixel ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%28x%2Cy%29%2C%0A%20%20%20%20f%5Fy%28x%2Cy%29%29), and then the value of the polynomial at ![inline formula](https://latex.codecogs.com/png.latex?%28f%5Fx%28x%2Cy%29%2C%20f%5Fy%28x%2Cy%29%29) is taken as the
//!    interpolated pixel value. In OpenCV, you can choose between several interpolation methods. See
//!    [resize] for details.
//! 
//!     
//! Note: The geometrical transformations do not work with `CV_8S` or `CV_32S` images.
//! 
//!    # Miscellaneous Image Transformations
//!    # Drawing Functions
//! 
//!    Drawing functions work with matrices/images of arbitrary depth. The boundaries of the shapes can be
//!    rendered with antialiasing (implemented only for 8-bit images for now). All the functions include
//!    the parameter color that uses an RGB value (that may be constructed with the Scalar constructor )
//!    for color images and brightness for grayscale images. For color images, the channel ordering is
//!    normally *Blue, Green, Red*. This is what imshow, imread, and imwrite expect. So, if you form a
//!    color using the Scalar constructor, it should look like:
//! 
//!    ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BScalar%7D%20%28blue%20%5C%5F%20component%2C%20green%20%5C%5F%20component%2C%20red%20%5C%5F%20component%5B%2C%20alpha%20%5C%5F%20component%5D%29)
//! 
//!    If you are using your own image rendering and I/O functions, you can use any channel ordering. The
//!    drawing functions process each channel independently and do not depend on the channel order or even
//!    on the used color space. The whole image can be converted from BGR to RGB or to a different color
//!    space using cvtColor .
//! 
//!    If a drawn figure is partially or completely outside the image, the drawing functions clip it. Also,
//!    many drawing functions can handle pixel coordinates specified with sub-pixel accuracy. This means
//!    that the coordinates can be passed as fixed-point numbers encoded as integers. The number of
//!    fractional bits is specified by the shift parameter and the real point coordinates are calculated as
//!    ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BPoint%7D%28x%2Cy%29%5Crightarrow%5Ctexttt%7BPoint2f%7D%28x%2A2%5E%7B%2Dshift%7D%2Cy%2A2%5E%7B%2Dshift%7D%29) . This feature is
//!    especially effective when rendering antialiased shapes.
//! 
//!     
//! Note: The functions do not support alpha-transparency when the target image is 4-channel. In this
//!    case, the color[3] is simply copied to the repainted pixels. Thus, if you want to paint
//!    semi-transparent shapes, you can paint them in a separate buffer and then blend it with the main
//!    image.
//! 
//!    # Color Space Conversions
//!    # ColorMaps in OpenCV
//! 
//!    The human perception isn't built for observing fine changes in grayscale images. Human eyes are more
//!    sensitive to observing changes between colors, so you often need to recolor your grayscale images to
//!    get a clue about them. OpenCV now comes with various colormaps to enhance the visualization in your
//!    computer vision application.
//! 
//!    In OpenCV you only need applyColorMap to apply a colormap on a given image. The following sample
//!    code reads the path to an image from command line, applies a Jet colormap on it and shows the
//!    result:
//! 
//!    @include snippets/imgproc_applyColorMap.cpp
//! ## See also
//! [colormap_types]
//! 
//!    # Planar Subdivision
//! 
//!    The Subdiv2D class described in this section is used to perform various planar subdivision on
//!    a set of 2D points (represented as vector of Point2f). OpenCV subdivides a plane into triangles
//!    using the Delaunay's algorithm, which corresponds to the dual graph of the Voronoi diagram.
//!    In the figure below, the Delaunay's triangulation is marked with black lines and the Voronoi
//!    diagram with red lines.
//! 
//!    ![Delaunay triangulation (black) and Voronoi (red)](https://docs.opencv.org/4.12.0/delaunay_voronoi.png)
//! 
//!    The subdivisions can be used for the 3D piece-wise transformation of a plane, morphing, fast
//!    location of points on the plane, building special graphs (such as NNG,RNG), and so forth.
//! 
//!    # Histograms
//!    # Structural Analysis and Shape Descriptors
//!    # Motion Analysis and Object Tracking
//!    # Feature Detection
//!    # Object Detection
//!    # Image Segmentation
//!    # Hardware Acceleration Layer
//!        # Functions
//!        # Interface
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::GeneralizedHoughTraitConst, super::GeneralizedHoughTrait, super::GeneralizedHoughBallardTraitConst, super::GeneralizedHoughBallardTrait, super::GeneralizedHoughGuilTraitConst, super::GeneralizedHoughGuilTrait, super::CLAHETraitConst, super::CLAHETrait, super::Subdiv2DTraitConst, super::Subdiv2DTrait, super::LineSegmentDetectorTraitConst, super::LineSegmentDetectorTrait, super::LineIteratorTraitConst, super::LineIteratorTrait, super::IntelligentScissorsMBTraitConst, super::IntelligentScissorsMBTrait };
}

/// the threshold value ![inline formula](https://latex.codecogs.com/png.latex?T%28x%2C%20y%29) is a weighted sum (cross-correlation with a Gaussian
/// window) of the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BblockSize%7D%20%5Ctimes%20%5Ctexttt%7BblockSize%7D) neighborhood of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29)
/// minus C . The default sigma (standard deviation) is used for the specified blockSize . See
/// #getGaussianKernel
pub const ADAPTIVE_THRESH_GAUSSIAN_C: i32 = 1;
/// the threshold value ![inline formula](https://latex.codecogs.com/png.latex?T%28x%2Cy%29) is a mean of the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BblockSize%7D%20%5Ctimes%0A%5Ctexttt%7BblockSize%7D) neighborhood of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) minus C
pub const ADAPTIVE_THRESH_MEAN_C: i32 = 0;
/// Same as CCL_GRANA. It is preferable to use the flag with the name of the algorithm (CCL_BBDT) rather than the one with the name of the first author (CCL_GRANA).
pub const CCL_BBDT: i32 = 4;
/// Spaghetti [Bolelli2019](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2019) algorithm for 8-way connectivity, Spaghetti4C [Bolelli2021](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2021) algorithm for 4-way connectivity. The parallel implementation described in [Bolelli2017](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2017) is available for both Spaghetti and Spaghetti4C.
pub const CCL_BOLELLI: i32 = 2;
/// Spaghetti [Bolelli2019](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2019) algorithm for 8-way connectivity, Spaghetti4C [Bolelli2021](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2021) algorithm for 4-way connectivity.
pub const CCL_DEFAULT: i32 = -1;
/// BBDT [Grana2010](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Grana2010) algorithm for 8-way connectivity, SAUF algorithm for 4-way connectivity. The parallel implementation described in [Bolelli2017](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2017) is available for both BBDT and SAUF.
pub const CCL_GRANA: i32 = 1;
/// Same as CCL_WU. It is preferable to use the flag with the name of the algorithm (CCL_SAUF) rather than the one with the name of the first author (CCL_WU).
pub const CCL_SAUF: i32 = 3;
/// Same as CCL_BOLELLI. It is preferable to use the flag with the name of the algorithm (CCL_SPAGHETTI) rather than the one with the name of the first author (CCL_BOLELLI).
pub const CCL_SPAGHETTI: i32 = 5;
/// SAUF [Wu2009](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Wu2009) algorithm for 8-way connectivity, SAUF algorithm for 4-way connectivity. The parallel implementation described in [Bolelli2017](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2017) is available for SAUF.
pub const CCL_WU: i32 = 0;
/// The total area (in pixels) of the connected component
pub const CC_STAT_AREA: i32 = 4;
/// The vertical size of the bounding box
pub const CC_STAT_HEIGHT: i32 = 3;
/// The leftmost (x) coordinate which is the inclusive start of the bounding
/// box in the horizontal direction.
pub const CC_STAT_LEFT: i32 = 0;
/// Max enumeration value. Used internally only for memory allocation
pub const CC_STAT_MAX: i32 = 5;
/// The topmost (y) coordinate which is the inclusive start of the bounding
/// box in the vertical direction.
pub const CC_STAT_TOP: i32 = 1;
/// The horizontal size of the bounding box
pub const CC_STAT_WIDTH: i32 = 2;
/// stores absolutely all the contour points. That is, any 2 subsequent points (x1,y1) and
/// (x2,y2) of the contour will be either horizontal, vertical or diagonal neighbors, that is,
/// max(abs(x1-x2),abs(y2-y1))==1.
pub const CHAIN_APPROX_NONE: i32 = 1;
/// compresses horizontal, vertical, and diagonal segments and leaves only their end points.
/// For example, an up-right rectangular contour is encoded with 4 points.
pub const CHAIN_APPROX_SIMPLE: i32 = 2;
/// applies one of the flavors of the Teh-Chin chain approximation algorithm [TehChin89](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_TehChin89)
pub const CHAIN_APPROX_TC89_KCOS: i32 = 4;
/// applies one of the flavors of the Teh-Chin chain approximation algorithm [TehChin89](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_TehChin89)
pub const CHAIN_APPROX_TC89_L1: i32 = 3;
/// ![autumn](https://docs.opencv.org/4.12.0/colorscale_autumn.jpg)
pub const COLORMAP_AUTUMN: i32 = 0;
/// ![bone](https://docs.opencv.org/4.12.0/colorscale_bone.jpg)
pub const COLORMAP_BONE: i32 = 1;
/// ![cividis](https://docs.opencv.org/4.12.0/colorscale_cividis.jpg)
pub const COLORMAP_CIVIDIS: i32 = 17;
/// ![cool](https://docs.opencv.org/4.12.0/colorscale_cool.jpg)
pub const COLORMAP_COOL: i32 = 8;
/// ![deepgreen](https://docs.opencv.org/4.12.0/colorscale_deepgreen.jpg)
pub const COLORMAP_DEEPGREEN: i32 = 21;
/// ![hot](https://docs.opencv.org/4.12.0/colorscale_hot.jpg)
pub const COLORMAP_HOT: i32 = 11;
/// ![HSV](https://docs.opencv.org/4.12.0/colorscale_hsv.jpg)
pub const COLORMAP_HSV: i32 = 9;
/// ![inferno](https://docs.opencv.org/4.12.0/colorscale_inferno.jpg)
pub const COLORMAP_INFERNO: i32 = 14;
/// ![jet](https://docs.opencv.org/4.12.0/colorscale_jet.jpg)
pub const COLORMAP_JET: i32 = 2;
/// ![magma](https://docs.opencv.org/4.12.0/colorscale_magma.jpg)
pub const COLORMAP_MAGMA: i32 = 13;
/// ![ocean](https://docs.opencv.org/4.12.0/colorscale_ocean.jpg)
pub const COLORMAP_OCEAN: i32 = 5;
/// ![parula](https://docs.opencv.org/4.12.0/colorscale_parula.jpg)
pub const COLORMAP_PARULA: i32 = 12;
/// ![pink](https://docs.opencv.org/4.12.0/colorscale_pink.jpg)
pub const COLORMAP_PINK: i32 = 10;
/// ![plasma](https://docs.opencv.org/4.12.0/colorscale_plasma.jpg)
pub const COLORMAP_PLASMA: i32 = 15;
/// ![rainbow](https://docs.opencv.org/4.12.0/colorscale_rainbow.jpg)
pub const COLORMAP_RAINBOW: i32 = 4;
/// ![spring](https://docs.opencv.org/4.12.0/colorscale_spring.jpg)
pub const COLORMAP_SPRING: i32 = 7;
/// ![summer](https://docs.opencv.org/4.12.0/colorscale_summer.jpg)
pub const COLORMAP_SUMMER: i32 = 6;
/// ![turbo](https://docs.opencv.org/4.12.0/colorscale_turbo.jpg)
pub const COLORMAP_TURBO: i32 = 20;
/// ![twilight](https://docs.opencv.org/4.12.0/colorscale_twilight.jpg)
pub const COLORMAP_TWILIGHT: i32 = 18;
/// ![twilight shifted](https://docs.opencv.org/4.12.0/colorscale_twilight_shifted.jpg)
pub const COLORMAP_TWILIGHT_SHIFTED: i32 = 19;
/// ![viridis](https://docs.opencv.org/4.12.0/colorscale_viridis.jpg)
pub const COLORMAP_VIRIDIS: i32 = 16;
/// ![winter](https://docs.opencv.org/4.12.0/colorscale_winter.jpg)
pub const COLORMAP_WINTER: i32 = 3;
/// convert between RGB/BGR and BGR555 (16-bit images)
pub const COLOR_BGR2BGR555: i32 = 22;
/// convert between RGB/BGR and BGR565 (16-bit images)
pub const COLOR_BGR2BGR565: i32 = 12;
/// add alpha channel to RGB or BGR image
pub const COLOR_BGR2BGRA: i32 = 0;
/// convert between RGB/BGR and grayscale, [color_convert_rgb_gray] "color conversions"
pub const COLOR_BGR2GRAY: i32 = 6;
/// convert RGB/BGR to HLS (hue lightness saturation) with H range 0..180 if 8 bit image, [color_convert_rgb_hls] "color conversions"
pub const COLOR_BGR2HLS: i32 = 52;
/// convert RGB/BGR to HLS (hue lightness saturation) with H range 0..255 if 8 bit image, [color_convert_rgb_hls] "color conversions"
pub const COLOR_BGR2HLS_FULL: i32 = 68;
/// convert RGB/BGR to HSV (hue saturation value) with H range 0..180 if 8 bit image, [color_convert_rgb_hsv] "color conversions"
pub const COLOR_BGR2HSV: i32 = 40;
/// convert RGB/BGR to HSV (hue saturation value) with H range 0..255 if 8 bit image, [color_convert_rgb_hsv] "color conversions"
pub const COLOR_BGR2HSV_FULL: i32 = 66;
/// convert RGB/BGR to CIE Lab, [color_convert_rgb_lab] "color conversions"
pub const COLOR_BGR2Lab: i32 = 44;
/// convert RGB/BGR to CIE Luv, [color_convert_rgb_luv] "color conversions"
pub const COLOR_BGR2Luv: i32 = 50;
pub const COLOR_BGR2RGB: i32 = 4;
/// convert between RGB and BGR color spaces (with or without alpha channel)
pub const COLOR_BGR2RGBA: i32 = 2;
/// convert RGB/BGR to CIE XYZ, [color_convert_rgb_xyz] "color conversions"
pub const COLOR_BGR2XYZ: i32 = 32;
/// convert RGB/BGR to luma-chroma (aka YCC), [color_convert_rgb_ycrcb] "color conversions"
pub const COLOR_BGR2YCrCb: i32 = 36;
/// convert between RGB/BGR and YUV
pub const COLOR_BGR2YUV: i32 = 82;
/// convert between BGR and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGR2YUV_I420: i32 = 128;
/// synonym to I420
pub const COLOR_BGR2YUV_IYUV: i32 = 128;
/// synonym to UYVY
pub const COLOR_BGR2YUV_UYNV: i32 = 144;
/// convert between BGR and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGR2YUV_UYVY: i32 = 144;
/// synonym to UYVY
pub const COLOR_BGR2YUV_Y422: i32 = 144;
/// synonym to YUY2
pub const COLOR_BGR2YUV_YUNV: i32 = 148;
/// convert between BGR and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGR2YUV_YUY2: i32 = 148;
/// synonym to YUY2
pub const COLOR_BGR2YUV_YUYV: i32 = 148;
/// convert between BGR and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGR2YUV_YV12: i32 = 132;
/// convert between BGR and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGR2YUV_YVYU: i32 = 150;
pub const COLOR_BGR5552BGR: i32 = 24;
pub const COLOR_BGR5552BGRA: i32 = 28;
pub const COLOR_BGR5552GRAY: i32 = 31;
pub const COLOR_BGR5552RGB: i32 = 25;
pub const COLOR_BGR5552RGBA: i32 = 29;
pub const COLOR_BGR5652BGR: i32 = 14;
pub const COLOR_BGR5652BGRA: i32 = 18;
pub const COLOR_BGR5652GRAY: i32 = 21;
pub const COLOR_BGR5652RGB: i32 = 15;
pub const COLOR_BGR5652RGBA: i32 = 19;
/// remove alpha channel from RGB or BGR image
pub const COLOR_BGRA2BGR: i32 = 1;
pub const COLOR_BGRA2BGR555: i32 = 26;
pub const COLOR_BGRA2BGR565: i32 = 16;
pub const COLOR_BGRA2GRAY: i32 = 10;
pub const COLOR_BGRA2RGB: i32 = 3;
pub const COLOR_BGRA2RGBA: i32 = 5;
/// convert between BGRA and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGRA2YUV_I420: i32 = 130;
/// synonym to I420
pub const COLOR_BGRA2YUV_IYUV: i32 = 130;
/// synonym to UYVY
pub const COLOR_BGRA2YUV_UYNV: i32 = 146;
/// convert between BGRA and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGRA2YUV_UYVY: i32 = 146;
/// synonym to UYVY
pub const COLOR_BGRA2YUV_Y422: i32 = 146;
/// synonym to YUY2
pub const COLOR_BGRA2YUV_YUNV: i32 = 152;
/// convert between BGRA and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGRA2YUV_YUY2: i32 = 152;
/// synonym to YUY2
pub const COLOR_BGRA2YUV_YUYV: i32 = 152;
/// convert between BGRA and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGRA2YUV_YV12: i32 = 134;
/// convert between BGRA and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_BGRA2YUV_YVYU: i32 = 154;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2BGR: i32 = 46;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2BGRA: i32 = 139;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2BGR_EA: i32 = 135;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2BGR_VNG: i32 = 62;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2GRAY: i32 = 86;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2RGB: i32 = 48;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2RGBA: i32 = 141;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2RGB_EA: i32 = 137;
/// equivalent to RGGB Bayer pattern
pub const COLOR_BayerBG2RGB_VNG: i32 = 64;
pub const COLOR_BayerBGGR2BGR: i32 = 48;
pub const COLOR_BayerBGGR2BGRA: i32 = 141;
pub const COLOR_BayerBGGR2BGR_EA: i32 = 137;
pub const COLOR_BayerBGGR2BGR_VNG: i32 = 64;
pub const COLOR_BayerBGGR2GRAY: i32 = 88;
pub const COLOR_BayerBGGR2RGB: i32 = 46;
pub const COLOR_BayerBGGR2RGBA: i32 = 139;
pub const COLOR_BayerBGGR2RGB_EA: i32 = 135;
pub const COLOR_BayerBGGR2RGB_VNG: i32 = 62;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2BGR: i32 = 47;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2BGRA: i32 = 140;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2BGR_EA: i32 = 136;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2BGR_VNG: i32 = 63;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2GRAY: i32 = 87;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2RGB: i32 = 49;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2RGBA: i32 = 142;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2RGB_EA: i32 = 138;
/// equivalent to GRBG Bayer pattern
pub const COLOR_BayerGB2RGB_VNG: i32 = 65;
pub const COLOR_BayerGBRG2BGR: i32 = 49;
pub const COLOR_BayerGBRG2BGRA: i32 = 142;
pub const COLOR_BayerGBRG2BGR_EA: i32 = 138;
pub const COLOR_BayerGBRG2BGR_VNG: i32 = 65;
pub const COLOR_BayerGBRG2GRAY: i32 = 89;
pub const COLOR_BayerGBRG2RGB: i32 = 47;
pub const COLOR_BayerGBRG2RGBA: i32 = 140;
pub const COLOR_BayerGBRG2RGB_EA: i32 = 136;
pub const COLOR_BayerGBRG2RGB_VNG: i32 = 63;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2BGR: i32 = 49;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2BGRA: i32 = 142;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2BGR_EA: i32 = 138;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2BGR_VNG: i32 = 65;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2GRAY: i32 = 89;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2RGB: i32 = 47;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2RGBA: i32 = 140;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2RGB_EA: i32 = 136;
/// equivalent to GBRG Bayer pattern
pub const COLOR_BayerGR2RGB_VNG: i32 = 63;
pub const COLOR_BayerGRBG2BGR: i32 = 47;
pub const COLOR_BayerGRBG2BGRA: i32 = 140;
pub const COLOR_BayerGRBG2BGR_EA: i32 = 136;
pub const COLOR_BayerGRBG2BGR_VNG: i32 = 63;
pub const COLOR_BayerGRBG2GRAY: i32 = 87;
pub const COLOR_BayerGRBG2RGB: i32 = 49;
pub const COLOR_BayerGRBG2RGBA: i32 = 142;
pub const COLOR_BayerGRBG2RGB_EA: i32 = 138;
pub const COLOR_BayerGRBG2RGB_VNG: i32 = 65;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2BGR: i32 = 48;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2BGRA: i32 = 141;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2BGR_EA: i32 = 137;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2BGR_VNG: i32 = 64;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2GRAY: i32 = 88;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2RGB: i32 = 46;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2RGBA: i32 = 139;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2RGB_EA: i32 = 135;
/// equivalent to BGGR Bayer pattern
pub const COLOR_BayerRG2RGB_VNG: i32 = 62;
pub const COLOR_BayerRGGB2BGR: i32 = 46;
pub const COLOR_BayerRGGB2BGRA: i32 = 139;
pub const COLOR_BayerRGGB2BGR_EA: i32 = 135;
pub const COLOR_BayerRGGB2BGR_VNG: i32 = 62;
pub const COLOR_BayerRGGB2GRAY: i32 = 86;
pub const COLOR_BayerRGGB2RGB: i32 = 48;
pub const COLOR_BayerRGGB2RGBA: i32 = 141;
pub const COLOR_BayerRGGB2RGB_EA: i32 = 137;
pub const COLOR_BayerRGGB2RGB_VNG: i32 = 64;
pub const COLOR_COLORCVT_MAX: i32 = 155;
pub const COLOR_GRAY2BGR: i32 = 8;
/// convert between grayscale and BGR555 (16-bit images)
pub const COLOR_GRAY2BGR555: i32 = 30;
/// convert between grayscale to BGR565 (16-bit images)
pub const COLOR_GRAY2BGR565: i32 = 20;
pub const COLOR_GRAY2BGRA: i32 = 9;
pub const COLOR_GRAY2RGB: i32 = 8;
pub const COLOR_GRAY2RGBA: i32 = 9;
/// backward conversions HLS to RGB/BGR with H range 0..180 if 8 bit image
pub const COLOR_HLS2BGR: i32 = 60;
/// backward conversions HLS to RGB/BGR with H range 0..255 if 8 bit image
pub const COLOR_HLS2BGR_FULL: i32 = 72;
pub const COLOR_HLS2RGB: i32 = 61;
pub const COLOR_HLS2RGB_FULL: i32 = 73;
/// backward conversions HSV to RGB/BGR with H range 0..180 if 8 bit image
pub const COLOR_HSV2BGR: i32 = 54;
/// backward conversions HSV to RGB/BGR with H range 0..255 if 8 bit image
pub const COLOR_HSV2BGR_FULL: i32 = 70;
pub const COLOR_HSV2RGB: i32 = 55;
pub const COLOR_HSV2RGB_FULL: i32 = 71;
pub const COLOR_LBGR2Lab: i32 = 74;
pub const COLOR_LBGR2Luv: i32 = 76;
pub const COLOR_LRGB2Lab: i32 = 75;
pub const COLOR_LRGB2Luv: i32 = 77;
pub const COLOR_Lab2BGR: i32 = 56;
pub const COLOR_Lab2LBGR: i32 = 78;
pub const COLOR_Lab2LRGB: i32 = 79;
pub const COLOR_Lab2RGB: i32 = 57;
pub const COLOR_Luv2BGR: i32 = 58;
pub const COLOR_Luv2LBGR: i32 = 80;
pub const COLOR_Luv2LRGB: i32 = 81;
pub const COLOR_Luv2RGB: i32 = 59;
pub const COLOR_RGB2BGR: i32 = 4;
pub const COLOR_RGB2BGR555: i32 = 23;
pub const COLOR_RGB2BGR565: i32 = 13;
pub const COLOR_RGB2BGRA: i32 = 2;
pub const COLOR_RGB2GRAY: i32 = 7;
pub const COLOR_RGB2HLS: i32 = 53;
pub const COLOR_RGB2HLS_FULL: i32 = 69;
pub const COLOR_RGB2HSV: i32 = 41;
pub const COLOR_RGB2HSV_FULL: i32 = 67;
pub const COLOR_RGB2Lab: i32 = 45;
pub const COLOR_RGB2Luv: i32 = 51;
pub const COLOR_RGB2RGBA: i32 = 0;
pub const COLOR_RGB2XYZ: i32 = 33;
pub const COLOR_RGB2YCrCb: i32 = 37;
pub const COLOR_RGB2YUV: i32 = 83;
/// convert between RGB and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGB2YUV_I420: i32 = 127;
/// synonym to I420
pub const COLOR_RGB2YUV_IYUV: i32 = 127;
/// synonym to UYVY
pub const COLOR_RGB2YUV_UYNV: i32 = 143;
/// convert between RGB and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGB2YUV_UYVY: i32 = 143;
/// synonym to UYVY
pub const COLOR_RGB2YUV_Y422: i32 = 143;
/// synonym to YUY2
pub const COLOR_RGB2YUV_YUNV: i32 = 147;
/// convert between RGB and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGB2YUV_YUY2: i32 = 147;
/// synonym to YUY2
pub const COLOR_RGB2YUV_YUYV: i32 = 147;
/// convert between RGB and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGB2YUV_YV12: i32 = 131;
/// convert between RGB and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGB2YUV_YVYU: i32 = 149;
pub const COLOR_RGBA2BGR: i32 = 3;
pub const COLOR_RGBA2BGR555: i32 = 27;
pub const COLOR_RGBA2BGR565: i32 = 17;
pub const COLOR_RGBA2BGRA: i32 = 5;
pub const COLOR_RGBA2GRAY: i32 = 11;
pub const COLOR_RGBA2RGB: i32 = 1;
/// convert between RGBA and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGBA2YUV_I420: i32 = 129;
/// synonym to I420
pub const COLOR_RGBA2YUV_IYUV: i32 = 129;
/// synonym to UYVY
pub const COLOR_RGBA2YUV_UYNV: i32 = 145;
/// convert between RGBA and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGBA2YUV_UYVY: i32 = 145;
/// synonym to UYVY
pub const COLOR_RGBA2YUV_Y422: i32 = 145;
/// synonym to YUY2
pub const COLOR_RGBA2YUV_YUNV: i32 = 151;
/// convert between RGBA and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGBA2YUV_YUY2: i32 = 151;
/// synonym to YUY2
pub const COLOR_RGBA2YUV_YUYV: i32 = 151;
/// convert between RGBA and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGBA2YUV_YV12: i32 = 133;
/// convert between RGBA and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_RGBA2YUV_YVYU: i32 = 153;
/// alpha premultiplication
pub const COLOR_RGBA2mRGBA: i32 = 125;
pub const COLOR_XYZ2BGR: i32 = 34;
pub const COLOR_XYZ2RGB: i32 = 35;
pub const COLOR_YCrCb2BGR: i32 = 38;
pub const COLOR_YCrCb2RGB: i32 = 39;
pub const COLOR_YUV2BGR: i32 = 84;
/// synonym to IYUV
pub const COLOR_YUV2BGRA_I420: i32 = 105;
/// convert between 4:2:0-subsampled YUV YV12 and BGRA, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_IYUV: i32 = 105;
/// convert between 4:2:0-subsampled YUV NV12 and BGRA, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_NV12: i32 = 95;
/// convert between 4:2:0-subsampled YUV NV21 and BGRA, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_NV21: i32 = 97;
/// synonym to UYVY
pub const COLOR_YUV2BGRA_UYNV: i32 = 112;
/// convert between YUV UYVY and BGRA, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_UYVY: i32 = 112;
/// synonym to UYVY
pub const COLOR_YUV2BGRA_Y422: i32 = 112;
/// synonym to YUY2
pub const COLOR_YUV2BGRA_YUNV: i32 = 120;
/// convert between YUV YUY2 and BGRA, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_YUY2: i32 = 120;
/// synonym to YUY2
pub const COLOR_YUV2BGRA_YUYV: i32 = 120;
/// convert between 4:2:0-subsampled YUV YV12 and BGRA, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_YV12: i32 = 103;
/// convert between YUV YVYU and BGRA, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGRA_YVYU: i32 = 122;
/// synonym to IYUV
pub const COLOR_YUV2BGR_I420: i32 = 101;
/// convert between 4:2:0-subsampled YUV IYUV and BGR, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_IYUV: i32 = 101;
/// convert between 4:2:0-subsampled YUV NV12 and BGR, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_NV12: i32 = 91;
/// convert between 4:2:0-subsampled YUV NV21 and BGR, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_NV21: i32 = 93;
/// synonym to UYVY
pub const COLOR_YUV2BGR_UYNV: i32 = 108;
/// convert between YUV UYVY and BGR, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_UYVY: i32 = 108;
/// synonym to UYVY
pub const COLOR_YUV2BGR_Y422: i32 = 108;
/// synonym to YUY2
pub const COLOR_YUV2BGR_YUNV: i32 = 116;
/// convert between YUV YUY2 and BGR, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_YUY2: i32 = 116;
/// synonym to YUY2
pub const COLOR_YUV2BGR_YUYV: i32 = 116;
/// convert between 4:2:0-subsampled YUV YV12 and BGR, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_YV12: i32 = 99;
/// convert between YUV YVYU and BGR, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2BGR_YVYU: i32 = 118;
/// extract Y channel from YUV 4:2:0 image
pub const COLOR_YUV2GRAY_420: i32 = 106;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV2GRAY_I420: i32 = 106;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV2GRAY_IYUV: i32 = 106;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV2GRAY_NV12: i32 = 106;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV2GRAY_NV21: i32 = 106;
/// synonym to COLOR_YUV2GRAY_UYVY
pub const COLOR_YUV2GRAY_UYNV: i32 = 123;
/// extract Y channel from YUV 4:2:2 image
pub const COLOR_YUV2GRAY_UYVY: i32 = 123;
/// synonym to COLOR_YUV2GRAY_UYVY
pub const COLOR_YUV2GRAY_Y422: i32 = 123;
/// synonym to COLOR_YUV2GRAY_YUY2
pub const COLOR_YUV2GRAY_YUNV: i32 = 124;
/// extract Y channel from YUV 4:2:2 image
pub const COLOR_YUV2GRAY_YUY2: i32 = 124;
/// synonym to COLOR_YUV2GRAY_YUY2
pub const COLOR_YUV2GRAY_YUYV: i32 = 124;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV2GRAY_YV12: i32 = 106;
/// synonym to COLOR_YUV2GRAY_YUY2
pub const COLOR_YUV2GRAY_YVYU: i32 = 124;
pub const COLOR_YUV2RGB: i32 = 85;
/// synonym to IYUV
pub const COLOR_YUV2RGBA_I420: i32 = 104;
/// convert between 4:2:0-subsampled YUV YV12 and RGBA, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_IYUV: i32 = 104;
/// convert between 4:2:0-subsampled YUV NV12 and RGBA, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_NV12: i32 = 94;
/// convert between 4:2:0-subsampled YUV NV21 and RGBA, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_NV21: i32 = 96;
/// synonym to UYVY
pub const COLOR_YUV2RGBA_UYNV: i32 = 111;
/// convert between YUV UYVY and RGBA, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_UYVY: i32 = 111;
/// synonym to UYVY
pub const COLOR_YUV2RGBA_Y422: i32 = 111;
/// synonym to YUY2
pub const COLOR_YUV2RGBA_YUNV: i32 = 119;
/// convert between YUV YUY2 and RGBA, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_YUY2: i32 = 119;
/// synonym to YUY2
pub const COLOR_YUV2RGBA_YUYV: i32 = 119;
/// convert between 4:2:0-subsampled YUV YV12 and RGBA, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_YV12: i32 = 102;
/// convert between YUV YVYU and RGBA, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGBA_YVYU: i32 = 121;
/// synonym to IYUV
pub const COLOR_YUV2RGB_I420: i32 = 100;
/// convert between 4:2:0-subsampled YUV IYUV and RGB, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_IYUV: i32 = 100;
/// convert between 4:2:0-subsampled YUV NV12 and RGB, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_NV12: i32 = 90;
/// convert between 4:2:0-subsampled YUV NV21 and RGB, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_NV21: i32 = 92;
/// synonym to UYVY
pub const COLOR_YUV2RGB_UYNV: i32 = 107;
/// convert between YUV UYVY and RGB, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_UYVY: i32 = 107;
/// synonym to UYVY
pub const COLOR_YUV2RGB_Y422: i32 = 107;
/// synonym to YUY2
pub const COLOR_YUV2RGB_YUNV: i32 = 115;
/// convert between YUV YUY2 and RGB, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_YUY2: i32 = 115;
/// synonym to YUY2
pub const COLOR_YUV2RGB_YUYV: i32 = 115;
/// convert between 4:2:0-subsampled YUV YV12 and RGB, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_YV12: i32 = 98;
/// convert between YUV YVYU and RGB, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
pub const COLOR_YUV2RGB_YVYU: i32 = 117;
/// synonym to YV12
pub const COLOR_YUV420p2BGR: i32 = 99;
/// synonym to YV12
pub const COLOR_YUV420p2BGRA: i32 = 103;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV420p2GRAY: i32 = 106;
/// synonym to YV12
pub const COLOR_YUV420p2RGB: i32 = 98;
/// synonym to YV12
pub const COLOR_YUV420p2RGBA: i32 = 102;
/// synonym to NV21
pub const COLOR_YUV420sp2BGR: i32 = 93;
/// synonym to NV21
pub const COLOR_YUV420sp2BGRA: i32 = 97;
/// synonym to COLOR_YUV2GRAY_420
pub const COLOR_YUV420sp2GRAY: i32 = 106;
/// synonym to NV21
pub const COLOR_YUV420sp2RGB: i32 = 92;
/// synonym to NV21
pub const COLOR_YUV420sp2RGBA: i32 = 96;
/// alpha premultiplication
pub const COLOR_mRGBA2RGBA: i32 = 126;
/// ![block formula](https://latex.codecogs.com/png.latex?I%5F1%28A%2CB%29%20%3D%20%20%5Csum%20%5F%7Bi%3D1%2E%2E%2E7%7D%20%20%5Cleft%20%7C%20%20%5Cfrac%7B1%7D%7Bm%5EA%5Fi%7D%20%2D%20%20%5Cfrac%7B1%7D%7Bm%5EB%5Fi%7D%20%5Cright%20%7C)
pub const CONTOURS_MATCH_I1: i32 = 1;
/// ![block formula](https://latex.codecogs.com/png.latex?I%5F2%28A%2CB%29%20%3D%20%20%5Csum%20%5F%7Bi%3D1%2E%2E%2E7%7D%20%20%5Cleft%20%7C%20m%5EA%5Fi%20%2D%20m%5EB%5Fi%20%20%5Cright%20%7C)
pub const CONTOURS_MATCH_I2: i32 = 2;
/// ![block formula](https://latex.codecogs.com/png.latex?I%5F3%28A%2CB%29%20%3D%20%20%5Cmax%20%5F%7Bi%3D1%2E%2E%2E7%7D%20%20%5Cfrac%7B%20%5Cleft%7C%20m%5EA%5Fi%20%2D%20m%5EB%5Fi%20%5Cright%7C%20%7D%7B%20%5Cleft%7C%20m%5EA%5Fi%20%5Cright%7C%20%7D)
pub const CONTOURS_MATCH_I3: i32 = 3;
/// distance = max(|x1-x2|,|y1-y2|)
pub const DIST_C: i32 = 3;
/// distance = c^2(|x|/c-log(1+|x|/c)), c = 1.3998
pub const DIST_FAIR: i32 = 5;
/// distance = |x|<c ? x^2/2 : c(|x|-c/2), c=1.345
pub const DIST_HUBER: i32 = 7;
/// distance = |x1-x2| + |y1-y2|
pub const DIST_L1: i32 = 1;
/// L1-L2 metric: distance = 2(sqrt(1+x*x/2) - 1))
pub const DIST_L12: i32 = 4;
/// the simple euclidean distance
pub const DIST_L2: i32 = 2;
/// each connected component of zeros in src (as well as all the non-zero pixels closest to the
/// connected component) will be assigned the same label
pub const DIST_LABEL_CCOMP: i32 = 0;
/// each zero pixel (and all the non-zero pixels closest to it) gets its own label.
pub const DIST_LABEL_PIXEL: i32 = 1;
/// mask=3
pub const DIST_MASK_3: i32 = 3;
/// mask=5
pub const DIST_MASK_5: i32 = 5;
pub const DIST_MASK_PRECISE: i32 = 0;
/// User defined distance
pub const DIST_USER: i32 = -1;
/// distance = c^2/2(1-exp(-(x/c)^2)), c = 2.9846
pub const DIST_WELSCH: i32 = 6;
pub const FILLED: i32 = -1;
pub const FILTER_SCHARR: i32 = -1;
/// If set, the difference between the current pixel and seed pixel is considered. Otherwise,
/// the difference between neighbor pixels is considered (that is, the range is floating).
pub const FLOODFILL_FIXED_RANGE: i32 = 65536;
/// If set, the function does not change the image ( newVal is ignored), and only fills the
/// mask with the value specified in bits 8-16 of flags as described above. This option only make
/// sense in function variants that have the mask parameter.
pub const FLOODFILL_MASK_ONLY: i32 = 131072;
/// normal size serif font
pub const FONT_HERSHEY_COMPLEX: i32 = 3;
/// smaller version of FONT_HERSHEY_COMPLEX
pub const FONT_HERSHEY_COMPLEX_SMALL: i32 = 5;
/// normal size sans-serif font (more complex than FONT_HERSHEY_SIMPLEX)
pub const FONT_HERSHEY_DUPLEX: i32 = 2;
/// small size sans-serif font
pub const FONT_HERSHEY_PLAIN: i32 = 1;
/// more complex variant of FONT_HERSHEY_SCRIPT_SIMPLEX
pub const FONT_HERSHEY_SCRIPT_COMPLEX: i32 = 7;
/// hand-writing style font
pub const FONT_HERSHEY_SCRIPT_SIMPLEX: i32 = 6;
/// normal size sans-serif font
pub const FONT_HERSHEY_SIMPLEX: i32 = 0;
/// normal size serif font (more complex than FONT_HERSHEY_COMPLEX)
pub const FONT_HERSHEY_TRIPLEX: i32 = 4;
/// flag for italic font
pub const FONT_ITALIC: i32 = 16;
/// an obvious background pixels
pub const GC_BGD: i32 = 0;
/// The value means that the algorithm should just resume.
pub const GC_EVAL: i32 = 2;
/// The value means that the algorithm should just run the grabCut algorithm (a single iteration) with the fixed model
pub const GC_EVAL_FREEZE_MODEL: i32 = 3;
/// an obvious foreground (object) pixel
pub const GC_FGD: i32 = 1;
/// The function initializes the state using the provided mask. Note that GC_INIT_WITH_RECT
/// and GC_INIT_WITH_MASK can be combined. Then, all the pixels outside of the ROI are
/// automatically initialized with GC_BGD .
pub const GC_INIT_WITH_MASK: i32 = 1;
/// The function initializes the state and the mask using the provided rectangle. After that it
/// runs iterCount iterations of the algorithm.
pub const GC_INIT_WITH_RECT: i32 = 0;
/// a possible background pixel
pub const GC_PR_BGD: i32 = 2;
/// a possible foreground pixel
pub const GC_PR_FGD: i32 = 3;
/// Bhattacharyya distance
/// (In fact, OpenCV computes Hellinger distance, which is related to Bhattacharyya coefficient.)
/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Csqrt%7B1%20%2D%20%5Cfrac%7B1%7D%7B%5Csqrt%7B%5Cbar%7BH%5F1%7D%20%5Cbar%7BH%5F2%7D%20N%5E2%7D%7D%20%5Csum%5FI%20%5Csqrt%7BH%5F1%28I%29%20%5Ccdot%20H%5F2%28I%29%7D%7D)
pub const HISTCMP_BHATTACHARYYA: i32 = 3;
/// Chi-Square
/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Csum%20%5FI%20%20%5Cfrac%7B%5Cleft%28H%5F1%28I%29%2DH%5F2%28I%29%5Cright%29%5E2%7D%7BH%5F1%28I%29%7D)
pub const HISTCMP_CHISQR: i32 = 1;
/// Alternative Chi-Square
/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%202%20%2A%20%5Csum%20%5FI%20%20%5Cfrac%7B%5Cleft%28H%5F1%28I%29%2DH%5F2%28I%29%5Cright%29%5E2%7D%7BH%5F1%28I%29%2BH%5F2%28I%29%7D)
/// This alternative formula is regularly used for texture comparison. See e.g. [Puzicha1997](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Puzicha1997)
pub const HISTCMP_CHISQR_ALT: i32 = 4;
/// Correlation
/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Cfrac%7B%5Csum%5FI%20%28H%5F1%28I%29%20%2D%20%5Cbar%7BH%5F1%7D%29%20%28H%5F2%28I%29%20%2D%20%5Cbar%7BH%5F2%7D%29%7D%7B%5Csqrt%7B%5Csum%5FI%28H%5F1%28I%29%20%2D%20%5Cbar%7BH%5F1%7D%29%5E2%20%5Csum%5FI%28H%5F2%28I%29%20%2D%20%5Cbar%7BH%5F2%7D%29%5E2%7D%7D)
/// where
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbar%7BH%5Fk%7D%20%3D%20%20%5Cfrac%7B1%7D%7BN%7D%20%5Csum%20%5FJ%20H%5Fk%28J%29)
/// and ![inline formula](https://latex.codecogs.com/png.latex?N) is a total number of histogram bins.
pub const HISTCMP_CORREL: i32 = 0;
/// Synonym for HISTCMP_BHATTACHARYYA
pub const HISTCMP_HELLINGER: i32 = 3;
/// Intersection
/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Csum%20%5FI%20%20%5Cmin%20%28H%5F1%28I%29%2C%20H%5F2%28I%29%29)
pub const HISTCMP_INTERSECT: i32 = 2;
/// Kullback-Leibler divergence
/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%5Csum%20%5FI%20H%5F1%28I%29%20%5Clog%20%5Cleft%28%5Cfrac%7BH%5F1%28I%29%7D%7BH%5F2%28I%29%7D%5Cright%29)
pub const HISTCMP_KL_DIV: i32 = 5;
/// basically *21HT*, described in [Yuen90](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Yuen90)
pub const HOUGH_GRADIENT: i32 = 3;
/// variation of HOUGH_GRADIENT to get better accuracy
pub const HOUGH_GRADIENT_ALT: i32 = 4;
/// multi-scale variant of the classical Hough transform. The lines are encoded the same way as
/// HOUGH_STANDARD.
pub const HOUGH_MULTI_SCALE: i32 = 2;
/// probabilistic Hough transform (more efficient in case if the picture contains a few long
/// linear segments). It returns line segments rather than the whole line. Each segment is
/// represented by starting and ending points, and the matrix must be (the created sequence will
/// be) of the CV_32SC4 type.
pub const HOUGH_PROBABILISTIC: i32 = 1;
/// classical or standard Hough transform. Every line is represented by two floating-point
/// numbers ![inline formula](https://latex.codecogs.com/png.latex?%28%5Crho%2C%20%5Ctheta%29) , where ![inline formula](https://latex.codecogs.com/png.latex?%5Crho) is a distance between (0,0) point and the line,
/// and ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta) is the angle between x-axis and the normal to the line. Thus, the matrix must
/// be (the created sequence will be) of CV_32FC2 type
pub const HOUGH_STANDARD: i32 = 0;
/// One of the rectangle is fully enclosed in the other
pub const INTERSECT_FULL: i32 = 2;
/// No intersection
pub const INTERSECT_NONE: i32 = 0;
/// There is a partial intersection
pub const INTERSECT_PARTIAL: i32 = 1;
/// resampling using pixel area relation. It may be a preferred method for image decimation, as
/// it gives moire'-free results. But when the image is zoomed, it is similar to the INTER_NEAREST
/// method.
pub const INTER_AREA: i32 = 3;
pub const INTER_BITS: i32 = 5;
pub const INTER_BITS2: i32 = 10;
/// bicubic interpolation
pub const INTER_CUBIC: i32 = 2;
/// Lanczos interpolation over 8x8 neighborhood
pub const INTER_LANCZOS4: i32 = 4;
/// bilinear interpolation
pub const INTER_LINEAR: i32 = 1;
/// Bit exact bilinear interpolation
pub const INTER_LINEAR_EXACT: i32 = 5;
/// mask for interpolation codes
pub const INTER_MAX: i32 = 7;
/// nearest neighbor interpolation
pub const INTER_NEAREST: i32 = 0;
/// Bit exact nearest neighbor interpolation. This will produce same results as
/// the nearest neighbor method in PIL, scikit-image or Matlab.
pub const INTER_NEAREST_EXACT: i32 = 6;
pub const INTER_TAB_SIZE: i32 = 32;
pub const INTER_TAB_SIZE2: i32 = 1024;
/// 4-connected line
pub const LINE_4: i32 = 4;
/// 8-connected line
pub const LINE_8: i32 = 8;
/// antialiased line
pub const LINE_AA: i32 = 16;
/// Advanced refinement. Number of false alarms is calculated, lines are
/// refined through increase of precision, decrement in size, etc.
pub const LSD_REFINE_ADV: i32 = 2;
/// No refinement applied
pub const LSD_REFINE_NONE: i32 = 0;
/// Standard refinement is applied. E.g. breaking arches into smaller straighter line approximations.
pub const LSD_REFINE_STD: i32 = 1;
/// A crosshair marker shape
pub const MARKER_CROSS: i32 = 0;
/// A diamond marker shape
pub const MARKER_DIAMOND: i32 = 3;
/// A square marker shape
pub const MARKER_SQUARE: i32 = 4;
/// A star marker shape, combination of cross and tilted cross
pub const MARKER_STAR: i32 = 2;
/// A 45 degree tilted crosshair marker shape
pub const MARKER_TILTED_CROSS: i32 = 1;
/// A downwards pointing triangle marker shape
pub const MARKER_TRIANGLE_DOWN: i32 = 6;
/// An upwards pointing triangle marker shape
pub const MARKER_TRIANGLE_UP: i32 = 5;
/// "black hat"
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bblackhat%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Bclose%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%2D%20%5Ctexttt%7Bsrc%7D)
pub const MORPH_BLACKHAT: i32 = 6;
/// a closing operation
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bclose%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Berode%7D%20%28%20%5Cmathrm%7Bdilate%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%29)
pub const MORPH_CLOSE: i32 = 3;
/// a cross-shaped structuring element:
/// ![block formula](https://latex.codecogs.com/png.latex?E%5F%7Bij%7D%20%3D%20%5Cbegin%7Bcases%7D%201%20%26%20%5Ctexttt%7Bif%20%7D%20%7Bi%3D%5Ctexttt%7Banchor%2Ey%20%7D%20%7Bor%20%7D%20%7Bj%3D%5Ctexttt%7Banchor%2Ex%7D%7D%7D%20%5C%5C0%20%26%20%5Ctexttt%7Botherwise%7D%20%5Cend%7Bcases%7D)
pub const MORPH_CROSS: i32 = 1;
/// a diamond structuring element defined by Manhattan distance
pub const MORPH_DIAMOND: i32 = 3;
/// see #dilate
pub const MORPH_DILATE: i32 = 1;
/// an elliptic structuring element, that is, a filled ellipse inscribed
/// into the rectangle Rect(0, 0, esize.width, esize.height)
pub const MORPH_ELLIPSE: i32 = 2;
/// see #erode
pub const MORPH_ERODE: i32 = 0;
/// a morphological gradient
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bmorph%5C%5Fgrad%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Bdilate%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%2D%20%5Cmathrm%7Berode%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29)
pub const MORPH_GRADIENT: i32 = 4;
/// "hit or miss"
/// .- Only supported for CV_8UC1 binary images. A tutorial can be found in the documentation
pub const MORPH_HITMISS: i32 = 7;
/// an opening operation
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bopen%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Bdilate%7D%20%28%20%5Cmathrm%7Berode%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%29)
pub const MORPH_OPEN: i32 = 2;
/// a rectangular structuring element:  ![block formula](https://latex.codecogs.com/png.latex?E%5F%7Bij%7D%3D1)
pub const MORPH_RECT: i32 = 0;
/// "top hat"
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Btophat%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Cmathrm%7Bopen%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29)
pub const MORPH_TOPHAT: i32 = 5;
/// retrieves all of the contours and organizes them into a two-level hierarchy. At the top
/// level, there are external boundaries of the components. At the second level, there are
/// boundaries of the holes. If there is another contour inside a hole of a connected component, it
/// is still put at the top level.
pub const RETR_CCOMP: i32 = 2;
/// retrieves only the extreme outer contours. It sets `hierarchy[i][2]=hierarchy[i][3]=-1` for
/// all the contours.
pub const RETR_EXTERNAL: i32 = 0;
pub const RETR_FLOODFILL: i32 = 4;
/// retrieves all of the contours without establishing any hierarchical relationships.
pub const RETR_LIST: i32 = 1;
/// retrieves all of the contours and reconstructs a full hierarchy of nested contours.
pub const RETR_TREE: i32 = 3;
pub const Subdiv2D_NEXT_AROUND_DST: i32 = 34;
pub const Subdiv2D_NEXT_AROUND_LEFT: i32 = 19;
pub const Subdiv2D_NEXT_AROUND_ORG: i32 = 0;
pub const Subdiv2D_NEXT_AROUND_RIGHT: i32 = 49;
pub const Subdiv2D_PREV_AROUND_DST: i32 = 51;
pub const Subdiv2D_PREV_AROUND_LEFT: i32 = 32;
pub const Subdiv2D_PREV_AROUND_ORG: i32 = 17;
pub const Subdiv2D_PREV_AROUND_RIGHT: i32 = 2;
/// Point location error
pub const Subdiv2D_PTLOC_ERROR: i32 = -2;
/// Point inside some facet
pub const Subdiv2D_PTLOC_INSIDE: i32 = 0;
/// Point on some edge
pub const Subdiv2D_PTLOC_ON_EDGE: i32 = 2;
/// Point outside the subdivision bounding rect
pub const Subdiv2D_PTLOC_OUTSIDE_RECT: i32 = -1;
/// Point coincides with one of the subdivision vertices
pub const Subdiv2D_PTLOC_VERTEX: i32 = 1;
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bmaxval%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B0%7D%7Botherwise%7D)
pub const THRESH_BINARY: i32 = 0;
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B%5Ctexttt%7Bmaxval%7D%7D%7Botherwise%7D)
pub const THRESH_BINARY_INV: i32 = 1;
/// flag, compute threshold only (useful for OTSU/TRIANGLE) but does not actually run thresholding
pub const THRESH_DRYRUN: i32 = 128;
pub const THRESH_MASK: i32 = 7;
/// flag, use Otsu algorithm to choose the optimal threshold value
pub const THRESH_OTSU: i32 = 8;
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bsrc%7D%28x%2Cy%29%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B0%7D%7Botherwise%7D)
pub const THRESH_TOZERO: i32 = 3;
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B%5Ctexttt%7Bsrc%7D%28x%2Cy%29%7D%7Botherwise%7D)
pub const THRESH_TOZERO_INV: i32 = 4;
/// flag, use Triangle algorithm to choose the optimal threshold value
pub const THRESH_TRIANGLE: i32 = 16;
/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bthreshold%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B%5Ctexttt%7Bsrc%7D%28x%2Cy%29%7D%7Botherwise%7D)
pub const THRESH_TRUNC: i32 = 2;
/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%27%28x%27%2Cy%27%29%20%5Ccdot%20I%27%28x%2Bx%27%2Cy%2By%27%29%29)
/// where
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20T%27%28x%27%2Cy%27%29%3DT%28x%27%2Cy%27%29%20%2D%201%2F%28w%20%5Ccdot%20h%29%20%5Ccdot%20%5Csum%20%5F%7B%0A%20%20%20x%27%27%2Cy%27%27%7D%20T%28x%27%27%2Cy%27%27%29%20%5C%5C%20I%27%28x%2Bx%27%2Cy%2By%27%29%3DI%28x%2Bx%27%2Cy%2By%27%29%20%2D%201%2F%28w%20%5Ccdot%20h%29%0A%20%20%20%5Ccdot%20%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20I%28x%2Bx%27%27%2Cy%2By%27%27%29%20%5Cend%7Barray%7D)
/// with mask:
/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20T%27%28x%27%2Cy%27%29%3DM%28x%27%2Cy%27%29%20%5Ccdot%20%5Cleft%28%20T%28x%27%2Cy%27%29%20%2D%0A%20%20%20%5Cfrac%7B1%7D%7B%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20M%28x%27%27%2Cy%27%27%29%7D%20%5Ccdot%20%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%0A%20%20%20%28T%28x%27%27%2Cy%27%27%29%20%5Ccdot%20M%28x%27%27%2Cy%27%27%29%29%20%5Cright%29%20%5C%5C%20I%27%28x%2Bx%27%2Cy%2By%27%29%3DM%28x%27%2Cy%27%29%0A%20%20%20%5Ccdot%20%5Cleft%28%20I%28x%2Bx%27%2Cy%2By%27%29%20%2D%20%5Cfrac%7B1%7D%7B%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20M%28x%27%27%2Cy%27%27%29%7D%0A%20%20%20%5Ccdot%20%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20%28I%28x%2Bx%27%27%2Cy%2By%27%27%29%20%5Ccdot%20M%28x%27%27%2Cy%27%27%29%29%20%5Cright%29%0A%20%20%20%5Cend%7Barray%7D%20)
pub const TM_CCOEFF: i32 = 4;
/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%20%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%27%28x%27%2Cy%27%29%20%5Ccdot%20I%27%28x%2Bx%27%2Cy%2By%27%29%29%20%7D%7B%0A%5Csqrt%7B%5Csum%5F%7Bx%27%2Cy%27%7DT%27%28x%27%2Cy%27%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20I%27%28x%2Bx%27%2Cy%2By%27%29%5E2%7D%0A%7D)
pub const TM_CCOEFF_NORMED: i32 = 5;
/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%29)
/// with mask:
/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%20M%28x%27%2Cy%27%29%0A%20%20%20%5E2%29)
pub const TM_CCORR: i32 = 2;
/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%29%7D%7B%5Csqrt%7B%0A%20%20%20%5Csum%5F%7Bx%27%2Cy%27%7DT%28x%27%2Cy%27%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20I%28x%2Bx%27%2Cy%2By%27%29%5E2%7D%7D)
/// with mask:
/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%5E2%29%7D%7B%5Csqrt%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20T%28x%27%2Cy%27%29%20%5Ccdot%20M%28x%27%2Cy%27%29%0A%20%20%20%5Cright%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%20M%28x%27%2Cy%27%29%0A%20%20%20%5Cright%29%5E2%7D%7D)
pub const TM_CCORR_NORMED: i32 = 3;
/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%5E2)
/// with mask:
/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2)
pub const TM_SQDIFF: i32 = 0;
/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%5E2%7D%7B%5Csqrt%7B%5Csum%5F%7B%0A%20%20%20x%27%2Cy%27%7DT%28x%27%2Cy%27%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20I%28x%2Bx%27%2Cy%2By%27%29%5E2%7D%7D)
/// with mask:
/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2%7D%7B%5Csqrt%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20T%28x%27%2Cy%27%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2%7D%7D)
pub const TM_SQDIFF_NORMED: i32 = 1;
/// flag, fills all of the destination image pixels. If some of them correspond to outliers in the
/// source image, they are set to zero
pub const WARP_FILL_OUTLIERS: i32 = 8;
/// flag, inverse transformation
/// 
/// For example, [linear_polar] or [log_polar] transforms:
/// - flag is __not__ set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28%20%5Crho%20%2C%20%5Cphi%20%29%20%3D%20src%28x%2Cy%29)
/// - flag is set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28x%2Cy%29%20%3D%20src%28%20%5Crho%20%2C%20%5Cphi%20%29)
pub const WARP_INVERSE_MAP: i32 = 16;
/// Remaps an image to/from polar space.
pub const WARP_POLAR_LINEAR: i32 = 0;
/// Remaps an image to/from semilog-polar space.
pub const WARP_POLAR_LOG: i32 = 256;
/// flag, inverse transformation
/// 
/// For example, [linear_polar] or [log_polar] transforms:
/// - flag is __not__ set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28%20%5Crho%20%2C%20%5Cphi%20%29%20%3D%20src%28x%2Cy%29)
/// - flag is set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28x%2Cy%29%20%3D%20src%28%20%5Crho%20%2C%20%5Cphi%20%29)
pub const WARP_RELATIVE_MAP: i32 = 32;
/// adaptive threshold algorithm
/// ## See also
/// adaptiveThreshold
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AdaptiveThresholdTypes {
	/// the threshold value ![inline formula](https://latex.codecogs.com/png.latex?T%28x%2Cy%29) is a mean of the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BblockSize%7D%20%5Ctimes%0A%5Ctexttt%7BblockSize%7D) neighborhood of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29) minus C
	ADAPTIVE_THRESH_MEAN_C = 0,
	/// the threshold value ![inline formula](https://latex.codecogs.com/png.latex?T%28x%2C%20y%29) is a weighted sum (cross-correlation with a Gaussian
	/// window) of the ![inline formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7BblockSize%7D%20%5Ctimes%20%5Ctexttt%7BblockSize%7D) neighborhood of ![inline formula](https://latex.codecogs.com/png.latex?%28x%2C%20y%29)
	/// minus C . The default sigma (standard deviation) is used for the specified blockSize . See
	/// #getGaussianKernel
	ADAPTIVE_THRESH_GAUSSIAN_C = 1,
}

opencv_type_enum! { crate::imgproc::AdaptiveThresholdTypes }

/// the color conversion codes
/// ## See also
/// [imgproc_color_conversions]
/// @ingroup imgproc_color_conversions
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorConversionCodes {
	/// add alpha channel to RGB or BGR image
	COLOR_BGR2BGRA = 0,
	// Duplicate, use COLOR_BGR2BGRA instead
	// COLOR_RGB2RGBA = 0,
	/// remove alpha channel from RGB or BGR image
	COLOR_BGRA2BGR = 1,
	// Duplicate, use COLOR_BGRA2BGR instead
	// COLOR_RGBA2RGB = 1,
	/// convert between RGB and BGR color spaces (with or without alpha channel)
	COLOR_BGR2RGBA = 2,
	// Duplicate, use COLOR_BGR2RGBA instead
	// COLOR_RGB2BGRA = 2,
	COLOR_RGBA2BGR = 3,
	// Duplicate, use COLOR_RGBA2BGR instead
	// COLOR_BGRA2RGB = 3,
	COLOR_BGR2RGB = 4,
	// Duplicate, use COLOR_BGR2RGB instead
	// COLOR_RGB2BGR = 4,
	COLOR_BGRA2RGBA = 5,
	// Duplicate, use COLOR_BGRA2RGBA instead
	// COLOR_RGBA2BGRA = 5,
	/// convert between RGB/BGR and grayscale, [color_convert_rgb_gray] "color conversions"
	COLOR_BGR2GRAY = 6,
	COLOR_RGB2GRAY = 7,
	COLOR_GRAY2BGR = 8,
	// Duplicate, use COLOR_GRAY2BGR instead
	// COLOR_GRAY2RGB = 8,
	COLOR_GRAY2BGRA = 9,
	// Duplicate, use COLOR_GRAY2BGRA instead
	// COLOR_GRAY2RGBA = 9,
	COLOR_BGRA2GRAY = 10,
	COLOR_RGBA2GRAY = 11,
	/// convert between RGB/BGR and BGR565 (16-bit images)
	COLOR_BGR2BGR565 = 12,
	COLOR_RGB2BGR565 = 13,
	COLOR_BGR5652BGR = 14,
	COLOR_BGR5652RGB = 15,
	COLOR_BGRA2BGR565 = 16,
	COLOR_RGBA2BGR565 = 17,
	COLOR_BGR5652BGRA = 18,
	COLOR_BGR5652RGBA = 19,
	/// convert between grayscale to BGR565 (16-bit images)
	COLOR_GRAY2BGR565 = 20,
	COLOR_BGR5652GRAY = 21,
	/// convert between RGB/BGR and BGR555 (16-bit images)
	COLOR_BGR2BGR555 = 22,
	COLOR_RGB2BGR555 = 23,
	COLOR_BGR5552BGR = 24,
	COLOR_BGR5552RGB = 25,
	COLOR_BGRA2BGR555 = 26,
	COLOR_RGBA2BGR555 = 27,
	COLOR_BGR5552BGRA = 28,
	COLOR_BGR5552RGBA = 29,
	/// convert between grayscale and BGR555 (16-bit images)
	COLOR_GRAY2BGR555 = 30,
	COLOR_BGR5552GRAY = 31,
	/// convert RGB/BGR to CIE XYZ, [color_convert_rgb_xyz] "color conversions"
	COLOR_BGR2XYZ = 32,
	COLOR_RGB2XYZ = 33,
	COLOR_XYZ2BGR = 34,
	COLOR_XYZ2RGB = 35,
	/// convert RGB/BGR to luma-chroma (aka YCC), [color_convert_rgb_ycrcb] "color conversions"
	COLOR_BGR2YCrCb = 36,
	COLOR_RGB2YCrCb = 37,
	COLOR_YCrCb2BGR = 38,
	COLOR_YCrCb2RGB = 39,
	/// convert RGB/BGR to HSV (hue saturation value) with H range 0..180 if 8 bit image, [color_convert_rgb_hsv] "color conversions"
	COLOR_BGR2HSV = 40,
	COLOR_RGB2HSV = 41,
	/// convert RGB/BGR to CIE Lab, [color_convert_rgb_lab] "color conversions"
	COLOR_BGR2Lab = 44,
	COLOR_RGB2Lab = 45,
	/// convert RGB/BGR to CIE Luv, [color_convert_rgb_luv] "color conversions"
	COLOR_BGR2Luv = 50,
	COLOR_RGB2Luv = 51,
	/// convert RGB/BGR to HLS (hue lightness saturation) with H range 0..180 if 8 bit image, [color_convert_rgb_hls] "color conversions"
	COLOR_BGR2HLS = 52,
	COLOR_RGB2HLS = 53,
	/// backward conversions HSV to RGB/BGR with H range 0..180 if 8 bit image
	COLOR_HSV2BGR = 54,
	COLOR_HSV2RGB = 55,
	COLOR_Lab2BGR = 56,
	COLOR_Lab2RGB = 57,
	COLOR_Luv2BGR = 58,
	COLOR_Luv2RGB = 59,
	/// backward conversions HLS to RGB/BGR with H range 0..180 if 8 bit image
	COLOR_HLS2BGR = 60,
	COLOR_HLS2RGB = 61,
	/// convert RGB/BGR to HSV (hue saturation value) with H range 0..255 if 8 bit image, [color_convert_rgb_hsv] "color conversions"
	COLOR_BGR2HSV_FULL = 66,
	COLOR_RGB2HSV_FULL = 67,
	/// convert RGB/BGR to HLS (hue lightness saturation) with H range 0..255 if 8 bit image, [color_convert_rgb_hls] "color conversions"
	COLOR_BGR2HLS_FULL = 68,
	COLOR_RGB2HLS_FULL = 69,
	/// backward conversions HSV to RGB/BGR with H range 0..255 if 8 bit image
	COLOR_HSV2BGR_FULL = 70,
	COLOR_HSV2RGB_FULL = 71,
	/// backward conversions HLS to RGB/BGR with H range 0..255 if 8 bit image
	COLOR_HLS2BGR_FULL = 72,
	COLOR_HLS2RGB_FULL = 73,
	COLOR_LBGR2Lab = 74,
	COLOR_LRGB2Lab = 75,
	COLOR_LBGR2Luv = 76,
	COLOR_LRGB2Luv = 77,
	COLOR_Lab2LBGR = 78,
	COLOR_Lab2LRGB = 79,
	COLOR_Luv2LBGR = 80,
	COLOR_Luv2LRGB = 81,
	/// convert between RGB/BGR and YUV
	COLOR_BGR2YUV = 82,
	COLOR_RGB2YUV = 83,
	COLOR_YUV2BGR = 84,
	COLOR_YUV2RGB = 85,
	/// convert between 4:2:0-subsampled YUV NV12 and RGB, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_NV12 = 90,
	/// convert between 4:2:0-subsampled YUV NV12 and BGR, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_NV12 = 91,
	/// convert between 4:2:0-subsampled YUV NV21 and RGB, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_NV21 = 92,
	/// convert between 4:2:0-subsampled YUV NV21 and BGR, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_NV21 = 93,
	// synonym to NV21
	// Duplicate, use COLOR_YUV2RGB_NV21 instead
	// COLOR_YUV420sp2RGB = 92,
	// synonym to NV21
	// Duplicate, use COLOR_YUV2BGR_NV21 instead
	// COLOR_YUV420sp2BGR = 93,
	/// convert between 4:2:0-subsampled YUV NV12 and RGBA, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_NV12 = 94,
	/// convert between 4:2:0-subsampled YUV NV12 and BGRA, two planes (in one or separate arrays): Y and U/V interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_NV12 = 95,
	/// convert between 4:2:0-subsampled YUV NV21 and RGBA, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_NV21 = 96,
	/// convert between 4:2:0-subsampled YUV NV21 and BGRA, two planes (in one or separate arrays): Y and V/U interleaved, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_NV21 = 97,
	// synonym to NV21
	// Duplicate, use COLOR_YUV2RGBA_NV21 instead
	// COLOR_YUV420sp2RGBA = 96,
	// synonym to NV21
	// Duplicate, use COLOR_YUV2BGRA_NV21 instead
	// COLOR_YUV420sp2BGRA = 97,
	/// convert between 4:2:0-subsampled YUV YV12 and RGB, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_YV12 = 98,
	/// convert between 4:2:0-subsampled YUV YV12 and BGR, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_YV12 = 99,
	/// convert between 4:2:0-subsampled YUV IYUV and RGB, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_IYUV = 100,
	/// convert between 4:2:0-subsampled YUV IYUV and BGR, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_IYUV = 101,
	// synonym to IYUV
	// Duplicate, use COLOR_YUV2RGB_IYUV instead
	// COLOR_YUV2RGB_I420 = 100,
	// synonym to IYUV
	// Duplicate, use COLOR_YUV2BGR_IYUV instead
	// COLOR_YUV2BGR_I420 = 101,
	// synonym to YV12
	// Duplicate, use COLOR_YUV2RGB_YV12 instead
	// COLOR_YUV420p2RGB = 98,
	// synonym to YV12
	// Duplicate, use COLOR_YUV2BGR_YV12 instead
	// COLOR_YUV420p2BGR = 99,
	/// convert between 4:2:0-subsampled YUV YV12 and RGBA, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_YV12 = 102,
	/// convert between 4:2:0-subsampled YUV YV12 and BGRA, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_YV12 = 103,
	/// convert between 4:2:0-subsampled YUV YV12 and RGBA, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_IYUV = 104,
	/// convert between 4:2:0-subsampled YUV YV12 and BGRA, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_IYUV = 105,
	// synonym to IYUV
	// Duplicate, use COLOR_YUV2RGBA_IYUV instead
	// COLOR_YUV2RGBA_I420 = 104,
	// synonym to IYUV
	// Duplicate, use COLOR_YUV2BGRA_IYUV instead
	// COLOR_YUV2BGRA_I420 = 105,
	// synonym to YV12
	// Duplicate, use COLOR_YUV2RGBA_YV12 instead
	// COLOR_YUV420p2RGBA = 102,
	// synonym to YV12
	// Duplicate, use COLOR_YUV2BGRA_YV12 instead
	// COLOR_YUV420p2BGRA = 103,
	/// extract Y channel from YUV 4:2:0 image
	COLOR_YUV2GRAY_420 = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV2GRAY_420 instead
	// COLOR_YUV2GRAY_NV21 = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV2GRAY_NV21 instead
	// COLOR_YUV2GRAY_NV12 = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV2GRAY_NV12 instead
	// COLOR_YUV2GRAY_YV12 = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV2GRAY_YV12 instead
	// COLOR_YUV2GRAY_IYUV = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV2GRAY_IYUV instead
	// COLOR_YUV2GRAY_I420 = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV2GRAY_I420 instead
	// COLOR_YUV420sp2GRAY = 106,
	// synonym to COLOR_YUV2GRAY_420
	// Duplicate, use COLOR_YUV420sp2GRAY instead
	// COLOR_YUV420p2GRAY = 106,
	/// convert between YUV UYVY and RGB, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_UYVY = 107,
	/// convert between YUV UYVY and BGR, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_UYVY = 108,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2RGB_UYVY instead
	// COLOR_YUV2RGB_Y422 = 107,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2BGR_UYVY instead
	// COLOR_YUV2BGR_Y422 = 108,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2RGB_Y422 instead
	// COLOR_YUV2RGB_UYNV = 107,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2BGR_Y422 instead
	// COLOR_YUV2BGR_UYNV = 108,
	/// convert between YUV UYVY and RGBA, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_UYVY = 111,
	/// convert between YUV UYVY and BGRA, YUV is 4:2:2-subsampled and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_UYVY = 112,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2RGBA_UYVY instead
	// COLOR_YUV2RGBA_Y422 = 111,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2BGRA_UYVY instead
	// COLOR_YUV2BGRA_Y422 = 112,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2RGBA_Y422 instead
	// COLOR_YUV2RGBA_UYNV = 111,
	// synonym to UYVY
	// Duplicate, use COLOR_YUV2BGRA_Y422 instead
	// COLOR_YUV2BGRA_UYNV = 112,
	/// convert between YUV YUY2 and RGB, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_YUY2 = 115,
	/// convert between YUV YUY2 and BGR, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_YUY2 = 116,
	/// convert between YUV YVYU and RGB, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGB_YVYU = 117,
	/// convert between YUV YVYU and BGR, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGR_YVYU = 118,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2RGB_YUY2 instead
	// COLOR_YUV2RGB_YUYV = 115,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2BGR_YUY2 instead
	// COLOR_YUV2BGR_YUYV = 116,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2RGB_YUYV instead
	// COLOR_YUV2RGB_YUNV = 115,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2BGR_YUYV instead
	// COLOR_YUV2BGR_YUNV = 116,
	/// convert between YUV YUY2 and RGBA, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_YUY2 = 119,
	/// convert between YUV YUY2 and BGRA, YUV is 4:2:2-subsampled and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_YUY2 = 120,
	/// convert between YUV YVYU and RGBA, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2RGBA_YVYU = 121,
	/// convert between YUV YVYU and BGRA, YUV is 4:2:2-subsampled and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_YUV2BGRA_YVYU = 122,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2RGBA_YUY2 instead
	// COLOR_YUV2RGBA_YUYV = 119,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2BGRA_YUY2 instead
	// COLOR_YUV2BGRA_YUYV = 120,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2RGBA_YUYV instead
	// COLOR_YUV2RGBA_YUNV = 119,
	// synonym to YUY2
	// Duplicate, use COLOR_YUV2BGRA_YUYV instead
	// COLOR_YUV2BGRA_YUNV = 120,
	/// extract Y channel from YUV 4:2:2 image
	COLOR_YUV2GRAY_UYVY = 123,
	/// extract Y channel from YUV 4:2:2 image
	COLOR_YUV2GRAY_YUY2 = 124,
	// synonym to COLOR_YUV2GRAY_UYVY
	// Duplicate, use COLOR_YUV2GRAY_UYVY instead
	// COLOR_YUV2GRAY_Y422 = 123,
	// synonym to COLOR_YUV2GRAY_UYVY
	// Duplicate, use COLOR_YUV2GRAY_Y422 instead
	// COLOR_YUV2GRAY_UYNV = 123,
	// synonym to COLOR_YUV2GRAY_YUY2
	// Duplicate, use COLOR_YUV2GRAY_YUY2 instead
	// COLOR_YUV2GRAY_YVYU = 124,
	// synonym to COLOR_YUV2GRAY_YUY2
	// Duplicate, use COLOR_YUV2GRAY_YVYU instead
	// COLOR_YUV2GRAY_YUYV = 124,
	// synonym to COLOR_YUV2GRAY_YUY2
	// Duplicate, use COLOR_YUV2GRAY_YUYV instead
	// COLOR_YUV2GRAY_YUNV = 124,
	/// alpha premultiplication
	COLOR_RGBA2mRGBA = 125,
	/// alpha premultiplication
	COLOR_mRGBA2RGBA = 126,
	/// convert between RGB and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_RGB2YUV_I420 = 127,
	/// convert between BGR and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_BGR2YUV_I420 = 128,
	// synonym to I420
	// Duplicate, use COLOR_RGB2YUV_I420 instead
	// COLOR_RGB2YUV_IYUV = 127,
	// synonym to I420
	// Duplicate, use COLOR_BGR2YUV_I420 instead
	// COLOR_BGR2YUV_IYUV = 128,
	/// convert between RGBA and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_RGBA2YUV_I420 = 129,
	/// convert between BGRA and 4:2:0-subsampled YUV I420, three planes in one array: Y, U and V, see [color_convert_rgb_yuv_42x]
	COLOR_BGRA2YUV_I420 = 130,
	// synonym to I420
	// Duplicate, use COLOR_RGBA2YUV_I420 instead
	// COLOR_RGBA2YUV_IYUV = 129,
	// synonym to I420
	// Duplicate, use COLOR_BGRA2YUV_I420 instead
	// COLOR_BGRA2YUV_IYUV = 130,
	/// convert between RGB and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_RGB2YUV_YV12 = 131,
	/// convert between BGR and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_BGR2YUV_YV12 = 132,
	/// convert between RGBA and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_RGBA2YUV_YV12 = 133,
	/// convert between BGRA and 4:2:0-subsampled YUV YV12, three planes in one array: Y, V and U, see [color_convert_rgb_yuv_42x]
	COLOR_BGRA2YUV_YV12 = 134,
	/// equivalent to RGGB Bayer pattern
	COLOR_BayerBG2BGR = 46,
	/// equivalent to GRBG Bayer pattern
	COLOR_BayerGB2BGR = 47,
	/// equivalent to BGGR Bayer pattern
	COLOR_BayerRG2BGR = 48,
	/// equivalent to GBRG Bayer pattern
	COLOR_BayerGR2BGR = 49,
	// Duplicate, use COLOR_BayerBG2BGR instead
	// COLOR_BayerRGGB2BGR = 46,
	// Duplicate, use COLOR_BayerGB2BGR instead
	// COLOR_BayerGRBG2BGR = 47,
	// Duplicate, use COLOR_BayerRG2BGR instead
	// COLOR_BayerBGGR2BGR = 48,
	// Duplicate, use COLOR_BayerGR2BGR instead
	// COLOR_BayerGBRG2BGR = 49,
	// Duplicate, use COLOR_BayerBGGR2BGR instead
	// COLOR_BayerRGGB2RGB = 48,
	// Duplicate, use COLOR_BayerGBRG2BGR instead
	// COLOR_BayerGRBG2RGB = 49,
	// Duplicate, use COLOR_BayerRGGB2BGR instead
	// COLOR_BayerBGGR2RGB = 46,
	// Duplicate, use COLOR_BayerGRBG2BGR instead
	// COLOR_BayerGBRG2RGB = 47,
	// equivalent to RGGB Bayer pattern
	// Duplicate, use COLOR_BayerRGGB2RGB instead
	// COLOR_BayerBG2RGB = 48,
	// equivalent to GRBG Bayer pattern
	// Duplicate, use COLOR_BayerGRBG2RGB instead
	// COLOR_BayerGB2RGB = 49,
	// equivalent to BGGR Bayer pattern
	// Duplicate, use COLOR_BayerBGGR2RGB instead
	// COLOR_BayerRG2RGB = 46,
	// equivalent to GBRG Bayer pattern
	// Duplicate, use COLOR_BayerGBRG2RGB instead
	// COLOR_BayerGR2RGB = 47,
	/// equivalent to RGGB Bayer pattern
	COLOR_BayerBG2GRAY = 86,
	/// equivalent to GRBG Bayer pattern
	COLOR_BayerGB2GRAY = 87,
	/// equivalent to BGGR Bayer pattern
	COLOR_BayerRG2GRAY = 88,
	/// equivalent to GBRG Bayer pattern
	COLOR_BayerGR2GRAY = 89,
	// Duplicate, use COLOR_BayerBG2GRAY instead
	// COLOR_BayerRGGB2GRAY = 86,
	// Duplicate, use COLOR_BayerGB2GRAY instead
	// COLOR_BayerGRBG2GRAY = 87,
	// Duplicate, use COLOR_BayerRG2GRAY instead
	// COLOR_BayerBGGR2GRAY = 88,
	// Duplicate, use COLOR_BayerGR2GRAY instead
	// COLOR_BayerGBRG2GRAY = 89,
	/// equivalent to RGGB Bayer pattern
	COLOR_BayerBG2BGR_VNG = 62,
	/// equivalent to GRBG Bayer pattern
	COLOR_BayerGB2BGR_VNG = 63,
	/// equivalent to BGGR Bayer pattern
	COLOR_BayerRG2BGR_VNG = 64,
	/// equivalent to GBRG Bayer pattern
	COLOR_BayerGR2BGR_VNG = 65,
	// Duplicate, use COLOR_BayerBG2BGR_VNG instead
	// COLOR_BayerRGGB2BGR_VNG = 62,
	// Duplicate, use COLOR_BayerGB2BGR_VNG instead
	// COLOR_BayerGRBG2BGR_VNG = 63,
	// Duplicate, use COLOR_BayerRG2BGR_VNG instead
	// COLOR_BayerBGGR2BGR_VNG = 64,
	// Duplicate, use COLOR_BayerGR2BGR_VNG instead
	// COLOR_BayerGBRG2BGR_VNG = 65,
	// Duplicate, use COLOR_BayerBGGR2BGR_VNG instead
	// COLOR_BayerRGGB2RGB_VNG = 64,
	// Duplicate, use COLOR_BayerGBRG2BGR_VNG instead
	// COLOR_BayerGRBG2RGB_VNG = 65,
	// Duplicate, use COLOR_BayerRGGB2BGR_VNG instead
	// COLOR_BayerBGGR2RGB_VNG = 62,
	// Duplicate, use COLOR_BayerGRBG2BGR_VNG instead
	// COLOR_BayerGBRG2RGB_VNG = 63,
	// equivalent to RGGB Bayer pattern
	// Duplicate, use COLOR_BayerRGGB2RGB_VNG instead
	// COLOR_BayerBG2RGB_VNG = 64,
	// equivalent to GRBG Bayer pattern
	// Duplicate, use COLOR_BayerGRBG2RGB_VNG instead
	// COLOR_BayerGB2RGB_VNG = 65,
	// equivalent to BGGR Bayer pattern
	// Duplicate, use COLOR_BayerBGGR2RGB_VNG instead
	// COLOR_BayerRG2RGB_VNG = 62,
	// equivalent to GBRG Bayer pattern
	// Duplicate, use COLOR_BayerGBRG2RGB_VNG instead
	// COLOR_BayerGR2RGB_VNG = 63,
	/// equivalent to RGGB Bayer pattern
	COLOR_BayerBG2BGR_EA = 135,
	/// equivalent to GRBG Bayer pattern
	COLOR_BayerGB2BGR_EA = 136,
	/// equivalent to BGGR Bayer pattern
	COLOR_BayerRG2BGR_EA = 137,
	/// equivalent to GBRG Bayer pattern
	COLOR_BayerGR2BGR_EA = 138,
	// Duplicate, use COLOR_BayerBG2BGR_EA instead
	// COLOR_BayerRGGB2BGR_EA = 135,
	// Duplicate, use COLOR_BayerGB2BGR_EA instead
	// COLOR_BayerGRBG2BGR_EA = 136,
	// Duplicate, use COLOR_BayerRG2BGR_EA instead
	// COLOR_BayerBGGR2BGR_EA = 137,
	// Duplicate, use COLOR_BayerGR2BGR_EA instead
	// COLOR_BayerGBRG2BGR_EA = 138,
	// Duplicate, use COLOR_BayerBGGR2BGR_EA instead
	// COLOR_BayerRGGB2RGB_EA = 137,
	// Duplicate, use COLOR_BayerGBRG2BGR_EA instead
	// COLOR_BayerGRBG2RGB_EA = 138,
	// Duplicate, use COLOR_BayerRGGB2BGR_EA instead
	// COLOR_BayerBGGR2RGB_EA = 135,
	// Duplicate, use COLOR_BayerGRBG2BGR_EA instead
	// COLOR_BayerGBRG2RGB_EA = 136,
	// equivalent to RGGB Bayer pattern
	// Duplicate, use COLOR_BayerRGGB2RGB_EA instead
	// COLOR_BayerBG2RGB_EA = 137,
	// equivalent to GRBG Bayer pattern
	// Duplicate, use COLOR_BayerGRBG2RGB_EA instead
	// COLOR_BayerGB2RGB_EA = 138,
	// equivalent to BGGR Bayer pattern
	// Duplicate, use COLOR_BayerBGGR2RGB_EA instead
	// COLOR_BayerRG2RGB_EA = 135,
	// equivalent to GBRG Bayer pattern
	// Duplicate, use COLOR_BayerGBRG2RGB_EA instead
	// COLOR_BayerGR2RGB_EA = 136,
	/// equivalent to RGGB Bayer pattern
	COLOR_BayerBG2BGRA = 139,
	/// equivalent to GRBG Bayer pattern
	COLOR_BayerGB2BGRA = 140,
	/// equivalent to BGGR Bayer pattern
	COLOR_BayerRG2BGRA = 141,
	/// equivalent to GBRG Bayer pattern
	COLOR_BayerGR2BGRA = 142,
	// Duplicate, use COLOR_BayerBG2BGRA instead
	// COLOR_BayerRGGB2BGRA = 139,
	// Duplicate, use COLOR_BayerGB2BGRA instead
	// COLOR_BayerGRBG2BGRA = 140,
	// Duplicate, use COLOR_BayerRG2BGRA instead
	// COLOR_BayerBGGR2BGRA = 141,
	// Duplicate, use COLOR_BayerGR2BGRA instead
	// COLOR_BayerGBRG2BGRA = 142,
	// Duplicate, use COLOR_BayerBGGR2BGRA instead
	// COLOR_BayerRGGB2RGBA = 141,
	// Duplicate, use COLOR_BayerGBRG2BGRA instead
	// COLOR_BayerGRBG2RGBA = 142,
	// Duplicate, use COLOR_BayerRGGB2BGRA instead
	// COLOR_BayerBGGR2RGBA = 139,
	// Duplicate, use COLOR_BayerGRBG2BGRA instead
	// COLOR_BayerGBRG2RGBA = 140,
	// equivalent to RGGB Bayer pattern
	// Duplicate, use COLOR_BayerRGGB2RGBA instead
	// COLOR_BayerBG2RGBA = 141,
	// equivalent to GRBG Bayer pattern
	// Duplicate, use COLOR_BayerGRBG2RGBA instead
	// COLOR_BayerGB2RGBA = 142,
	// equivalent to BGGR Bayer pattern
	// Duplicate, use COLOR_BayerBGGR2RGBA instead
	// COLOR_BayerRG2RGBA = 139,
	// equivalent to GBRG Bayer pattern
	// Duplicate, use COLOR_BayerGBRG2RGBA instead
	// COLOR_BayerGR2RGBA = 140,
	/// convert between RGB and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_RGB2YUV_UYVY = 143,
	/// convert between BGR and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_BGR2YUV_UYVY = 144,
	// synonym to UYVY
	// Duplicate, use COLOR_RGB2YUV_UYVY instead
	// COLOR_RGB2YUV_Y422 = 143,
	// synonym to UYVY
	// Duplicate, use COLOR_BGR2YUV_UYVY instead
	// COLOR_BGR2YUV_Y422 = 144,
	// synonym to UYVY
	// Duplicate, use COLOR_RGB2YUV_Y422 instead
	// COLOR_RGB2YUV_UYNV = 143,
	// synonym to UYVY
	// Duplicate, use COLOR_BGR2YUV_Y422 instead
	// COLOR_BGR2YUV_UYNV = 144,
	/// convert between RGBA and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_RGBA2YUV_UYVY = 145,
	/// convert between BGRA and YUV UYVU, YUV is 4:2:2 and interleaved as U/Y1/V/Y2, see [color_convert_rgb_yuv_42x]
	COLOR_BGRA2YUV_UYVY = 146,
	// synonym to UYVY
	// Duplicate, use COLOR_RGBA2YUV_UYVY instead
	// COLOR_RGBA2YUV_Y422 = 145,
	// synonym to UYVY
	// Duplicate, use COLOR_BGRA2YUV_UYVY instead
	// COLOR_BGRA2YUV_Y422 = 146,
	// synonym to UYVY
	// Duplicate, use COLOR_RGBA2YUV_Y422 instead
	// COLOR_RGBA2YUV_UYNV = 145,
	// synonym to UYVY
	// Duplicate, use COLOR_BGRA2YUV_Y422 instead
	// COLOR_BGRA2YUV_UYNV = 146,
	/// convert between RGB and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_RGB2YUV_YUY2 = 147,
	/// convert between BGR and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_BGR2YUV_YUY2 = 148,
	/// convert between RGB and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_RGB2YUV_YVYU = 149,
	/// convert between BGR and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_BGR2YUV_YVYU = 150,
	// synonym to YUY2
	// Duplicate, use COLOR_RGB2YUV_YUY2 instead
	// COLOR_RGB2YUV_YUYV = 147,
	// synonym to YUY2
	// Duplicate, use COLOR_BGR2YUV_YUY2 instead
	// COLOR_BGR2YUV_YUYV = 148,
	// synonym to YUY2
	// Duplicate, use COLOR_RGB2YUV_YUYV instead
	// COLOR_RGB2YUV_YUNV = 147,
	// synonym to YUY2
	// Duplicate, use COLOR_BGR2YUV_YUYV instead
	// COLOR_BGR2YUV_YUNV = 148,
	/// convert between RGBA and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_RGBA2YUV_YUY2 = 151,
	/// convert between BGRA and YUV YUY2, YUV is 4:2:2 and interleaved as Y1/U/Y2/V, see [color_convert_rgb_yuv_42x]
	COLOR_BGRA2YUV_YUY2 = 152,
	/// convert between RGBA and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_RGBA2YUV_YVYU = 153,
	/// convert between BGRA and YUV YVYU, YUV is 4:2:2 and interleaved as Y1/V/Y2/U, see [color_convert_rgb_yuv_42x]
	COLOR_BGRA2YUV_YVYU = 154,
	// synonym to YUY2
	// Duplicate, use COLOR_RGBA2YUV_YUY2 instead
	// COLOR_RGBA2YUV_YUYV = 151,
	// synonym to YUY2
	// Duplicate, use COLOR_BGRA2YUV_YUY2 instead
	// COLOR_BGRA2YUV_YUYV = 152,
	// synonym to YUY2
	// Duplicate, use COLOR_RGBA2YUV_YUYV instead
	// COLOR_RGBA2YUV_YUNV = 151,
	// synonym to YUY2
	// Duplicate, use COLOR_BGRA2YUV_YUYV instead
	// COLOR_BGRA2YUV_YUNV = 152,
	COLOR_COLORCVT_MAX = 155,
}

opencv_type_enum! { crate::imgproc::ColorConversionCodes }

/// GNU Octave/MATLAB equivalent colormaps
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColormapTypes {
	/// ![autumn](https://docs.opencv.org/4.12.0/colorscale_autumn.jpg)
	COLORMAP_AUTUMN = 0,
	/// ![bone](https://docs.opencv.org/4.12.0/colorscale_bone.jpg)
	COLORMAP_BONE = 1,
	/// ![jet](https://docs.opencv.org/4.12.0/colorscale_jet.jpg)
	COLORMAP_JET = 2,
	/// ![winter](https://docs.opencv.org/4.12.0/colorscale_winter.jpg)
	COLORMAP_WINTER = 3,
	/// ![rainbow](https://docs.opencv.org/4.12.0/colorscale_rainbow.jpg)
	COLORMAP_RAINBOW = 4,
	/// ![ocean](https://docs.opencv.org/4.12.0/colorscale_ocean.jpg)
	COLORMAP_OCEAN = 5,
	/// ![summer](https://docs.opencv.org/4.12.0/colorscale_summer.jpg)
	COLORMAP_SUMMER = 6,
	/// ![spring](https://docs.opencv.org/4.12.0/colorscale_spring.jpg)
	COLORMAP_SPRING = 7,
	/// ![cool](https://docs.opencv.org/4.12.0/colorscale_cool.jpg)
	COLORMAP_COOL = 8,
	/// ![HSV](https://docs.opencv.org/4.12.0/colorscale_hsv.jpg)
	COLORMAP_HSV = 9,
	/// ![pink](https://docs.opencv.org/4.12.0/colorscale_pink.jpg)
	COLORMAP_PINK = 10,
	/// ![hot](https://docs.opencv.org/4.12.0/colorscale_hot.jpg)
	COLORMAP_HOT = 11,
	/// ![parula](https://docs.opencv.org/4.12.0/colorscale_parula.jpg)
	COLORMAP_PARULA = 12,
	/// ![magma](https://docs.opencv.org/4.12.0/colorscale_magma.jpg)
	COLORMAP_MAGMA = 13,
	/// ![inferno](https://docs.opencv.org/4.12.0/colorscale_inferno.jpg)
	COLORMAP_INFERNO = 14,
	/// ![plasma](https://docs.opencv.org/4.12.0/colorscale_plasma.jpg)
	COLORMAP_PLASMA = 15,
	/// ![viridis](https://docs.opencv.org/4.12.0/colorscale_viridis.jpg)
	COLORMAP_VIRIDIS = 16,
	/// ![cividis](https://docs.opencv.org/4.12.0/colorscale_cividis.jpg)
	COLORMAP_CIVIDIS = 17,
	/// ![twilight](https://docs.opencv.org/4.12.0/colorscale_twilight.jpg)
	COLORMAP_TWILIGHT = 18,
	/// ![twilight shifted](https://docs.opencv.org/4.12.0/colorscale_twilight_shifted.jpg)
	COLORMAP_TWILIGHT_SHIFTED = 19,
	/// ![turbo](https://docs.opencv.org/4.12.0/colorscale_turbo.jpg)
	COLORMAP_TURBO = 20,
	/// ![deepgreen](https://docs.opencv.org/4.12.0/colorscale_deepgreen.jpg)
	COLORMAP_DEEPGREEN = 21,
}

opencv_type_enum! { crate::imgproc::ColormapTypes }

/// connected components algorithm
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConnectedComponentsAlgorithmsTypes {
	/// Spaghetti [Bolelli2019](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2019) algorithm for 8-way connectivity, Spaghetti4C [Bolelli2021](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2021) algorithm for 4-way connectivity.
	CCL_DEFAULT = -1,
	/// SAUF [Wu2009](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Wu2009) algorithm for 8-way connectivity, SAUF algorithm for 4-way connectivity. The parallel implementation described in [Bolelli2017](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2017) is available for SAUF.
	CCL_WU = 0,
	/// BBDT [Grana2010](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Grana2010) algorithm for 8-way connectivity, SAUF algorithm for 4-way connectivity. The parallel implementation described in [Bolelli2017](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2017) is available for both BBDT and SAUF.
	CCL_GRANA = 1,
	/// Spaghetti [Bolelli2019](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2019) algorithm for 8-way connectivity, Spaghetti4C [Bolelli2021](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2021) algorithm for 4-way connectivity. The parallel implementation described in [Bolelli2017](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Bolelli2017) is available for both Spaghetti and Spaghetti4C.
	CCL_BOLELLI = 2,
	/// Same as CCL_WU. It is preferable to use the flag with the name of the algorithm (CCL_SAUF) rather than the one with the name of the first author (CCL_WU).
	CCL_SAUF = 3,
	/// Same as CCL_GRANA. It is preferable to use the flag with the name of the algorithm (CCL_BBDT) rather than the one with the name of the first author (CCL_GRANA).
	CCL_BBDT = 4,
	/// Same as CCL_BOLELLI. It is preferable to use the flag with the name of the algorithm (CCL_SPAGHETTI) rather than the one with the name of the first author (CCL_BOLELLI).
	CCL_SPAGHETTI = 5,
}

opencv_type_enum! { crate::imgproc::ConnectedComponentsAlgorithmsTypes }

/// connected components statistics
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConnectedComponentsTypes {
	/// The leftmost (x) coordinate which is the inclusive start of the bounding
	/// box in the horizontal direction.
	CC_STAT_LEFT = 0,
	/// The topmost (y) coordinate which is the inclusive start of the bounding
	/// box in the vertical direction.
	CC_STAT_TOP = 1,
	/// The horizontal size of the bounding box
	CC_STAT_WIDTH = 2,
	/// The vertical size of the bounding box
	CC_STAT_HEIGHT = 3,
	/// The total area (in pixels) of the connected component
	CC_STAT_AREA = 4,
	/// Max enumeration value. Used internally only for memory allocation
	CC_STAT_MAX = 5,
}

opencv_type_enum! { crate::imgproc::ConnectedComponentsTypes }

/// the contour approximation algorithm
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ContourApproximationModes {
	/// stores absolutely all the contour points. That is, any 2 subsequent points (x1,y1) and
	/// (x2,y2) of the contour will be either horizontal, vertical or diagonal neighbors, that is,
	/// max(abs(x1-x2),abs(y2-y1))==1.
	CHAIN_APPROX_NONE = 1,
	/// compresses horizontal, vertical, and diagonal segments and leaves only their end points.
	/// For example, an up-right rectangular contour is encoded with 4 points.
	CHAIN_APPROX_SIMPLE = 2,
	/// applies one of the flavors of the Teh-Chin chain approximation algorithm [TehChin89](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_TehChin89)
	CHAIN_APPROX_TC89_L1 = 3,
	/// applies one of the flavors of the Teh-Chin chain approximation algorithm [TehChin89](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_TehChin89)
	CHAIN_APPROX_TC89_KCOS = 4,
}

opencv_type_enum! { crate::imgproc::ContourApproximationModes }

/// distanceTransform algorithm flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DistanceTransformLabelTypes {
	/// each connected component of zeros in src (as well as all the non-zero pixels closest to the
	/// connected component) will be assigned the same label
	DIST_LABEL_CCOMP = 0,
	/// each zero pixel (and all the non-zero pixels closest to it) gets its own label.
	DIST_LABEL_PIXEL = 1,
}

opencv_type_enum! { crate::imgproc::DistanceTransformLabelTypes }

/// Mask size for distance transform
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DistanceTransformMasks {
	/// mask=3
	DIST_MASK_3 = 3,
	/// mask=5
	DIST_MASK_5 = 5,
	DIST_MASK_PRECISE = 0,
}

opencv_type_enum! { crate::imgproc::DistanceTransformMasks }

/// Distance types for Distance Transform and M-estimators
/// ## See also
/// distanceTransform, fitLine
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DistanceTypes {
	/// User defined distance
	DIST_USER = -1,
	/// distance = |x1-x2| + |y1-y2|
	DIST_L1 = 1,
	/// the simple euclidean distance
	DIST_L2 = 2,
	/// distance = max(|x1-x2|,|y1-y2|)
	DIST_C = 3,
	/// L1-L2 metric: distance = 2(sqrt(1+x*x/2) - 1))
	DIST_L12 = 4,
	/// distance = c^2(|x|/c-log(1+|x|/c)), c = 1.3998
	DIST_FAIR = 5,
	/// distance = c^2/2(1-exp(-(x/c)^2)), c = 2.9846
	DIST_WELSCH = 6,
	/// distance = |x|<c ? x^2/2 : c(|x|-c/2), c=1.345
	DIST_HUBER = 7,
}

opencv_type_enum! { crate::imgproc::DistanceTypes }

/// floodfill algorithm flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FloodFillFlags {
	/// If set, the difference between the current pixel and seed pixel is considered. Otherwise,
	/// the difference between neighbor pixels is considered (that is, the range is floating).
	FLOODFILL_FIXED_RANGE = 65536,
	/// If set, the function does not change the image ( newVal is ignored), and only fills the
	/// mask with the value specified in bits 8-16 of flags as described above. This option only make
	/// sense in function variants that have the mask parameter.
	FLOODFILL_MASK_ONLY = 131072,
}

opencv_type_enum! { crate::imgproc::FloodFillFlags }

/// class of the pixel in GrabCut algorithm
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GrabCutClasses {
	/// an obvious background pixels
	GC_BGD = 0,
	/// an obvious foreground (object) pixel
	GC_FGD = 1,
	/// a possible background pixel
	GC_PR_BGD = 2,
	/// a possible foreground pixel
	GC_PR_FGD = 3,
}

opencv_type_enum! { crate::imgproc::GrabCutClasses }

/// GrabCut algorithm flags
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GrabCutModes {
	/// The function initializes the state and the mask using the provided rectangle. After that it
	/// runs iterCount iterations of the algorithm.
	GC_INIT_WITH_RECT = 0,
	/// The function initializes the state using the provided mask. Note that GC_INIT_WITH_RECT
	/// and GC_INIT_WITH_MASK can be combined. Then, all the pixels outside of the ROI are
	/// automatically initialized with GC_BGD .
	GC_INIT_WITH_MASK = 1,
	/// The value means that the algorithm should just resume.
	GC_EVAL = 2,
	/// The value means that the algorithm should just run the grabCut algorithm (a single iteration) with the fixed model
	GC_EVAL_FREEZE_MODEL = 3,
}

opencv_type_enum! { crate::imgproc::GrabCutModes }

/// Only a subset of Hershey fonts <https://en.wikipedia.org/wiki/Hershey_fonts> are supported
/// @ingroup imgproc_draw
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HersheyFonts {
	/// normal size sans-serif font
	FONT_HERSHEY_SIMPLEX = 0,
	/// small size sans-serif font
	FONT_HERSHEY_PLAIN = 1,
	/// normal size sans-serif font (more complex than FONT_HERSHEY_SIMPLEX)
	FONT_HERSHEY_DUPLEX = 2,
	/// normal size serif font
	FONT_HERSHEY_COMPLEX = 3,
	/// normal size serif font (more complex than FONT_HERSHEY_COMPLEX)
	FONT_HERSHEY_TRIPLEX = 4,
	/// smaller version of FONT_HERSHEY_COMPLEX
	FONT_HERSHEY_COMPLEX_SMALL = 5,
	/// hand-writing style font
	FONT_HERSHEY_SCRIPT_SIMPLEX = 6,
	/// more complex variant of FONT_HERSHEY_SCRIPT_SIMPLEX
	FONT_HERSHEY_SCRIPT_COMPLEX = 7,
	/// flag for italic font
	FONT_ITALIC = 16,
}

opencv_type_enum! { crate::imgproc::HersheyFonts }

/// Histogram comparison methods
/// @ingroup imgproc_hist
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HistCompMethods {
	/// Correlation
	/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Cfrac%7B%5Csum%5FI%20%28H%5F1%28I%29%20%2D%20%5Cbar%7BH%5F1%7D%29%20%28H%5F2%28I%29%20%2D%20%5Cbar%7BH%5F2%7D%29%7D%7B%5Csqrt%7B%5Csum%5FI%28H%5F1%28I%29%20%2D%20%5Cbar%7BH%5F1%7D%29%5E2%20%5Csum%5FI%28H%5F2%28I%29%20%2D%20%5Cbar%7BH%5F2%7D%29%5E2%7D%7D)
	/// where
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbar%7BH%5Fk%7D%20%3D%20%20%5Cfrac%7B1%7D%7BN%7D%20%5Csum%20%5FJ%20H%5Fk%28J%29)
	/// and ![inline formula](https://latex.codecogs.com/png.latex?N) is a total number of histogram bins.
	HISTCMP_CORREL = 0,
	/// Chi-Square
	/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Csum%20%5FI%20%20%5Cfrac%7B%5Cleft%28H%5F1%28I%29%2DH%5F2%28I%29%5Cright%29%5E2%7D%7BH%5F1%28I%29%7D)
	HISTCMP_CHISQR = 1,
	/// Intersection
	/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Csum%20%5FI%20%20%5Cmin%20%28H%5F1%28I%29%2C%20H%5F2%28I%29%29)
	HISTCMP_INTERSECT = 2,
	/// Bhattacharyya distance
	/// (In fact, OpenCV computes Hellinger distance, which is related to Bhattacharyya coefficient.)
	/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%20%5Csqrt%7B1%20%2D%20%5Cfrac%7B1%7D%7B%5Csqrt%7B%5Cbar%7BH%5F1%7D%20%5Cbar%7BH%5F2%7D%20N%5E2%7D%7D%20%5Csum%5FI%20%5Csqrt%7BH%5F1%28I%29%20%5Ccdot%20H%5F2%28I%29%7D%7D)
	HISTCMP_BHATTACHARYYA = 3,
	// Synonym for HISTCMP_BHATTACHARYYA
	// Duplicate, use HISTCMP_BHATTACHARYYA instead
	// HISTCMP_HELLINGER = 3,
	/// Alternative Chi-Square
	/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%202%20%2A%20%5Csum%20%5FI%20%20%5Cfrac%7B%5Cleft%28H%5F1%28I%29%2DH%5F2%28I%29%5Cright%29%5E2%7D%7BH%5F1%28I%29%2BH%5F2%28I%29%7D)
	/// This alternative formula is regularly used for texture comparison. See e.g. [Puzicha1997](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Puzicha1997)
	HISTCMP_CHISQR_ALT = 4,
	/// Kullback-Leibler divergence
	/// ![block formula](https://latex.codecogs.com/png.latex?d%28H%5F1%2CH%5F2%29%20%3D%20%5Csum%20%5FI%20H%5F1%28I%29%20%5Clog%20%5Cleft%28%5Cfrac%7BH%5F1%28I%29%7D%7BH%5F2%28I%29%7D%5Cright%29)
	HISTCMP_KL_DIV = 5,
}

opencv_type_enum! { crate::imgproc::HistCompMethods }

/// Variants of a Hough transform
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HoughModes {
	/// classical or standard Hough transform. Every line is represented by two floating-point
	/// numbers ![inline formula](https://latex.codecogs.com/png.latex?%28%5Crho%2C%20%5Ctheta%29) , where ![inline formula](https://latex.codecogs.com/png.latex?%5Crho) is a distance between (0,0) point and the line,
	/// and ![inline formula](https://latex.codecogs.com/png.latex?%5Ctheta) is the angle between x-axis and the normal to the line. Thus, the matrix must
	/// be (the created sequence will be) of CV_32FC2 type
	HOUGH_STANDARD = 0,
	/// probabilistic Hough transform (more efficient in case if the picture contains a few long
	/// linear segments). It returns line segments rather than the whole line. Each segment is
	/// represented by starting and ending points, and the matrix must be (the created sequence will
	/// be) of the CV_32SC4 type.
	HOUGH_PROBABILISTIC = 1,
	/// multi-scale variant of the classical Hough transform. The lines are encoded the same way as
	/// HOUGH_STANDARD.
	HOUGH_MULTI_SCALE = 2,
	/// basically *21HT*, described in [Yuen90](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Yuen90)
	HOUGH_GRADIENT = 3,
	/// variation of HOUGH_GRADIENT to get better accuracy
	HOUGH_GRADIENT_ALT = 4,
}

opencv_type_enum! { crate::imgproc::HoughModes }

/// interpolation algorithm
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterpolationFlags {
	/// nearest neighbor interpolation
	INTER_NEAREST = 0,
	/// bilinear interpolation
	INTER_LINEAR = 1,
	/// bicubic interpolation
	INTER_CUBIC = 2,
	/// resampling using pixel area relation. It may be a preferred method for image decimation, as
	/// it gives moire'-free results. But when the image is zoomed, it is similar to the INTER_NEAREST
	/// method.
	INTER_AREA = 3,
	/// Lanczos interpolation over 8x8 neighborhood
	INTER_LANCZOS4 = 4,
	/// Bit exact bilinear interpolation
	INTER_LINEAR_EXACT = 5,
	/// Bit exact nearest neighbor interpolation. This will produce same results as
	/// the nearest neighbor method in PIL, scikit-image or Matlab.
	INTER_NEAREST_EXACT = 6,
	/// mask for interpolation codes
	INTER_MAX = 7,
	/// flag, fills all of the destination image pixels. If some of them correspond to outliers in the
	/// source image, they are set to zero
	WARP_FILL_OUTLIERS = 8,
	/// flag, inverse transformation
	/// 
	/// For example, [linear_polar] or [log_polar] transforms:
	/// - flag is __not__ set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28%20%5Crho%20%2C%20%5Cphi%20%29%20%3D%20src%28x%2Cy%29)
	/// - flag is set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28x%2Cy%29%20%3D%20src%28%20%5Crho%20%2C%20%5Cphi%20%29)
	WARP_INVERSE_MAP = 16,
	/// flag, inverse transformation
	/// 
	/// For example, [linear_polar] or [log_polar] transforms:
	/// - flag is __not__ set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28%20%5Crho%20%2C%20%5Cphi%20%29%20%3D%20src%28x%2Cy%29)
	/// - flag is set: ![inline formula](https://latex.codecogs.com/png.latex?dst%28x%2Cy%29%20%3D%20src%28%20%5Crho%20%2C%20%5Cphi%20%29)
	WARP_RELATIVE_MAP = 32,
}

opencv_type_enum! { crate::imgproc::InterpolationFlags }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterpolationMasks {
	INTER_BITS = 5,
	INTER_BITS2 = 10,
	INTER_TAB_SIZE = 32,
	INTER_TAB_SIZE2 = 1024,
}

opencv_type_enum! { crate::imgproc::InterpolationMasks }

/// Variants of Line Segment %Detector
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LineSegmentDetectorModes {
	/// No refinement applied
	LSD_REFINE_NONE = 0,
	/// Standard refinement is applied. E.g. breaking arches into smaller straighter line approximations.
	LSD_REFINE_STD = 1,
	/// Advanced refinement. Number of false alarms is calculated, lines are
	/// refined through increase of precision, decrement in size, etc.
	LSD_REFINE_ADV = 2,
}

opencv_type_enum! { crate::imgproc::LineSegmentDetectorModes }

/// types of line
/// @ingroup imgproc_draw
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LineTypes {
	FILLED = -1,
	/// 4-connected line
	LINE_4 = 4,
	/// 8-connected line
	LINE_8 = 8,
	/// antialiased line
	LINE_AA = 16,
}

opencv_type_enum! { crate::imgproc::LineTypes }

/// Possible set of marker types used for the cv::drawMarker function
/// @ingroup imgproc_draw
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MarkerTypes {
	/// A crosshair marker shape
	MARKER_CROSS = 0,
	/// A 45 degree tilted crosshair marker shape
	MARKER_TILTED_CROSS = 1,
	/// A star marker shape, combination of cross and tilted cross
	MARKER_STAR = 2,
	/// A diamond marker shape
	MARKER_DIAMOND = 3,
	/// A square marker shape
	MARKER_SQUARE = 4,
	/// An upwards pointing triangle marker shape
	MARKER_TRIANGLE_UP = 5,
	/// A downwards pointing triangle marker shape
	MARKER_TRIANGLE_DOWN = 6,
}

opencv_type_enum! { crate::imgproc::MarkerTypes }

/// shape of the structuring element
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MorphShapes {
	/// a rectangular structuring element:  ![block formula](https://latex.codecogs.com/png.latex?E%5F%7Bij%7D%3D1)
	MORPH_RECT = 0,
	/// a cross-shaped structuring element:
	/// ![block formula](https://latex.codecogs.com/png.latex?E%5F%7Bij%7D%20%3D%20%5Cbegin%7Bcases%7D%201%20%26%20%5Ctexttt%7Bif%20%7D%20%7Bi%3D%5Ctexttt%7Banchor%2Ey%20%7D%20%7Bor%20%7D%20%7Bj%3D%5Ctexttt%7Banchor%2Ex%7D%7D%7D%20%5C%5C0%20%26%20%5Ctexttt%7Botherwise%7D%20%5Cend%7Bcases%7D)
	MORPH_CROSS = 1,
	/// an elliptic structuring element, that is, a filled ellipse inscribed
	/// into the rectangle Rect(0, 0, esize.width, esize.height)
	MORPH_ELLIPSE = 2,
	/// a diamond structuring element defined by Manhattan distance
	MORPH_DIAMOND = 3,
}

opencv_type_enum! { crate::imgproc::MorphShapes }

/// type of morphological operation
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MorphTypes {
	/// see #erode
	MORPH_ERODE = 0,
	/// see #dilate
	MORPH_DILATE = 1,
	/// an opening operation
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bopen%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Bdilate%7D%20%28%20%5Cmathrm%7Berode%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%29)
	MORPH_OPEN = 2,
	/// a closing operation
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bclose%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Berode%7D%20%28%20%5Cmathrm%7Bdilate%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%29)
	MORPH_CLOSE = 3,
	/// a morphological gradient
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bmorph%5C%5Fgrad%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Bdilate%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%2D%20%5Cmathrm%7Berode%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29)
	MORPH_GRADIENT = 4,
	/// "top hat"
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Btophat%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Ctexttt%7Bsrc%7D%20%2D%20%5Cmathrm%7Bopen%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29)
	MORPH_TOPHAT = 5,
	/// "black hat"
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%3D%20%5Cmathrm%7Bblackhat%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%3D%20%5Cmathrm%7Bclose%7D%20%28%20%5Ctexttt%7Bsrc%7D%20%2C%20%5Ctexttt%7Belement%7D%20%29%2D%20%5Ctexttt%7Bsrc%7D)
	MORPH_BLACKHAT = 6,
	/// "hit or miss"
	/// .- Only supported for CV_8UC1 binary images. A tutorial can be found in the documentation
	MORPH_HITMISS = 7,
}

opencv_type_enum! { crate::imgproc::MorphTypes }

/// types of intersection between rectangles
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RectanglesIntersectTypes {
	/// No intersection
	INTERSECT_NONE = 0,
	/// There is a partial intersection
	INTERSECT_PARTIAL = 1,
	/// One of the rectangle is fully enclosed in the other
	INTERSECT_FULL = 2,
}

opencv_type_enum! { crate::imgproc::RectanglesIntersectTypes }

/// mode of the contour retrieval algorithm
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RetrievalModes {
	/// retrieves only the extreme outer contours. It sets `hierarchy[i][2]=hierarchy[i][3]=-1` for
	/// all the contours.
	RETR_EXTERNAL = 0,
	/// retrieves all of the contours without establishing any hierarchical relationships.
	RETR_LIST = 1,
	/// retrieves all of the contours and organizes them into a two-level hierarchy. At the top
	/// level, there are external boundaries of the components. At the second level, there are
	/// boundaries of the holes. If there is another contour inside a hole of a connected component, it
	/// is still put at the top level.
	RETR_CCOMP = 2,
	/// retrieves all of the contours and reconstructs a full hierarchy of nested contours.
	RETR_TREE = 3,
	RETR_FLOODFILL = 4,
}

opencv_type_enum! { crate::imgproc::RetrievalModes }

/// Shape matching methods
/// 
/// ![inline formula](https://latex.codecogs.com/png.latex?A) denotes object1,![inline formula](https://latex.codecogs.com/png.latex?B) denotes object2
/// 
/// ![inline formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20m%5EA%5Fi%20%3D%20%20%5Cmathrm%7Bsign%7D%20%28h%5EA%5Fi%29%20%20%5Ccdot%20%5Clog%7Bh%5EA%5Fi%7D%20%5C%5C%20m%5EB%5Fi%20%3D%20%20%5Cmathrm%7Bsign%7D%20%28h%5EB%5Fi%29%20%20%5Ccdot%20%5Clog%7Bh%5EB%5Fi%7D%20%5Cend%7Barray%7D)
/// 
/// and ![inline formula](https://latex.codecogs.com/png.latex?h%5EA%5Fi%2C%20h%5EB%5Fi) are the Hu moments of ![inline formula](https://latex.codecogs.com/png.latex?A) and ![inline formula](https://latex.codecogs.com/png.latex?B) , respectively.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShapeMatchModes {
	/// ![block formula](https://latex.codecogs.com/png.latex?I%5F1%28A%2CB%29%20%3D%20%20%5Csum%20%5F%7Bi%3D1%2E%2E%2E7%7D%20%20%5Cleft%20%7C%20%20%5Cfrac%7B1%7D%7Bm%5EA%5Fi%7D%20%2D%20%20%5Cfrac%7B1%7D%7Bm%5EB%5Fi%7D%20%5Cright%20%7C)
	CONTOURS_MATCH_I1 = 1,
	/// ![block formula](https://latex.codecogs.com/png.latex?I%5F2%28A%2CB%29%20%3D%20%20%5Csum%20%5F%7Bi%3D1%2E%2E%2E7%7D%20%20%5Cleft%20%7C%20m%5EA%5Fi%20%2D%20m%5EB%5Fi%20%20%5Cright%20%7C)
	CONTOURS_MATCH_I2 = 2,
	/// ![block formula](https://latex.codecogs.com/png.latex?I%5F3%28A%2CB%29%20%3D%20%20%5Cmax%20%5F%7Bi%3D1%2E%2E%2E7%7D%20%20%5Cfrac%7B%20%5Cleft%7C%20m%5EA%5Fi%20%2D%20m%5EB%5Fi%20%5Cright%7C%20%7D%7B%20%5Cleft%7C%20m%5EA%5Fi%20%5Cright%7C%20%7D)
	CONTOURS_MATCH_I3 = 3,
}

opencv_type_enum! { crate::imgproc::ShapeMatchModes }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SpecialFilter {
	FILTER_SCHARR = -1,
}

opencv_type_enum! { crate::imgproc::SpecialFilter }

/// type of the template matching operation
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TemplateMatchModes {
	/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%5E2)
	/// with mask:
	/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2)
	TM_SQDIFF = 0,
	/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%5E2%7D%7B%5Csqrt%7B%5Csum%5F%7B%0A%20%20%20x%27%2Cy%27%7DT%28x%27%2Cy%27%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20I%28x%2Bx%27%2Cy%2By%27%29%5E2%7D%7D)
	/// with mask:
	/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20%28T%28x%27%2Cy%27%29%2DI%28x%2Bx%27%2Cy%2By%27%29%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2%7D%7B%5Csqrt%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20T%28x%27%2Cy%27%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%20%5Cright%29%5E2%7D%7D)
	TM_SQDIFF_NORMED = 1,
	/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%29)
	/// with mask:
	/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%20M%28x%27%2Cy%27%29%0A%20%20%20%5E2%29)
	TM_CCORR = 2,
	/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%29%7D%7B%5Csqrt%7B%0A%20%20%20%5Csum%5F%7Bx%27%2Cy%27%7DT%28x%27%2Cy%27%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20I%28x%2Bx%27%2Cy%2By%27%29%5E2%7D%7D)
	/// with mask:
	/// ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%28x%27%2Cy%27%29%20%5Ccdot%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%0A%20%20%20M%28x%27%2Cy%27%29%5E2%29%7D%7B%5Csqrt%7B%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20T%28x%27%2Cy%27%29%20%5Ccdot%20M%28x%27%2Cy%27%29%0A%20%20%20%5Cright%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20%5Cleft%28%20I%28x%2Bx%27%2Cy%2By%27%29%20%5Ccdot%20M%28x%27%2Cy%27%29%0A%20%20%20%5Cright%29%5E2%7D%7D)
	TM_CCORR_NORMED = 3,
	/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Csum%20%5F%7Bx%27%2Cy%27%7D%20%28T%27%28x%27%2Cy%27%29%20%5Ccdot%20I%27%28x%2Bx%27%2Cy%2By%27%29%29)
	/// where
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20T%27%28x%27%2Cy%27%29%3DT%28x%27%2Cy%27%29%20%2D%201%2F%28w%20%5Ccdot%20h%29%20%5Ccdot%20%5Csum%20%5F%7B%0A%20%20%20x%27%27%2Cy%27%27%7D%20T%28x%27%27%2Cy%27%27%29%20%5C%5C%20I%27%28x%2Bx%27%2Cy%2By%27%29%3DI%28x%2Bx%27%2Cy%2By%27%29%20%2D%201%2F%28w%20%5Ccdot%20h%29%0A%20%20%20%5Ccdot%20%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20I%28x%2Bx%27%27%2Cy%2By%27%27%29%20%5Cend%7Barray%7D)
	/// with mask:
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Barray%7D%7Bl%7D%20T%27%28x%27%2Cy%27%29%3DM%28x%27%2Cy%27%29%20%5Ccdot%20%5Cleft%28%20T%28x%27%2Cy%27%29%20%2D%0A%20%20%20%5Cfrac%7B1%7D%7B%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20M%28x%27%27%2Cy%27%27%29%7D%20%5Ccdot%20%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%0A%20%20%20%28T%28x%27%27%2Cy%27%27%29%20%5Ccdot%20M%28x%27%27%2Cy%27%27%29%29%20%5Cright%29%20%5C%5C%20I%27%28x%2Bx%27%2Cy%2By%27%29%3DM%28x%27%2Cy%27%29%0A%20%20%20%5Ccdot%20%5Cleft%28%20I%28x%2Bx%27%2Cy%2By%27%29%20%2D%20%5Cfrac%7B1%7D%7B%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20M%28x%27%27%2Cy%27%27%29%7D%0A%20%20%20%5Ccdot%20%5Csum%20%5F%7Bx%27%27%2Cy%27%27%7D%20%28I%28x%2Bx%27%27%2Cy%2By%27%27%29%20%5Ccdot%20M%28x%27%27%2Cy%27%27%29%29%20%5Cright%29%0A%20%20%20%5Cend%7Barray%7D%20)
	TM_CCOEFF = 4,
	/// !< ![block formula](https://latex.codecogs.com/png.latex?R%28x%2Cy%29%3D%20%5Cfrac%7B%20%5Csum%5F%7Bx%27%2Cy%27%7D%20%28T%27%28x%27%2Cy%27%29%20%5Ccdot%20I%27%28x%2Bx%27%2Cy%2By%27%29%29%20%7D%7B%0A%5Csqrt%7B%5Csum%5F%7Bx%27%2Cy%27%7DT%27%28x%27%2Cy%27%29%5E2%20%5Ccdot%20%5Csum%5F%7Bx%27%2Cy%27%7D%20I%27%28x%2Bx%27%2Cy%2By%27%29%5E2%7D%0A%7D)
	TM_CCOEFF_NORMED = 5,
}

opencv_type_enum! { crate::imgproc::TemplateMatchModes }

/// type of the threshold operation
/// ![threshold types](https://docs.opencv.org/4.12.0/threshold.png)
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThresholdTypes {
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bmaxval%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B0%7D%7Botherwise%7D)
	THRESH_BINARY = 0,
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B%5Ctexttt%7Bmaxval%7D%7D%7Botherwise%7D)
	THRESH_BINARY_INV = 1,
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bthreshold%7D%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B%5Ctexttt%7Bsrc%7D%28x%2Cy%29%7D%7Botherwise%7D)
	THRESH_TRUNC = 2,
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B%5Ctexttt%7Bsrc%7D%28x%2Cy%29%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B0%7D%7Botherwise%7D)
	THRESH_TOZERO = 3,
	/// ![block formula](https://latex.codecogs.com/png.latex?%5Ctexttt%7Bdst%7D%20%28x%2Cy%29%20%3D%20%20%5Cfork%7B0%7D%7Bif%20%5C%28%5Ctexttt%7Bsrc%7D%28x%2Cy%29%20%3E%20%5Ctexttt%7Bthresh%7D%5C%29%7D%7B%5Ctexttt%7Bsrc%7D%28x%2Cy%29%7D%7Botherwise%7D)
	THRESH_TOZERO_INV = 4,
	THRESH_MASK = 7,
	/// flag, use Otsu algorithm to choose the optimal threshold value
	THRESH_OTSU = 8,
	/// flag, use Triangle algorithm to choose the optimal threshold value
	THRESH_TRIANGLE = 16,
	/// flag, compute threshold only (useful for OTSU/TRIANGLE) but does not actually run thresholding
	THRESH_DRYRUN = 128,
}

opencv_type_enum! { crate::imgproc::ThresholdTypes }

/// \brief Specify the polar mapping mode
/// ## See also
/// warpPolar
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WarpPolarMode {
	/// Remaps an image to/from polar space.
	WARP_POLAR_LINEAR = 0,
	/// Remaps an image to/from semilog-polar space.
	WARP_POLAR_LOG = 256,
}

opencv_type_enum! { crate::imgproc::WarpPolarMode }

/// returns "magic" border value for erosion and dilation. It is automatically transformed to Scalar::all(-DBL_MAX) for dilation.
#[inline]
pub fn morphology_default_border_value() -> Result<core::Scalar> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_morphologyDefaultBorderValue(ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// Constant methods for [crate::imgproc::CLAHE]
pub trait CLAHETraitConst: core::AlgorithmTraitConst {
	fn as_raw_CLAHE(&self) -> *const c_void;

	/// Returns threshold value for contrast limiting.
	#[inline]
	fn get_clip_limit(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CLAHE_getClipLimit_const(self.as_raw_CLAHE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns Size defines the number of tiles in row and column.
	#[inline]
	fn get_tiles_grid_size(&self) -> Result<core::Size> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CLAHE_getTilesGridSize_const(self.as_raw_CLAHE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::CLAHE]
pub trait CLAHETrait: core::AlgorithmTrait + crate::imgproc::CLAHETraitConst {
	fn as_raw_mut_CLAHE(&mut self) -> *mut c_void;

	/// Equalizes the histogram of a grayscale image using Contrast Limited Adaptive Histogram Equalization.
	/// 
	/// ## Parameters
	/// * src: Source image of type CV_8UC1 or CV_16UC1.
	/// * dst: Destination image.
	#[inline]
	fn apply(&mut self, src: &impl core::ToInputArray, dst: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(src);
		output_array_arg!(dst);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CLAHE_apply_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_CLAHE(), src.as_raw__InputArray(), dst.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets threshold for contrast limiting.
	/// 
	/// ## Parameters
	/// * clipLimit: threshold value.
	#[inline]
	fn set_clip_limit(&mut self, clip_limit: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CLAHE_setClipLimit_double(self.as_raw_mut_CLAHE(), clip_limit, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Sets size of grid for histogram equalization. Input image will be divided into
	/// equally sized rectangular tiles.
	/// 
	/// ## Parameters
	/// * tileGridSize: defines the number of tiles in row and column.
	#[inline]
	fn set_tiles_grid_size(&mut self, tile_grid_size: core::Size) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CLAHE_setTilesGridSize_Size(self.as_raw_mut_CLAHE(), tile_grid_size.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn collect_garbage(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_CLAHE_collectGarbage(self.as_raw_mut_CLAHE(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Base class for Contrast Limited Adaptive Histogram Equalization.
pub struct CLAHE {
	ptr: *mut c_void
}

opencv_type_boxed! { CLAHE }

impl Drop for CLAHE {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_CLAHE_delete(self.as_raw_mut_CLAHE()) };
	}
}

unsafe impl Send for CLAHE {}

impl core::AlgorithmTraitConst for CLAHE {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for CLAHE {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::CLAHETraitConst for CLAHE {
	#[inline] fn as_raw_CLAHE(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::CLAHETrait for CLAHE {
	#[inline] fn as_raw_mut_CLAHE(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl CLAHE {
}

boxed_cast_base! { CLAHE, core::Algorithm, cv_CLAHE_to_Algorithm }

impl std::fmt::Debug for CLAHE {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("CLAHE")
			.finish()
	}
}

/// Constant methods for [crate::imgproc::GeneralizedHough]
pub trait GeneralizedHoughTraitConst: core::AlgorithmTraitConst {
	fn as_raw_GeneralizedHough(&self) -> *const c_void;

	#[inline]
	fn get_canny_low_thresh(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_getCannyLowThresh_const(self.as_raw_GeneralizedHough(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_canny_high_thresh(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_getCannyHighThresh_const(self.as_raw_GeneralizedHough(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_dist(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_getMinDist_const(self.as_raw_GeneralizedHough(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_dp(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_getDp_const(self.as_raw_GeneralizedHough(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_buffer_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_getMaxBufferSize_const(self.as_raw_GeneralizedHough(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::GeneralizedHough]
pub trait GeneralizedHoughTrait: core::AlgorithmTrait + crate::imgproc::GeneralizedHoughTraitConst {
	fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void;

	/// set template to search
	/// 
	/// ## C++ default parameters
	/// * templ_center: Point(-1,-1)
	#[inline]
	fn set_template(&mut self, templ: &impl core::ToInputArray, templ_center: core::Point) -> Result<()> {
		input_array_arg!(templ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setTemplate_const__InputArrayR_Point(self.as_raw_mut_GeneralizedHough(), templ.as_raw__InputArray(), templ_center.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// set template to search
	/// 
	/// ## Note
	/// This alternative version of [GeneralizedHoughTrait::set_template] function uses the following default values for its arguments:
	/// * templ_center: Point(-1,-1)
	#[inline]
	fn set_template_def(&mut self, templ: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(templ);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setTemplate_const__InputArrayR(self.as_raw_mut_GeneralizedHough(), templ.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * templ_center: Point(-1,-1)
	#[inline]
	fn set_template_1(&mut self, edges: &impl core::ToInputArray, dx: &impl core::ToInputArray, dy: &impl core::ToInputArray, templ_center: core::Point) -> Result<()> {
		input_array_arg!(edges);
		input_array_arg!(dx);
		input_array_arg!(dy);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR_Point(self.as_raw_mut_GeneralizedHough(), edges.as_raw__InputArray(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), templ_center.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [GeneralizedHoughTrait::set_template] function uses the following default values for its arguments:
	/// * templ_center: Point(-1,-1)
	#[inline]
	fn set_template_def_1(&mut self, edges: &impl core::ToInputArray, dx: &impl core::ToInputArray, dy: &impl core::ToInputArray) -> Result<()> {
		input_array_arg!(edges);
		input_array_arg!(dx);
		input_array_arg!(dy);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setTemplate_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_GeneralizedHough(), edges.as_raw__InputArray(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// find template on image
	/// 
	/// ## C++ default parameters
	/// * votes: noArray()
	#[inline]
	fn detect(&mut self, image: &impl core::ToInputArray, positions: &mut impl core::ToOutputArray, votes: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(positions);
		output_array_arg!(votes);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_GeneralizedHough(), image.as_raw__InputArray(), positions.as_raw__OutputArray(), votes.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// find template on image
	/// 
	/// ## Note
	/// This alternative version of [GeneralizedHoughTrait::detect] function uses the following default values for its arguments:
	/// * votes: noArray()
	#[inline]
	fn detect_def(&mut self, image: &impl core::ToInputArray, positions: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(positions);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_GeneralizedHough(), image.as_raw__InputArray(), positions.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * votes: noArray()
	#[inline]
	fn detect_with_edges(&mut self, edges: &impl core::ToInputArray, dx: &impl core::ToInputArray, dy: &impl core::ToInputArray, positions: &mut impl core::ToOutputArray, votes: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(edges);
		input_array_arg!(dx);
		input_array_arg!(dy);
		output_array_arg!(positions);
		output_array_arg!(votes);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_GeneralizedHough(), edges.as_raw__InputArray(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), positions.as_raw__OutputArray(), votes.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [GeneralizedHoughTrait::detect_with_edges] function uses the following default values for its arguments:
	/// * votes: noArray()
	#[inline]
	fn detect_with_edges_def(&mut self, edges: &impl core::ToInputArray, dx: &impl core::ToInputArray, dy: &impl core::ToInputArray, positions: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(edges);
		input_array_arg!(dx);
		input_array_arg!(dy);
		output_array_arg!(positions);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_detect_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_GeneralizedHough(), edges.as_raw__InputArray(), dx.as_raw__InputArray(), dy.as_raw__InputArray(), positions.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Canny low threshold.
	#[inline]
	fn set_canny_low_thresh(&mut self, canny_low_thresh: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setCannyLowThresh_int(self.as_raw_mut_GeneralizedHough(), canny_low_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Canny high threshold.
	#[inline]
	fn set_canny_high_thresh(&mut self, canny_high_thresh: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setCannyHighThresh_int(self.as_raw_mut_GeneralizedHough(), canny_high_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Minimum distance between the centers of the detected objects.
	#[inline]
	fn set_min_dist(&mut self, min_dist: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setMinDist_double(self.as_raw_mut_GeneralizedHough(), min_dist, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Inverse ratio of the accumulator resolution to the image resolution.
	#[inline]
	fn set_dp(&mut self, dp: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setDp_double(self.as_raw_mut_GeneralizedHough(), dp, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Maximal size of inner buffers.
	#[inline]
	fn set_max_buffer_size(&mut self, max_buffer_size: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHough_setMaxBufferSize_int(self.as_raw_mut_GeneralizedHough(), max_buffer_size, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// finds arbitrary template in the grayscale image using Generalized Hough Transform
pub struct GeneralizedHough {
	ptr: *mut c_void
}

opencv_type_boxed! { GeneralizedHough }

impl Drop for GeneralizedHough {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_GeneralizedHough_delete(self.as_raw_mut_GeneralizedHough()) };
	}
}

unsafe impl Send for GeneralizedHough {}

impl core::AlgorithmTraitConst for GeneralizedHough {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GeneralizedHough {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughTraitConst for GeneralizedHough {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::GeneralizedHoughTrait for GeneralizedHough {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GeneralizedHough {
}

boxed_cast_descendant! { GeneralizedHough, crate::imgproc::GeneralizedHoughBallard, cv_GeneralizedHough_to_GeneralizedHoughBallard }

boxed_cast_descendant! { GeneralizedHough, crate::imgproc::GeneralizedHoughGuil, cv_GeneralizedHough_to_GeneralizedHoughGuil }

boxed_cast_base! { GeneralizedHough, core::Algorithm, cv_GeneralizedHough_to_Algorithm }

impl std::fmt::Debug for GeneralizedHough {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GeneralizedHough")
			.finish()
	}
}

/// Constant methods for [crate::imgproc::GeneralizedHoughBallard]
pub trait GeneralizedHoughBallardTraitConst: crate::imgproc::GeneralizedHoughTraitConst {
	fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void;

	#[inline]
	fn get_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughBallard_getLevels_const(self.as_raw_GeneralizedHoughBallard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_votes_threshold(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughBallard_getVotesThreshold_const(self.as_raw_GeneralizedHoughBallard(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::GeneralizedHoughBallard]
pub trait GeneralizedHoughBallardTrait: crate::imgproc::GeneralizedHoughBallardTraitConst + crate::imgproc::GeneralizedHoughTrait {
	fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void;

	/// R-Table levels.
	#[inline]
	fn set_levels(&mut self, levels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughBallard_setLevels_int(self.as_raw_mut_GeneralizedHoughBallard(), levels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// The accumulator threshold for the template centers at the detection stage. The smaller it is, the more false positions may be detected.
	#[inline]
	fn set_votes_threshold(&mut self, votes_threshold: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughBallard_setVotesThreshold_int(self.as_raw_mut_GeneralizedHoughBallard(), votes_threshold, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// finds arbitrary template in the grayscale image using Generalized Hough Transform
/// 
/// Detects position only without translation and rotation [Ballard1981](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Ballard1981) .
pub struct GeneralizedHoughBallard {
	ptr: *mut c_void
}

opencv_type_boxed! { GeneralizedHoughBallard }

impl Drop for GeneralizedHoughBallard {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_GeneralizedHoughBallard_delete(self.as_raw_mut_GeneralizedHoughBallard()) };
	}
}

unsafe impl Send for GeneralizedHoughBallard {}

impl core::AlgorithmTraitConst for GeneralizedHoughBallard {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GeneralizedHoughBallard {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughTraitConst for GeneralizedHoughBallard {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::GeneralizedHoughTrait for GeneralizedHoughBallard {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughBallardTraitConst for GeneralizedHoughBallard {
	#[inline] fn as_raw_GeneralizedHoughBallard(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::GeneralizedHoughBallardTrait for GeneralizedHoughBallard {
	#[inline] fn as_raw_mut_GeneralizedHoughBallard(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GeneralizedHoughBallard {
}

boxed_cast_base! { GeneralizedHoughBallard, core::Algorithm, cv_GeneralizedHoughBallard_to_Algorithm }

boxed_cast_base! { GeneralizedHoughBallard, crate::imgproc::GeneralizedHough, cv_GeneralizedHoughBallard_to_GeneralizedHough }

impl std::fmt::Debug for GeneralizedHoughBallard {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GeneralizedHoughBallard")
			.finish()
	}
}

/// Constant methods for [crate::imgproc::GeneralizedHoughGuil]
pub trait GeneralizedHoughGuilTraitConst: crate::imgproc::GeneralizedHoughTraitConst {
	fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void;

	#[inline]
	fn get_xi(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getXi_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_levels(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getLevels_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_angle_epsilon(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getAngleEpsilon_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_angle(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getMinAngle_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_angle(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getMaxAngle_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_angle_step(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getAngleStep_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_angle_thresh(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getAngleThresh_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_min_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getMinScale_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_max_scale(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getMaxScale_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_scale_step(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getScaleStep_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_scale_thresh(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getScaleThresh_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_pos_thresh(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_getPosThresh_const(self.as_raw_GeneralizedHoughGuil(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::GeneralizedHoughGuil]
pub trait GeneralizedHoughGuilTrait: crate::imgproc::GeneralizedHoughGuilTraitConst + crate::imgproc::GeneralizedHoughTrait {
	fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void;

	/// Angle difference in degrees between two points in feature.
	#[inline]
	fn set_xi(&mut self, xi: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setXi_double(self.as_raw_mut_GeneralizedHoughGuil(), xi, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Feature table levels.
	#[inline]
	fn set_levels(&mut self, levels: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setLevels_int(self.as_raw_mut_GeneralizedHoughGuil(), levels, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Maximal difference between angles that treated as equal.
	#[inline]
	fn set_angle_epsilon(&mut self, angle_epsilon: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setAngleEpsilon_double(self.as_raw_mut_GeneralizedHoughGuil(), angle_epsilon, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Minimal rotation angle to detect in degrees.
	#[inline]
	fn set_min_angle(&mut self, min_angle: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setMinAngle_double(self.as_raw_mut_GeneralizedHoughGuil(), min_angle, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Maximal rotation angle to detect in degrees.
	#[inline]
	fn set_max_angle(&mut self, max_angle: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setMaxAngle_double(self.as_raw_mut_GeneralizedHoughGuil(), max_angle, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Angle step in degrees.
	#[inline]
	fn set_angle_step(&mut self, angle_step: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setAngleStep_double(self.as_raw_mut_GeneralizedHoughGuil(), angle_step, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Angle votes threshold.
	#[inline]
	fn set_angle_thresh(&mut self, angle_thresh: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setAngleThresh_int(self.as_raw_mut_GeneralizedHoughGuil(), angle_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Minimal scale to detect.
	#[inline]
	fn set_min_scale(&mut self, min_scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setMinScale_double(self.as_raw_mut_GeneralizedHoughGuil(), min_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Maximal scale to detect.
	#[inline]
	fn set_max_scale(&mut self, max_scale: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setMaxScale_double(self.as_raw_mut_GeneralizedHoughGuil(), max_scale, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Scale step.
	#[inline]
	fn set_scale_step(&mut self, scale_step: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setScaleStep_double(self.as_raw_mut_GeneralizedHoughGuil(), scale_step, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Scale votes threshold.
	#[inline]
	fn set_scale_thresh(&mut self, scale_thresh: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setScaleThresh_int(self.as_raw_mut_GeneralizedHoughGuil(), scale_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Position votes threshold.
	#[inline]
	fn set_pos_thresh(&mut self, pos_thresh: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_GeneralizedHoughGuil_setPosThresh_int(self.as_raw_mut_GeneralizedHoughGuil(), pos_thresh, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// finds arbitrary template in the grayscale image using Generalized Hough Transform
/// 
/// Detects position, translation and rotation [Guil1999](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Guil1999) .
pub struct GeneralizedHoughGuil {
	ptr: *mut c_void
}

opencv_type_boxed! { GeneralizedHoughGuil }

impl Drop for GeneralizedHoughGuil {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_GeneralizedHoughGuil_delete(self.as_raw_mut_GeneralizedHoughGuil()) };
	}
}

unsafe impl Send for GeneralizedHoughGuil {}

impl core::AlgorithmTraitConst for GeneralizedHoughGuil {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for GeneralizedHoughGuil {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughTraitConst for GeneralizedHoughGuil {
	#[inline] fn as_raw_GeneralizedHough(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::GeneralizedHoughTrait for GeneralizedHoughGuil {
	#[inline] fn as_raw_mut_GeneralizedHough(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::GeneralizedHoughGuilTraitConst for GeneralizedHoughGuil {
	#[inline] fn as_raw_GeneralizedHoughGuil(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::GeneralizedHoughGuilTrait for GeneralizedHoughGuil {
	#[inline] fn as_raw_mut_GeneralizedHoughGuil(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl GeneralizedHoughGuil {
}

boxed_cast_base! { GeneralizedHoughGuil, core::Algorithm, cv_GeneralizedHoughGuil_to_Algorithm }

boxed_cast_base! { GeneralizedHoughGuil, crate::imgproc::GeneralizedHough, cv_GeneralizedHoughGuil_to_GeneralizedHough }

impl std::fmt::Debug for GeneralizedHoughGuil {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("GeneralizedHoughGuil")
			.finish()
	}
}

/// Constant methods for [crate::imgproc::LineIterator]
pub trait LineIteratorTraitConst {
	fn as_raw_LineIterator(&self) -> *const c_void;

	#[inline]
	fn ptr(&self) -> *const u8 {
		let ret = unsafe { sys::cv_LineIterator_propPtr_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn ptr0(&self) -> *const u8 {
		let ret = unsafe { sys::cv_LineIterator_propPtr0_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn step(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propStep_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn elem_size(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propElemSize_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn err(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propErr_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn count(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propCount_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn minus_delta(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propMinusDelta_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn plus_delta(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propPlusDelta_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn minus_step(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propMinusStep_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn plus_step(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propPlusStep_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn minus_shift(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propMinusShift_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn plus_shift(&self) -> i32 {
		let ret = unsafe { sys::cv_LineIterator_propPlusShift_const(self.as_raw_LineIterator()) };
		ret
	}
	
	#[inline]
	fn p(&self) -> core::Point {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_propP_const(self.as_raw_LineIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		ret
	}
	
	#[inline]
	fn ptmode(&self) -> bool {
		let ret = unsafe { sys::cv_LineIterator_propPtmode_const(self.as_raw_LineIterator()) };
		ret
	}
	
	/// Returns coordinates of the current pixel.
	#[inline]
	fn pos(&self) -> Result<core::Point> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_pos_const(self.as_raw_LineIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::LineIterator]
pub trait LineIteratorTrait: crate::imgproc::LineIteratorTraitConst {
	fn as_raw_mut_LineIterator(&mut self) -> *mut c_void;

	#[inline]
	fn ptr_mut(&mut self) -> *mut u8 {
		let ret = unsafe { sys::cv_LineIterator_propPtr(self.as_raw_mut_LineIterator()) };
		ret
	}
	
	#[inline]
	unsafe fn set_ptr(&mut self, val: *const u8) {
		let ret = { sys::cv_LineIterator_propPtr_unsigned_charX(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propStep_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_elem_size(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propElemSize_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_err(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propErr_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_count(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propCount_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_minus_delta(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propMinusDelta_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_plus_delta(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propPlusDelta_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_minus_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propMinusStep_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_plus_step(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propPlusStep_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_minus_shift(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propMinusShift_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_plus_shift(&mut self, val: i32) {
		let ret = unsafe { sys::cv_LineIterator_propPlusShift_const_int(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn set_p(&mut self, val: core::Point) {
		let ret = unsafe { sys::cv_LineIterator_propP_const_Point(self.as_raw_mut_LineIterator(), val.opencv_as_extern()) };
		ret
	}
	
	#[inline]
	fn set_ptmode(&mut self, val: bool) {
		let ret = unsafe { sys::cv_LineIterator_propPtmode_const_bool(self.as_raw_mut_LineIterator(), val) };
		ret
	}
	
	#[inline]
	fn init(&mut self, img: &core::Mat, bounding_area_rect: core::Rect, pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_init_const_MatX_Rect_Point_Point_int_bool(self.as_raw_mut_LineIterator(), img.as_raw_Mat(), bounding_area_rect.opencv_as_extern(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), connectivity, left_to_right, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns pointer to the current pixel.
	#[inline]
	fn try_deref_mut(&mut self) -> Result<*mut u8> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_operatorX(self.as_raw_mut_LineIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Moves iterator to the next pixel on the line.
	/// 
	/// This is the prefix version (++it).
	#[inline]
	fn incr(&mut self) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_operatorAA(self.as_raw_mut_LineIterator(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

/// Class for iterating over all pixels on a raster line segment.
/// 
/// The class LineIterator is used to get each pixel of a raster line connecting
/// two specified points.
/// It can be treated as a versatile implementation of the Bresenham algorithm
/// where you can stop at each pixel and do some extra processing, for
/// example, grab pixel values along the line or draw a line with an effect
/// (for example, with XOR operation).
/// 
/// The number of pixels along the line is stored in LineIterator::count.
/// The method LineIterator::pos returns the current position in the image:
/// 
/// ```C++
/// // grabs pixels along the line (pt1, pt2)
/// // from 8-bit 3-channel image to the buffer
/// LineIterator it(img, pt1, pt2, 8);
/// LineIterator it2 = it;
/// vector<Vec3b> buf(it.count);
/// 
/// for(int i = 0; i < it.count; i++, ++it)
///    buf[i] = *(const Vec3b*)*it;
/// 
/// // alternative way of iterating through the line
/// for(int i = 0; i < it2.count; i++, ++it2)
/// {
///    Vec3b val = img.at<Vec3b>(it2.pos());
///    CV_Assert(buf[i] == val);
/// }
/// ```
/// 
pub struct LineIterator {
	ptr: *mut c_void
}

opencv_type_boxed! { LineIterator }

impl Drop for LineIterator {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LineIterator_delete(self.as_raw_mut_LineIterator()) };
	}
}

unsafe impl Send for LineIterator {}

impl crate::imgproc::LineIteratorTraitConst for LineIterator {
	#[inline] fn as_raw_LineIterator(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::LineIteratorTrait for LineIterator {
	#[inline] fn as_raw_mut_LineIterator(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LineIterator {
	/// Initializes iterator object for the given line and image.
	/// 
	/// The returned iterator can be used to traverse all pixels on a line that
	/// connects the given two points.
	/// The line will be clipped on the image boundaries.
	/// 
	/// ## Parameters
	/// * img: Underlying image.
	/// * pt1: First endpoint of the line.
	/// * pt2: The other endpoint of the line.
	/// * connectivity: Pixel connectivity of the iterator. Valid values are 4 (iterator can move
	/// up, down, left and right) and 8 (iterator can also move diagonally).
	/// * leftToRight: If true, the line is traversed from the leftmost endpoint to the rightmost
	/// endpoint. Otherwise, the line is traversed from \p pt1 to \p pt2.
	/// 
	/// ## C++ default parameters
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new(img: &core::Mat, pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_const_MatR_Point_Point_int_bool(img.as_raw_Mat(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), connectivity, left_to_right, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Initializes iterator object for the given line and image.
	/// 
	/// The returned iterator can be used to traverse all pixels on a line that
	/// connects the given two points.
	/// The line will be clipped on the image boundaries.
	/// 
	/// ## Parameters
	/// * img: Underlying image.
	/// * pt1: First endpoint of the line.
	/// * pt2: The other endpoint of the line.
	/// * connectivity: Pixel connectivity of the iterator. Valid values are 4 (iterator can move
	/// up, down, left and right) and 8 (iterator can also move diagonally).
	/// * leftToRight: If true, the line is traversed from the leftmost endpoint to the rightmost
	/// endpoint. Otherwise, the line is traversed from \p pt1 to \p pt2.
	/// 
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_def(img: &core::Mat, pt1: core::Point, pt2: core::Point) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_const_MatR_Point_Point(img.as_raw_Mat(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_1(pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_Point_Point_int_bool(pt1.opencv_as_extern(), pt2.opencv_as_extern(), connectivity, left_to_right, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_def_1(pt1: core::Point, pt2: core::Point) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_Point_Point(pt1.opencv_as_extern(), pt2.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_2(bounding_area_size: core::Size, pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_Size_Point_Point_int_bool(bounding_area_size.opencv_as_extern(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), connectivity, left_to_right, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_def_2(bounding_area_size: core::Size, pt1: core::Point, pt2: core::Point) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_Size_Point_Point(bounding_area_size.opencv_as_extern(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_3(bounding_area_rect: core::Rect, pt1: core::Point, pt2: core::Point, connectivity: i32, left_to_right: bool) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_Rect_Point_Point_int_bool(bounding_area_rect.opencv_as_extern(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), connectivity, left_to_right, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// ## Note
	/// This alternative version of [new] function uses the following default values for its arguments:
	/// * connectivity: 8
	/// * left_to_right: false
	#[inline]
	pub fn new_def_3(bounding_area_rect: core::Rect, pt1: core::Point, pt2: core::Point) -> Result<crate::imgproc::LineIterator> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineIterator_LineIterator_Rect_Point_Point(bounding_area_rect.opencv_as_extern(), pt1.opencv_as_extern(), pt2.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::LineIterator::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl std::fmt::Debug for LineIterator {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineIterator")
			.field("ptr", &crate::imgproc::LineIteratorTraitConst::ptr(self))
			.field("ptr0", &crate::imgproc::LineIteratorTraitConst::ptr0(self))
			.field("step", &crate::imgproc::LineIteratorTraitConst::step(self))
			.field("elem_size", &crate::imgproc::LineIteratorTraitConst::elem_size(self))
			.field("err", &crate::imgproc::LineIteratorTraitConst::err(self))
			.field("count", &crate::imgproc::LineIteratorTraitConst::count(self))
			.field("minus_delta", &crate::imgproc::LineIteratorTraitConst::minus_delta(self))
			.field("plus_delta", &crate::imgproc::LineIteratorTraitConst::plus_delta(self))
			.field("minus_step", &crate::imgproc::LineIteratorTraitConst::minus_step(self))
			.field("plus_step", &crate::imgproc::LineIteratorTraitConst::plus_step(self))
			.field("minus_shift", &crate::imgproc::LineIteratorTraitConst::minus_shift(self))
			.field("plus_shift", &crate::imgproc::LineIteratorTraitConst::plus_shift(self))
			.field("p", &crate::imgproc::LineIteratorTraitConst::p(self))
			.field("ptmode", &crate::imgproc::LineIteratorTraitConst::ptmode(self))
			.finish()
	}
}

/// Constant methods for [crate::imgproc::LineSegmentDetector]
pub trait LineSegmentDetectorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_LineSegmentDetector(&self) -> *const c_void;

}

/// Mutable methods for [crate::imgproc::LineSegmentDetector]
pub trait LineSegmentDetectorTrait: core::AlgorithmTrait + crate::imgproc::LineSegmentDetectorTraitConst {
	fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void;

	/// Finds lines in the input image.
	/// 
	/// This is the output of the default parameters of the algorithm on the above shown image.
	/// 
	/// ![image](https://docs.opencv.org/4.12.0/building_lsd.png)
	/// 
	/// ## Parameters
	/// * image: A grayscale (CV_8UC1) input image. If only a roi needs to be selected, use:
	/// `lsd_ptr-\>detect(image(roi), lines, ...); lines += Scalar(roi.x, roi.y, roi.x, roi.y);`
	/// * lines: A vector of Vec4f elements specifying the beginning and ending point of a line. Where
	/// Vec4f is (x1, y1, x2, y2), point 1 is the start, point 2 - end. Returned lines are strictly
	/// oriented depending on the gradient.
	/// * width: Vector of widths of the regions, where the lines are found. E.g. Width of line.
	/// * prec: Vector of precisions with which the lines are found.
	/// * nfa: Vector containing number of false alarms in the line region, with precision of 10%. The
	/// bigger the value, logarithmically better the detection.
	/// - -1 corresponds to 10 mean false alarms
	/// - 0 corresponds to 1 mean false alarm
	/// - 1 corresponds to 0.1 mean false alarms
	/// This vector will be calculated only when the objects type is #LSD_REFINE_ADV.
	/// 
	/// ## C++ default parameters
	/// * width: noArray()
	/// * prec: noArray()
	/// * nfa: noArray()
	#[inline]
	fn detect(&mut self, image: &impl core::ToInputArray, lines: &mut impl core::ToOutputArray, width: &mut impl core::ToOutputArray, prec: &mut impl core::ToOutputArray, nfa: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(lines);
		output_array_arg!(width);
		output_array_arg!(prec);
		output_array_arg!(nfa);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR(self.as_raw_mut_LineSegmentDetector(), image.as_raw__InputArray(), lines.as_raw__OutputArray(), width.as_raw__OutputArray(), prec.as_raw__OutputArray(), nfa.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds lines in the input image.
	/// 
	/// This is the output of the default parameters of the algorithm on the above shown image.
	/// 
	/// ![image](https://docs.opencv.org/4.12.0/building_lsd.png)
	/// 
	/// ## Parameters
	/// * image: A grayscale (CV_8UC1) input image. If only a roi needs to be selected, use:
	/// `lsd_ptr-\>detect(image(roi), lines, ...); lines += Scalar(roi.x, roi.y, roi.x, roi.y);`
	/// * lines: A vector of Vec4f elements specifying the beginning and ending point of a line. Where
	/// Vec4f is (x1, y1, x2, y2), point 1 is the start, point 2 - end. Returned lines are strictly
	/// oriented depending on the gradient.
	/// * width: Vector of widths of the regions, where the lines are found. E.g. Width of line.
	/// * prec: Vector of precisions with which the lines are found.
	/// * nfa: Vector containing number of false alarms in the line region, with precision of 10%. The
	/// bigger the value, logarithmically better the detection.
	/// - -1 corresponds to 10 mean false alarms
	/// - 0 corresponds to 1 mean false alarm
	/// - 1 corresponds to 0.1 mean false alarms
	/// This vector will be calculated only when the objects type is #LSD_REFINE_ADV.
	/// 
	/// ## Note
	/// This alternative version of [LineSegmentDetectorTrait::detect] function uses the following default values for its arguments:
	/// * width: noArray()
	/// * prec: noArray()
	/// * nfa: noArray()
	#[inline]
	fn detect_def(&mut self, image: &impl core::ToInputArray, lines: &mut impl core::ToOutputArray) -> Result<()> {
		input_array_arg!(image);
		output_array_arg!(lines);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineSegmentDetector_detect_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_LineSegmentDetector(), image.as_raw__InputArray(), lines.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draws the line segments on a given image.
	/// ## Parameters
	/// * image: The image, where the lines will be drawn. Should be bigger or equal to the image,
	/// where the lines were found.
	/// * lines: A vector of the lines that needed to be drawn.
	#[inline]
	fn draw_segments(&mut self, image: &mut impl core::ToInputOutputArray, lines: &impl core::ToInputArray) -> Result<()> {
		input_output_array_arg!(image);
		input_array_arg!(lines);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineSegmentDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR(self.as_raw_mut_LineSegmentDetector(), image.as_raw__InputOutputArray(), lines.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draws two groups of lines in blue and red, counting the non overlapping (mismatching) pixels.
	/// 
	/// ## Parameters
	/// * size: The size of the image, where lines1 and lines2 were found.
	/// * lines1: The first group of lines that needs to be drawn. It is visualized in blue color.
	/// * lines2: The second group of lines. They visualized in red color.
	/// * image: Optional image, where the lines will be drawn. The image should be color(3-channel)
	/// in order for lines1 and lines2 to be drawn in the above mentioned colors.
	/// 
	/// ## C++ default parameters
	/// * image: noArray()
	#[inline]
	fn compare_segments(&mut self, size: core::Size, lines1: &impl core::ToInputArray, lines2: &impl core::ToInputArray, image: &mut impl core::ToInputOutputArray) -> Result<i32> {
		input_array_arg!(lines1);
		input_array_arg!(lines2);
		input_output_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR_const__InputOutputArrayR(self.as_raw_mut_LineSegmentDetector(), &size, lines1.as_raw__InputArray(), lines2.as_raw__InputArray(), image.as_raw__InputOutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Draws two groups of lines in blue and red, counting the non overlapping (mismatching) pixels.
	/// 
	/// ## Parameters
	/// * size: The size of the image, where lines1 and lines2 were found.
	/// * lines1: The first group of lines that needs to be drawn. It is visualized in blue color.
	/// * lines2: The second group of lines. They visualized in red color.
	/// * image: Optional image, where the lines will be drawn. The image should be color(3-channel)
	/// in order for lines1 and lines2 to be drawn in the above mentioned colors.
	/// 
	/// ## Note
	/// This alternative version of [LineSegmentDetectorTrait::compare_segments] function uses the following default values for its arguments:
	/// * image: noArray()
	#[inline]
	fn compare_segments_def(&mut self, size: core::Size, lines1: &impl core::ToInputArray, lines2: &impl core::ToInputArray) -> Result<i32> {
		input_array_arg!(lines1);
		input_array_arg!(lines2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_LineSegmentDetector_compareSegments_const_SizeR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_LineSegmentDetector(), &size, lines1.as_raw__InputArray(), lines2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Line segment detector class
/// 
/// following the algorithm described at [Rafael12](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Rafael12) .
/// 
/// 
/// Note: Implementation has been removed from OpenCV version 3.4.6 to 3.4.15 and version 4.1.0 to 4.5.3 due original code license conflict.
/// restored again after [Computation of a NFA](https://github.com/rafael-grompone-von-gioi/binomial_nfa) code published under the MIT license.
pub struct LineSegmentDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { LineSegmentDetector }

impl Drop for LineSegmentDetector {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_LineSegmentDetector_delete(self.as_raw_mut_LineSegmentDetector()) };
	}
}

unsafe impl Send for LineSegmentDetector {}

impl core::AlgorithmTraitConst for LineSegmentDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LineSegmentDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::imgproc::LineSegmentDetectorTraitConst for LineSegmentDetector {
	#[inline] fn as_raw_LineSegmentDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::LineSegmentDetectorTrait for LineSegmentDetector {
	#[inline] fn as_raw_mut_LineSegmentDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LineSegmentDetector {
}

boxed_cast_base! { LineSegmentDetector, core::Algorithm, cv_LineSegmentDetector_to_Algorithm }

impl std::fmt::Debug for LineSegmentDetector {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("LineSegmentDetector")
			.finish()
	}
}

/// Constant methods for [crate::imgproc::Subdiv2D]
pub trait Subdiv2DTraitConst {
	fn as_raw_Subdiv2D(&self) -> *const c_void;

	/// Returns a list of all edges.
	/// 
	/// ## Parameters
	/// * edgeList: Output vector.
	/// 
	/// The function gives each edge as a 4 numbers vector, where each two are one of the edge
	/// vertices. i.e. org_x = v[0], org_y = v[1], dst_x = v[2], dst_y = v[3].
	#[inline]
	fn get_edge_list(&self, edge_list: &mut core::Vector<core::Vec4f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getEdgeList_const_vectorLVec4fGR(self.as_raw_Subdiv2D(), edge_list.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns a list of the leading edge ID connected to each triangle.
	/// 
	/// ## Parameters
	/// * leadingEdgeList: Output vector.
	/// 
	/// The function gives one edge ID for each triangle.
	#[inline]
	fn get_leading_edge_list(&self, leading_edge_list: &mut core::Vector<i32>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getLeadingEdgeList_const_vectorLintGR(self.as_raw_Subdiv2D(), leading_edge_list.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns a list of all triangles.
	/// 
	/// ## Parameters
	/// * triangleList: Output vector.
	/// 
	/// The function gives each triangle as a 6 numbers vector, where each two are one of the triangle
	/// vertices. i.e. p1_x = v[0], p1_y = v[1], p2_x = v[2], p2_y = v[3], p3_x = v[4], p3_y = v[5].
	#[inline]
	fn get_triangle_list(&self, triangle_list: &mut core::Vector<core::Vec6f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getTriangleList_const_vectorLVec6fGR(self.as_raw_Subdiv2D(), triangle_list.as_raw_mut_VectorOfVec6f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns vertex location from vertex ID.
	/// 
	/// ## Parameters
	/// * vertex: vertex ID.
	/// * firstEdge: Optional. The first edge ID which is connected to the vertex.
	/// ## Returns
	/// vertex (x,y)
	/// 
	/// ## C++ default parameters
	/// * first_edge: 0
	#[inline]
	fn get_vertex(&self, vertex: i32, first_edge: &mut i32) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getVertex_const_int_intX(self.as_raw_Subdiv2D(), vertex, first_edge, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns vertex location from vertex ID.
	/// 
	/// ## Parameters
	/// * vertex: vertex ID.
	/// * firstEdge: Optional. The first edge ID which is connected to the vertex.
	/// ## Returns
	/// vertex (x,y)
	/// 
	/// ## Note
	/// This alternative version of [Subdiv2DTraitConst::get_vertex] function uses the following default values for its arguments:
	/// * first_edge: 0
	#[inline]
	fn get_vertex_def(&self, vertex: i32) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getVertex_const_int(self.as_raw_Subdiv2D(), vertex, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns one of the edges related to the given edge.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// * nextEdgeType: Parameter specifying which of the related edges to return.
	/// The following values are possible:
	/// *   NEXT_AROUND_ORG next around the edge origin ( eOnext on the picture below if e is the input edge)
	/// *   NEXT_AROUND_DST next around the edge vertex ( eDnext )
	/// *   PREV_AROUND_ORG previous around the edge origin (reversed eRnext )
	/// *   PREV_AROUND_DST previous around the edge destination (reversed eLnext )
	/// *   NEXT_AROUND_LEFT next around the left facet ( eLnext )
	/// *   NEXT_AROUND_RIGHT next around the right facet ( eRnext )
	/// *   PREV_AROUND_LEFT previous around the left facet (reversed eOnext )
	/// *   PREV_AROUND_RIGHT previous around the right facet (reversed eDnext )
	/// 
	/// ![sample output](https://docs.opencv.org/4.12.0/quadedge.png)
	/// 
	/// ## Returns
	/// edge ID related to the input edge.
	#[inline]
	fn get_edge(&self, edge: i32, next_edge_type: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getEdge_const_int_int(self.as_raw_Subdiv2D(), edge, next_edge_type, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns next edge around the edge origin.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// 
	/// ## Returns
	/// an integer which is next edge ID around the edge origin: eOnext on the
	/// picture above if e is the input edge).
	#[inline]
	fn next_edge(&self, edge: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_nextEdge_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns another edge of the same quad-edge.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// * rotate: Parameter specifying which of the edges of the same quad-edge as the input
	/// one to return. The following values are possible:
	/// *   0 - the input edge ( e on the picture below if e is the input edge)
	/// *   1 - the rotated edge ( eRot )
	/// *   2 - the reversed edge (reversed e (in green))
	/// *   3 - the reversed rotated edge (reversed eRot (in green))
	/// 
	/// ## Returns
	/// one of the edges ID of the same quad-edge as the input edge.
	#[inline]
	fn rotate_edge(&self, edge: i32, rotate: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_rotateEdge_const_int_int(self.as_raw_Subdiv2D(), edge, rotate, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn sym_edge(&self, edge: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_symEdge_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the edge origin.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// * orgpt: Output vertex location.
	/// 
	/// ## Returns
	/// vertex ID.
	/// 
	/// ## C++ default parameters
	/// * orgpt: 0
	#[inline]
	fn edge_org(&self, edge: i32, orgpt: &mut core::Point2f) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_edgeOrg_const_int_Point2fX(self.as_raw_Subdiv2D(), edge, orgpt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the edge origin.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// * orgpt: Output vertex location.
	/// 
	/// ## Returns
	/// vertex ID.
	/// 
	/// ## Note
	/// This alternative version of [Subdiv2DTraitConst::edge_org] function uses the following default values for its arguments:
	/// * orgpt: 0
	#[inline]
	fn edge_org_def(&self, edge: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_edgeOrg_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the edge destination.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// * dstpt: Output vertex location.
	/// 
	/// ## Returns
	/// vertex ID.
	/// 
	/// ## C++ default parameters
	/// * dstpt: 0
	#[inline]
	fn edge_dst(&self, edge: i32, dstpt: &mut core::Point2f) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_edgeDst_const_int_Point2fX(self.as_raw_Subdiv2D(), edge, dstpt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the edge destination.
	/// 
	/// ## Parameters
	/// * edge: Subdivision edge ID.
	/// * dstpt: Output vertex location.
	/// 
	/// ## Returns
	/// vertex ID.
	/// 
	/// ## Note
	/// This alternative version of [Subdiv2DTraitConst::edge_dst] function uses the following default values for its arguments:
	/// * dstpt: 0
	#[inline]
	fn edge_dst_def(&self, edge: i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_edgeDst_const_int(self.as_raw_Subdiv2D(), edge, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::Subdiv2D]
pub trait Subdiv2DTrait: crate::imgproc::Subdiv2DTraitConst {
	fn as_raw_mut_Subdiv2D(&mut self) -> *mut c_void;

	/// Creates a new empty Delaunay subdivision
	/// 
	/// ## Parameters
	/// * rect: Rectangle that includes all of the 2D points that are to be added to the subdivision.
	#[inline]
	fn init_delaunay(&mut self, rect: core::Rect) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_initDelaunay_Rect(self.as_raw_mut_Subdiv2D(), rect.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Insert a single point into a Delaunay triangulation.
	/// 
	/// ## Parameters
	/// * pt: Point to insert.
	/// 
	/// The function inserts a single point into a subdivision and modifies the subdivision topology
	/// appropriately. If a point with the same coordinates exists already, no new point is added.
	/// ## Returns
	/// the ID of the point.
	/// 
	/// 
	/// Note: If the point is outside of the triangulation specified rect a runtime error is raised.
	#[inline]
	fn insert(&mut self, pt: core::Point2f) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_insert_Point2f(self.as_raw_mut_Subdiv2D(), pt.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Insert multiple points into a Delaunay triangulation.
	/// 
	/// ## Parameters
	/// * ptvec: Points to insert.
	/// 
	/// The function inserts a vector of points into a subdivision and modifies the subdivision topology
	/// appropriately.
	#[inline]
	fn insert_multiple(&mut self, ptvec: &core::Vector<core::Point2f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_insert_const_vectorLPoint2fGR(self.as_raw_mut_Subdiv2D(), ptvec.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns the location of a point within a Delaunay triangulation.
	/// 
	/// ## Parameters
	/// * pt: Point to locate.
	/// * edge: Output edge that the point belongs to or is located to the right of it.
	/// * vertex: Optional output vertex the input point coincides with.
	/// 
	/// The function locates the input point within the subdivision and gives one of the triangle edges
	/// or vertices.
	/// 
	/// ## Returns
	/// an integer which specify one of the following five cases for point location:
	/// *  The point falls into some facet. The function returns [PTLOC_INSIDE] and edge will contain one of
	///    edges of the facet.
	/// *  The point falls onto the edge. The function returns [PTLOC_ON_EDGE] and edge will contain this edge.
	/// *  The point coincides with one of the subdivision vertices. The function returns [PTLOC_VERTEX] and
	///    vertex will contain a pointer to the vertex.
	/// *  The point is outside the subdivision reference rectangle. The function returns [PTLOC_OUTSIDE_RECT]
	///    and no pointers are filled.
	/// *  One of input arguments is invalid. A runtime error is raised or, if silent or "parent" error
	///    processing mode is selected, [PTLOC_ERROR] is returned.
	#[inline]
	fn locate(&mut self, pt: core::Point2f, edge: &mut i32, vertex: &mut i32) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_locate_Point2f_intR_intR(self.as_raw_mut_Subdiv2D(), pt.opencv_as_extern(), edge, vertex, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds the subdivision vertex closest to the given point.
	/// 
	/// ## Parameters
	/// * pt: Input point.
	/// * nearestPt: Output subdivision vertex point.
	/// 
	/// The function is another function that locates the input point within the subdivision. It finds the
	/// subdivision vertex that is the closest to the input point. It is not necessarily one of vertices
	/// of the facet containing the input point, though the facet (located using locate() ) is used as a
	/// starting point.
	/// 
	/// ## Returns
	/// vertex ID.
	/// 
	/// ## C++ default parameters
	/// * nearest_pt: 0
	#[inline]
	fn find_nearest(&mut self, pt: core::Point2f, nearest_pt: &mut core::Point2f) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_findNearest_Point2f_Point2fX(self.as_raw_mut_Subdiv2D(), pt.opencv_as_extern(), nearest_pt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Finds the subdivision vertex closest to the given point.
	/// 
	/// ## Parameters
	/// * pt: Input point.
	/// * nearestPt: Output subdivision vertex point.
	/// 
	/// The function is another function that locates the input point within the subdivision. It finds the
	/// subdivision vertex that is the closest to the input point. It is not necessarily one of vertices
	/// of the facet containing the input point, though the facet (located using locate() ) is used as a
	/// starting point.
	/// 
	/// ## Returns
	/// vertex ID.
	/// 
	/// ## Note
	/// This alternative version of [Subdiv2DTrait::find_nearest] function uses the following default values for its arguments:
	/// * nearest_pt: 0
	#[inline]
	fn find_nearest_def(&mut self, pt: core::Point2f) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_findNearest_Point2f(self.as_raw_mut_Subdiv2D(), pt.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Returns a list of all Voronoi facets.
	/// 
	/// ## Parameters
	/// * idx: Vector of vertices IDs to consider. For all vertices you can pass empty vector.
	/// * facetList: Output vector of the Voronoi facets.
	/// * facetCenters: Output vector of the Voronoi facets center points.
	#[inline]
	fn get_voronoi_facet_list(&mut self, idx: &core::Vector<i32>, facet_list: &mut core::Vector<core::Vector<core::Point2f>>, facet_centers: &mut core::Vector<core::Point2f>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_getVoronoiFacetList_const_vectorLintGR_vectorLvectorLPoint2fGGR_vectorLPoint2fGR(self.as_raw_mut_Subdiv2D(), idx.as_raw_VectorOfi32(), facet_list.as_raw_mut_VectorOfVectorOfPoint2f(), facet_centers.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct Subdiv2D {
	ptr: *mut c_void
}

opencv_type_boxed! { Subdiv2D }

impl Drop for Subdiv2D {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_Subdiv2D_delete(self.as_raw_mut_Subdiv2D()) };
	}
}

unsafe impl Send for Subdiv2D {}

impl crate::imgproc::Subdiv2DTraitConst for Subdiv2D {
	#[inline] fn as_raw_Subdiv2D(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::Subdiv2DTrait for Subdiv2D {
	#[inline] fn as_raw_mut_Subdiv2D(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl Subdiv2D {
	/// creates an empty Subdiv2D object.
	/// To create a new empty Delaunay subdivision you need to use the [init_delaunay] function.
	#[inline]
	pub fn default() -> Result<crate::imgproc::Subdiv2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_Subdiv2D(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::Subdiv2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// creates an empty Subdiv2D object.
	/// To create a new empty Delaunay subdivision you need to use the [init_delaunay] function.
	/// 
	/// ## Overloaded parameters
	/// 
	/// 
	/// ## Parameters
	/// * rect: Rectangle that includes all of the 2D points that are to be added to the subdivision.
	/// 
	/// The function creates an empty Delaunay subdivision where 2D points can be added using the function
	/// insert() . All of the points to be added must be within the specified rectangle, otherwise a runtime
	/// error is raised.
	#[inline]
	pub fn new(rect: core::Rect) -> Result<crate::imgproc::Subdiv2D> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_Subdiv2D_Subdiv2D_Rect(rect.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::Subdiv2D::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl std::fmt::Debug for Subdiv2D {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("Subdiv2D")
			.finish()
	}
}

/// Constant methods for [crate::imgproc::IntelligentScissorsMB]
pub trait IntelligentScissorsMBTraitConst {
	fn as_raw_IntelligentScissorsMB(&self) -> *const c_void;

	/// Extracts optimal contour for the given target point on the image
	/// 
	/// 
	/// Note: buildMap() must be called before this call
	/// 
	/// ## Parameters
	/// * targetPt: The target point
	/// * contour:[out] The list of pixels which contains optimal path between the source and the target points of the image. Type is CV_32SC2 (compatible with `std::vector<Point>`)
	/// * backward: Flag to indicate reverse order of retrieved pixels (use "true" value to fetch points from the target to the source point)
	/// 
	/// ## C++ default parameters
	/// * backward: false
	#[inline]
	fn get_contour(&self, target_pt: core::Point, contour: &mut impl core::ToOutputArray, backward: bool) -> Result<()> {
		output_array_arg!(contour);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR_bool(self.as_raw_IntelligentScissorsMB(), &target_pt, contour.as_raw__OutputArray(), backward, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// Extracts optimal contour for the given target point on the image
	/// 
	/// 
	/// Note: buildMap() must be called before this call
	/// 
	/// ## Parameters
	/// * targetPt: The target point
	/// * contour:[out] The list of pixels which contains optimal path between the source and the target points of the image. Type is CV_32SC2 (compatible with `std::vector<Point>`)
	/// * backward: Flag to indicate reverse order of retrieved pixels (use "true" value to fetch points from the target to the source point)
	/// 
	/// ## Note
	/// This alternative version of [IntelligentScissorsMBTraitConst::get_contour] function uses the following default values for its arguments:
	/// * backward: false
	#[inline]
	fn get_contour_def(&self, target_pt: core::Point, contour: &mut impl core::ToOutputArray) -> Result<()> {
		output_array_arg!(contour);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_getContour_const_const_PointR_const__OutputArrayR(self.as_raw_IntelligentScissorsMB(), &target_pt, contour.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Mutable methods for [crate::imgproc::IntelligentScissorsMB]
pub trait IntelligentScissorsMBTrait: crate::imgproc::IntelligentScissorsMBTraitConst {
	fn as_raw_mut_IntelligentScissorsMB(&mut self) -> *mut c_void;

	/// Specify weights of feature functions
	/// 
	/// Consider keeping weights normalized (sum of weights equals to 1.0)
	/// Discrete dynamic programming (DP) goal is minimization of costs between pixels.
	/// 
	/// ## Parameters
	/// * weight_non_edge: Specify cost of non-edge pixels (default: 0.43f)
	/// * weight_gradient_direction: Specify cost of gradient direction function (default: 0.43f)
	/// * weight_gradient_magnitude: Specify cost of gradient magnitude function (default: 0.14f)
	#[inline]
	fn set_weights(&mut self, weight_non_edge: f32, weight_gradient_direction: f32, weight_gradient_magnitude: f32) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setWeights_float_float_float(self.as_raw_mut_IntelligentScissorsMB(), weight_non_edge, weight_gradient_direction, weight_gradient_magnitude, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Specify gradient magnitude max value threshold
	/// 
	/// Zero limit value is used to disable gradient magnitude thresholding (default behavior, as described in original article).
	/// Otherwize pixels with `gradient magnitude >= threshold` have zero cost.
	/// 
	/// 
	/// Note: Thresholding should be used for images with irregular regions (to avoid stuck on parameters from high-contract areas, like embedded logos).
	/// 
	/// ## Parameters
	/// * gradient_magnitude_threshold_max: Specify gradient magnitude max value threshold (default: 0, disabled)
	/// 
	/// ## C++ default parameters
	/// * gradient_magnitude_threshold_max: 0.0f
	#[inline]
	fn set_gradient_magnitude_max_limit(&mut self, gradient_magnitude_threshold_max: f32) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit_float(self.as_raw_mut_IntelligentScissorsMB(), gradient_magnitude_threshold_max, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Specify gradient magnitude max value threshold
	/// 
	/// Zero limit value is used to disable gradient magnitude thresholding (default behavior, as described in original article).
	/// Otherwize pixels with `gradient magnitude >= threshold` have zero cost.
	/// 
	/// 
	/// Note: Thresholding should be used for images with irregular regions (to avoid stuck on parameters from high-contract areas, like embedded logos).
	/// 
	/// ## Parameters
	/// * gradient_magnitude_threshold_max: Specify gradient magnitude max value threshold (default: 0, disabled)
	/// 
	/// ## Note
	/// This alternative version of [IntelligentScissorsMBTrait::set_gradient_magnitude_max_limit] function uses the following default values for its arguments:
	/// * gradient_magnitude_threshold_max: 0.0f
	#[inline]
	fn set_gradient_magnitude_max_limit_def(&mut self) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setGradientMagnitudeMaxLimit(self.as_raw_mut_IntelligentScissorsMB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Switch to "Laplacian Zero-Crossing" edge feature extractor and specify its parameters
	/// 
	/// This feature extractor is used by default according to article.
	/// 
	/// Implementation has additional filtering for regions with low-amplitude noise.
	/// This filtering is enabled through parameter of minimal gradient amplitude (use some small value 4, 8, 16).
	/// 
	/// 
	/// Note: Current implementation of this feature extractor is based on processing of grayscale images (color image is converted to grayscale image first).
	/// 
	/// 
	/// Note: Canny edge detector is a bit slower, but provides better results (especially on color images): use setEdgeFeatureCannyParameters().
	/// 
	/// ## Parameters
	/// * gradient_magnitude_min_value: Minimal gradient magnitude value for edge pixels (default: 0, check is disabled)
	/// 
	/// ## C++ default parameters
	/// * gradient_magnitude_min_value: 0.0f
	#[inline]
	fn set_edge_feature_zero_crossing_parameters(&mut self, gradient_magnitude_min_value: f32) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters_float(self.as_raw_mut_IntelligentScissorsMB(), gradient_magnitude_min_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Switch to "Laplacian Zero-Crossing" edge feature extractor and specify its parameters
	/// 
	/// This feature extractor is used by default according to article.
	/// 
	/// Implementation has additional filtering for regions with low-amplitude noise.
	/// This filtering is enabled through parameter of minimal gradient amplitude (use some small value 4, 8, 16).
	/// 
	/// 
	/// Note: Current implementation of this feature extractor is based on processing of grayscale images (color image is converted to grayscale image first).
	/// 
	/// 
	/// Note: Canny edge detector is a bit slower, but provides better results (especially on color images): use setEdgeFeatureCannyParameters().
	/// 
	/// ## Parameters
	/// * gradient_magnitude_min_value: Minimal gradient magnitude value for edge pixels (default: 0, check is disabled)
	/// 
	/// ## Note
	/// This alternative version of [IntelligentScissorsMBTrait::set_edge_feature_zero_crossing_parameters] function uses the following default values for its arguments:
	/// * gradient_magnitude_min_value: 0.0f
	#[inline]
	fn set_edge_feature_zero_crossing_parameters_def(&mut self) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setEdgeFeatureZeroCrossingParameters(self.as_raw_mut_IntelligentScissorsMB(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Switch edge feature extractor to use Canny edge detector
	/// 
	/// 
	/// Note: "Laplacian Zero-Crossing" feature extractor is used by default (following to original article)
	/// ## See also
	/// Canny
	/// 
	/// ## C++ default parameters
	/// * aperture_size: 3
	/// * l2gradient: false
	#[inline]
	fn set_edge_feature_canny_parameters(&mut self, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double_int_bool(self.as_raw_mut_IntelligentScissorsMB(), threshold1, threshold2, aperture_size, l2gradient, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Switch edge feature extractor to use Canny edge detector
	/// 
	/// 
	/// Note: "Laplacian Zero-Crossing" feature extractor is used by default (following to original article)
	/// ## See also
	/// Canny
	/// 
	/// ## Note
	/// This alternative version of [IntelligentScissorsMBTrait::set_edge_feature_canny_parameters] function uses the following default values for its arguments:
	/// * aperture_size: 3
	/// * l2gradient: false
	#[inline]
	fn set_edge_feature_canny_parameters_def(&mut self, threshold1: f64, threshold2: f64) -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_setEdgeFeatureCannyParameters_double_double(self.as_raw_mut_IntelligentScissorsMB(), threshold1, threshold2, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Specify input image and extract image features
	/// 
	/// ## Parameters
	/// * image: input image. Type is [CV_8UC1] / #CV_8UC3
	#[inline]
	fn apply_image(&mut self, image: &impl core::ToInputArray) -> Result<crate::imgproc::IntelligentScissorsMB> {
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_applyImage_const__InputArrayR(self.as_raw_mut_IntelligentScissorsMB(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Specify custom features of input image
	/// 
	/// Customized advanced variant of applyImage() call.
	/// 
	/// ## Parameters
	/// * non_edge: Specify cost of non-edge pixels. Type is CV_8UC1. Expected values are `{0, 1}`.
	/// * gradient_direction: Specify gradient direction feature. Type is CV_32FC2. Values are expected to be normalized: `x^2 + y^2 == 1`
	/// * gradient_magnitude: Specify cost of gradient magnitude function: Type is CV_32FC1. Values should be in range `[0, 1]`.
	/// * image: **Optional parameter**. Must be specified if subset of features is specified (non-specified features are calculated internally)
	/// 
	/// ## C++ default parameters
	/// * image: noArray()
	#[inline]
	fn apply_image_features(&mut self, non_edge: &impl core::ToInputArray, gradient_direction: &impl core::ToInputArray, gradient_magnitude: &impl core::ToInputArray, image: &impl core::ToInputArray) -> Result<crate::imgproc::IntelligentScissorsMB> {
		input_array_arg!(non_edge);
		input_array_arg!(gradient_direction);
		input_array_arg!(gradient_magnitude);
		input_array_arg!(image);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_IntelligentScissorsMB(), non_edge.as_raw__InputArray(), gradient_direction.as_raw__InputArray(), gradient_magnitude.as_raw__InputArray(), image.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Specify custom features of input image
	/// 
	/// Customized advanced variant of applyImage() call.
	/// 
	/// ## Parameters
	/// * non_edge: Specify cost of non-edge pixels. Type is CV_8UC1. Expected values are `{0, 1}`.
	/// * gradient_direction: Specify gradient direction feature. Type is CV_32FC2. Values are expected to be normalized: `x^2 + y^2 == 1`
	/// * gradient_magnitude: Specify cost of gradient magnitude function: Type is CV_32FC1. Values should be in range `[0, 1]`.
	/// * image: **Optional parameter**. Must be specified if subset of features is specified (non-specified features are calculated internally)
	/// 
	/// ## Note
	/// This alternative version of [IntelligentScissorsMBTrait::apply_image_features] function uses the following default values for its arguments:
	/// * image: noArray()
	#[inline]
	fn apply_image_features_def(&mut self, non_edge: &impl core::ToInputArray, gradient_direction: &impl core::ToInputArray, gradient_magnitude: &impl core::ToInputArray) -> Result<crate::imgproc::IntelligentScissorsMB> {
		input_array_arg!(non_edge);
		input_array_arg!(gradient_direction);
		input_array_arg!(gradient_magnitude);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_applyImageFeatures_const__InputArrayR_const__InputArrayR_const__InputArrayR(self.as_raw_mut_IntelligentScissorsMB(), non_edge.as_raw__InputArray(), gradient_direction.as_raw__InputArray(), gradient_magnitude.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	/// Prepares a map of optimal paths for the given source point on the image
	/// 
	/// 
	/// Note: applyImage() / applyImageFeatures() must be called before this call
	/// 
	/// ## Parameters
	/// * sourcePt: The source point used to find the paths
	#[inline]
	fn build_map(&mut self, source_pt: core::Point) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_buildMap_const_PointR(self.as_raw_mut_IntelligentScissorsMB(), &source_pt, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

/// Intelligent Scissors image segmentation
/// 
/// This class is used to find the path (contour) between two points
/// which can be used for image segmentation.
/// 
/// Usage example:
/// [usage_example_intelligent_scissors](https://github.com/opencv/opencv/blob/4.12.0/samples/cpp/tutorial_code/snippets/imgproc_segmentation.cpp#L1)
/// 
/// Reference: <a href="http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.138.3811&rep=rep1&type=pdf">"Intelligent Scissors for Image Composition"</a>
/// algorithm designed by Eric N. Mortensen and William A. Barrett, Brigham Young University
/// [Mortensen95intelligentscissors](https://docs.opencv.org/4.12.0/d0/de3/citelist.html#CITEREF_Mortensen95intelligentscissors)
pub struct IntelligentScissorsMB {
	ptr: *mut c_void
}

opencv_type_boxed! { IntelligentScissorsMB }

impl Drop for IntelligentScissorsMB {
	#[inline]
	fn drop(&mut self) {
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_delete(self.as_raw_mut_IntelligentScissorsMB()) };
	}
}

unsafe impl Send for IntelligentScissorsMB {}

impl crate::imgproc::IntelligentScissorsMBTraitConst for IntelligentScissorsMB {
	#[inline] fn as_raw_IntelligentScissorsMB(&self) -> *const c_void { self.as_raw() }
}

impl crate::imgproc::IntelligentScissorsMBTrait for IntelligentScissorsMB {
	#[inline] fn as_raw_mut_IntelligentScissorsMB(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl IntelligentScissorsMB {
	#[inline]
	pub fn default() -> Result<crate::imgproc::IntelligentScissorsMB> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_segmentation_IntelligentScissorsMB_IntelligentScissorsMB(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::imgproc::IntelligentScissorsMB::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

impl Clone for IntelligentScissorsMB {
	#[inline]
	fn clone(&self) -> Self {
		unsafe { Self::from_raw(sys::cv_segmentation_IntelligentScissorsMB_implicitClone_const(self.as_raw_IntelligentScissorsMB())) }
	}
}

impl std::fmt::Debug for IntelligentScissorsMB {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("IntelligentScissorsMB")
			.finish()
	}
}
