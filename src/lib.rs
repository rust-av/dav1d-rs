use dav1d_sys::*;

pub use av_data::pixel;
use std::ffi::{c_int, c_void};
use std::fmt::{self, Debug};
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

        match err {
            DAV1D_ERR_AGAIN => Error::Again,
            DAV1D_ERR_INVAL => Error::InvalidArgument,
            DAV1D_ERR_NOMEM => Error::NotEnoughMemory,
            DAV1D_ERR_NOPROTOOPT => Error::UnsupportedBitstream,
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

/// Picture parameters used for allocation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PictureParameters {
    /// Width in pixels.
    pub w: u32,
    /// Height in pixels.
    pub h: u32,
    /// Format of the picture.
    pub layout: PixelLayout,
    /// Bits per pixel (8 or 10 bits).
    pub bit_depth: usize,
}

// Number of bytes to align AND pad picture memory buffers by, so that SIMD
// implementations can over-read by a few bytes, and use aligned read/write
// instructions.
pub const PICTURE_ALIGNMENT: usize = 64;

/// Allocation for a picture.
#[derive(Debug, PartialEq, Eq)]
pub struct PictureAllocation<D: Send + 'static> {
    /// Pointers to planar image data.
    ///
    /// Y is `data[0]`, U is `data[1]`, V is `data[2]`.
    ///
    /// The data should be `u8` (for 8 bits) or `u16` (for 10 bits).
    ///
    /// The `data[0]`, `data[1]` and `data[2]` must be [`PICTURE_ALIGNMENT`] byte aligned and with a pixel
    /// width/height multiple of 128 pixels. Any allocated memory area should also be padded by
    /// [`PICTURE_ALIGNMENT`] bytes.
    ///
    /// `data[1]` and `data[2]` must share the same `stride[1]`.
    pub data: [*mut u8; 3],

    /// Number of bytes between 2 lines in `data[]` for luma in case of `stride[0]` or chroma in
    /// case of `stride[1]`.
    pub stride: [isize; 2],

    /// Allocator data that can be retrieved back from the picture later.
    pub allocator_data: D,
}

unsafe impl<D: Send + 'static> Send for PictureAllocation<D> {}

pub trait PictureAllocator: Send + Sync + 'static {
    /// Allocator data that is stored together with the [`Picture`].
    ///
    /// This can be retrieved from the picture via [`Picture::allocator_data()`].
    type AllocatorData: Send + 'static;

    /// Allocate the picture buffer based on `pic_params`.
    ///
    /// See the [`PictureAllocation`] documentation for the requirements on the allocated memory.
    ///
    /// This function will be called on the main thread (the thread which calls
    /// [`Decoder::get_picture()`]).
    ///
    /// # Safety
    ///
    /// This function needs to allocate enough memory with the constraints outlined in the
    /// [`PictureAllocation`] documentation.
    unsafe fn alloc_picture(
        &self,
        pic_params: &PictureParameters,
    ) -> Result<PictureAllocation<Self::AllocatorData>, Error>;

    /// Release the picture buffer.
    ///
    /// If frame threading is used, this function may be called by the main
    /// thread (the thread which calls [`Decoder::get_picture()`]) or any of the frame
    /// threads and thus must be thread-safe. If frame threading is not used,
    /// this function will only be called on the main thread.
    ///
    /// # Safety
    ///
    /// This function needs to release the memory in `allocation` and can assume that it
    /// corresponds to a previous call to [`PictureAllocator::alloc_picture()`].
    unsafe fn release_picture(&self, allocation: PictureAllocation<Self::AllocatorData>);
}

/// Default allocator.
///
/// Note that this allocator can't be directly instantiated.
#[derive(Debug)]
pub struct DefaultAllocator(());

impl PictureAllocator for DefaultAllocator {
    type AllocatorData = ();

    unsafe fn alloc_picture(
        &self,
        _pic_params: &PictureParameters,
    ) -> Result<PictureAllocation<Self::AllocatorData>, Error> {
        unimplemented!()
    }

    unsafe fn release_picture(&self, _allocation: PictureAllocation<Self::AllocatorData>) {
        unimplemented!()
    }
}

/// Settings for creating a new [`Decoder`] instance.
/// See documentation for native `Dav1dSettings` struct.
#[derive(Debug)]
pub struct Settings {
    dav1d_settings: Dav1dSettings,
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    /// Creates a new [`Settings`] instance with default settings.
    pub fn new() -> Self {
        unsafe {
            let mut dav1d_settings = mem::MaybeUninit::uninit();

            dav1d_default_settings(dav1d_settings.as_mut_ptr());

            Self {
                dav1d_settings: dav1d_settings.assume_init(),
            }
        }
    }

    pub fn set_n_threads(&mut self, n_threads: u32) {
        self.dav1d_settings.n_threads = n_threads as i32;
    }

    pub fn get_n_threads(&self) -> u32 {
        self.dav1d_settings.n_threads as u32
    }

    pub fn set_max_frame_delay(&mut self, max_frame_delay: u32) {
        self.dav1d_settings.max_frame_delay = max_frame_delay as i32;
    }

