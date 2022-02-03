use dav1d_sys::*;

use std::ffi::c_void;
use std::fmt;
use std::i64;
use std::mem;
use std::ptr;
use std::sync::Arc;

/// Error enum return by various `dav1d` operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Try again.
    ///
    /// If this is returned by [`Decoder::send_data`] or [`Decoder::send_pending_data`] then there
    /// are decoded frames pending that first have to be retrieved via [`Decoder::get_picture`]
    /// before processing any further pending data.
    ///
    /// If this is returned by [`Decoder::get_picture`] then no decoded frames are pending
    /// currently and more data needs to be sent to the decoder.
    Again,
    /// Invalid argument.
    ///
    /// One of the arguments passed to the function was invalid.
    InvalidArgument,
    /// Not enough memory.
    ///
    /// Not enough memory is currently available for performing this operation.
    NotEnoughMemory,
    /// Unsupported bitstream.
    ///
    /// The provided bitstream is not supported by `dav1d`.
    UnsupportedBitstream,
    /// Unknown error.
    UnknownError(i32),
}

impl From<i32> for Error {
    fn from(err: i32) -> Self {
        assert!(err < 0);

        // Convert to i32
        const AGAIN: i32 = EAGAIN as i32;
        const INVAL: i32 = EINVAL as i32;
        const NOMEM: i32 = ENOMEM as i32;
        const NOPROTOOPT: i32 = ENOPROTOOPT as i32;

        // Correctly handle non-negative errnos
        #[allow(unused_comparisons)]
        let err = if EPERM < 0 { err } else { -err };

        match err {
            AGAIN => Error::Again,
            INVAL => Error::InvalidArgument,
            NOMEM => Error::NotEnoughMemory,
            NOPROTOOPT => Error::UnsupportedBitstream,
            _ => Error::UnknownError(err),
        }
    }
}

impl Error {
    pub const fn is_again(&self) -> bool {
        matches!(self, Error::Again)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Again => write!(fmt, "Try again"),
            Error::InvalidArgument => write!(fmt, "Invalid argument"),
            Error::NotEnoughMemory => write!(fmt, "Not enough memory available"),
            Error::UnsupportedBitstream => write!(fmt, "Unsupported bitstream"),
            Error::UnknownError(err) => write!(fmt, "Unknown error {}", err),
        }
    }
}

impl std::error::Error for Error {}

/// A `dav1d` decoder instance.
#[derive(Debug)]
pub struct Decoder {
    dec: ptr::NonNull<Dav1dContext>,
    pending_data: Option<Dav1dData>,
}

unsafe extern "C" fn release_wrapped_data<T: AsRef<[u8]>>(_data: *const u8, cookie: *mut c_void) {
    let buf = Box::from_raw(cookie as *mut T);
    drop(buf);
}

impl Decoder {
    /// Creates a new [`Decoder`] instance with the default settings.
    pub fn new() -> Result<Self, Error> {
        unsafe {
            let mut settings = mem::MaybeUninit::uninit();
            let mut dec = mem::MaybeUninit::uninit();

            dav1d_default_settings(settings.as_mut_ptr());

            let settings = settings.assume_init();

            let ret = dav1d_open(dec.as_mut_ptr(), &settings);

            if ret < 0 {
                return Err(Error::from(ret));
            }

            Ok(Decoder {
                dec: ptr::NonNull::new(dec.assume_init()).unwrap(),
                pending_data: None,
            })
        }
    }

    /// Flush the decoder.
    ///
    /// This flushes all delayed frames in the decoder and clears the internal decoder state.
    ///
    /// All currently pending frames are available afterwards via [`Decoder::get_picture`].
    pub fn flush(&mut self) {
        unsafe {
            dav1d_flush(self.dec.as_ptr());
            if let Some(mut pending_data) = self.pending_data.take() {
                dav1d_data_unref(&mut pending_data);
            }
        }
    }

