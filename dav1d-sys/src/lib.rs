#![allow(non_upper_case_globals)]

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

pub const Dav1dObuType_DAV1D_OBU_SEQ_HDR: Dav1dObuType = 1;
pub const Dav1dObuType_DAV1D_OBU_TD: Dav1dObuType = 2;
pub const Dav1dObuType_DAV1D_OBU_FRAME_HDR: Dav1dObuType = 3;
pub const Dav1dObuType_DAV1D_OBU_TILE_GRP: Dav1dObuType = 4;
pub const Dav1dObuType_DAV1D_OBU_METADATA: Dav1dObuType = 5;
pub const Dav1dObuType_DAV1D_OBU_FRAME: Dav1dObuType = 6;
pub const Dav1dObuType_DAV1D_OBU_REDUNDANT_FRAME_HDR: Dav1dObuType = 7;
pub const Dav1dObuType_DAV1D_OBU_PADDING: Dav1dObuType = 15;

pub type Dav1dObuType = ::std::os::raw::c_uint;
pub const Dav1dTxfmMode_DAV1D_TX_4X4_ONLY: Dav1dTxfmMode = 0;
pub const Dav1dTxfmMode_DAV1D_TX_LARGEST: Dav1dTxfmMode = 1;
pub const Dav1dTxfmMode_DAV1D_TX_SWITCHABLE: Dav1dTxfmMode = 2;
pub const Dav1dTxfmMode_DAV1D_N_TX_MODES: Dav1dTxfmMode = 3;

pub type Dav1dTxfmMode = ::std::os::raw::c_uint;
pub const Dav1dFilterMode_DAV1D_FILTER_8TAP_REGULAR: Dav1dFilterMode = 0;
pub const Dav1dFilterMode_DAV1D_FILTER_8TAP_SMOOTH: Dav1dFilterMode = 1;
pub const Dav1dFilterMode_DAV1D_FILTER_8TAP_SHARP: Dav1dFilterMode = 2;
pub const Dav1dFilterMode_DAV1D_N_SWITCHABLE_FILTERS: Dav1dFilterMode = 3;
pub const Dav1dFilterMode_DAV1D_FILTER_BILINEAR: Dav1dFilterMode = 3;
pub const Dav1dFilterMode_DAV1D_N_FILTERS: Dav1dFilterMode = 4;
pub const Dav1dFilterMode_DAV1D_FILTER_SWITCHABLE: Dav1dFilterMode = 4;

pub type Dav1dFilterMode = ::std::os::raw::c_uint;
pub const Dav1dAdaptiveBoolean_DAV1D_OFF: Dav1dAdaptiveBoolean = 0;
pub const Dav1dAdaptiveBoolean_DAV1D_ON: Dav1dAdaptiveBoolean = 1;
pub const Dav1dAdaptiveBoolean_DAV1D_ADAPTIVE: Dav1dAdaptiveBoolean = 2;

pub type Dav1dAdaptiveBoolean = ::std::os::raw::c_uint;
pub const Dav1dRestorationType_DAV1D_RESTORATION_NONE: Dav1dRestorationType = 0;
pub const Dav1dRestorationType_DAV1D_RESTORATION_SWITCHABLE: Dav1dRestorationType = 1;
pub const Dav1dRestorationType_DAV1D_RESTORATION_WIENER: Dav1dRestorationType = 2;
pub const Dav1dRestorationType_DAV1D_RESTORATION_SGRPROJ: Dav1dRestorationType = 3;

pub type Dav1dRestorationType = ::std::os::raw::c_uint;
pub const Dav1dWarpedMotionType_DAV1D_WM_TYPE_IDENTITY: Dav1dWarpedMotionType = 0;
pub const Dav1dWarpedMotionType_DAV1D_WM_TYPE_TRANSLATION: Dav1dWarpedMotionType = 1;
pub const Dav1dWarpedMotionType_DAV1D_WM_TYPE_ROT_ZOOM: Dav1dWarpedMotionType = 2;
pub const Dav1dWarpedMotionType_DAV1D_WM_TYPE_AFFINE: Dav1dWarpedMotionType = 3;

