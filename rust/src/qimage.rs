use cxx::{ExternType, type_id};
use std::fmt;
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type TransformationMode = cxx_qt_lib::TransformationMode;
        type AspectRatioMode = cxx_qt_lib::AspectRatioMode;
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_9))]
        type Orientations = cxx_qt_lib::Orientations;
    }

    /// This enum type is used to describe how pixel values should be inverted in the [`QImage::invert_pixels`] function.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QImageInvertMode {
        /// Invert only the RGB values and leave the alpha channel unchanged.
        InvertRgb,
        /// Invert all channels, including the alpha channel.
        InvertRgba,
    }

    /// The type of image format available in Qt.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QImageFormat {
        /// The image is invalid.
        Format_Invalid,
        /// The image is stored using 1-bit per pixel. Bytes are packed with the most significant bit (MSB) first.
        Format_Mono,
        /// The image is stored using 1-bit per pixel. Bytes are packed with the less significant bit (LSB) first.
        Format_MonoLSB,
        /// The image is stored using 8-bit indexes into a colormap.
        Format_Indexed8,
        /// The image is stored using a 32-bit RGB format (0xffRRGGBB).
        Format_RGB32,
        /// The image is stored using a 32-bit ARGB format (0xAARRGGBB).
        Format_ARGB32,
        /// The image is stored using a premultiplied 32-bit ARGB format (0xAARRGGBB), i.e. the red, green, and blue channels are multiplied by the alpha component divided by 255. (If RR, GG, or BB has a higher value than the alpha channel, the results are undefined.) Certain operations (such as image composition using alpha blending) are faster using premultiplied ARGB32 than with plain ARGB32.
        Format_ARGB32_Premultiplied,
        /// The image is stored using a 16-bit RGB format (5-6-5).
        Format_RGB16,
        /// The image is stored using a premultiplied 24-bit ARGB format (8-5-6-5).
        Format_ARGB8565_Premultiplied,
        /// The image is stored using a 24-bit RGB format (6-6-6). The unused most significant bits is always zero.
        Format_RGB666,
        /// The image is stored using a premultiplied 24-bit ARGB format (6-6-6-6).
        Format_ARGB6666_Premultiplied,
        /// The image is stored using a 16-bit RGB format (5-5-5). The unused most significant bit is always zero.
        Format_RGB555,
        /// The image is stored using a premultiplied 24-bit ARGB format (8-5-5-5).
        Format_ARGB8555_Premultiplied,
        /// The image is stored using a 24-bit RGB format (8-8-8).
        Format_RGB888,
        /// The image is stored using a 16-bit RGB format (4-4-4). The unused bits are always zero.
        Format_RGB444,
        /// The image is stored using a premultiplied 16-bit ARGB format (4-4-4-4).
        Format_ARGB4444_Premultiplied,
        /// The image is stored using a 32-bit byte-ordered RGB(x) format (8-8-8-8). This is the same as the Format_RGBA8888 except alpha must always be 255.
        Format_RGBX8888,
        /// The image is stored using a 32-bit byte-ordered RGBA format (8-8-8-8). Unlike ARGB32 this is a byte-ordered format, which means the 32bit encoding differs between big endian and little endian architectures, being respectively (0xRRGGBBAA) and (0xAABBGGRR). The order of the colors is the same on any architecture if read as bytes 0xRR,0xGG,0xBB,0xAA.
        Format_RGBA8888,
        /// The image is stored using a premultiplied 32-bit byte-ordered RGBA format (8-8-8-8).
        Format_RGBA8888_Premultiplied,
        /// The image is stored using a 32-bit BGR format (x-10-10-10).
        Format_BGR30,
        /// The image is stored using a 32-bit premultiplied ABGR format (2-10-10-10).
        Format_A2BGR30_Premultiplied,
        /// The image is stored using a 32-bit RGB format (x-10-10-10).
        Format_RGB30,
        /// The image is stored using a 32-bit premultiplied ARGB format (2-10-10-10).
        Format_A2RGB30_Premultiplied,
        /// The image is stored using an 8-bit alpha only format.
        Format_Alpha8,
        /// The image is stored using an 8-bit grayscale format.
        Format_Grayscale8,
        /// The image is stored using a 64-bit halfword-ordered RGB(x) format (16-16-16-16). This is the same as the Format_RGBA64 except alpha must always be 65535.
        Format_RGBX64,
        /// The image is stored using a 64-bit halfword-ordered RGBA format (16-16-16-16).
        Format_RGBA64,
        /// The image is stored using a premultiplied 64-bit halfword-ordered RGBA format (16-16-16-16).
        Format_RGBA64_Premultiplied,
        /// The image is stored using an 16-bit grayscale format.
        Format_Grayscale16,
        /// The image is stored using a 24-bit BGR format.
        Format_BGR888,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qimage.h");
        type QImage = super::QImage;
        include!("cxx-qt-lib/qsize.h");
        type QSize = cxx_qt_lib::QSize;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qrect.h");
        type QRect = cxx_qt_lib::QRect;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = cxx_qt_lib::QPoint;
        include!("cxx-qt-lib/qsizef.h");
        #[allow(dead_code)]
        type QSizeF = cxx_qt_lib::QSizeF;
        type QImageCleanupFunction = super::QImageCleanupFunction;

        type qreal = cxx_qt_lib::qreal;

        /// Returns a sub-area of the image as a new image.
        ///
        /// The returned image is copied from the position (`rectangle.x()`, `rectangle.y()`) in this image, and will always have the size of the given `rectangle`.
        ///
        /// In areas beyond this image, pixels are set to 0. For 32-bit RGB images, this means black; for 32-bit ARGB images, this means transparent black; for 8-bit images, this means the color with index 0 in the color table which can be anything; for 1-bit images, this means Qt::color0.
        ///
        /// If the given `rectangle` is a null rectangle the entire image is copied.
        fn copy(self: &QImage, rectangle: &QRect) -> QImage;

        /// Returns `true` if it is a null image, otherwise returns `false`.
        ///
        /// A null image has all parameters set to zero and no allocated data.
        #[rust_name = "is_null"]
        #[allow(dead_code)]
        fn isNull(self: &QImage) -> bool;

        /// Returns the enclosing rectangle (`0`, `0`, `width()`, `height()`) of the image.
        fn rect(self: &QImage) -> QRect;

        #[rust_name = "set_device_pixel_ratio"]
        fn setDevicePixelRatio(self: &mut QImage, scale_factor: qreal);

        #[rust_name = "scan_line"]
        fn scanLine(self: &mut QImage, i: i32) -> *mut u8;

        #[rust_name = "set_text"]
        fn setText(self: &mut QImage, key: &QString, text: &QString);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type QImageFormat;
        type QImageInvertMode;
        type c_void = cxx_qt_lib::c_void;

        #[doc(hidden)]
        #[rust_name = "qimage_init_default"]
        fn construct() -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_init_from_width_and_height_and_image_format"]
        fn construct(width: i32, height: i32, format: QImageFormat) -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_drop"]
        fn drop(image: &mut QImage);

        #[doc(hidden)]
        #[allow(dead_code)]
        #[rust_name = "qimage_init_from_data"]
        fn qimageInitFromData(data: &[u8], format: &str) -> QImage;

        #[doc(hidden)]
        #[allow(dead_code)]
        #[rust_name = "qimage_cache_key"]
        fn qimageCacheKey(image: &QImage) -> i64;

        #[doc(hidden)]
        #[rust_name = "qimage_eq"]
        fn operatorEq(a: &QImage, b: &QImage) -> bool;

        #[doc(hidden)]
        #[rust_name = "qimage_to_debug_qstring"]
        fn toDebugQString(image: &QImage) -> QString;
    }
}