    pub fn get_max_frame_delay(&self) -> u32 {
        self.dav1d_settings.max_frame_delay as u32
    }

    pub fn set_apply_grain(&mut self, apply_grain: bool) {
        self.dav1d_settings.apply_grain = i32::from(apply_grain);
    }

    pub fn get_apply_grain(&self) -> bool {
        self.dav1d_settings.apply_grain != 0
    }

    pub fn set_operating_point(&mut self, operating_point: u32) {
        self.dav1d_settings.operating_point = operating_point as i32;
    }

    pub fn get_operating_point(&self) -> u32 {
        self.dav1d_settings.operating_point as u32
    }

    pub fn set_all_layers(&mut self, all_layers: bool) {
        self.dav1d_settings.all_layers = i32::from(all_layers);
    }

    pub fn get_all_layers(&self) -> bool {
        self.dav1d_settings.all_layers != 0
    }

    pub fn set_frame_size_limit(&mut self, frame_size_limit: u32) {
        self.dav1d_settings.frame_size_limit = frame_size_limit;
    }

    pub fn get_frame_size_limit(&self) -> u32 {
        self.dav1d_settings.frame_size_limit
    }

    pub fn set_strict_std_compliance(&mut self, strict_std_compliance: bool) {
        self.dav1d_settings.strict_std_compliance = i32::from(strict_std_compliance);
    }

    pub fn get_strict_std_compliance(&self) -> bool {
        self.dav1d_settings.strict_std_compliance != 0
    }

    pub fn set_output_invisible_frames(&mut self, output_invisible_frames: bool) {
        self.dav1d_settings.output_invisible_frames = i32::from(output_invisible_frames);
    }

    pub fn get_output_invisible_frames(&self) -> bool {
        self.dav1d_settings.output_invisible_frames != 0
    }

    pub fn set_inloop_filters(&mut self, inloop_filters: InloopFilterType) {
        self.dav1d_settings.inloop_filters = inloop_filters.bits();
    }

    pub fn get_inloop_filters(&self) -> InloopFilterType {
        InloopFilterType::from_bits_truncate(self.dav1d_settings.inloop_filters)
    }

    pub fn set_decode_frame_type(&mut self, decode_frame_type: DecodeFrameType) {
        self.dav1d_settings.decode_frame_type = decode_frame_type.into();
    }

    pub fn get_decode_frame_type(&self) -> DecodeFrameType {
        DecodeFrameType::try_from(self.dav1d_settings.decode_frame_type)
            .expect("Invalid Dav1dDecodeFrameType")
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
    pub struct InloopFilterType: u32 {
        const DEBLOCK = DAV1D_INLOOPFILTER_DEBLOCK;
        const CDEF = DAV1D_INLOOPFILTER_CDEF;
        const RESTORATION = DAV1D_INLOOPFILTER_RESTORATION;
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub enum DecodeFrameType {
    #[default]
    All,
    Reference,
    Intra,
    Key,
}

impl TryFrom<u32> for DecodeFrameType {
    type Error = TryFromEnumError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            DAV1D_DECODEFRAMETYPE_ALL => Ok(DecodeFrameType::All),
            DAV1D_DECODEFRAMETYPE_REFERENCE => Ok(DecodeFrameType::Reference),
            DAV1D_DECODEFRAMETYPE_INTRA => Ok(DecodeFrameType::Intra),
            DAV1D_DECODEFRAMETYPE_KEY => Ok(DecodeFrameType::Key),
            _ => Err(TryFromEnumError(())),
        }
    }
}

impl From<DecodeFrameType> for u32 {
    fn from(v: DecodeFrameType) -> u32 {
        match v {
            DecodeFrameType::All => DAV1D_DECODEFRAMETYPE_ALL,
            DecodeFrameType::Reference => DAV1D_DECODEFRAMETYPE_REFERENCE,
            DecodeFrameType::Intra => DAV1D_DECODEFRAMETYPE_INTRA,
            DecodeFrameType::Key => DAV1D_DECODEFRAMETYPE_KEY,
        }
    }
}

/// The error type returned when a conversion from a C enum fails.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromEnumError(());

impl std::fmt::Display for TryFromEnumError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Invalid enum value")
    }
}

impl From<std::convert::Infallible> for TryFromEnumError {
    fn from(x: std::convert::Infallible) -> TryFromEnumError {
        match x {}
    }
}

impl std::error::Error for TryFromEnumError {}

/// A `dav1d` decoder instance.
#[derive(Debug)]
pub struct Decoder<A: PictureAllocator = DefaultAllocator> {
    dec: ptr::NonNull<Dav1dContext>,
    pending_data: Option<Dav1dData>,
    allocator: Option<Arc<A>>,
}

static_assertions::assert_impl_all!(Decoder<DefaultAllocator>: Send, Sync, Debug);

unsafe extern "C" fn release_wrapped_data<T: AsRef<[u8]>>(_data: *const u8, cookie: *mut c_void) {
    let buf = Box::from_raw(cookie as *mut T);
    drop(buf);
}

