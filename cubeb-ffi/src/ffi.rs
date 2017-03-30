use std::os::raw::{c_long,c_void};
use std::ops;
use std::default::Default;

#[macro_export]
macro_rules! log_internal {
    ($level: expr, $msg: expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            if $level <= $crate::g_log_level {
                if let Some(log_callback) = $crate::g_log_callback {
                    let cstr = ::std::ffi::CString::new(concat!("%s:%d: ", $msg, "\n")).unwrap();
                    log_callback(cstr.as_ptr(), file!(), line!());
                }
            }
        }
    };
    ($level: expr, $fmt: expr, $($arg:tt)+) => {
        #[allow(unused_unsafe)]
        unsafe {
            if $level <= $crate::g_log_level {
                if let Some(log_callback) = $crate::g_log_callback {
                    let cstr = ::std::ffi::CString::new(concat!("%s:%d: ", $fmt, "\n")).unwrap();
                    log_callback(cstr.as_ptr(), file!(), line!(), $($arg)+);
                }
            }
        }
    }
}

#[macro_export]
macro_rules! logv {
    ($msg: expr) => (log_internal!($crate::LogLevel::Verbose, $msg));
    ($fmt: expr, $($arg: tt)+) => (log_internal!($create::LogLevel::Verbose, $fmt, $($arg)*));
}

#[macro_export]
macro_rules! log {
    ($msg: expr) => (log_internal!($crate::LogLevel::Normal, $msg));
    ($fmt: expr, $($arg: tt)+) => (log_internal!($crate::LogLevel::Normal, $fmt, $($arg)*));
}

pub enum Context {}
pub enum Stream {}

// TODO endian check
pub const SAMPLE_S16NE: SampleFormat = SampleFormat::S16LE;
pub const SAMPLE_FLOAT32NE: SampleFormat = SampleFormat::Float32LE;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SampleFormat {
    S16LE = 0,
    S16BE = 1,
    Float32LE = 2,
    Float32BE = 3,
}