pub use ffi::QImageFormat;

/// The `QImage` class provides a hardware-independent image representation that allows direct access to the pixel data, and can be used as a paint device.
///
/// Qt Documentation: [QImage](https://doc.qt.io/qt/qimage.html#details)
#[repr(C)]
pub struct QImage {
    // Static checks on the C++ side ensure this is true.
    // See qimage.cpp
    _painters: MaybeUninit<u16>,
    #[cfg(cxxqt_qt_version_major = "5")]
    _reserved: MaybeUninit<usize>,
    _d: MaybeUninit<usize>,
    _vtable: MaybeUninit<usize>,
}

impl Clone for QImage {
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        self.copy(&self.rect())
    }
}

impl Default for QImage {
    /// Constructs a null image.
    fn default() -> Self {
        ffi::qimage_init_default()
    }
}

impl PartialEq for QImage {
    fn eq(&self, other: &Self) -> bool {
        ffi::qimage_eq(self, other)
    }
}

impl Eq for QImage {}

impl fmt::Debug for QImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qimage_to_debug_qstring(self).fmt(f)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size & alignment is the same.
unsafe impl ExternType for QImage {
    type Id = type_id!("QImage");
    type Kind = cxx::kind::Trivial;
}

impl Drop for QImage {
    fn drop(&mut self) {
        ffi::qimage_drop(self);
    }
}

// Static assertions on the C++ side assert that this type is equal to:
// void(*)(void*)
#[repr(transparent)]
struct QImageCleanupFunction(extern "C" fn(*mut ffi::c_void));

unsafe impl ExternType for QImageCleanupFunction {
    type Id = type_id!("QImageCleanupFunction");

    type Kind = cxx::kind::Trivial;
}

impl QImage {
    /// Convert raw image data to a `QImage`.
    ///
    /// The data must be in the given `format`.
    /// See [QImageReader::supportedImageFormats](https://doc.qt.io/qt/qimagereader.html#supportedImageFormats)() for the list of supported formats.
    ///
    /// If `format` is `None`, the format will be quessed from the image header.
    #[allow(dead_code)]
    pub fn from_data(data: &[u8], format: Option<&str>) -> Option<Self> {
        let image = ffi::qimage_init_from_data(data, format.unwrap_or(""));
        if image.is_null() { None } else { Some(image) }
    }

    /// Returns a number that identifies the contents of this `QImage` object. Distinct `QImage` objects can only have the same key if they refer to the same contents.
    ///
    /// The key will change when the image is altered.
    #[allow(dead_code)]
    pub fn cache_key(&self) -> i64 {
        ffi::qimage_cache_key(self)
    }

    /// Construct a `QImage` from a given `width`, `height`, and image `format`.
    pub fn from_width_height_and_format(width: i32, height: i32, format: QImageFormat) -> Self {
        ffi::qimage_init_from_width_and_height_and_image_format(width, height, format)
    }
}