impl Decoder {
    /// Creates a new [`Decoder`] instance with given [`Settings`].
    pub fn with_settings(settings: &Settings) -> Result<Self, Error> {
        unsafe {
            let mut dec = mem::MaybeUninit::uninit();

            let ret = dav1d_open(dec.as_mut_ptr(), &settings.dav1d_settings);

            if ret < 0 {
                return Err(Error::from(ret));
            }

            Ok(Decoder {
                dec: ptr::NonNull::new(dec.assume_init()).unwrap(),
                pending_data: None,
                allocator: None,
            })
        }
    }

    /// Creates a new [`Decoder`] instance with the default settings.
    pub fn new() -> Result<Self, Error> {
        Self::with_settings(&Settings::default())
    }
}

unsafe extern "C" fn alloc_picture_callback<A: PictureAllocator>(
    pic: *mut Dav1dPicture,
    cookie: *mut c_void,
) -> c_int {
    let allocator = &*(cookie as *const A);

    let pic_parameters = PictureParameters {
        w: (*pic).p.w as u32,
        h: (*pic).p.h as u32,
        layout: match (*pic).p.layout {
            DAV1D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
            DAV1D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
            DAV1D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
            DAV1D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
            _ => unreachable!(),
        },
        bit_depth: (*pic).p.bpc as usize,
    };

    let res = allocator.alloc_picture(&pic_parameters);
    match res {
        Ok(allocation) => {
            (*pic).data[0] = allocation.data[0] as *mut c_void;
            (*pic).data[1] = allocation.data[1] as *mut c_void;
            (*pic).data[2] = allocation.data[2] as *mut c_void;
            (*pic).stride[0] = allocation.stride[0];
            (*pic).stride[1] = allocation.stride[1];
            (*pic).allocator_data =
                Box::into_raw(Box::new(allocation.allocator_data)) as *mut c_void;

            0
        }
        Err(err) => match err {
            Error::Again => DAV1D_ERR_AGAIN,
            Error::InvalidArgument => DAV1D_ERR_INVAL,
            Error::NotEnoughMemory => DAV1D_ERR_NOMEM,
            Error::UnsupportedBitstream => DAV1D_ERR_NOPROTOOPT,
            Error::UnknownError(err) => {
                assert!(err < 0);
                err
            }
        },
    }
}

unsafe extern "C" fn release_picture_callback<A: PictureAllocator>(
    pic: *mut Dav1dPicture,
    cookie: *mut c_void,
) {
    let allocator = &*(cookie as *const A);
    let allocator_data = Box::from_raw((*pic).allocator_data as *mut A::AllocatorData);
    let allocation = PictureAllocation {
        data: [
            (*pic).data[0] as *mut u8,
            (*pic).data[1] as *mut u8,
            (*pic).data[2] as *mut u8,
        ],
        stride: (*pic).stride,
        allocator_data: *allocator_data,
    };
    allocator.release_picture(allocation);
}

impl<A: PictureAllocator> Decoder<A> {
    /// Creates a new [`Decoder`] instance with given [`Settings`] and [`PictureAllocator`].
    pub fn with_settings_and_allocator(settings: &Settings, allocator: A) -> Result<Self, Error> {
        unsafe {
            let allocator = Arc::new(allocator);

            let mut dec = mem::MaybeUninit::uninit();

            let settings = Dav1dSettings {
                allocator: Dav1dPicAllocator {
                    cookie: &*allocator as *const A as *mut c_void,
                    alloc_picture_callback: Some(alloc_picture_callback::<A>),
                    release_picture_callback: Some(release_picture_callback::<A>),
                },
                ..settings.dav1d_settings
            };
            let ret = dav1d_open(dec.as_mut_ptr(), &settings);

            if ret < 0 {
                return Err(Error::from(ret));
            }

            Ok(Decoder {
                dec: ptr::NonNull::new(dec.assume_init()).unwrap(),
                pending_data: None,
                allocator: Some(allocator),
            })
        }
    }

    /// Creates a new [`Decoder`] instance with the default settings and the given
    /// [`PictureAllocator`].
    pub fn with_allocator(allocator: A) -> Result<Self, Error> {
        Self::with_settings_and_allocator(&Settings::default(), allocator)
    }
}

impl<A: PictureAllocator> Decoder<A> {
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
    ///
    /// To make most use of frame threading this function should only be called once per submitted
    /// input frame and not until it returns `Err([Error::Again])`. Calling it in a loop should
    /// only be done to drain all pending frames at the end.
    pub fn get_picture(&mut self) -> Result<Picture<A>, Error> {
        unsafe {
            let mut pic: Dav1dPicture = mem::zeroed();
            let ret = dav1d_get_picture(self.dec.as_ptr(), &mut pic);

            if ret < 0 {
                Err(Error::from(ret))
            } else {
                let inner = InnerPicture { pic };
                Ok(Picture {
                    inner: Arc::new(inner),
                    allocator: self.allocator.clone(),
                })
            }
        }
    }

    /// Get the decoder delay.
    pub fn get_frame_delay(&self) -> u32 {
        unsafe { dav1d_get_frame_delay(self.dec.as_ptr()) as u32 }
    }
}