pub type DeviceId = *const c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Disabled = 0,
    Normal = 1,
    Verbose = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChannelLayout {
    Undefined = 0,
    DualMono = 1,
    DualMonoLfe = 2,
    Mono = 3,
    MonoLfe = 4,
    Stereo = 5,
    StereoLfe = 6,
    Layout3F = 7,
    Layout3FLfe = 8,
    Layout2F1 = 9,
    Layout2F1Lfe = 10,
    Layout3F1 = 11,
    Layout3F1Lfe = 12,
    Layout2F2 = 13,
    Layout2F2Lfe = 14,
    Layout3F2 = 15,
    Layout3F2Lfe = 16,
    Layout3F3RLfe = 17,
    Layout3F4Lfe = 18,
    Max = 19,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct StreamParams {
    pub format: SampleFormat,
    pub rate: u32,
    pub channels: u32,
    pub layout: ChannelLayout,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Device {
    pub output_name: *mut i8,
    pub input_name: *mut i8,
}

impl Default for Device {
    fn default() -> Self {
        Device {
            output_name: 0 as *mut _,
            input_name: 0 as *mut _,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum State {
    Started = 0,
    Stopped = 1,
    Drained = 2,
    Error = 3,
}

pub const OK: i32 = 0;
pub const ERROR: i32 = -1;
pub const ERROR_INVALID_FORMAT: i32 = -2;
pub const ERROR_INVALID_PARAMETER: i32 = -3;
pub const ERROR_NOT_SUPPORTED: i32 = -4;
pub const ERROR_DEVICE_UNAVAILABLE: i32 = -5;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DeviceType(u32);

const DEVICE_TYPE_UNKNOWN: u32 = 0b00;
const DEVICE_TYPE_INPUT:u32 = 0b01;
const DEVICE_TYPE_OUTPUT: u32 = 0b10;
const DEVICE_TYPE_ALL: u32 = 0b11;

impl DeviceType {
    pub fn unknown() -> Self { DeviceType(DEVICE_TYPE_UNKNOWN) }

    #[inline] pub fn input() -> Self { DeviceType(DEVICE_TYPE_INPUT) }
    #[inline] pub fn output() -> Self { DeviceType(DEVICE_TYPE_OUTPUT) }
    #[inline] pub fn all() -> Self { DeviceType(DEVICE_TYPE_ALL) }

    #[inline] pub fn is_input(&self) -> bool { self.contains(DeviceType::input()) }
    #[inline] pub fn is_output(&self) -> bool { self.contains(DeviceType::output()) }

    #[inline] pub fn contains(&self, other: Self) -> bool { (*self & other) == other }
    #[inline] pub fn insert(&mut self, other: Self) { self.0 |= other.0; }
    #[inline] pub fn remove(&mut self, other: Self) { self.0 &= !other.0; }
}

impl ops::BitOr for DeviceType {
    type Output = DeviceType;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        DeviceType(self.0 | other.0)
    }
}

impl ops::BitXor for DeviceType {
    type Output = DeviceType;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        DeviceType(self.0 ^ other.0)
    }
}

impl ops::BitAnd for DeviceType {
    type Output = DeviceType;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        DeviceType(self.0 & other.0)
    }
}

impl ops::Sub for DeviceType {
    type Output = DeviceType;

    #[inline]
    fn sub(self, other: Self) -> Self {
        DeviceType(self.0 & !other.0)
    }
}

impl ops::Not for DeviceType {
    type Output = DeviceType;

    #[inline]
    fn not(self) -> Self {
        DeviceType(!self.0 & DEVICE_FMT_ALL)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceState {
    Disabled = 0,
    Unplugged = 1,
    Enabled = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DeviceFmt(u32);

const DEVICE_FMT_S16LE: u32 = 0x0010;
const DEVICE_FMT_S16BE: u32 = 0x0020;
const DEVICE_FMT_F32LE: u32 = 0x1000;
const DEVICE_FMT_F32BE: u32 = 0x2000;
const DEVICE_FMT_S16_MASK: u32 = DEVICE_FMT_S16LE | DEVICE_FMT_S16BE;
const DEVICE_FMT_F32_MASK: u32 = DEVICE_FMT_F32LE | DEVICE_FMT_F32BE;
const DEVICE_FMT_ALL: u32   = DEVICE_FMT_S16_MASK | DEVICE_FMT_F32_MASK;


impl DeviceFmt {
    pub fn empty() -> Self { DeviceFmt(0) }

    #[inline] pub fn s16_le() -> Self { DeviceFmt(DEVICE_FMT_S16LE) }
    #[inline] pub fn s16_be() -> Self { DeviceFmt(DEVICE_FMT_S16BE) }
    #[inline] pub fn f32_le() -> Self { DeviceFmt(DEVICE_FMT_F32LE) }
    #[inline] pub fn f32_be() -> Self { DeviceFmt(DEVICE_FMT_F32BE) }
    #[inline] pub fn all() -> Self { DeviceFmt(DEVICE_FMT_ALL) }

    #[inline] pub fn s16_ne() -> Self {
        if cfg!(target_endian = "little") {
            DeviceFmt::s16_le()
        } else {
            DeviceFmt::s16_be()
        }
    }

    #[inline] pub fn f32_ne() -> Self {
        if cfg!(target_endian = "little") {
            DeviceFmt::f32_le()
        } else {
            DeviceFmt::f32_be()
        }
    }

    #[inline] pub fn contains(&self, other: Self) -> bool { (*self & other) == other }
    #[inline] pub fn insert(&mut self, other: Self) { self.0 |= other.0; }
    #[inline] pub fn remove(&mut self, other: Self) { self.0 &= !other.0; }
}

impl ops::BitOr for DeviceFmt {
    type Output = DeviceFmt;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        DeviceFmt(self.0 | other.0)
    }
}

impl ops::BitXor for DeviceFmt {
    type Output = DeviceFmt;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        DeviceFmt(self.0 ^ other.0)
    }
}

impl ops::BitAnd for DeviceFmt {
    type Output = DeviceFmt;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        DeviceFmt(self.0 & other.0)
    }
}

impl ops::Sub for DeviceFmt {
    type Output = DeviceFmt;

