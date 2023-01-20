// TODO: Use core::ffi once we depend on Rust >= 1.64
use std::os::raw::{c_char, c_int, c_uint, c_void};

pub const DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;
pub const DAV1D_OBU_TD: Dav1dObuType = 2;
pub const DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const DAV1D_OBU_PADDING: Dav1dObuType = 15;
pub type Dav1dObuType = c_uint;

pub const DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
pub const DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;
pub type Dav1dTxfmMode = c_uint;

pub const DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub const DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;
pub type Dav1dFilterMode = c_uint;

pub const DAV1D_OFF: Dav1dAdaptiveBoolean = 0;
pub const DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;
pub type Dav1dAdaptiveBoolean = c_uint;

pub const DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
pub const DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;
pub type Dav1dRestorationType = c_uint;

pub const DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
pub const DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;
pub type Dav1dWarpedMotionType = c_uint;

pub const DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub const DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;
pub type Dav1dPixelLayout = c_uint;

pub const DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;
pub const DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;
pub type Dav1dFrameType = c_uint;

pub const DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = 1;
pub const DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = 2;
pub const DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = 4;
pub const DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = 5;
pub const DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = 6;
pub const DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = 7;
pub const DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = 8;
pub const DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = 9;
pub const DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = 10;
pub const DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = 11;
pub const DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = 12;
pub const DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = 22;
pub const DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;
pub type Dav1dColorPrimaries = c_uint;

pub const DAV1D_TRC_BT709: Dav1dTransferCharacteristics = 1;
pub const DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics = 2;
pub const DAV1D_TRC_BT470M: Dav1dTransferCharacteristics = 4;
pub const DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics = 5;
pub const DAV1D_TRC_BT601: Dav1dTransferCharacteristics = 6;
pub const DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics = 7;
pub const DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics = 8;
pub const DAV1D_TRC_LOG100: Dav1dTransferCharacteristics = 9;
pub const DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics = 10;
pub const DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics = 11;
pub const DAV1D_TRC_BT1361: Dav1dTransferCharacteristics = 12;
pub const DAV1D_TRC_SRGB: Dav1dTransferCharacteristics = 13;
pub const DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics = 14;
pub const DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics = 15;
pub const DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics = 16;
pub const DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics = 17;
pub const DAV1D_TRC_HLG: Dav1dTransferCharacteristics = 18;
pub const DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;
pub type Dav1dTransferCharacteristics = c_uint;

pub const DAV1D_MC_IDENTITY: Dav1dMatrixCoefficients = 0;
pub const DAV1D_MC_BT709: Dav1dMatrixCoefficients = 1;
pub const DAV1D_MC_UNKNOWN: Dav1dMatrixCoefficients = 2;
pub const DAV1D_MC_FCC: Dav1dMatrixCoefficients = 4;
pub const DAV1D_MC_BT470BG: Dav1dMatrixCoefficients = 5;
pub const DAV1D_MC_BT601: Dav1dMatrixCoefficients = 6;
pub const DAV1D_MC_SMPTE240: Dav1dMatrixCoefficients = 7;
pub const DAV1D_MC_SMPTE_YCGCO: Dav1dMatrixCoefficients = 8;
pub const DAV1D_MC_BT2020_NCL: Dav1dMatrixCoefficients = 9;
pub const DAV1D_MC_BT2020_CL: Dav1dMatrixCoefficients = 10;
pub const DAV1D_MC_SMPTE2085: Dav1dMatrixCoefficients = 11;
pub const DAV1D_MC_CHROMAT_NCL: Dav1dMatrixCoefficients = 12;
pub const DAV1D_MC_CHROMAT_CL: Dav1dMatrixCoefficients = 13;
pub const DAV1D_MC_ICTCP: Dav1dMatrixCoefficients = 14;
pub const DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;
pub type Dav1dMatrixCoefficients = c_uint;

pub const DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;
pub const DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;
pub type Dav1dChromaSamplePosition = c_uint;

pub const DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;
pub const DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub type Dav1dInloopFilterType = c_uint;