impl<A: PictureAllocator> Drop for Decoder<A> {
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

unsafe impl<A: PictureAllocator> Send for Decoder<A> {}
unsafe impl<A: PictureAllocator> Sync for Decoder<A> {}

#[derive(Debug)]
struct InnerPicture {
    pub pic: Dav1dPicture,
}

/// A decoded frame.
#[derive(Debug)]
pub struct Picture<A: PictureAllocator = DefaultAllocator> {
    inner: Arc<InnerPicture>,
    allocator: Option<Arc<A>>,
}

impl<A: PictureAllocator> Clone for Picture<A> {
    fn clone(&self) -> Self {
        Picture {
            inner: self.inner.clone(),
            allocator: self.allocator.clone(),
        }
    }
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
#[derive(Debug)]
pub struct Plane<A: PictureAllocator = DefaultAllocator>(Picture<A>, PlanarImageComponent);

impl<A: PictureAllocator> Clone for Plane<A> {
    fn clone(&self) -> Self {
        Plane(self.0.clone(), self.1)
    }
}

impl<A: PictureAllocator> AsRef<[u8]> for Plane<A> {
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

impl<A: PictureAllocator> std::ops::Deref for Plane<A> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

static_assertions::assert_impl_all!(Plane<DefaultAllocator>: Send, Sync, Clone, Debug);

/// Number of bits per component.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BitsPerComponent(pub usize);

impl<A: PictureAllocator> Picture<A> {
    /// Stride in pixels of the `component` for the decoded frame.
    pub fn stride(&self, component: PlanarImageComponent) -> u32 {
        let s = match component {
            PlanarImageComponent::Y => 0,
            _ => 1,
        };
        self.inner.pic.stride[s] as u32
    }

    /// Raw pointer to the data of the `component` for the decoded frame.
    pub fn plane_data_ptr(&self, component: PlanarImageComponent) -> *mut c_void {
        let index: usize = component.into();
        self.inner.pic.data[index]
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
        (self.stride(component), height)
    }

    /// Plane data of the `component` for the decoded frame.
    pub fn plane(&self, component: PlanarImageComponent) -> Plane<A> {
        Plane(self.clone(), component)
    }

    /// Bit depth of the plane data.
    ///
    /// This returns 8 or 16 for the underlying integer type used for the plane data.
    ///
    /// Check [`Picture::bits_per_component`] for the number of bits that are used.
    pub fn bit_depth(&self) -> usize {
        self.inner.pic.p.bpc as usize
    }

    /// Bits used per component of the plane data.
    ///
    /// Check [`Picture::bit_depth`] for the number of storage bits.
    pub fn bits_per_component(&self) -> Option<BitsPerComponent> {
        unsafe {
            match (*self.inner.pic.seq_hdr).hbd {
                0 => Some(BitsPerComponent(8)),
                1 => Some(BitsPerComponent(10)),
                2 => Some(BitsPerComponent(12)),
                _ => None,
            }
        }
    }

    /// Width of the frame.
    pub fn width(&self) -> u32 {
        self.inner.pic.p.w as u32
    }

    /// Height of the frame.
    pub fn height(&self) -> u32 {
        self.inner.pic.p.h as u32
    }

    /// Pixel layout of the frame.
    pub fn pixel_layout(&self) -> PixelLayout {
        #[allow(non_upper_case_globals)]
        match self.inner.pic.p.layout {
            DAV1D_PIXEL_LAYOUT_I400 => PixelLayout::I400,
            DAV1D_PIXEL_LAYOUT_I420 => PixelLayout::I420,
            DAV1D_PIXEL_LAYOUT_I422 => PixelLayout::I422,
            DAV1D_PIXEL_LAYOUT_I444 => PixelLayout::I444,
            _ => unreachable!(),
        }
    }

    /// Timestamp of the frame.
    ///
    /// This is the same timestamp as the one provided to [`Decoder::send_data`].
    pub fn timestamp(&self) -> Option<i64> {
        let ts = self.inner.pic.m.timestamp;
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
        self.inner.pic.m.duration
    }

    /// Offset of the frame.
    ///
    /// This is the same offset as the one provided to [`Decoder::send_data`] or `-1` if none was
    /// provided.
    pub fn offset(&self) -> i64 {
        self.inner.pic.m.offset
    }

    /// Chromaticity coordinates of the source colour primaries.
    pub fn color_primaries(&self) -> pixel::ColorPrimaries {
        unsafe {
            #[allow(non_upper_case_globals)]
            match (*self.inner.pic.seq_hdr).pri {
                DAV1D_COLOR_PRI_BT709 => pixel::ColorPrimaries::BT709,
                DAV1D_COLOR_PRI_UNKNOWN => pixel::ColorPrimaries::Unspecified,
                DAV1D_COLOR_PRI_BT470M => pixel::ColorPrimaries::BT470M,
                DAV1D_COLOR_PRI_BT470BG => pixel::ColorPrimaries::BT470BG,
                DAV1D_COLOR_PRI_BT601 => pixel::ColorPrimaries::BT470BG,
                DAV1D_COLOR_PRI_SMPTE240 => pixel::ColorPrimaries::ST240M,
                DAV1D_COLOR_PRI_FILM => pixel::ColorPrimaries::Film,
                DAV1D_COLOR_PRI_BT2020 => pixel::ColorPrimaries::BT2020,
                DAV1D_COLOR_PRI_XYZ => pixel::ColorPrimaries::ST428,
                DAV1D_COLOR_PRI_SMPTE431 => pixel::ColorPrimaries::P3DCI,
                DAV1D_COLOR_PRI_SMPTE432 => pixel::ColorPrimaries::P3Display,
                DAV1D_COLOR_PRI_EBU3213 => pixel::ColorPrimaries::Tech3213,
                23..=DAV1D_COLOR_PRI_RESERVED => pixel::ColorPrimaries::Unspecified,
                _ => unreachable!(),
            }
        }
    }