    #[inline]
    fn sub(self, other: Self) -> Self {
        DeviceFmt(self.0 & !other.0)
    }
}

impl ops::Not for DeviceFmt {
    type Output = DeviceFmt;

    #[inline]
    fn not(self) -> Self {
        DeviceFmt(!self.0 & DEVICE_FMT_ALL)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DevicePref(u32);

const DEVICE_PREF_MULTIMEDIA: u32 = 0x1;
const DEVICE_PREF_VOICE: u32 = 0x2;
const DEVICE_PREF_NOTIFICATION: u32 = 0x4;
const DEVICE_PREF_ALL: u32 = 0xF;

impl DevicePref {
    pub fn none() -> Self { DevicePref(0) }

    #[inline] pub fn multimedia() -> Self { DevicePref(DEVICE_PREF_MULTIMEDIA) }
    #[inline] pub fn voice() -> Self { DevicePref(DEVICE_PREF_VOICE) }
    #[inline] pub fn notification() -> Self { DevicePref(DEVICE_PREF_NOTIFICATION) }
    #[inline] pub fn all() -> Self { DevicePref(DEVICE_PREF_ALL) }

    #[inline] pub fn contains(&self, other: Self) -> bool { (*self & other) == other }
    #[inline] pub fn insert(&mut self, other: Self) { self.0 |= other.0; }
    #[inline] pub fn remove(&mut self, other: Self) { self.0 &= !other.0; }
}

impl ops::BitOr for DevicePref {
    type Output = DevicePref;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        DevicePref(self.0 | other.0)
    }
}

impl ops::BitXor for DevicePref {
    type Output = DevicePref;

    #[inline]
    fn bitxor(self, other: Self) -> Self {
        DevicePref(self.0 ^ other.0)
    }
}

impl ops::BitAnd for DevicePref {
    type Output = DevicePref;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        DevicePref(self.0 & other.0)
    }
}

impl ops::Sub for DevicePref {
    type Output = DevicePref;

    #[inline]
    fn sub(self, other: Self) -> Self {
        DevicePref(self.0 & !other.0)
    }
}

impl ops::Not for DevicePref {
    type Output = DevicePref;