    /// Send new AV1 data to the decoder.
    ///
    /// After this returned `Ok(())` or `Err([Error::Again])` there might be decoded frames
    /// available via [`Decoder::get_picture`].
    ///
    /// # Panics
    ///
    /// If a previous call returned [`Error::Again`] then this must not be called again until
    /// [`Decoder::send_pending_data`] has returned `Ok(())`.
    pub fn send_data<T: AsRef<[u8]> + Send + 'static>(
        &mut self,
        buf: T,
        offset: Option<i64>,
        timestamp: Option<i64>,
        duration: Option<i64>,
    ) -> Result<(), Error> {
        assert!(
            self.pending_data.is_none(),
            "Have pending data that needs to be handled first"
        );

        let buf = Box::new(buf);
        let slice = (*buf).as_ref();
        let len = slice.len();

        unsafe {
            let mut data: Dav1dData = mem::zeroed();
            let _ret = dav1d_data_wrap(
                &mut data,
                slice.as_ptr(),
                len,
                Some(release_wrapped_data::<T>),
                Box::into_raw(buf) as *mut c_void,
            );
            if let Some(offset) = offset {
                data.m.offset = offset;
            }
            if let Some(timestamp) = timestamp {
                data.m.timestamp = timestamp;
            }
            if let Some(duration) = duration {
                data.m.duration = duration;
            }

            let ret = dav1d_send_data(self.dec.as_ptr(), &mut data);
            if ret < 0 {
                let ret = Error::from(ret);

                if ret.is_again() {
                    self.pending_data = Some(data);
                } else {
                    dav1d_data_unref(&mut data);
                }

                return Err(ret);
            }

            if data.sz > 0 {
                self.pending_data = Some(data);
                return Err(Error::Again);
            }

            Ok(())
        }
    }

    /// Sends any pending data to the decoder.
    ///
    /// This has to be called after [`Decoder::send_data`] has returned `Err([Error::Again])` to
    /// consume any futher pending data.
    ///
    /// After this returned `Ok(())` or `Err([Error::Again])` there might be decoded frames
    /// available via [`Decoder::get_picture`].
    pub fn send_pending_data(&mut self) -> Result<(), Error> {
        let mut data = match self.pending_data.take() {
            None => {
                return Ok(());
            }
            Some(data) => data,
        };

        unsafe {
            let ret = dav1d_send_data(self.dec.as_ptr(), &mut data);
            if ret < 0 {
                let ret = Error::from(ret);

                if ret.is_again() {
                    self.pending_data = Some(data);
                } else {
                    dav1d_data_unref(&mut data);
                }

                return Err(ret);
            }

            if data.sz > 0 {
                self.pending_data = Some(data);
                return Err(Error::Again);
            }

            Ok(())
        }
    }

    /// Get the next decoded frame from the decoder.
    ///
    /// If this returns `Err([Error::Again])` then further data has to be sent to the decoder
    /// before further decoded frames become available.
    pub fn get_picture(&mut self) -> Result<Picture, Error> {
        unsafe {
            let mut pic: Dav1dPicture = mem::zeroed();
            let ret = dav1d_get_picture(self.dec.as_ptr(), &mut pic);

            if ret < 0 {
                Err(Error::from(ret))
            } else {
                let inner = InnerPicture { pic };
                Ok(Picture {
                    inner: Arc::new(inner),
                })
            }
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            if let Some(mut pending_data) = self.pending_data.take() {
                dav1d_data_unref(&mut pending_data);
            }
            let mut dec = self.dec.as_ptr();
            dav1d_close(&mut dec);
        };
    }
}

unsafe impl Send for Decoder {}
unsafe impl Sync for Decoder {}

#[derive(Debug)]
struct InnerPicture {
    pub pic: Dav1dPicture,
}

/// A decoded frame.
#[derive(Debug, Clone)]
pub struct Picture {
    inner: Arc<InnerPicture>,
}

/// Pixel layout of a frame.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PixelLayout {
    /// Monochrome.
    I400,
    /// 4:2:0 planar.
    I420,
    /// 4:2:2 planar.
    I422,
    /// 4:4:4 planar.
    I444,
}

/// Frame component.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PlanarImageComponent {
    /// Y component.
    Y,
    /// U component.
    U,
    /// V component.
    V,
}

impl From<usize> for PlanarImageComponent {
    fn from(index: usize) -> Self {
        match index {
            0 => PlanarImageComponent::Y,
            1 => PlanarImageComponent::U,
            2 => PlanarImageComponent::V,
            _ => panic!("Invalid YUV index: {}", index),
        }
    }
}

impl From<PlanarImageComponent> for usize {
    fn from(component: PlanarImageComponent) -> Self {
        match component {
            PlanarImageComponent::Y => 0,
            PlanarImageComponent::U => 1,
            PlanarImageComponent::V => 2,
        }
    }
}