    /// Transfer characteristics function.
    pub fn transfer_characteristic(&self) -> pixel::TransferCharacteristic {
        unsafe {
            #[allow(non_upper_case_globals)]
            match (*self.inner.pic.seq_hdr).trc {
                DAV1D_TRC_BT709 => pixel::TransferCharacteristic::BT1886,
                DAV1D_TRC_UNKNOWN => pixel::TransferCharacteristic::Unspecified,
                DAV1D_TRC_BT470M => pixel::TransferCharacteristic::BT470M,
                DAV1D_TRC_BT470BG => pixel::TransferCharacteristic::BT470BG,
                DAV1D_TRC_BT601 => pixel::TransferCharacteristic::ST170M,
                DAV1D_TRC_SMPTE240 => pixel::TransferCharacteristic::ST240M,
                DAV1D_TRC_LINEAR => pixel::TransferCharacteristic::Linear,
                DAV1D_TRC_LOG100 => pixel::TransferCharacteristic::Logarithmic100,
                DAV1D_TRC_LOG100_SQRT10 => pixel::TransferCharacteristic::Logarithmic316,
                DAV1D_TRC_IEC61966 => pixel::TransferCharacteristic::SRGB,
                DAV1D_TRC_BT1361 => pixel::TransferCharacteristic::BT1886,
                DAV1D_TRC_SRGB => pixel::TransferCharacteristic::SRGB,
                DAV1D_TRC_BT2020_10BIT => pixel::TransferCharacteristic::BT2020Ten,
                DAV1D_TRC_BT2020_12BIT => pixel::TransferCharacteristic::BT2020Twelve,
                DAV1D_TRC_SMPTE2084 => pixel::TransferCharacteristic::PerceptualQuantizer,
                DAV1D_TRC_SMPTE428 => pixel::TransferCharacteristic::ST428,
                DAV1D_TRC_HLG => pixel::TransferCharacteristic::HybridLogGamma,
                19..=DAV1D_TRC_RESERVED => pixel::TransferCharacteristic::Unspecified,
                _ => unreachable!(),
            }
        }
    }

    /// Matrix coefficients used in deriving luma and chroma signals from the
    /// green, blue and red or X, Y and Z primaries.
    pub fn matrix_coefficients(&self) -> pixel::MatrixCoefficients {
        unsafe {
            #[allow(non_upper_case_globals)]
            match (*self.inner.pic.seq_hdr).mtrx {
                DAV1D_MC_IDENTITY => pixel::MatrixCoefficients::Identity,
                DAV1D_MC_BT709 => pixel::MatrixCoefficients::BT709,
                DAV1D_MC_UNKNOWN => pixel::MatrixCoefficients::Unspecified,
                DAV1D_MC_FCC => pixel::MatrixCoefficients::BT470M,
                DAV1D_MC_BT470BG => pixel::MatrixCoefficients::BT470BG,
                DAV1D_MC_BT601 => pixel::MatrixCoefficients::BT470BG,
                DAV1D_MC_SMPTE240 => pixel::MatrixCoefficients::ST240M,
                DAV1D_MC_SMPTE_YCGCO => pixel::MatrixCoefficients::YCgCo,
                DAV1D_MC_BT2020_NCL => pixel::MatrixCoefficients::BT2020NonConstantLuminance,
                DAV1D_MC_BT2020_CL => pixel::MatrixCoefficients::BT2020ConstantLuminance,
                DAV1D_MC_SMPTE2085 => pixel::MatrixCoefficients::ST2085,
                DAV1D_MC_CHROMAT_NCL => {
                    pixel::MatrixCoefficients::ChromaticityDerivedNonConstantLuminance
                }
                DAV1D_MC_CHROMAT_CL => {
                    pixel::MatrixCoefficients::ChromaticityDerivedConstantLuminance
                }
                DAV1D_MC_ICTCP => pixel::MatrixCoefficients::ICtCp,
                15..=DAV1D_MC_RESERVED => pixel::MatrixCoefficients::Unspecified,
                _ => unreachable!(),
            }
        }
    }

    /// YUV color range.
    pub fn color_range(&self) -> pixel::YUVRange {
        unsafe {
            match (*self.inner.pic.seq_hdr).color_range {
                0 => pixel::YUVRange::Limited,
                _ => pixel::YUVRange::Full,
            }
        }
    }