pub type Dav1dWarpedMotionType = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dWarpedMotionParams {
    pub type_: Dav1dWarpedMotionType,
    pub matrix: [i32; 6usize],
    pub u: Dav1dWarpedMotionParams__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Dav1dWarpedMotionParams__bindgen_ty_1 {
    pub p: Dav1dWarpedMotionParams__bindgen_ty_1__bindgen_ty_1,
    pub abcd: [i16; 4usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dWarpedMotionParams__bindgen_ty_1__bindgen_ty_1 {
    pub alpha: i16,
    pub beta: i16,
    pub gamma: i16,
    pub delta: i16,
}

pub const Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I400: Dav1dPixelLayout = 0;
pub const Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I420: Dav1dPixelLayout = 1;
pub const Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I422: Dav1dPixelLayout = 2;
pub const Dav1dPixelLayout_DAV1D_PIXEL_LAYOUT_I444: Dav1dPixelLayout = 3;

pub type Dav1dPixelLayout = ::std::os::raw::c_uint;
pub const Dav1dFrameType_DAV1D_FRAME_TYPE_KEY: Dav1dFrameType = 0;
pub const Dav1dFrameType_DAV1D_FRAME_TYPE_INTER: Dav1dFrameType = 1;
pub const Dav1dFrameType_DAV1D_FRAME_TYPE_INTRA: Dav1dFrameType = 2;
pub const Dav1dFrameType_DAV1D_FRAME_TYPE_SWITCH: Dav1dFrameType = 3;

pub type Dav1dFrameType = ::std::os::raw::c_uint;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_BT709: Dav1dColorPrimaries = 1;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_UNKNOWN: Dav1dColorPrimaries = 2;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_BT470M: Dav1dColorPrimaries = 4;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_BT470BG: Dav1dColorPrimaries = 5;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_BT601: Dav1dColorPrimaries = 6;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_SMPTE240: Dav1dColorPrimaries = 7;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_FILM: Dav1dColorPrimaries = 8;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_BT2020: Dav1dColorPrimaries = 9;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_XYZ: Dav1dColorPrimaries = 10;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_SMPTE431: Dav1dColorPrimaries = 11;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_SMPTE432: Dav1dColorPrimaries = 12;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_EBU3213: Dav1dColorPrimaries = 22;
pub const Dav1dColorPrimaries_DAV1D_COLOR_PRI_RESERVED: Dav1dColorPrimaries = 255;

pub type Dav1dColorPrimaries = ::std::os::raw::c_uint;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT709: Dav1dTransferCharacteristics = 1;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_UNKNOWN: Dav1dTransferCharacteristics = 2;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT470M: Dav1dTransferCharacteristics = 4;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT470BG: Dav1dTransferCharacteristics = 5;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT601: Dav1dTransferCharacteristics = 6;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_SMPTE240: Dav1dTransferCharacteristics = 7;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_LINEAR: Dav1dTransferCharacteristics = 8;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_LOG100: Dav1dTransferCharacteristics = 9;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_LOG100_SQRT10: Dav1dTransferCharacteristics = 10;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_IEC61966: Dav1dTransferCharacteristics = 11;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT1361: Dav1dTransferCharacteristics = 12;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_SRGB: Dav1dTransferCharacteristics = 13;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT2020_10BIT: Dav1dTransferCharacteristics = 14;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_BT2020_12BIT: Dav1dTransferCharacteristics = 15;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_SMPTE2084: Dav1dTransferCharacteristics = 16;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_SMPTE428: Dav1dTransferCharacteristics = 17;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_HLG: Dav1dTransferCharacteristics = 18;
pub const Dav1dTransferCharacteristics_DAV1D_TRC_RESERVED: Dav1dTransferCharacteristics = 255;

pub type Dav1dTransferCharacteristics = ::std::os::raw::c_uint;
pub const Dav1dMatrixCoefficients_DAV1D_MC_IDENTITY: Dav1dMatrixCoefficients = 0;
pub const Dav1dMatrixCoefficients_DAV1D_MC_BT709: Dav1dMatrixCoefficients = 1;
pub const Dav1dMatrixCoefficients_DAV1D_MC_UNKNOWN: Dav1dMatrixCoefficients = 2;
pub const Dav1dMatrixCoefficients_DAV1D_MC_FCC: Dav1dMatrixCoefficients = 4;
pub const Dav1dMatrixCoefficients_DAV1D_MC_BT470BG: Dav1dMatrixCoefficients = 5;
pub const Dav1dMatrixCoefficients_DAV1D_MC_BT601: Dav1dMatrixCoefficients = 6;
pub const Dav1dMatrixCoefficients_DAV1D_MC_SMPTE240: Dav1dMatrixCoefficients = 7;
pub const Dav1dMatrixCoefficients_DAV1D_MC_SMPTE_YCGCO: Dav1dMatrixCoefficients = 8;
pub const Dav1dMatrixCoefficients_DAV1D_MC_BT2020_NCL: Dav1dMatrixCoefficients = 9;
pub const Dav1dMatrixCoefficients_DAV1D_MC_BT2020_CL: Dav1dMatrixCoefficients = 10;
pub const Dav1dMatrixCoefficients_DAV1D_MC_SMPTE2085: Dav1dMatrixCoefficients = 11;
pub const Dav1dMatrixCoefficients_DAV1D_MC_CHROMAT_NCL: Dav1dMatrixCoefficients = 12;
pub const Dav1dMatrixCoefficients_DAV1D_MC_CHROMAT_CL: Dav1dMatrixCoefficients = 13;
pub const Dav1dMatrixCoefficients_DAV1D_MC_ICTCP: Dav1dMatrixCoefficients = 14;
pub const Dav1dMatrixCoefficients_DAV1D_MC_RESERVED: Dav1dMatrixCoefficients = 255;

pub type Dav1dMatrixCoefficients = ::std::os::raw::c_uint;
pub const Dav1dChromaSamplePosition_DAV1D_CHR_UNKNOWN: Dav1dChromaSamplePosition = 0;
pub const Dav1dChromaSamplePosition_DAV1D_CHR_VERTICAL: Dav1dChromaSamplePosition = 1;
pub const Dav1dChromaSamplePosition_DAV1D_CHR_COLOCATED: Dav1dChromaSamplePosition = 2;

pub type Dav1dChromaSamplePosition = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dContentLightLevel {
    pub max_content_light_level: ::std::os::raw::c_int,
    pub max_frame_average_light_level: ::std::os::raw::c_int,
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
    pub profile: ::std::os::raw::c_int,
    pub max_width: ::std::os::raw::c_int,
    pub max_height: ::std::os::raw::c_int,
    pub layout: Dav1dPixelLayout,
    pub pri: Dav1dColorPrimaries,
    pub trc: Dav1dTransferCharacteristics,
    pub mtrx: Dav1dMatrixCoefficients,
    pub chr: Dav1dChromaSamplePosition,
    pub hbd: ::std::os::raw::c_int,
    pub color_range: ::std::os::raw::c_int,
    pub num_operating_points: ::std::os::raw::c_int,
    pub operating_points: [Dav1dSequenceHeader_Dav1dSequenceHeaderOperatingPoint; 32usize],
    pub still_picture: ::std::os::raw::c_int,
    pub reduced_still_picture_header: ::std::os::raw::c_int,
    pub timing_info_present: ::std::os::raw::c_int,
    pub num_units_in_tick: ::std::os::raw::c_int,
    pub time_scale: ::std::os::raw::c_int,
    pub equal_picture_interval: ::std::os::raw::c_int,
    pub num_ticks_per_picture: ::std::os::raw::c_uint,
    pub decoder_model_info_present: ::std::os::raw::c_int,
    pub encoder_decoder_buffer_delay_length: ::std::os::raw::c_int,
    pub num_units_in_decoding_tick: ::std::os::raw::c_int,
    pub buffer_removal_delay_length: ::std::os::raw::c_int,
    pub frame_presentation_delay_length: ::std::os::raw::c_int,
    pub display_model_info_present: ::std::os::raw::c_int,
    pub width_n_bits: ::std::os::raw::c_int,
    pub height_n_bits: ::std::os::raw::c_int,
    pub frame_id_numbers_present: ::std::os::raw::c_int,
    pub delta_frame_id_n_bits: ::std::os::raw::c_int,
    pub frame_id_n_bits: ::std::os::raw::c_int,
    pub sb128: ::std::os::raw::c_int,
    pub filter_intra: ::std::os::raw::c_int,
    pub intra_edge_filter: ::std::os::raw::c_int,
    pub inter_intra: ::std::os::raw::c_int,
    pub masked_compound: ::std::os::raw::c_int,
    pub warped_motion: ::std::os::raw::c_int,
    pub dual_filter: ::std::os::raw::c_int,
    pub order_hint: ::std::os::raw::c_int,
    pub jnt_comp: ::std::os::raw::c_int,
    pub ref_frame_mvs: ::std::os::raw::c_int,
    pub screen_content_tools: Dav1dAdaptiveBoolean,
    pub force_integer_mv: Dav1dAdaptiveBoolean,
    pub order_hint_n_bits: ::std::os::raw::c_int,
    pub super_res: ::std::os::raw::c_int,
    pub cdef: ::std::os::raw::c_int,
    pub restoration: ::std::os::raw::c_int,
    pub ss_hor: ::std::os::raw::c_int,
    pub ss_ver: ::std::os::raw::c_int,
    pub monochrome: ::std::os::raw::c_int,
    pub color_description_present: ::std::os::raw::c_int,
    pub separate_uv_delta_q: ::std::os::raw::c_int,
    pub film_grain_present: ::std::os::raw::c_int,
    pub operating_parameter_info:
        [Dav1dSequenceHeader_Dav1dSequenceHeaderOperatingParameterInfo; 32usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSequenceHeader_Dav1dSequenceHeaderOperatingPoint {
    pub major_level: ::std::os::raw::c_int,
    pub minor_level: ::std::os::raw::c_int,
    pub initial_display_delay: ::std::os::raw::c_int,
    pub idc: ::std::os::raw::c_int,
    pub tier: ::std::os::raw::c_int,
    pub decoder_model_param_present: ::std::os::raw::c_int,
    pub display_model_param_present: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSequenceHeader_Dav1dSequenceHeaderOperatingParameterInfo {
    pub decoder_buffer_delay: ::std::os::raw::c_int,
    pub encoder_buffer_delay: ::std::os::raw::c_int,
    pub low_delay_mode: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSegmentationData {
    pub delta_q: ::std::os::raw::c_int,
    pub delta_lf_y_v: ::std::os::raw::c_int,
    pub delta_lf_y_h: ::std::os::raw::c_int,
    pub delta_lf_u: ::std::os::raw::c_int,
    pub delta_lf_v: ::std::os::raw::c_int,
    pub ref_: ::std::os::raw::c_int,
    pub skip: ::std::os::raw::c_int,
    pub globalmv: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSegmentationDataSet {
    pub d: [Dav1dSegmentationData; 8usize],
    pub preskip: ::std::os::raw::c_int,
    pub last_active_segid: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dLoopfilterModeRefDeltas {
    pub mode_delta: [::std::os::raw::c_int; 2usize],
    pub ref_delta: [::std::os::raw::c_int; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFilmGrainData {
    pub seed: ::std::os::raw::c_uint,
    pub num_y_points: ::std::os::raw::c_int,
    pub y_points: [[u8; 2usize]; 14usize],
    pub chroma_scaling_from_luma: ::std::os::raw::c_int,
    pub num_uv_points: [::std::os::raw::c_int; 2usize],
    pub uv_points: [[[u8; 2usize]; 10usize]; 2usize],
    pub scaling_shift: ::std::os::raw::c_int,
    pub ar_coeff_lag: ::std::os::raw::c_int,
    pub ar_coeffs_y: [i8; 24usize],
    pub ar_coeffs_uv: [[i8; 28usize]; 2usize],
    pub ar_coeff_shift: u64,
    pub grain_scale_shift: ::std::os::raw::c_int,
    pub uv_mult: [::std::os::raw::c_int; 2usize],
    pub uv_luma_mult: [::std::os::raw::c_int; 2usize],
    pub uv_offset: [::std::os::raw::c_int; 2usize],
    pub overlap_flag: ::std::os::raw::c_int,
    pub clip_to_restricted_range: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dav1dFrameHeader {
    pub film_grain: Dav1dFrameHeader__bindgen_ty_1,
    pub frame_type: Dav1dFrameType,
    pub width: [::std::os::raw::c_int; 2usize],
    pub height: ::std::os::raw::c_int,
    pub frame_offset: ::std::os::raw::c_int,
    pub temporal_id: ::std::os::raw::c_int,
    pub spatial_id: ::std::os::raw::c_int,
    pub show_existing_frame: ::std::os::raw::c_int,
    pub existing_frame_idx: ::std::os::raw::c_int,
    pub frame_id: ::std::os::raw::c_int,
    pub frame_presentation_delay: ::std::os::raw::c_int,
    pub show_frame: ::std::os::raw::c_int,
    pub showable_frame: ::std::os::raw::c_int,
    pub error_resilient_mode: ::std::os::raw::c_int,
    pub disable_cdf_update: ::std::os::raw::c_int,
    pub allow_screen_content_tools: ::std::os::raw::c_int,
    pub force_integer_mv: ::std::os::raw::c_int,
    pub frame_size_override: ::std::os::raw::c_int,
    pub primary_ref_frame: ::std::os::raw::c_int,
    pub buffer_removal_time_present: ::std::os::raw::c_int,
    pub operating_points: [Dav1dFrameHeader_Dav1dFrameHeaderOperatingPoint; 32usize],
    pub refresh_frame_flags: ::std::os::raw::c_int,
    pub render_width: ::std::os::raw::c_int,
    pub render_height: ::std::os::raw::c_int,
    pub super_res: Dav1dFrameHeader__bindgen_ty_2,
    pub have_render_size: ::std::os::raw::c_int,
    pub allow_intrabc: ::std::os::raw::c_int,
    pub frame_ref_short_signaling: ::std::os::raw::c_int,
    pub refidx: [::std::os::raw::c_int; 7usize],
    pub hp: ::std::os::raw::c_int,
    pub subpel_filter_mode: Dav1dFilterMode,
    pub switchable_motion_mode: ::std::os::raw::c_int,
    pub use_ref_frame_mvs: ::std::os::raw::c_int,
    pub refresh_context: ::std::os::raw::c_int,
    pub tiling: Dav1dFrameHeader__bindgen_ty_3,
    pub quant: Dav1dFrameHeader__bindgen_ty_4,
    pub segmentation: Dav1dFrameHeader__bindgen_ty_5,
    pub delta: Dav1dFrameHeader__bindgen_ty_6,
    pub all_lossless: ::std::os::raw::c_int,
    pub loopfilter: Dav1dFrameHeader__bindgen_ty_7,
    pub cdef: Dav1dFrameHeader__bindgen_ty_8,
    pub restoration: Dav1dFrameHeader__bindgen_ty_9,
    pub txfm_mode: Dav1dTxfmMode,
    pub switchable_comp_refs: ::std::os::raw::c_int,
    pub skip_mode_allowed: ::std::os::raw::c_int,
    pub skip_mode_enabled: ::std::os::raw::c_int,
    pub skip_mode_refs: [::std::os::raw::c_int; 2usize],
    pub warp_motion: ::std::os::raw::c_int,
    pub reduced_txtp_set: ::std::os::raw::c_int,
    pub gmv: [Dav1dWarpedMotionParams; 7usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_1 {
    pub data: Dav1dFilmGrainData,
    pub present: ::std::os::raw::c_int,
    pub update: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader_Dav1dFrameHeaderOperatingPoint {
    pub buffer_removal_time: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_2 {
    pub width_scale_denominator: ::std::os::raw::c_int,
    pub enabled: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_3 {
    pub uniform: ::std::os::raw::c_int,
    pub n_bytes: ::std::os::raw::c_uint,
    pub min_log2_cols: ::std::os::raw::c_int,
    pub max_log2_cols: ::std::os::raw::c_int,
    pub log2_cols: ::std::os::raw::c_int,
    pub cols: ::std::os::raw::c_int,
    pub min_log2_rows: ::std::os::raw::c_int,
    pub max_log2_rows: ::std::os::raw::c_int,
    pub log2_rows: ::std::os::raw::c_int,
    pub rows: ::std::os::raw::c_int,
    pub col_start_sb: [u16; 65usize],
    pub row_start_sb: [u16; 65usize],
    pub update: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_4 {
    pub yac: ::std::os::raw::c_int,
    pub ydc_delta: ::std::os::raw::c_int,
    pub udc_delta: ::std::os::raw::c_int,
    pub uac_delta: ::std::os::raw::c_int,
    pub vdc_delta: ::std::os::raw::c_int,
    pub vac_delta: ::std::os::raw::c_int,
    pub qm: ::std::os::raw::c_int,
    pub qm_y: ::std::os::raw::c_int,
    pub qm_u: ::std::os::raw::c_int,
    pub qm_v: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_5 {
    pub enabled: ::std::os::raw::c_int,
    pub update_map: ::std::os::raw::c_int,
    pub temporal: ::std::os::raw::c_int,
    pub update_data: ::std::os::raw::c_int,
    pub seg_data: Dav1dSegmentationDataSet,
    pub lossless: [::std::os::raw::c_int; 8usize],
    pub qidx: [::std::os::raw::c_int; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_6 {
    pub q: Dav1dFrameHeader__bindgen_ty_6__bindgen_ty_1,
    pub lf: Dav1dFrameHeader__bindgen_ty_6__bindgen_ty_2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_6__bindgen_ty_1 {
    pub present: ::std::os::raw::c_int,
    pub res_log2: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_6__bindgen_ty_2 {
    pub present: ::std::os::raw::c_int,
    pub res_log2: ::std::os::raw::c_int,
    pub multi: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_7 {
    pub level_y: [::std::os::raw::c_int; 2usize],
    pub level_u: ::std::os::raw::c_int,
    pub level_v: ::std::os::raw::c_int,
    pub mode_ref_delta_enabled: ::std::os::raw::c_int,
    pub mode_ref_delta_update: ::std::os::raw::c_int,
    pub mode_ref_deltas: Dav1dLoopfilterModeRefDeltas,
    pub sharpness: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_8 {
    pub damping: ::std::os::raw::c_int,
    pub n_bits: ::std::os::raw::c_int,
    pub y_strength: [::std::os::raw::c_int; 8usize],
    pub uv_strength: [::std::os::raw::c_int; 8usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dFrameHeader__bindgen_ty_9 {
    pub type_: [Dav1dRestorationType; 3usize],
    pub unit_size: [::std::os::raw::c_int; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dPictureParameters {
    pub w: ::std::os::raw::c_int,
    pub h: ::std::os::raw::c_int,
    pub layout: Dav1dPixelLayout,
    pub bpc: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dPicture {
    pub seq_hdr: *mut Dav1dSequenceHeader,
    pub frame_hdr: *mut Dav1dFrameHeader,
    pub data: [*mut ::std::os::raw::c_void; 3usize],
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
    pub allocator_data: *mut ::std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dPicAllocator {
    pub cookie: *mut ::std::os::raw::c_void,
    pub alloc_picture_callback: ::std::option::Option<
        unsafe extern "C" fn(
            pic: *mut Dav1dPicture,
            cookie: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub release_picture_callback: ::std::option::Option<
        unsafe extern "C" fn(pic: *mut Dav1dPicture, cookie: *mut ::std::os::raw::c_void),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dData {
    pub data: *const u8,
    pub sz: usize,
    pub ref_: *mut Dav1dRef,
    pub m: Dav1dDataProps,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dContext {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dLogger {
    pub cookie: *mut ::std::os::raw::c_void,
    pub callback: *mut ::std::os::raw::c_void,
    // FIXME: Use the following once std::ffi::VaList is stable
    // ::std::option::Option<
    //     unsafe extern "C" fn(
    //         cookie: *mut ::std::os::raw::c_void,
    //         format: *const ::std::os::raw::c_char,
    //         ap: *mut ::std::ffi::VaList,
    //     ),
    //  >,
}

pub const Dav1dInloopFilterType_DAV1D_INLOOPFILTER_NONE: Dav1dInloopFilterType = 0;
pub const Dav1dInloopFilterType_DAV1D_INLOOPFILTER_DEBLOCK: Dav1dInloopFilterType = 1;
pub const Dav1dInloopFilterType_DAV1D_INLOOPFILTER_CDEF: Dav1dInloopFilterType = 2;
pub const Dav1dInloopFilterType_DAV1D_INLOOPFILTER_RESTORATION: Dav1dInloopFilterType = 4;
pub const Dav1dInloopFilterType_DAV1D_INLOOPFILTER_ALL: Dav1dInloopFilterType = 7;
pub type Dav1dInloopFilterType = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dSettings {
    pub n_threads: ::std::os::raw::c_int,
    pub max_frame_delay: ::std::os::raw::c_int,
    pub apply_grain: ::std::os::raw::c_int,
    pub operating_point: ::std::os::raw::c_int,
    pub all_layers: ::std::os::raw::c_int,
    pub frame_size_limit: ::std::os::raw::c_uint,
    pub allocator: Dav1dPicAllocator,
    pub logger: Dav1dLogger,
    pub strict_std_compliance: ::std::os::raw::c_int,
    pub output_invisible_frames: ::std::os::raw::c_int,
    pub inloop_filters: Dav1dInloopFilterType,
    pub reserved: [u8; 20usize],
}

pub const Dav1dEventFlags_DAV1D_EVENT_FLAG_NEW_SEQUENCE: Dav1dEventFlags = 1;
pub const Dav1dEventFlags_DAV1D_EVENT_FLAG_NEW_OP_PARAMS_INFO: Dav1dEventFlags = 2;
pub type Dav1dEventFlags = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Dav1dRef {
    pub _address: u8,
}

extern "C" {
    pub fn dav1d_version() -> *const ::std::os::raw::c_char;

    pub fn dav1d_default_settings(s: *mut Dav1dSettings);

    pub fn dav1d_parse_sequence_header(
        out: *mut Dav1dSequenceHeader,
        buf: *const u8,
        sz: usize,
    ) -> ::std::os::raw::c_int;

    pub fn dav1d_open(
        c_out: *mut *mut Dav1dContext,
        s: *const Dav1dSettings,
    ) -> ::std::os::raw::c_int;

    pub fn dav1d_send_data(c: *mut Dav1dContext, in_: *mut Dav1dData) -> ::std::os::raw::c_int;

    pub fn dav1d_flush(c: *mut Dav1dContext);

    pub fn dav1d_get_picture(c: *mut Dav1dContext, out: *mut Dav1dPicture)
        -> ::std::os::raw::c_int;

    pub fn dav1d_get_decode_error_data_props(
        c: *mut Dav1dContext,
        out: *mut Dav1dDataProps,
    ) -> ::std::os::raw::c_int;

    pub fn dav1d_apply_grain(
        c: *mut Dav1dContext,
        out: *mut Dav1dPicture,
        in_: *const Dav1dPicture,
    ) -> ::std::os::raw::c_int;

    pub fn dav1d_get_event_flags(
        c: *mut Dav1dContext,
        flags: *mut Dav1dEventFlags,
    ) -> ::std::os::raw::c_int;

    pub fn dav1d_close(c_out: *mut *mut Dav1dContext);

    pub fn dav1d_picture_unref(p: *mut Dav1dPicture);

    pub fn dav1d_data_props_unref(props: *mut Dav1dDataProps);

    pub fn dav1d_data_create(data: *mut Dav1dData, sz: usize) -> *mut u8;

    pub fn dav1d_data_wrap(
        data: *mut Dav1dData,
        buf: *const u8,
        sz: usize,
        free_callback: ::std::option::Option<
            unsafe extern "C" fn(buf: *const u8, cookie: *mut ::std::os::raw::c_void),
        >,
        cookie: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

    pub fn dav1d_data_wrap_user_data(
        data: *mut Dav1dData,
        user_data: *const u8,
        free_callback: ::std::option::Option<
            unsafe extern "C" fn(user_data: *const u8, cookie: *mut ::std::os::raw::c_void),
        >,
        cookie: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;

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