pub const DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags = 1;
pub const DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags = 2;
pub type Dav1dEventFlags = c_uint;

pub const DAV1D_MAX_THREADS: c_int = 256;
pub const DAV1D_MAX_FRAME_DELAY: c_int = 256;

pub const DAV1D_MAX_CDEF_STRENGTHS: usize = 8;
pub const DAV1D_MAX_OPERATING_POINTS: usize = 32;
pub const DAV1D_MAX_TILE_COLS: usize = 64;
pub const DAV1D_MAX_TILE_ROWS: usize = 64;
pub const DAV1D_MAX_SEGMENTS: usize = 8;
pub const DAV1D_NUM_REF_FRAMES: usize = 8;
pub const DAV1D_PRIMARY_REF_NONE: usize = 7;
pub const DAV1D_REFS_PER_FRAME: usize = 7;
pub const DAV1D_TOTAL_REFS_PER_FRAME: usize = DAV1D_REFS_PER_FRAME + 1;

#[cfg(feature = "v1_1")]
pub const DAV1D_DECODEFRAMETYPE_ALL: Dav1dDecodeFrameType = 0;
#[cfg(feature = "v1_1")]
pub const DAV1D_DECODEFRAMETYPE_REFERENCE: Dav1dDecodeFrameType = 1;
#[cfg(feature = "v1_1")]
pub const DAV1D_DECODEFRAMETYPE_INTRA: Dav1dDecodeFrameType = 2;
#[cfg(feature = "v1_1")]
pub const DAV1D_DECODEFRAMETYPE_KEY: Dav1dDecodeFrameType = 3;
#[cfg(feature = "v1_1")]
pub type Dav1dDecodeFrameType = c_uint;

// Conversion of the C DAV1D_ERR macro
pub const fn dav1d_err(errno: c_int) -> c_int {
    if libc::EPERM < 0 {
        errno
    } else {
        -errno
    }
}