    /// Sample position for subsampled chroma.
    pub fn chroma_location(&self) -> pixel::ChromaLocation {
        // According to y4m mapping declared in dav1d's output/y4m2.c and applied from FFmpeg's yuv4mpegdec.c
        unsafe {
            match (*self.inner.pic.seq_hdr).chr {
                DAV1D_CHR_UNKNOWN | DAV1D_CHR_COLOCATED => pixel::ChromaLocation::Center,
                DAV1D_CHR_VERTICAL => pixel::ChromaLocation::Left,
                _ => unreachable!(),
            }
        }
    }

    /// Allocator data of the picture.
    pub fn allocator_data(&self) -> Option<&A::AllocatorData> {
        unsafe {
            if self.inner.pic.allocator_data.is_null() {
                None
            } else {
                Some(&*(self.inner.pic.allocator_data as *const A::AllocatorData))
            }
        }
    }

    /// Content light level information.
    pub fn content_light(&self) -> Option<ContentLightLevel> {
        unsafe {
            if self.inner.pic.content_light.is_null() {
                None
            } else {
                Some(ContentLightLevel {
                    max_content_light_level: (*self.inner.pic.content_light)
                        .max_content_light_level,
                    max_frame_average_light_level: (*self.inner.pic.content_light)
                        .max_frame_average_light_level,
                })
            }
        }
    }

    ///Mastering display information.
    pub fn mastering_display(&self) -> Option<MasteringDisplay> {
        unsafe {
            if self.inner.pic.mastering_display.is_null() {
                None
            } else {
                Some(MasteringDisplay {
                    primaries: (*self.inner.pic.mastering_display).primaries,
                    white_point: (*self.inner.pic.mastering_display).white_point,
                    max_luminance: (*self.inner.pic.mastering_display).max_luminance,
                    min_luminance: (*self.inner.pic.mastering_display).min_luminance,
                })
            }
        }
    }
}

static_assertions::assert_impl_all!(Picture<DefaultAllocator>: Send, Sync, Clone, Debug);

unsafe impl Send for InnerPicture {}
unsafe impl Sync for InnerPicture {}

impl Drop for InnerPicture {
    fn drop(&mut self) {
        unsafe {
            dav1d_picture_unref(&mut self.pic);
        }
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashSet,
        fmt, ptr,
        sync::{self, atomic},
    };

    mod ivf {
        use bitstream_io::{BitRead, BitReader, LittleEndian};
        use std::io;

        #[derive(Debug, PartialEq, Eq)]
        pub struct Header {
            pub tag: [u8; 4],
            pub w: u16,
            pub h: u16,
            pub timebase_num: u32,
            pub timebase_den: u32,
        }

        pub fn read_header(r: &mut dyn io::Read) -> io::Result<Header> {
            let mut br = BitReader::endian(r, LittleEndian);

            let mut signature = [0u8; 4];
            let mut tag = [0u8; 4];

            br.read_bytes(&mut signature)?;
            let _v0: u16 = br.read(16)?;
            let _v1: u16 = br.read(16)?;
            br.read_bytes(&mut tag)?;

            let w: u16 = br.read(16)?;
            let h: u16 = br.read(16)?;

            let timebase_den: u32 = br.read(32)?;
            let timebase_num: u32 = br.read(32)?;

            let _: u32 = br.read(32)?;
            let _: u32 = br.read(32)?;

            Ok(Header {
                tag,
                w,
                h,
                timebase_num,
                timebase_den,
            })
        }

        pub struct Packet {
            pub data: Box<[u8]>,
            pub pts: u64,
        }

        pub fn read_packet(r: &mut dyn io::Read) -> io::Result<Packet> {
            let mut br = BitReader::endian(r, LittleEndian);

            let len: u32 = br.read(32)?;
            let pts: u64 = br.read(64)?;
            let mut buf = vec![0u8; len as usize];

            br.read_bytes(&mut buf)?;

            Ok(Packet {
                data: buf.into_boxed_slice(),
                pts,
            })
        }
    }

    static TEST_FILE_420_8: &[u8] = include_bytes!("../test-420-8.ivf");
    static TEST_FILE_420_12: &[u8] = include_bytes!("../test-420-12.ivf");

    fn handle_pending_pictures<A: super::PictureAllocator + fmt::Debug>(
        dec: &mut super::Decoder<A>,
        pictures: &mut Vec<super::Picture<A>>,
        drain: bool,
    ) {
        loop {
            match dec.get_picture() {
                Ok(p) => {
                    println!("{:?}", p);
                    pictures.push(p);
                }
                // Need to send more data to the decoder before it can decode new pictures
                Err(e) if e.is_again() => return,
                Err(e) => {
                    panic!("Error getting decoded pictures: {}", e);
                }
            }

            if !drain {
                break;
            }
        }
    }

