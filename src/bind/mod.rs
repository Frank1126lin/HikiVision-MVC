#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

mod bindings;
use anyhow::Result;
pub use bindings::*;

// 实现default，用于获取GIGE相机信息
impl Default for MV_GIGE_DEVICE_INFO {
    fn default() -> Self {
        MV_GIGE_DEVICE_INFO {
            nIpCfgOption: 0u32,
            nIpCfgCurrent: 0u32,
            nCurrentIp: (192 << 24) | (168 << 16) | (1 << 8) | 2,
            nCurrentSubNetMask: 0u32,
            nDefultGateWay: 0u32,
            chManufacturerName: [0u8; 32usize],
            chModelName: [0u8; 32usize],
            chDeviceVersion: [0u8; 32usize],
            chManufacturerSpecificInfo: [0u8; 48usize],
            chSerialNumber: [0u8; 16usize],
            chUserDefinedName: [0u8; 16usize],
            nNetExport: (192 << 24) | (168 << 16) | (1 << 8) | 1,
            nReserved: [0u32; 4usize],
        }
    }
}

// 实现default，用于获取USB相机信息
impl Default for MV_USB3_DEVICE_INFO {
    fn default() -> Self {
        MV_USB3_DEVICE_INFO {
            CrtlInEndPoint: 0u8,
            CrtlOutEndPoint: 0u8,
            StreamEndPoint: 0u8,
            EventEndPoint: 0u8,
            idVendor: 0u16,
            idProduct: 0u16,
            nDeviceNumber: 0u32,
            chDeviceGUID: [0u8; 64usize],
            chVendorName: [0u8; 64usize],
            chModelName: [0u8; 64usize],
            chFamilyName: [0u8; 64usize],
            chDeviceVersion: [0u8; 64usize],
            chManufacturerName: [0u8; 64usize],
            chSerialNumber: [0u8; 64usize],
            chUserDefinedName: [0u8; 64usize],
            nbcdUSB: 0u32,
            nDeviceAddress: 0u32,
            nReserved: [0u32; 2usize],
        }
    }
}

// 实现default，用于获取Camlink相机信息
impl Default for MV_CamL_DEV_INFO {
    fn default() -> Self {
        MV_CamL_DEV_INFO {
            chPortID: [0u8; 64usize],
            chModelName: [0u8; 64usize],
            chFamilyName: [0u8; 64usize],
            chDeviceVersion: [0u8; 64usize],
            chManufacturerName: [0u8; 64usize],
            chSerialNumber: [0u8; 64usize],
            nReserved: [0u32; 38usize],
        }
    }
}

// 实现default，用于获取相机信息
impl Default for MV_CC_DEVICE_INFO {
    fn default() -> Self {
        MV_CC_DEVICE_INFO {
            nMajorVer: 0u16,
            nMinorVer: 0u16,
            nMacAddrHigh: 0u32,
            nMacAddrLow: 0u32,
            nTLayerType: MV_GIGE_DEVICE,
            nReserved: [0u32; 4usize],
            SpecialInfo: _MV_CC_DEVICE_INFO___bindgen_ty_1 {
                stGigEInfo: MV_GIGE_DEVICE_INFO::default(),
            },
        }
    }
}

// 实现default，用于获取相机列表
impl Default for MV_CC_DEVICE_INFO_LIST {
    fn default() -> Self {
        MV_CC_DEVICE_INFO_LIST {
            nDeviceNum: 0u32,
            pDeviceInfo: [&mut MV_CC_DEVICE_INFO::default(); 256usize],
        }
    }
}