/// A single plane of a decoded frame.
///
/// This can be used like a `&[u8]`.
#[derive(Clone, Debug)]
pub struct Plane(Picture, PlanarImageComponent);

impl AsRef<[u8]> for Plane {
    fn as_ref(&self) -> &[u8] {
        let (stride, height) = self.0.plane_data_geometry(self.1);
        unsafe {
            std::slice::from_raw_parts(
                self.0.plane_data_ptr(self.1) as *const u8,
                (stride * height) as usize,
            )
        }
    }
}

impl std::ops::Deref for Plane {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

unsafe impl Send for Plane {}
unsafe impl Sync for Plane {}

/// Number of bits per component.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitsPerComponent(pub usize);

impl Picture {
    /// Stride in pixels of the `component` for the decoded frame.
    pub fn stride(&self, component: PlanarImageComponent) -> u32 {
        let s = match component {
            PlanarImageComponent::Y => 0,
            _ => 1,
        };
        (*self.inner).pic.stride[s] as u32
    }

    /// Raw pointer to the data of the `component` for the decoded frame.
    pub fn plane_data_ptr(&self, component: PlanarImageComponent) -> *mut c_void {
        let index: usize = component.into();
        (*self.inner).pic.data[index]
    }

    /// Plane geometry of the `component` for the decoded frame.
    ///
    /// This returns the stride and height.
    pub fn plane_data_geometry(&self, component: PlanarImageComponent) -> (u32, u32) {
        let height = match component {
            PlanarImageComponent::Y => self.height(),
            _ => match self.pixel_layout() {
                PixelLayout::I420 => (self.height() + 1) / 2,
                PixelLayout::I400 | PixelLayout::I422 | PixelLayout::I444 => self.height(),
            },
        };
        (self.stride(component) as u32, height)
    }

    /// Plane data of the `component` for the decoded frame.
    pub fn plane(&self, component: PlanarImageComponent) -> Plane {
        Plane(self.clone(), component)
    }

    /// Bit depth of the plane data.
    ///
    /// This returns 8 or 16 for the underlying integer type used for the plane data.
    ///
    /// Check [`Picture::bits_per_component`] for the number of bits that are used.
    pub fn bit_depth(&self) -> usize {
        (*self.inner).pic.p.bpc as usize
    }

    /// Bits used per component of the plane data.
    ///
    /// Check [`Picture::bit_depth`] for the number of storage bits.
    pub fn bits_per_component(&self) -> Option<BitsPerComponent> {
        unsafe {
            match (*(*self.inner).pic.seq_hdr).hbd {
                0 => Some(BitsPerComponent(8)),
                1 => Some(BitsPerComponent(10)),
                2 => Some(BitsPerComponent(12)),
                _ => None,
            }
        }
    }

    /// Width of the frame.
    pub fn width(&self) -> u32 {
        (*self.inner).pic.p.w as u32
    }

    /// Height of the frame.
    pub fn height(&self) -> u32 {
        (*self.inner).pic.p.h as u32
    }

    /// Pixel layout of the frame.
    pub fn pixel_layout(&self) -> PixelLayout {
        #[allow(non_upper_case_globals)]
        match (*self.inner).pic.p.layout {
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
            Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
            _ => unreachable!(),
        }
    }

    /// Timestamp of the frame.
    ///
    /// This is the same timestamp as the one provided to [`Decoder::send_data`].
    pub fn timestamp(&self) -> Option<i64> {
        let ts = (*self.inner).pic.m.timestamp;
        if ts == i64::MIN {
            None
        } else {
            Some(ts)
        }
    }

    /// Duration of the frame.
    ///
    /// This is the same duration as the one provided to [`Decoder::send_data`] or `0` if none was
    /// provided.
    pub fn duration(&self) -> i64 {
        (*self.inner).pic.m.duration as i64
    }

    /// Offset of the frame.
    ///
    /// This is the same offset as the one provided to [`Decoder::send_data`] or `-1` if none was
    /// provided.
    pub fn offset(&self) -> i64 {
        (*self.inner).pic.m.offset
    }
}

unsafe impl Send for Picture {}
unsafe impl Sync for Picture {}

impl Drop for InnerPicture {
    fn drop(&mut self) {
        unsafe {
            dav1d_picture_unref(&mut self.pic);
        }
    }
}