    fn check_pictures<A: super::PictureAllocator>(pictures: &[super::Picture<A>], bpp: usize) {
        assert_eq!(pictures.len(), 5);

        let pts = [0, 33, 67, 100, 133];

        for (i, picture) in pictures.iter().enumerate() {
            assert_eq!(picture.width(), 320);
            assert_eq!(picture.height(), 240);
            assert_eq!(picture.bit_depth(), bpp);
            assert_eq!(
                picture.bits_per_component(),
                Some(super::BitsPerComponent(bpp))
            );
            assert_eq!(picture.pixel_layout(), super::PixelLayout::I420);
            assert_eq!(
                picture.color_primaries(),
                super::pixel::ColorPrimaries::BT709
            );
            assert_eq!(
                picture.transfer_characteristic(),
                super::pixel::TransferCharacteristic::BT1886
            );
            assert_eq!(
                picture.matrix_coefficients(),
                super::pixel::MatrixCoefficients::BT709,
            );
            assert_eq!(
                picture.chroma_location(),
                super::pixel::ChromaLocation::Center,
            );
            assert_eq!(picture.timestamp(), Some(pts[i]));
            assert_eq!(picture.offset(), i as i64);

            let stride_mult = if bpp == 8 { 1 } else { 2 };

            assert!(picture.stride(super::PlanarImageComponent::Y) >= 320 * stride_mult);
            assert!(picture.stride(super::PlanarImageComponent::U) >= 160 * stride_mult);
            assert!(picture.stride(super::PlanarImageComponent::V) >= 160 * stride_mult);

            assert_eq!(
                picture
                    .plane_data_geometry(super::PlanarImageComponent::Y)
                    .1,
                240
            );
            assert_eq!(
                picture
                    .plane_data_geometry(super::PlanarImageComponent::U)
                    .1,
                120
            );
            assert_eq!(
                picture
                    .plane_data_geometry(super::PlanarImageComponent::V)
                    .1,
                120
            );

            assert_eq!(
                picture.plane(super::PlanarImageComponent::Y).len(),
                picture.stride(super::PlanarImageComponent::Y) as usize * 240
            );

            assert_eq!(
                picture.plane(super::PlanarImageComponent::U).len(),
                picture.stride(super::PlanarImageComponent::U) as usize * 120
            );

            assert_eq!(
                picture.plane(super::PlanarImageComponent::V).len(),
                picture.stride(super::PlanarImageComponent::V) as usize * 120
            );
        }
    }

    fn decode_file<A: super::PictureAllocator + fmt::Debug>(
        file: &[u8],
        mut dec: super::Decoder<A>,
        pictures: &mut Vec<super::Picture<A>>,
    ) {
        use std::io;

        let mut r = io::BufReader::new(file);
        let header = ivf::read_header(&mut r).unwrap();
        println!("{:?}", header);

        let mut idx = 0;

        while let Ok(packet) = ivf::read_packet(&mut r) {
            println!("Packet {}", packet.pts);

            // Let's use millisecond timestamps
            let pts =
                1000 * packet.pts as i64 * header.timebase_num as i64 / header.timebase_den as i64;

            // Send packet to the decoder
            match dec.send_data(packet.data, Some(idx), Some(pts), None) {
                Err(e) if e.is_again() => {
                    // If the decoder did not consume all data, output all
                    // pending pictures and send pending data to the decoder
                    // until it is all used up.
                    loop {
                        handle_pending_pictures(&mut dec, pictures, false);

                        match dec.send_pending_data() {
                            Err(e) if e.is_again() => continue,
                            Err(e) => {
                                panic!("Error sending pending data to the decoder: {}", e);
                            }
                            _ => break,
                        }
                    }
                }
                Err(e) => {
                    panic!("Error sending data to the decoder: {}", e);
                }
                _ => (),
            }

            // Handle all pending pictures before sending the next data.
            handle_pending_pictures(&mut dec, pictures, false);

            idx += 1;
        }

        // Handle all pending pictures that were not output yet.
        handle_pending_pictures(&mut dec, pictures, true);
    }

    #[test]
    fn test_basic_420_8() {
        let dec = super::Decoder::new().expect("failed to create decoder instance");
        let mut pictures = vec![];
        decode_file(TEST_FILE_420_8, dec, &mut pictures);
        check_pictures(&pictures, 8);
    }

    #[test]
    fn test_basic_420_12() {
        let dec = super::Decoder::new().expect("failed to create decoder instance");
        let mut pictures = vec![];
        decode_file(TEST_FILE_420_12, dec, &mut pictures);
        check_pictures(&pictures, 12);
    }

    #[derive(Debug)]
    struct TestAllocator {
        counter: atomic::AtomicUsize,
        allocated: sync::Arc<atomic::AtomicUsize>,
    }

    impl TestAllocator {
        fn new(allocated: &sync::Arc<atomic::AtomicUsize>) -> Self {
            TestAllocator {
                counter: atomic::AtomicUsize::new(0),
                allocated: allocated.clone(),
            }
        }
    }

    impl super::PictureAllocator for TestAllocator {
        type AllocatorData = (usize, [std::alloc::Layout; 2]);