    #[inline]
    fn not(self) -> Self {
        DevicePref(!self.0 & DEVICE_FMT_ALL)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceInfo {
    pub devid: DeviceId,
    pub device_id: *const i8,
    pub friendly_name: *const i8,
    pub group_id: *const i8,
    pub vendor_name: *const i8,
    pub devtype: DeviceType,
    pub state: DeviceState,
    pub preferred: DevicePref,
    pub format: DeviceFmt,
    pub default_format: DeviceFmt,
    pub max_channels: u32,
    pub default_rate: u32,
    pub max_rate: u32,
    pub min_rate: u32,
    pub latency_lo: u32,
    pub latency_hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceCollection {
    /// Device count in collection.
    pub count: u32,
    /// Array of pointers to device info.
    pub device: [*const DeviceInfo; 0],
}

pub type DataCallback = Option<unsafe extern "C" fn(stream: *mut Stream, user_ptr: *mut c_void, input_buffer: *const c_void, output_buffer: *mut c_void, nframes: i64) -> i64>;
pub type StateCallback = Option<unsafe extern "C" fn(stream: *mut Stream, user_ptr: *mut c_void, state: State)>;
pub type DeviceChangedCallback = Option<unsafe extern "C" fn(user_ptr: *mut c_void)>;
pub type DeviceCollectionChangedCallback = Option<unsafe extern "C" fn(context: *mut Context, user_ptr: *mut c_void)>;
pub type LogCallback = Option<unsafe extern "C" fn(fmt: *const i8, ...)>;

#[repr(C)]
pub struct Ops {
    pub init: Option<unsafe extern "C" fn(context: *mut *mut Context, context_name: *const i8) -> i32>,
    pub get_backend_id: Option<unsafe extern "C" fn(context: *mut Context) -> *const i8>,
    pub get_max_channel_count: Option<unsafe extern "C" fn(context: *mut Context, max_channels: *mut u32) -> i32>,
    pub get_min_latency:  Option<unsafe extern "C" fn(context: *mut Context, params: StreamParams, latency_ms: *mut u32) -> i32>,
    pub get_preferred_sample_rate:  Option<unsafe extern "C" fn(context: *mut Context, rate: *mut u32) -> i32>,
    pub get_preferred_channel_layout:  Option<unsafe extern "C" fn(context: *mut Context, layout: *mut ChannelLayout) -> i32>,
    pub enumerate_devices:  Option<unsafe extern "C" fn(context: *mut Context, devtype: DeviceType, collection: *mut *mut DeviceCollection) -> i32>,
    pub destroy:  Option<unsafe extern "C" fn(context: *mut Context)>,
    pub stream_init: Option<unsafe extern "C" fn(context: *mut Context, stream: *mut *mut Stream, stream_name: *const i8, input_device: DeviceId, input_stream_params: *mut StreamParams, output_device: DeviceId, output_stream_params: *mut StreamParams, latency: u32, data_callback: DataCallback, state_callback: StateCallback, user_ptr: *mut c_void) -> i32>,
    pub stream_destroy: Option<unsafe extern "C" fn(stream: *mut Stream)>,
    pub stream_start: Option<unsafe extern "C" fn(stream: *mut Stream) -> i32>,
    pub stream_stop: Option<unsafe extern "C" fn(stream: *mut Stream) -> i32>,
    pub stream_get_position: Option<unsafe extern "C" fn(stream: *mut Stream, position: *mut u64) -> i32>,
    pub stream_get_latency: Option<unsafe extern "C" fn(stream: *mut Stream, latency: *mut u32) -> i32>,
    pub stream_set_volume: Option<unsafe extern "C" fn(stream: *mut Stream, volumes: f32) -> i32>,
    pub stream_set_panning: Option<unsafe extern "C" fn(stream: *mut Stream, panning: f32)-> i32>,
    pub stream_get_current_device: Option<unsafe extern "C" fn(stream: *mut Stream, device: *mut *const Device) -> i32>,
    pub stream_device_destroy: Option<unsafe extern "C" fn(stream: *mut Stream, device: *mut Device) -> i32>,
    pub stream_register_device_changed_callback: Option<unsafe extern "C" fn(stream: *mut Stream, device_changed_callback: DeviceChangedCallback) -> i32>,
    pub register_device_collection_changed: Option<unsafe extern "C" fn(context: *mut Context, devtype: DeviceType, callback: DeviceCollectionChangedCallback, user_ptr: *mut c_void) -> i32>
}

extern "C" {
    pub static g_log_level: LogLevel;
    pub static g_log_callback: LogCallback;
}

#[repr(C)]
#[derive(Clone,Copy,Debug)]
pub struct LayoutMap {
    pub name: *const i8,
    pub channels: u32,
    pub layout: ChannelLayout
}

// cubeb_mixer.h
#[repr(C)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum Channel {
  Invalid           = -1,
  Mono              = 0,
  Left              = 1,
  Right             = 2,
  Center            = 3,
  LeftSurround      = 4,
  RightSurround     = 5,
  RearLeftSurround  = 6,
  RearCenter        = 7,
  RearRightSurround = 8,
  LowFrequency      = 9,
  Max               = 10
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ChannelMap {
    pub channels: u32,
    pub map: [Channel;Channel::Max as usize],
}
impl ::std::default::Default for ChannelMap {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    pub fn cubeb_channel_map_to_layout(channel_map: *const ChannelMap) -> ChannelLayout;
    pub fn cubeb_should_upmix(stream: *const StreamParams, mixer: *const StreamParams) -> bool;
    pub fn cubeb_should_downmix(stream: *const StreamParams, mixer: *const StreamParams) -> bool;
    pub fn cubeb_downmix_float(input: *const f32, inframes: c_long, output: *mut f32,
                               in_channels: u32, out_channels: u32,
                               in_layout: ChannelLayout, out_layout: ChannelLayout);
    pub fn cubeb_upmix_float(input: *const f32, inframes: c_long, output: *mut f32,
                             in_channels: u32, out_channels: u32);
}

#[test]
fn bindgen_test_layout_stream_params() {
    assert_eq!(::std::mem::size_of::<StreamParams>(),
               16usize,
               concat!("Size of: ", stringify!(StreamParams)));
    assert_eq!(::std::mem::align_of::<StreamParams>(),
               4usize,
               concat!("Alignment of ", stringify!(StreamParams)));
    assert_eq!(unsafe { &(*(0 as *const StreamParams)).format as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(StreamParams),
                       "::",
                       stringify!(format)));
    assert_eq!(unsafe { &(*(0 as *const StreamParams)).rate as *const _ as usize },
               4usize,
               concat!("Alignment of field: ",
                       stringify!(StreamParams),
                       "::",
                       stringify!(rate)));
    assert_eq!(unsafe { &(*(0 as *const StreamParams)).channels as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(StreamParams),
                       "::",
                       stringify!(channels)));
    assert_eq!(unsafe { &(*(0 as *const StreamParams)).layout as *const _ as usize },
               12usize,
               concat!("Alignment of field: ",
                       stringify!(StreamParams),
                       "::",
                       stringify!(layout)));
}

#[test]
fn bindgen_test_layout_cubeb_device() {
    assert_eq!(::std::mem::size_of::<Device>(),
               16usize,
               concat!("Size of: ", stringify!(Device)));
    assert_eq!(::std::mem::align_of::<Device>(),
               8usize,
               concat!("Alignment of ", stringify!(Device)));
    assert_eq!(unsafe { &(*(0 as *const Device)).output_name as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(Device),
                       "::",
                       stringify!(output_name)));
    assert_eq!(unsafe { &(*(0 as *const Device)).input_name as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(Device),
                       "::",
                       stringify!(input_name)));
}

#[test]
fn bindgen_test_layout_cubeb_device_info() {
    assert_eq!(::std::mem::size_of::<DeviceInfo>(),
               88usize,
               concat!("Size of: ", stringify!(DeviceInfo)));
    assert_eq!(::std::mem::align_of::<DeviceInfo>(),
               8usize,
               concat!("Alignment of ", stringify!(DeviceInfo)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).devid as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(devid)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).device_id as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(device_id)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).friendly_name as *const _ as usize },
               16usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(friendly_name)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).group_id as *const _ as usize },
               24usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(group_id)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).vendor_name as *const _ as usize },
               32usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(vendor_name)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).devtype as *const _ as usize },
               40usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(type_)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).state as *const _ as usize },
               44usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(state)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).preferred as *const _ as usize },
               48usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(preferred)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).format as *const _ as usize },
               52usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(format)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).default_format as *const _ as usize },
               56usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(default_format)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).max_channels as *const _ as usize },
               60usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(max_channels)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).default_rate as *const _ as usize },
               64usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(default_rate)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).max_rate as *const _ as usize },
               68usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(max_rate)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).min_rate as *const _ as usize },
               72usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(min_rate)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).latency_lo as *const _ as usize },
               76usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(latency_lo)));
    assert_eq!(unsafe { &(*(0 as *const DeviceInfo)).latency_hi as *const _ as usize },
               80usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceInfo),
                       "::",
                       stringify!(latency_hi)));
}

#[test]
fn bindgen_test_layout_cubeb_device_collection() {
    assert_eq!(::std::mem::size_of::<DeviceCollection>(),
               16usize,
               concat!("Size of: ", stringify!(DeviceCollection)));
    assert_eq!(::std::mem::align_of::<DeviceCollection>(),
               8usize,
               concat!("Alignment of ", stringify!(DeviceCollection)));
    assert_eq!(unsafe { &(*(0 as *const DeviceCollection)).count as *const _ as usize },
               0usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceCollection),
                       "::",
                       stringify!(count)));
    assert_eq!(unsafe { &(*(0 as *const DeviceCollection)).device as *const _ as usize },
               8usize,
               concat!("Alignment of field: ",
                       stringify!(DeviceCollection),
                       "::",
                       stringify!(device)));

}

#[test]
fn test_normal_logging() {
    log!("This is log at normal level");
}

#[test]
fn test_verbose_logging() {
    logv!("This is a log at verbose level");
}