// 实现default，用于获取图像信息
impl Default for MV_FRAME_OUT_INFO_EX {
    fn default() -> Self {
        MV_FRAME_OUT_INFO_EX {
            nWidth: 0u16,
            nHeight: 0u16,
            enPixelType: -1,
            nFrameNum: 0u32,
            nDevTimeStampHigh: 0u32,
            nDevTimeStampLow: 0u32,
            nReserved0: 0u32,
            nHostTimeStamp: 0i64,
            nFrameLen: 0u32,
            nSecondCount: 0u32,
            nCycleCount: 0u32,
            nCycleOffset: 0u32,
            fGain: 0f32,
            fExposureTime: 30000.0f32,
            nAverageBrightness: 0u32,
            nRed: 0u32,
            nGreen: 0u32,
            nBlue: 0u32,
            nFrameCounter: 0u32,
            nTriggerIndex: 0u32,
            nInput: 0u32,
            nOutput: 0u32,
            nOffsetX: 0u16,
            nOffsetY: 0u16,
            nChunkWidth: 0u16,
            nChunkHeight: 0u16,
            nLostPacket: 0u32,
            nUnparsedChunkNum: 0u32,
            UnparsedChunkList: _MV_FRAME_OUT_INFO_EX___bindgen_ty_1 { nAligning: 0i64 },
            nReserved: [0u32; 36usize],
        }
    }
}

// 实现default，用于获取图像信息
impl Default for MV_FRAME_OUT {
    fn default() -> Self {
        MV_FRAME_OUT {
            pBufAddr: &mut 0,
            stFrameInfo: MV_FRAME_OUT_INFO_EX::default(),
            nRes: [0u32; 16usize],
        }
    }
}

// 实现default，用于保存图像
impl Default for MV_SAVE_IMAGE_PARAM_EX {
    fn default() -> Self {
        MV_SAVE_IMAGE_PARAM_EX {
            pData: 0u8 as *mut ::std::os::raw::c_uchar,
            nDataLen: 0u32,
            enPixelType: -1i32,
            nWidth: 0u16,
            nHeight: 0u16,
            pImageBuffer: 0 as *mut ::std::os::raw::c_uchar,
            nImageLen: 0u32,
            nBufferSize: 0u32,
            enImageType: MV_SAVE_IAMGE_TYPE_MV_Image_Undefined,
            nJpgQuality: 95u32,
            iMethodValue: 2u32,
            nReserved: [0u32; 3usize],
        }
    }
}

// 实现default, 用于获取INT属性值
impl Default for MVCC_INTVALUE_EX {
    fn default() -> Self {
        MVCC_INTVALUE_EX {
            nCurValue: 0i64,
            nMax: 0i64,
            nMin: 0i64,
            nInc: 0i64,
            nReserved: [0u32; 16usize],
        }
    }
}

/// ch:获取SDK版本号 | en:Get SDK Version
pub fn get_sdk_version() -> Result<[u8; 4]> {
    let ret = unsafe { MV_CC_GetSDKVersion() };
    // u32 转 [u8;4]
    let ret = ret.to_be_bytes();
    Ok(ret)
}

///brief: 获取支持的传输层
///return: 支持的传输层编号
pub fn enum_tls() -> Result<i32> {
    let ret = unsafe { MV_CC_EnumerateTls() };
    Ok(ret)
}

// ch:枚举设备 | en:Enumerate Device
/// n: u32 ntypelayer: MV_GIGE_DEVICE | MV_USB_DEVICE
/// pstDevList: *mut MV_CC_DEVICE_INFO_LIST
pub fn enum_devices(n: u32, pstDevList: *mut MV_CC_DEVICE_INFO_LIST) -> Result<u32> {
    let ret = unsafe { MV_CC_EnumDevices(n, pstDevList) } as u32;
    Ok(ret)
}

// ch:判断设备是否可达 | en:Is the device accessible
pub fn is_device_accessible(d: *mut MV_CC_DEVICE_INFO, nAm: u32) -> Result<bool> {
    let ret = unsafe { MV_CC_IsDeviceAccessible(d, nAm) };
    match ret {
        0 => Ok(false),
        _ => Ok(true),
    }
}

// 创建句柄
pub fn create_handle(
    handle: &mut *mut ::std::os::raw::c_void,
    pstDevinfo: *const MV_CC_DEVICE_INFO,
) -> Result<u32> {
    let ret = unsafe { MV_CC_CreateHandle(handle, pstDevinfo) } as u32;
    Ok(ret)
}