        unsafe fn alloc_picture(
            &self,
            pic_params: &crate::PictureParameters,
        ) -> Result<crate::PictureAllocation<Self::AllocatorData>, crate::Error> {
            fn align(x: usize) -> usize {
                (x + 128 - 1) & !(128 - 1)
            }

            let stride_mult = if pic_params.bit_depth == 8 { 1 } else { 2 };

            let (stride, height) = match pic_params.layout {
                crate::PixelLayout::I400 => (
                    [align(pic_params.w as usize) * stride_mult, 0],
                    [align(pic_params.h as usize), 0],
                ),
                crate::PixelLayout::I420 => (
                    [
                        align(pic_params.w as usize) * stride_mult,
                        align((pic_params.w as usize + 1) / 2) * stride_mult,
                    ],
                    [
                        align(pic_params.h as usize),
                        align((pic_params.h as usize + 1) / 2),
                    ],
                ),
                crate::PixelLayout::I422 => (
                    [
                        align(pic_params.w as usize) * stride_mult,
                        align((pic_params.w as usize + 1) / 2) * stride_mult,
                    ],
                    [align(pic_params.h as usize), align(pic_params.h as usize)],
                ),
                crate::PixelLayout::I444 => (
                    [
                        align(pic_params.w as usize) * stride_mult,
                        align(pic_params.w as usize) * stride_mult,
                    ],
                    [align(pic_params.h as usize), align(pic_params.h as usize)],
                ),
            };

            let layout_0 = std::alloc::Layout::from_size_align(
                height[0] * stride[0],
                super::PICTURE_ALIGNMENT,
            )
            .unwrap();

            let data_0 = std::alloc::alloc(layout_0);

            let layout_1;
            let data_1;
            let data_2;
            if stride[1] > 0 {
                layout_1 = std::alloc::Layout::from_size_align(
                    height[1] * stride[1],
                    super::PICTURE_ALIGNMENT,
                )
                .unwrap();
                data_1 = std::alloc::alloc(layout_1);
                data_2 = std::alloc::alloc(layout_1);
            } else {
                layout_1 = layout_0;
                data_1 = ptr::null_mut();
                data_2 = ptr::null_mut();
            }

            self.allocated.fetch_add(1, atomic::Ordering::SeqCst);

            Ok(super::PictureAllocation {
                data: [data_0, data_1, data_2],
                stride: [stride[0] as isize, stride[1] as isize],
                allocator_data: (
                    self.counter.fetch_add(1, atomic::Ordering::SeqCst),
                    [layout_0, layout_1],
                ),
            })
        }

        unsafe fn release_picture(
            &self,
            allocation: crate::PictureAllocation<Self::AllocatorData>,
        ) {
            let prev = self.allocated.fetch_sub(1, atomic::Ordering::SeqCst);
            assert!(prev > 0);

            std::alloc::dealloc(allocation.data[0], allocation.allocator_data.1[0]);
            if !allocation.data[1].is_null() {
                std::alloc::dealloc(allocation.data[1], allocation.allocator_data.1[1]);
                std::alloc::dealloc(allocation.data[2], allocation.allocator_data.1[1]);
            }
        }
    }

    #[test]
    fn test_allocator_420_8() {
        let allocated = sync::Arc::new(atomic::AtomicUsize::new(0));

        let dec = super::Decoder::with_allocator(TestAllocator::new(&allocated))
            .expect("failed to create decoder instance");

        let mut pictures = vec![];
        decode_file(TEST_FILE_420_8, dec, &mut pictures);
        check_pictures(&pictures, 8);

        let mut indices = HashSet::new();
        for picture in &pictures {
            let allocator_data = picture.allocator_data().unwrap();
            assert!(indices.insert(allocator_data.0));
        }
        assert_eq!(indices.len(), 5);

        assert_eq!(allocated.load(atomic::Ordering::SeqCst), 5);
        drop(pictures);
        assert_eq!(allocated.load(atomic::Ordering::SeqCst), 0);
    }

    #[test]
    fn test_allocator_420_12() {
        let allocated = sync::Arc::new(atomic::AtomicUsize::new(0));

        let dec = super::Decoder::with_allocator(TestAllocator::new(&allocated))
            .expect("failed to create decoder instance");

        let mut pictures = vec![];
        decode_file(TEST_FILE_420_12, dec, &mut pictures);
        check_pictures(&pictures, 12);

        let mut indices = HashSet::new();
        for picture in &pictures {
            let allocator_data = picture.allocator_data().unwrap();
            assert!(indices.insert(allocator_data.0));
        }
        assert_eq!(indices.len(), 5);

        assert_eq!(allocated.load(atomic::Ordering::SeqCst), 5);
        drop(pictures);
        assert_eq!(allocated.load(atomic::Ordering::SeqCst), 0);
    }
}

/// Content light level information as specified in CEA-861.3, Appendix A.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContentLightLevel {
    /// Maximum content light level (MaxCLL) in candela per square metre.
    pub max_content_light_level: u16,
    /// Maximum frame average light level (MaxFLL) in candela per square metre.
    pub max_frame_average_light_level: u16,
}

/// Mastering display information as specified in SMPTE ST 2086.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MasteringDisplay {
    /// Red/green/blue XY coordinates of primaries in CIE 1931 color space as 0.16 fixed-point number.
    pub primaries: [[u16; 2usize]; 3usize],
    /// XY coordinates of white point in CIE 1931 color space as 0.16 fixed-point number.
    pub white_point: [u16; 2usize],
    /// Maximum luminance in candela per square metre as 24.8 fixed-point number.
    pub max_luminance: u32,
    /// Minimum luminance in candela per square metre as 18.14 fixed-point number.
    pub min_luminance: u32,
}