pub const DAV1D_ERR_AGAIN: c_int = dav1d_err(libc::EAGAIN);
pub const DAV1D_ERR_INVAL: c_int = dav1d_err(libc::EINVAL);
pub const DAV1D_ERR_NOMEM: c_int = dav1d_err(libc::ENOMEM);
pub const DAV1D_ERR_NOPROTOOPT: c_int = dav1d_err(libc::ENOPROTOOPT);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dUserData {
    pub data: *const u8,
    pub ref_: *mut Dav1dRef,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dDataProps {
    pub timestamp: i64,
    pub duration: i64,
    pub offset: i64,
    pub size: usize,
    pub user_data: Dav1dUserData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dWarpedMotionParams {
    pub type_: Dav1dWarpedMotionType,
    pub matrix: [i32; 6usize],
    pub u: Dav1dWarpedMotionParamsU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Dav1dWarpedMotionParamsU {
    pub p: Dav1dWarpedMotionParamsUP,
    pub abcd: [i16; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dWarpedMotionParamsUP {
    pub alpha: i16,
    pub beta: i16,
    pub gamma: i16,
    pub delta: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dContentLightLevel {
    pub max_content_light_level: c_int,
    pub max_frame_average_light_level: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dMasteringDisplay {
    pub primaries: [[u16; 2usize]; 3usize],
    pub white_point: [u16; 2usize],
    pub max_luminance: u32,
    pub min_luminance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dITUTT35 {
    pub country_code: u8,
    pub country_code_extension_byte: u8,
    pub payload_size: usize,
    pub payload: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSequenceHeader {
    pub profile: c_int,
    pub max_width: c_int,
    pub max_height: c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: c_int,
    pub color_range: c_int,
    pub num_operating_points: c_int,
    pub operating_points: [Dav1dSequenceHeaderOperatingPoint; DAV1D_MAX_OPERATING_POINTS],
    pub still_picture: c_int,
    pub reduced_still_picture_header: c_int,
    pub timing_info_present: c_int,
    pub num_units_in_tick: c_int,
    pub time_scale: c_int,
    pub equal_picture_interval: c_int,
    pub num_ticks_per_picture: c_uint,
    pub decoder_model_info_present: c_int,
    pub encoder_decoder_buffer_delay_length: c_int,
    pub num_units_in_decoding_tick: c_int,
    pub buffer_removal_delay_length: c_int,
    pub frame_presentation_delay_length: c_int,
    pub display_model_info_present: c_int,
    pub width_n_bits: c_int,
    pub height_n_bits: c_int,
    pub frame_id_numbers_present: c_int,
    pub delta_frame_id_n_bits: c_int,
    pub frame_id_n_bits: c_int,
    pub sb128: c_int,
    pub filter_intra: c_int,
    pub intra_edge_filter: c_int,
    pub inter_intra: c_int,
    pub masked_compound: c_int,
    pub warped_motion: c_int,
    pub dual_filter: c_int,
    pub order_hint: c_int,
    pub jnt_comp: c_int,
    pub ref_frame_mvs: c_int,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: c_int,
    pub super_res: c_int,
    pub cdef: c_int,
    pub restoration: c_int,
    pub ss_hor: c_int,
    pub ss_ver: c_int,
    pub monochrome: c_int,
    pub color_description_present: c_int,
    pub separate_uv_delta_q: c_int,
    pub film_grain_present: c_int,
    pub operating_parameter_info:
        [Dav1dSequenceHeaderOperatingParameterInfo; DAV1D_MAX_OPERATING_POINTS],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSequenceHeaderOperatingPoint {
    pub major_level: c_int,
    pub minor_level: c_int,
    pub initial_display_delay: c_int,
    pub idc: c_int,
    pub tier: c_int,
    pub decoder_model_param_present: c_int,
    pub display_model_param_present: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: c_int,
    pub encoder_buffer_delay: c_int,
    pub low_delay_mode: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSegmentationData {
    pub delta_q: c_int,
    pub delta_lf_y_v: c_int,
    pub delta_lf_y_h: c_int,
    pub delta_lf_u: c_int,
    pub delta_lf_v: c_int,
    pub ref_: c_int,
    pub skip: c_int,
    pub globalmv: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; DAV1D_MAX_SEGMENTS],
    pub preskip: c_int,
    pub last_active_segid: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [c_int; 2usize],
    pub ref_delta: [c_int; DAV1D_TOTAL_REFS_PER_FRAME],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFilmGrainData {
    pub seed: c_uint,
    pub num_y_points: c_int,
    pub y_points: [[u8; 2usize]; 14usize],
    pub chroma_scaling_from_luma: c_int,
    pub num_uv_points: [c_int; 2usize],
    pub uv_points: [[[u8; 2usize]; 10usize]; 2usize],
    pub scaling_shift: c_int,
    pub ar_coeff_lag: c_int,
    pub ar_coeffs_y: [i8; 24usize],
    pub ar_coeffs_uv: [[i8; 28usize]; 2usize],
    pub ar_coeff_shift: u64,
    pub grain_scale_shift: c_int,
    pub uv_mult: [c_int; 2usize],
    pub uv_luma_mult: [c_int; 2usize],
    pub uv_offset: [c_int; 2usize],
    pub overlap_flag: c_int,
    pub clip_to_restricted_range: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFrameHeader {
    pub film_grain: Dav1dFrameHeaderFilmGrain,
    pub frame_type: Dav1dFrameType,
    pub width: [c_int; 2usize],
    pub height: c_int,
    pub frame_offset: c_int,
    pub temporal_id: c_int,
    pub spatial_id: c_int,
    pub show_existing_frame: c_int,
    pub existing_frame_idx: c_int,
    pub frame_id: c_int,
    pub frame_presentation_delay: c_int,
    pub show_frame: c_int,
    pub showable_frame: c_int,
    pub error_resilient_mode: c_int,
    pub disable_cdf_update: c_int,
    pub allow_screen_content_tools: c_int,
    pub force_integer_mv: c_int,
    pub frame_size_override: c_int,
    pub primary_ref_frame: c_int,
    pub buffer_removal_time_present: c_int,
    pub operating_points: [Dav1dFrameHeaderOperatingPoint; DAV1D_MAX_OPERATING_POINTS],
    pub refresh_frame_flags: c_int,
    pub render_width: c_int,
    pub render_height: c_int,
    pub super_res: Dav1dFrameHeaderSuperRes,
    pub have_render_size: c_int,
    pub allow_intrabc: c_int,
    pub frame_ref_short_signaling: c_int,
    pub refidx: [c_int; DAV1D_REFS_PER_FRAME],
    pub hp: c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: c_int,
    pub use_ref_frame_mvs: c_int,
    pub refresh_context: c_int,
    pub tiling: Dav1dFrameHeaderTiling,
    pub quant: Dav1dFrameHeaderQuant,
    pub segmentation: Dav1dFrameHeaderSegmentation,
    pub delta: Dav1dFrameHeaderDelta,
    pub all_lossless: c_int,
    pub loopfilter: Dav1dFrameHeaderLoopfilter,
    pub cdef: Dav1dFrameHeaderCDef,
    pub restoration: Dav1dFrameHeaderRestoration,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: c_int,
    pub skip_mode_allowed: c_int,
    pub skip_mode_enabled: c_int,
    pub skip_mode_refs: [c_int; 2usize],
    pub warp_motion: c_int,
    pub reduced_txtp_set: c_int,
    pub gmv: [Dav1dWarpedMotionParams; DAV1D_REFS_PER_FRAME],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderFilmGrain {
    pub data: Dav1dFilmGrainData,
    pub present: c_int,
    pub update: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderSuperRes {
    pub width_scale_denominator: c_int,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderTiling {
    pub uniform: c_int,
    pub n_bytes: c_uint,
    pub min_log2_cols: c_int,
    pub max_log2_cols: c_int,
    pub log2_cols: c_int,
    pub cols: c_int,
    pub min_log2_rows: c_int,
    pub max_log2_rows: c_int,
    pub log2_rows: c_int,
    pub rows: c_int,
    pub col_start_sb: [u16; DAV1D_MAX_TILE_COLS + 1],
    pub row_start_sb: [u16; DAV1D_MAX_TILE_ROWS + 1],
    pub update: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderQuant {
    pub yac: c_int,
    pub ydc_delta: c_int,
    pub udc_delta: c_int,
    pub uac_delta: c_int,
    pub vdc_delta: c_int,
    pub vac_delta: c_int,
    pub qm: c_int,
    pub qm_y: c_int,
    pub qm_u: c_int,
    pub qm_v: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderSegmentation {
    pub enabled: c_int,
    pub update_map: c_int,
    pub temporal: c_int,
    pub update_data: c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [c_int; DAV1D_MAX_SEGMENTS],
    pub qidx: [c_int; DAV1D_MAX_SEGMENTS],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderDelta {
    pub q: Dav1dDeltaQ,
    pub lf: Dav1dDeltaLF,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dDeltaQ {
    pub present: c_int,
    pub res_log2: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dDeltaLF {
    pub present: c_int,
    pub res_log2: c_int,
    pub multi: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderLoopfilter {
    pub level_y: [c_int; 2usize],
    pub level_u: c_int,
    pub level_v: c_int,
    pub mode_ref_delta_enabled: c_int,
    pub mode_ref_delta_update: c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderCDef {
    pub damping: c_int,
    pub n_bits: c_int,
    pub y_strength: [c_int; DAV1D_MAX_CDEF_STRENGTHS],
    pub uv_strength: [c_int; DAV1D_MAX_CDEF_STRENGTHS],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeaderRestoration {
    pub type_: [Dav1dRestorationType; 3usize],
    pub unit_size: [c_int; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dPictureParameters {
    pub w: c_int,
    pub h: c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut c_void; 3usize],
    pub stride: [isize; 2usize],
    pub p: Dav1dPictureParameters,
    pub m: Dav1dDataProps,
    pub content_light: *mut Dav1dContentLightLevel,
    pub mastering_display: *mut Dav1dMasteringDisplay,
    pub itut_t35: *mut Dav1dITUTT35,
    pub reserved: [usize; 4usize],
    pub frame_hdr_ref: *mut Dav1dRef,
    pub seq_hdr_ref: *mut Dav1dRef,
    pub content_light_ref: *mut Dav1dRef,
    pub mastering_display_ref: *mut Dav1dRef,
    pub itut_t35_ref: *mut Dav1dRef,
    pub reserved_ref: [usize; 4usize],
    pub ref_: *mut Dav1dRef,
    pub allocator_data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dPicAllocator {
    pub cookie: *mut c_void,
    pub alloc_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void) -> c_int>,
    pub release_picture_callback:
        Option<unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut c_void)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dData {
    pub data: *const u8,
    pub sz: usize,
    pub ref_: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Dav1dContext(c_void);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dLogger {
    pub cookie: *mut c_void,
    pub callback: *mut c_void,
    // FIXME: Use the following once std::ffi::VaList is stable
    // Option<
    //     unsafe extern "C" fn(
    //         cookie: *mut c_void,
    //         format: *const c_char,
    //         ap: *mut ::std::ffi::VaList,
    //     ),
    //  >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSettings {
    pub n_threads: c_int,
    pub max_frame_delay: c_int,
    pub apply_grain: c_int,
    pub operating_point: c_int,
    pub all_layers: c_int,
    pub frame_size_limit: c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: c_int,
    pub output_invisible_frames: c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    #[cfg(feature = "v1_1")]
    pub decode_frame_type: Dav1dDecodeFrameType,
    #[cfg(feature = "v1_1")]
    pub reserved: [u8; 20usize],
    #[cfg(not(feature = "v1_1"))]
    pub reserved: [u8; 16usize],
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Dav1dRef(c_void);

extern "C" {
    pub fn dav1d_version() -> *const c_char;

    pub fn dav1d_default_settings(s: *mut Dav1dSettings);

    pub fn dav1d_parse_sequence_header(
        out: *mut Dav1dSequenceHeader,
        buf: *const u8,
        sz: usize,
    ) -> c_int;

    pub fn dav1d_open(c_out: *mut *mut Dav1dContext, s: *const Dav1dSettings) -> c_int;

    pub fn dav1d_send_data(c: *mut Dav1dContext, in_: *mut Dav1dData) -> c_int;

    pub fn dav1d_flush(c: *mut Dav1dContext);

    pub fn dav1d_get_picture(c: *mut Dav1dContext, out: *mut Dav1dPicture) -> c_int;

    pub fn dav1d_get_decode_error_data_props(
        c: *mut Dav1dContext,
        out: *mut Dav1dDataProps,
    ) -> c_int;

    #[cfg(feature = "v1_1")]
    pub fn dav1d_get_frame_delay(c: *mut Dav1dContext) -> c_int;

    pub fn dav1d_apply_grain(
        c: *mut Dav1dContext,
        out: *mut Dav1dPicture,
        in_: *const Dav1dPicture,
    ) -> c_int;

    pub fn dav1d_get_event_flags(c: *mut Dav1dContext, flags: *mut Dav1dEventFlags) -> c_int;

    pub fn dav1d_close(c_out: *mut *mut Dav1dContext);

    pub fn dav1d_picture_unref(p: *mut Dav1dPicture);

    pub fn dav1d_data_props_unref(props: *mut Dav1dDataProps);

    pub fn dav1d_data_create(data: *mut Dav1dData, sz: usize) -> *mut u8;

    pub fn dav1d_data_wrap(
        data: *mut Dav1dData,
        buf: *const u8,
        sz: usize,
        free_callback: Option<unsafe extern "C" fn(buf: *const u8, cookie: *mut c_void)>,
        cookie: *mut c_void,
    ) -> c_int;

    pub fn dav1d_data_wrap_user_data(
        data: *mut Dav1dData,
        user_data: *const u8,
        free_callback: Option<unsafe extern "C" fn(user_data: *const u8, cookie: *mut c_void)>,
        cookie: *mut c_void,
    ) -> c_int;

    pub fn dav1d_data_unref(data: *mut Dav1dData);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn version() {
        println!("{}", unsafe {
            CStr::from_ptr(dav1d_version()).to_string_lossy()
        });
    }
}