// 销毁句柄
pub fn destroy_handle(handle: *mut ::std::os::raw::c_void) -> Result<u32> {
    let ret = unsafe { MV_CC_DestroyHandle(handle) } as u32;
    Ok(ret)
}

// ch:打开设备 | en:Open Device
pub fn open(handle: *mut ::std::os::raw::c_void, nam: u32, nsok: u16) -> Result<u32> {
    let nRet = unsafe { MV_CC_OpenDevice(handle, nam, nsok) } as u32;
    Ok(nRet)
}

// ch:关闭设备 | en:Close Device
pub fn close(handle: *mut ::std::os::raw::c_void) -> Result<u32> {
    let nRet = unsafe { MV_CC_CloseDevice(handle) } as u32;
    Ok(nRet)
}

// ch:判断相机是否处于连接状态 | en:Is The Device Connected
pub fn is_device_connected(handle: *mut ::std::os::raw::c_void) -> Result<bool> {
    let ret = unsafe { MV_CC_IsDeviceConnected(handle) };
    match ret {
        0 => Ok(false),
        _ => Ok(true),
    }
}

// ch:开启抓图 | en:Start Grabbing
pub fn start_grabbing(handle: *mut ::std::os::raw::c_void) -> Result<u32> {
    let ret = unsafe { MV_CC_StartGrabbing(handle) } as u32;
    Ok(ret)
}

// ch:停止抓图 | en:Stop Grabbing
pub fn stop_grabbing(handle: *mut ::std::os::raw::c_void) -> Result<u32> {
    let ret = unsafe { MV_CC_StopGrabbing(handle) } as u32;
    Ok(ret)
}

// ch:主动获取一帧图像数据 | en:Get one frame initiatively
pub fn get_image_buffer(
    handle: *mut ::std::os::raw::c_void,
    pframe: *mut MV_FRAME_OUT,
    nMsec: u32,
) -> Result<u32> {
    let ret = unsafe { MV_CC_GetImageBuffer(handle, pframe, nMsec) } as u32;
    Ok(ret)
}

// ch:释放图像缓存 | en:Free image buffer
pub fn free_image_buffer(
    handle: *mut ::std::os::raw::c_void,
    pframe: *mut MV_FRAME_OUT,
) -> Result<u32> {
    let ret = unsafe { MV_CC_FreeImageBuffer(handle, pframe) } as u32;
    Ok(ret)
}

// ch:获取设备信息 | en:Get device information
pub fn get_device_info(
    handle: *mut ::std::os::raw::c_void,
    p: *mut MV_CC_DEVICE_INFO,
) -> Result<u32> {
    let ret = unsafe { MV_CC_GetDeviceInfo(handle, p) } as u32;
    Ok(ret)
}

// ch:强制IP | en:Force IP
pub fn force_ip(
    handle: *mut ::std::os::raw::c_void,
    nip: u32,
    nsubmask: u32,
    ndefaultgateway: u32,
) -> Result<u32> {
    let ret = unsafe { MV_GIGE_ForceIpEx(handle, nip, nsubmask, ndefaultgateway) } as u32;
    Ok(ret)
}

// ch:保存图片 | en:save image
pub fn save_image(
    handle: *mut ::std::os::raw::c_void,
    pSaveParam: *mut MV_SAVE_IMAGE_PARAM_EX,
) -> Result<u32> {
    let ret = unsafe { MV_CC_SaveImageEx2(handle, pSaveParam) } as u32;
    Ok(ret)
}

// 获取属性数值
pub fn set_float_value(
    handle: *mut ::std::os::raw::c_void,
    strKey: *const ::std::os::raw::c_char,
    fValue: f32,
) -> Result<u32> {
    let ret = unsafe { MV_CC_SetFloatValue(handle, strKey, fValue) } as u32;
    Ok(ret)
}

// 设置相机取图格式
pub fn set_pixel_format(
    handle: *mut ::std::os::raw::c_void,
    nValue: ::std::os::raw::c_uint,
) -> Result<u32> {
    let ret = unsafe { MV_CC_SetPixelFormat(handle, nValue) } as u32;
    Ok(ret)
}
