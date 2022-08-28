#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]

use windows::{
    Win32::Media::Audio::*,
};

pub const CPolicyConfigVistaClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x294935ce_f637_4e7c_a41b_ab255460b862);

#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPolicyConfigVista(pub ::windows::core::IUnknown);
impl IPolicyConfigVista {    
    pub unsafe fn SetDefaultEndpoint(&self, wszDeviceId: ::windows::core::PWSTR, role: ERole) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszDeviceId), ::core::mem::transmute(role)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPolicyConfigVista {
    type Vtable = IPolicyConfigVista_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x568b9108_44bf_40b4_9006_86afe5b5a620);
}
impl ::core::convert::From<IPolicyConfigVista> for ::windows::core::IUnknown {
    fn from(value: IPolicyConfigVista) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPolicyConfigVista> for ::windows::core::IUnknown {
    fn from(value: &IPolicyConfigVista) -> Self {
        value.0.clone()
    }
}
// impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPolicyConfigVista {
//     fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
//         ::windows::core::Param::Owned(self.0)
//     }
// }
// impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPolicyConfigVista {
//     fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
//         ::windows::core::Param::Borrowed(&self.0)
//     }
// }
#[repr(C)]
#[doc(hidden)]
pub struct IPolicyConfigVista_abi(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, waveformatex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetMixFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: i32, waveformatex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetDeviceFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, waveformatex0: *mut ::core::ffi::c_void, waveformatex1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetDeviceFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: i32, param1: *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetProcessingPeriod
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetProcessingPeriod
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, devicesharemode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetShareMode
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, devicesharemode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetShareMode
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, key: *mut ::core::ffi::c_void, propvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetPropertyValue
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, key: *mut ::core::ffi::c_void, propvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetPropertyValue
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, role: ERole) -> ::windows::core::HRESULT, // SetDefaultEndpoint
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: i32) -> ::windows::core::HRESULT, // SetEndpointVisibility
);



pub const CPolicyConfigClient: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x870af99c_171d_4f9e_af0d_e63df40c2bc9);

#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPolicyConfig(pub ::windows::core::IUnknown);
impl IPolicyConfig {    
    pub unsafe fn SetDefaultEndpoint(&self, wszDeviceId: ::windows::core::PWSTR, role: ERole) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(wszDeviceId), ::core::mem::transmute(role)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPolicyConfig {
    type Vtable = IPolicyConfig_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf8679f50_850a_41cf_9c72_430f290290c8);
}
impl ::core::convert::From<IPolicyConfig> for ::windows::core::IUnknown {
    fn from(value: IPolicyConfig) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPolicyConfig> for ::windows::core::IUnknown {
    fn from(value: &IPolicyConfig) -> Self {
        value.0.clone()
    }
}
// impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPolicyConfig {
//     fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
//         ::windows::core::Param::Owned(self.0)
//     }
// }
// impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPolicyConfig {
//     fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
//         ::windows::core::Param::Borrowed(&self.0)
//     }
// }
#[repr(C)]
#[doc(hidden)]
pub struct IPolicyConfig_abi(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, waveformatex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetMixFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: i32, waveformatex: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetDeviceFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR) -> ::windows::core::HRESULT, // ResetDeviceFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, waveformatex0: *mut ::core::ffi::c_void, waveformatex1:  *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetDeviceFormat
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: i32, param1: *mut ::core::ffi::c_void, param1:  *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetProcessingPeriod
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetProcessingPeriod
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, devicesharemode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetShareMode
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, devicesharemode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetShareMode
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, key: *mut ::core::ffi::c_void, propvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // GetPropertyValue
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, key: *mut ::core::ffi::c_void, propvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, // SetPropertyValue
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, role: ERole) -> ::windows::core::HRESULT, // SetDefaultEndpoint
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PWSTR, param0: i32) -> ::windows::core::HRESULT, // SetEndpointVisibility
);