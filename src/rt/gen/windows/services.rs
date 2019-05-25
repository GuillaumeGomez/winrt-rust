pub mod cortana { // Windows.Services.Cortana
use crate::prelude::*;
DEFINE_IID!(IID_ICortanaActionableInsights, 2501822129, 64643, 22637, 139, 132, 36, 82, 200, 152, 22, 37);
RT_INTERFACE!{interface ICortanaActionableInsights(ICortanaActionableInsightsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaActionableInsights] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT,
    fn IsAvailableAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn ShowInsightsForImageAsync(&self, imageStream: *mut super::super::storage::streams::IRandomAccessStreamReference, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn ShowInsightsForImageWithOptionsAsync(&self, imageStream: *mut super::super::storage::streams::IRandomAccessStreamReference, options: *mut CortanaActionableInsightsOptions, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ShowInsightsForTextAsync(&self, text: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ShowInsightsForTextWithOptionsAsync(&self, text: HSTRING, options: *mut CortanaActionableInsightsOptions, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn ShowInsightsAsync(&self, datapackage: *mut super::super::applicationmodel::datatransfer::DataPackage, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn ShowInsightsWithOptionsAsync(&self, datapackage: *mut super::super::applicationmodel::datatransfer::DataPackage, options: *mut CortanaActionableInsightsOptions, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<ICortanaActionableInsights> {
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::super::system::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_User)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn is_available_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).IsAvailableAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn show_insights_for_image_async(&self, imageStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ShowInsightsForImageAsync)(self.as_abi() as *const _ as *mut _, imageStream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn show_insights_for_image_with_options_async(&self, imageStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>, options: &ComPtr<CortanaActionableInsightsOptions>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ShowInsightsForImageWithOptionsAsync)(self.as_abi() as *const _ as *mut _, imageStream.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_insights_for_text_async(&self, text: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ShowInsightsForTextAsync)(self.as_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_insights_for_text_with_options_async(&self, text: &HStringArg, options: &ComPtr<CortanaActionableInsightsOptions>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ShowInsightsForTextWithOptionsAsync)(self.as_abi() as *const _ as *mut _, text.get(), options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn show_insights_async(&self, datapackage: &ComPtr<super::super::applicationmodel::datatransfer::DataPackage>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ShowInsightsAsync)(self.as_abi() as *const _ as *mut _, datapackage.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn show_insights_with_options_async(&self, datapackage: &ComPtr<super::super::applicationmodel::datatransfer::DataPackage>, options: &ComPtr<CortanaActionableInsightsOptions>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ShowInsightsWithOptionsAsync)(self.as_abi() as *const _ as *mut _, datapackage.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CortanaActionableInsights: ICortanaActionableInsights}
impl RtActivatable<ICortanaActionableInsightsStatics> for CortanaActionableInsights {}
impl CortanaActionableInsights {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<CortanaActionableInsights>>> {
        <Self as RtActivatable<ICortanaActionableInsightsStatics>>::get_activation_factory().get_default()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &ComPtr<super::super::system::User>) -> Result<Option<ComPtr<CortanaActionableInsights>>> {
        <Self as RtActivatable<ICortanaActionableInsightsStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(CortanaActionableInsights(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,67,111,114,116,97,110,97,46,67,111,114,116,97,110,97,65,99,116,105,111,110,97,98,108,101,73,110,115,105,103,104,116,115,0]) [CLSID_CortanaActionableInsights]);
DEFINE_IID!(IID_ICortanaActionableInsightsOptions, 2864888783, 38786, 21536, 184, 30, 122, 229, 106, 243, 24, 21);
RT_INTERFACE!{interface ICortanaActionableInsightsOptions(ICortanaActionableInsightsOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaActionableInsightsOptions] {
    fn get_ContentSourceWebLink(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_ContentSourceWebLink(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_SurroundingText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SurroundingText(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<ICortanaActionableInsightsOptions> {
    #[inline] pub fn get_content_source_web_link(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ContentSourceWebLink)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_source_web_link(&self, value: &ComPtr<foundation::Uri>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ContentSourceWebLink)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_surrounding_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SurroundingText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_surrounding_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_SurroundingText)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CortanaActionableInsightsOptions: ICortanaActionableInsightsOptions}
impl RtActivatable<IActivationFactory> for CortanaActionableInsightsOptions {}
DEFINE_CLSID!(CortanaActionableInsightsOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,67,111,114,116,97,110,97,46,67,111,114,116,97,110,97,65,99,116,105,111,110,97,98,108,101,73,110,115,105,103,104,116,115,79,112,116,105,111,110,115,0]) [CLSID_CortanaActionableInsightsOptions]);
DEFINE_IID!(IID_ICortanaActionableInsightsStatics, 3051279378, 40239, 23733, 155, 5, 53, 106, 11, 131, 108, 16);
RT_INTERFACE!{static interface ICortanaActionableInsightsStatics(ICortanaActionableInsightsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaActionableInsightsStatics] {
    fn GetDefault(&self, out: *mut *mut CortanaActionableInsights) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut super::super::system::User, out: *mut *mut CortanaActionableInsights) -> HRESULT
}}
impl ComPtr<ICortanaActionableInsightsStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<CortanaActionableInsights>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDefault)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(&self, user: &ComPtr<super::super::system::User>) -> Result<Option<ComPtr<CortanaActionableInsights>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetForUser)(self.as_abi() as *const _ as *mut _, user.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum CortanaPermission: i32 {
    BrowsingHistory = 0, Calendar = 1, CallHistory = 2, Contacts = 3, Email = 4, InputPersonalization = 5, Location = 6, Messaging = 7, Microphone = 8, Personalization = 9, PhoneCall = 10,
}}
RT_ENUM! { enum CortanaPermissionsChangeResult: i32 {
    Success = 0, Unavailable = 1, DisabledByPolicy = 2,
}}
DEFINE_IID!(IID_ICortanaPermissionsManager, 420688096, 34453, 17290, 149, 69, 61, 164, 232, 34, 221, 180);
RT_INTERFACE!{interface ICortanaPermissionsManager(ICortanaPermissionsManagerVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaPermissionsManager] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT,
    fn ArePermissionsGrantedAsync(&self, permissions: *mut foundation::collections::IIterable<CortanaPermission>, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn GrantPermissionsAsync(&self, permissions: *mut foundation::collections::IIterable<CortanaPermission>, out: *mut *mut foundation::IAsyncOperation<CortanaPermissionsChangeResult>) -> HRESULT,
    fn RevokePermissionsAsync(&self, permissions: *mut foundation::collections::IIterable<CortanaPermission>, out: *mut *mut foundation::IAsyncOperation<CortanaPermissionsChangeResult>) -> HRESULT
}}
impl ComPtr<ICortanaPermissionsManager> {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsSupported)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn are_permissions_granted_async(&self, permissions: &ComPtr<foundation::collections::IIterable<CortanaPermission>>) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ArePermissionsGrantedAsync)(self.as_abi() as *const _ as *mut _, permissions.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn grant_permissions_async(&self, permissions: &ComPtr<foundation::collections::IIterable<CortanaPermission>>) -> Result<ComPtr<foundation::IAsyncOperation<CortanaPermissionsChangeResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GrantPermissionsAsync)(self.as_abi() as *const _ as *mut _, permissions.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn revoke_permissions_async(&self, permissions: &ComPtr<foundation::collections::IIterable<CortanaPermission>>) -> Result<ComPtr<foundation::IAsyncOperation<CortanaPermissionsChangeResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RevokePermissionsAsync)(self.as_abi() as *const _ as *mut _, permissions.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CortanaPermissionsManager: ICortanaPermissionsManager}
impl RtActivatable<ICortanaPermissionsManagerStatics> for CortanaPermissionsManager {}
impl CortanaPermissionsManager {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<CortanaPermissionsManager>>> {
        <Self as RtActivatable<ICortanaPermissionsManagerStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(CortanaPermissionsManager(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,67,111,114,116,97,110,97,46,67,111,114,116,97,110,97,80,101,114,109,105,115,115,105,111,110,115,77,97,110,97,103,101,114,0]) [CLSID_CortanaPermissionsManager]);
DEFINE_IID!(IID_ICortanaPermissionsManagerStatics, 1991370362, 45125, 17428, 157, 109, 42, 211, 165, 254, 58, 126);
RT_INTERFACE!{static interface ICortanaPermissionsManagerStatics(ICortanaPermissionsManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaPermissionsManagerStatics] {
    fn GetDefault(&self, out: *mut *mut CortanaPermissionsManager) -> HRESULT
}}
impl ComPtr<ICortanaPermissionsManagerStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<CortanaPermissionsManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDefault)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICortanaSettings, 1423274407, 32866, 16628, 171, 231, 222, 223, 214, 151, 176, 25);
RT_INTERFACE!{interface ICortanaSettings(ICortanaSettingsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaSettings] {
    fn get_HasUserConsentToVoiceActivation(&self, out: *mut bool) -> HRESULT,
    fn get_IsVoiceActivationEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsVoiceActivationEnabled(&self, value: bool) -> HRESULT
}}
impl ComPtr<ICortanaSettings> {
    #[inline] pub fn get_has_user_consent_to_voice_activation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HasUserConsentToVoiceActivation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_voice_activation_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsVoiceActivationEnabled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_voice_activation_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_IsVoiceActivationEnabled)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CortanaSettings: ICortanaSettings}
impl RtActivatable<ICortanaSettingsStatics> for CortanaSettings {}
impl CortanaSettings {
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<ICortanaSettingsStatics>>::get_activation_factory().is_supported()
    }
    #[inline] pub fn get_default() -> Result<Option<ComPtr<CortanaSettings>>> {
        <Self as RtActivatable<ICortanaSettingsStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(CortanaSettings(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,67,111,114,116,97,110,97,46,67,111,114,116,97,110,97,83,101,116,116,105,110,103,115,0]) [CLSID_CortanaSettings]);
DEFINE_IID!(IID_ICortanaSettingsStatics, 2334969214, 11968, 17517, 146, 133, 51, 240, 124, 232, 172, 4);
RT_INTERFACE!{static interface ICortanaSettingsStatics(ICortanaSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICortanaSettingsStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT,
    fn GetDefault(&self, out: *mut *mut CortanaSettings) -> HRESULT
}}
impl ComPtr<ICortanaSettingsStatics> {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsSupported)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<CortanaSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDefault)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.Services.Cortana
pub mod maps { // Windows.Services.Maps
use crate::prelude::*;
DEFINE_IID!(IID_IEnhancedWaypoint, 3978726516, 22803, 4582, 139, 119, 134, 243, 12, 168, 147, 211);
RT_INTERFACE!{interface IEnhancedWaypoint(IEnhancedWaypointVtbl): IInspectable(IInspectableVtbl) [IID_IEnhancedWaypoint] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Point(&self, out: *mut *mut super::super::devices::geolocation::Geopoint) -> HRESULT,
    fn get_Kind(&self, out: *mut WaypointKind) -> HRESULT
}}
impl ComPtr<IEnhancedWaypoint> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_point(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::Geopoint>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Point)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<WaypointKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Kind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class EnhancedWaypoint: IEnhancedWaypoint}
impl RtActivatable<IEnhancedWaypointFactory> for EnhancedWaypoint {}
impl EnhancedWaypoint {
    #[cfg(feature="windows-devices")] #[inline] pub fn create(point: &ComPtr<super::super::devices::geolocation::Geopoint>, kind: WaypointKind) -> Result<ComPtr<EnhancedWaypoint>> {
        <Self as RtActivatable<IEnhancedWaypointFactory>>::get_activation_factory().create(point, kind)
    }
}
DEFINE_CLSID!(EnhancedWaypoint(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,69,110,104,97,110,99,101,100,87,97,121,112,111,105,110,116,0]) [CLSID_EnhancedWaypoint]);
DEFINE_IID!(IID_IEnhancedWaypointFactory, 2944828535, 41642, 18141, 182, 69, 35, 179, 27, 138, 166, 199);
RT_INTERFACE!{static interface IEnhancedWaypointFactory(IEnhancedWaypointFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IEnhancedWaypointFactory] {
    #[cfg(feature="windows-devices")] fn Create(&self, point: *mut super::super::devices::geolocation::Geopoint, kind: WaypointKind, out: *mut *mut EnhancedWaypoint) -> HRESULT
}}
impl ComPtr<IEnhancedWaypointFactory> {
    #[cfg(feature="windows-devices")] #[inline] pub fn create(&self, point: &ComPtr<super::super::devices::geolocation::Geopoint>, kind: WaypointKind) -> Result<ComPtr<EnhancedWaypoint>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, point.as_abi() as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IManeuverWarning, 3248713098, 9776, 17272, 158, 74, 110, 68, 37, 61, 206, 186);
RT_INTERFACE!{interface IManeuverWarning(IManeuverWarningVtbl): IInspectable(IInspectableVtbl) [IID_IManeuverWarning] {
    fn get_Kind(&self, out: *mut ManeuverWarningKind) -> HRESULT,
    fn get_Severity(&self, out: *mut ManeuverWarningSeverity) -> HRESULT
}}
impl ComPtr<IManeuverWarning> {
    #[inline] pub fn get_kind(&self) -> Result<ManeuverWarningKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Kind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_severity(&self) -> Result<ManeuverWarningSeverity> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Severity)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ManeuverWarning: IManeuverWarning}
RT_ENUM! { enum ManeuverWarningKind: i32 {
    None = 0, Accident = 1, AdministrativeDivisionChange = 2, Alert = 3, BlockedRoad = 4, CheckTimetable = 5, Congestion = 6, Construction = 7, CountryChange = 8, DisabledVehicle = 9, GateAccess = 10, GetOffTransit = 11, GetOnTransit = 12, IllegalUTurn = 13, MassTransit = 14, Miscellaneous = 15, NoIncident = 16, Other = 17, OtherNews = 18, OtherTrafficIncidents = 19, PlannedEvent = 20, PrivateRoad = 21, RestrictedTurn = 22, RoadClosures = 23, RoadHazard = 24, ScheduledConstruction = 25, SeasonalClosures = 26, Tollbooth = 27, TollRoad = 28, TollZoneEnter = 29, TollZoneExit = 30, TrafficFlow = 31, TransitLineChange = 32, UnpavedRoad = 33, UnscheduledConstruction = 34, Weather = 35,
}}
RT_ENUM! { enum ManeuverWarningSeverity: i32 {
    None = 0, LowImpact = 1, Minor = 2, Moderate = 3, Serious = 4,
}}
DEFINE_IID!(IID_IMapAddress, 3483871603, 41908, 17556, 179, 255, 203, 169, 77, 182, 150, 153);
RT_INTERFACE!{interface IMapAddress(IMapAddressVtbl): IInspectable(IInspectableVtbl) [IID_IMapAddress] {
    fn get_BuildingName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BuildingFloor(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BuildingRoom(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BuildingWing(&self, out: *mut HSTRING) -> HRESULT,
    fn get_StreetNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Street(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Neighborhood(&self, out: *mut HSTRING) -> HRESULT,
    fn get_District(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Town(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Region(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RegionCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Country(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CountryCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PostCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Continent(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IMapAddress> {
    #[inline] pub fn get_building_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BuildingName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_building_floor(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BuildingFloor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_building_room(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BuildingRoom)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_building_wing(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BuildingWing)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_street_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StreetNumber)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_street(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Street)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_neighborhood(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Neighborhood)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_district(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_District)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_town(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Town)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Region)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_region_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RegionCode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_country(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Country)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_country_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CountryCode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_post_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PostCode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_continent(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Continent)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapAddress: IMapAddress}
DEFINE_IID!(IID_IMapAddress2, 1976397297, 58797, 17833, 191, 64, 108, 242, 86, 193, 221, 19);
RT_INTERFACE!{interface IMapAddress2(IMapAddress2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapAddress2] {
    fn get_FormattedAddress(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IMapAddress2> {
    #[inline] pub fn get_formatted_address(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_FormattedAddress)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapLocation, 1007107927, 3492, 17128, 158, 226, 169, 111, 207, 35, 113, 220);
RT_INTERFACE!{interface IMapLocation(IMapLocationVtbl): IInspectable(IInspectableVtbl) [IID_IMapLocation] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Point(&self, out: *mut *mut super::super::devices::geolocation::Geopoint) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Address(&self, out: *mut *mut MapAddress) -> HRESULT
}}
impl ComPtr<IMapLocation> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_point(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::Geopoint>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Point)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_address(&self) -> Result<Option<ComPtr<MapAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Address)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapLocation: IMapLocation}
RT_ENUM! { enum MapLocationDesiredAccuracy: i32 {
    High = 0, Low = 1,
}}
RT_CLASS!{static class MapLocationFinder}
impl RtActivatable<IMapLocationFinderStatics> for MapLocationFinder {}
impl RtActivatable<IMapLocationFinderStatics2> for MapLocationFinder {}
impl MapLocationFinder {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_at_async(queryPoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> {
        <Self as RtActivatable<IMapLocationFinderStatics>>::get_activation_factory().find_locations_at_async(queryPoint)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_async(searchText: &HStringArg, referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> {
        <Self as RtActivatable<IMapLocationFinderStatics>>::get_activation_factory().find_locations_async(searchText, referencePoint)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_with_max_count_async(searchText: &HStringArg, referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>, maxCount: u32) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> {
        <Self as RtActivatable<IMapLocationFinderStatics>>::get_activation_factory().find_locations_with_max_count_async(searchText, referencePoint, maxCount)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_at_with_accuracy_async(queryPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, accuracy: MapLocationDesiredAccuracy) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> {
        <Self as RtActivatable<IMapLocationFinderStatics2>>::get_activation_factory().find_locations_at_with_accuracy_async(queryPoint, accuracy)
    }
}
DEFINE_CLSID!(MapLocationFinder(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,76,111,99,97,116,105,111,110,70,105,110,100,101,114,0]) [CLSID_MapLocationFinder]);
DEFINE_IID!(IID_IMapLocationFinderResult, 1139929465, 59596, 17910, 190, 210, 84, 204, 191, 150, 93, 154);
RT_INTERFACE!{interface IMapLocationFinderResult(IMapLocationFinderResultVtbl): IInspectable(IInspectableVtbl) [IID_IMapLocationFinderResult] {
    fn get_Locations(&self, out: *mut *mut foundation::collections::IVectorView<MapLocation>) -> HRESULT,
    fn get_Status(&self, out: *mut MapLocationFinderStatus) -> HRESULT
}}
impl ComPtr<IMapLocationFinderResult> {
    #[inline] pub fn get_locations(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MapLocation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Locations)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<MapLocationFinderStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MapLocationFinderResult: IMapLocationFinderResult}
DEFINE_IID!(IID_IMapLocationFinderStatics, 831183709, 7261, 20277, 162, 223, 170, 202, 148, 149, 149, 23);
RT_INTERFACE!{static interface IMapLocationFinderStatics(IMapLocationFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapLocationFinderStatics] {
    #[cfg(feature="windows-devices")] fn FindLocationsAtAsync(&self, queryPoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindLocationsAsync(&self, searchText: HSTRING, referencePoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindLocationsWithMaxCountAsync(&self, searchText: HSTRING, referencePoint: *mut super::super::devices::geolocation::Geopoint, maxCount: u32, out: *mut *mut foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT
}}
impl ComPtr<IMapLocationFinderStatics> {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_at_async(&self, queryPoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindLocationsAtAsync)(self.as_abi() as *const _ as *mut _, queryPoint.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_async(&self, searchText: &HStringArg, referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindLocationsAsync)(self.as_abi() as *const _ as *mut _, searchText.get(), referencePoint.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_with_max_count_async(&self, searchText: &HStringArg, referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>, maxCount: u32) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindLocationsWithMaxCountAsync)(self.as_abi() as *const _ as *mut _, searchText.get(), referencePoint.as_abi() as *const _ as *mut _, maxCount, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapLocationFinderStatics2, 2509933462, 25733, 19965, 133, 26, 51, 172, 49, 126, 58, 246);
RT_INTERFACE!{static interface IMapLocationFinderStatics2(IMapLocationFinderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapLocationFinderStatics2] {
    #[cfg(feature="windows-devices")] fn FindLocationsAtWithAccuracyAsync(&self, queryPoint: *mut super::super::devices::geolocation::Geopoint, accuracy: MapLocationDesiredAccuracy, out: *mut *mut foundation::IAsyncOperation<MapLocationFinderResult>) -> HRESULT
}}
impl ComPtr<IMapLocationFinderStatics2> {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_locations_at_with_accuracy_async(&self, queryPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, accuracy: MapLocationDesiredAccuracy) -> Result<ComPtr<foundation::IAsyncOperation<MapLocationFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindLocationsAtWithAccuracyAsync)(self.as_abi() as *const _ as *mut _, queryPoint.as_abi() as *const _ as *mut _, accuracy, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum MapLocationFinderStatus: i32 {
    Success = 0, UnknownError = 1, InvalidCredentials = 2, BadLocation = 3, IndexFailure = 4, NetworkFailure = 5, NotSupported = 6,
}}
RT_CLASS!{static class MapManager}
impl RtActivatable<IMapManagerStatics> for MapManager {}
impl MapManager {
    #[inline] pub fn show_downloaded_maps_ui() -> Result<()> {
        <Self as RtActivatable<IMapManagerStatics>>::get_activation_factory().show_downloaded_maps_ui()
    }
    #[inline] pub fn show_maps_update_ui() -> Result<()> {
        <Self as RtActivatable<IMapManagerStatics>>::get_activation_factory().show_maps_update_ui()
    }
}
DEFINE_CLSID!(MapManager(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,77,97,110,97,103,101,114,0]) [CLSID_MapManager]);
DEFINE_IID!(IID_IMapManagerStatics, 937682197, 33460, 19796, 143, 217, 175, 38, 36, 179, 1, 28);
RT_INTERFACE!{static interface IMapManagerStatics(IMapManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapManagerStatics] {
    fn ShowDownloadedMapsUI(&self) -> HRESULT,
    fn ShowMapsUpdateUI(&self) -> HRESULT
}}
impl ComPtr<IMapManagerStatics> {
    #[inline] pub fn show_downloaded_maps_ui(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ShowDownloadedMapsUI)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn show_maps_update_ui(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ShowMapsUpdateUI)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum MapManeuverNotices: u32 {
    None = 0, Toll = 1, Unpaved = 2,
}}
DEFINE_IID!(IID_IMapRoute, 4211586866, 22605, 17795, 156, 96, 100, 31, 234, 39, 67, 73);
RT_INTERFACE!{interface IMapRoute(IMapRouteVtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_BoundingBox(&self, out: *mut *mut super::super::devices::geolocation::GeoboundingBox) -> HRESULT,
    fn get_LengthInMeters(&self, out: *mut f64) -> HRESULT,
    fn get_EstimatedDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut super::super::devices::geolocation::Geopath) -> HRESULT,
    fn get_Legs(&self, out: *mut *mut foundation::collections::IVectorView<MapRouteLeg>) -> HRESULT,
    fn get_IsTrafficBased(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IMapRoute> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_bounding_box(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::GeoboundingBox>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BoundingBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_length_in_meters(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LengthInMeters)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_estimated_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EstimatedDuration)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_path(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::Geopath>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_legs(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MapRouteLeg>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Legs)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_traffic_based(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTrafficBased)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MapRoute: IMapRoute}
DEFINE_IID!(IID_IMapRoute2, 3519403020, 8723, 19120, 162, 96, 70, 179, 129, 105, 190, 172);
RT_INTERFACE!{interface IMapRoute2(IMapRoute2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute2] {
    fn get_ViolatedRestrictions(&self, out: *mut MapRouteRestrictions) -> HRESULT,
    fn get_HasBlockedRoads(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IMapRoute2> {
    #[inline] pub fn get_violated_restrictions(&self) -> Result<MapRouteRestrictions> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ViolatedRestrictions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_blocked_roads(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HasBlockedRoads)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRoute3, 2240618158, 62125, 17055, 187, 55, 205, 33, 9, 79, 252, 146);
RT_INTERFACE!{interface IMapRoute3(IMapRoute3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute3] {
    fn get_DurationWithoutTraffic(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_TrafficCongestion(&self, out: *mut TrafficCongestion) -> HRESULT
}}
impl ComPtr<IMapRoute3> {
    #[inline] pub fn get_duration_without_traffic(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DurationWithoutTraffic)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_traffic_congestion(&self) -> Result<TrafficCongestion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrafficCongestion)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRoute4, 913083557, 12371, 20385, 128, 255, 212, 117, 243, 237, 30, 110);
RT_INTERFACE!{interface IMapRoute4(IMapRoute4Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRoute4] {
    fn get_IsScenic(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IMapRoute4> {
    #[inline] pub fn get_is_scenic(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsScenic)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRouteDrivingOptions, 1746220621, 50908, 18071, 164, 82, 177, 143, 143, 11, 103, 161);
RT_INTERFACE!{interface IMapRouteDrivingOptions(IMapRouteDrivingOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteDrivingOptions] {
    fn get_MaxAlternateRouteCount(&self, out: *mut u32) -> HRESULT,
    fn put_MaxAlternateRouteCount(&self, value: u32) -> HRESULT,
    fn get_InitialHeading(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn put_InitialHeading(&self, value: *mut foundation::IReference<f64>) -> HRESULT,
    fn get_RouteOptimization(&self, out: *mut MapRouteOptimization) -> HRESULT,
    fn put_RouteOptimization(&self, value: MapRouteOptimization) -> HRESULT,
    fn get_RouteRestrictions(&self, out: *mut MapRouteRestrictions) -> HRESULT,
    fn put_RouteRestrictions(&self, value: MapRouteRestrictions) -> HRESULT
}}
impl ComPtr<IMapRouteDrivingOptions> {
    #[inline] pub fn get_max_alternate_route_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_MaxAlternateRouteCount)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_alternate_route_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_MaxAlternateRouteCount)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_initial_heading(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InitialHeading)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_initial_heading(&self, value: &ComPtr<foundation::IReference<f64>>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_InitialHeading)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_route_optimization(&self) -> Result<MapRouteOptimization> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_RouteOptimization)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_route_optimization(&self, value: MapRouteOptimization) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_RouteOptimization)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_route_restrictions(&self) -> Result<MapRouteRestrictions> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_RouteRestrictions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_route_restrictions(&self, value: MapRouteRestrictions) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_RouteRestrictions)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MapRouteDrivingOptions: IMapRouteDrivingOptions}
impl RtActivatable<IActivationFactory> for MapRouteDrivingOptions {}
DEFINE_CLSID!(MapRouteDrivingOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,82,111,117,116,101,68,114,105,118,105,110,103,79,112,116,105,111,110,115,0]) [CLSID_MapRouteDrivingOptions]);
DEFINE_IID!(IID_IMapRouteDrivingOptions2, 903644784, 49816, 18640, 181, 173, 130, 84, 96, 100, 86, 3);
RT_INTERFACE!{interface IMapRouteDrivingOptions2(IMapRouteDrivingOptions2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteDrivingOptions2] {
    fn get_DepartureTime(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn put_DepartureTime(&self, value: *mut foundation::IReference<foundation::DateTime>) -> HRESULT
}}
impl ComPtr<IMapRouteDrivingOptions2> {
    #[inline] pub fn get_departure_time(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DepartureTime)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_departure_time(&self, value: &ComPtr<foundation::IReference<foundation::DateTime>>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_DepartureTime)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class MapRouteFinder}
impl RtActivatable<IMapRouteFinderStatics> for MapRouteFinder {}
impl RtActivatable<IMapRouteFinderStatics2> for MapRouteFinder {}
impl RtActivatable<IMapRouteFinderStatics3> for MapRouteFinder {}
impl MapRouteFinder {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_async(startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_async(startPoint, endPoint)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_async(startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_with_optimization_async(startPoint, endPoint, optimization)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_and_restrictions_async(startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_with_optimization_and_restrictions_async(startPoint, endPoint, optimization, restrictions)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_restrictions_and_heading_async(startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_with_optimization_restrictions_and_heading_async(startPoint, endPoint, optimization, restrictions, headingInDegrees)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_async(wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_async(wayPoints)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_and_optimization_async(wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>, optimization: MapRouteOptimization) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_and_optimization_async(wayPoints, optimization)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_optimization_and_restrictions_async(wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_optimization_and_restrictions_async(wayPoints, optimization, restrictions)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_optimization_restrictions_and_heading_async(wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_driving_route_from_waypoints_optimization_restrictions_and_heading_async(wayPoints, optimization, restrictions, headingInDegrees)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_walking_route_async(startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_walking_route_async(startPoint, endPoint)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_walking_route_from_waypoints_async(wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics>>::get_activation_factory().get_walking_route_from_waypoints_async(wayPoints)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_options_async(startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, options: &ComPtr<MapRouteDrivingOptions>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics2>>::get_activation_factory().get_driving_route_with_options_async(startPoint, endPoint, options)
    }
    #[inline] pub fn get_driving_route_from_enhanced_waypoints_async(waypoints: &ComPtr<foundation::collections::IIterable<EnhancedWaypoint>>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics3>>::get_activation_factory().get_driving_route_from_enhanced_waypoints_async(waypoints)
    }
    #[inline] pub fn get_driving_route_from_enhanced_waypoints_with_options_async(waypoints: &ComPtr<foundation::collections::IIterable<EnhancedWaypoint>>, options: &ComPtr<MapRouteDrivingOptions>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> {
        <Self as RtActivatable<IMapRouteFinderStatics3>>::get_activation_factory().get_driving_route_from_enhanced_waypoints_with_options_async(waypoints, options)
    }
}
DEFINE_CLSID!(MapRouteFinder(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,82,111,117,116,101,70,105,110,100,101,114,0]) [CLSID_MapRouteFinder]);
DEFINE_IID!(IID_IMapRouteFinderResult, 2825429786, 37922, 18092, 140, 161, 177, 97, 77, 75, 251, 226);
RT_INTERFACE!{interface IMapRouteFinderResult(IMapRouteFinderResultVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderResult] {
    fn get_Route(&self, out: *mut *mut MapRoute) -> HRESULT,
    fn get_Status(&self, out: *mut MapRouteFinderStatus) -> HRESULT
}}
impl ComPtr<IMapRouteFinderResult> {
    #[inline] pub fn get_route(&self) -> Result<Option<ComPtr<MapRoute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Route)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<MapRouteFinderStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MapRouteFinderResult: IMapRouteFinderResult}
DEFINE_IID!(IID_IMapRouteFinderResult2, 544250989, 55564, 18120, 145, 198, 125, 75, 228, 239, 178, 21);
RT_INTERFACE!{interface IMapRouteFinderResult2(IMapRouteFinderResult2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderResult2] {
    fn get_AlternateRoutes(&self, out: *mut *mut foundation::collections::IVectorView<MapRoute>) -> HRESULT
}}
impl ComPtr<IMapRouteFinderResult2> {
    #[inline] pub fn get_alternate_routes(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MapRoute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AlternateRoutes)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRouteFinderStatics, 3097871631, 7268, 19514, 129, 235, 31, 124, 21, 42, 251, 187);
RT_INTERFACE!{static interface IMapRouteFinderStatics(IMapRouteFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderStatics] {
    #[cfg(feature="windows-devices")] fn GetDrivingRouteAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptimizationAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptimizationAndRestrictionsAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsAsync(&self, wayPoints: *mut foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsAndOptimizationAsync(&self, wayPoints: *mut foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(&self, wayPoints: *mut foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(&self, wayPoints: *mut foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetWalkingRouteAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn GetWalkingRouteFromWaypointsAsync(&self, wayPoints: *mut foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT
}}
impl ComPtr<IMapRouteFinderStatics> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_async(&self, startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteAsync)(self.as_abi() as *const _ as *mut _, startPoint.as_abi() as *const _ as *mut _, endPoint.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_async(&self, startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteWithOptimizationAsync)(self.as_abi() as *const _ as *mut _, startPoint.as_abi() as *const _ as *mut _, endPoint.as_abi() as *const _ as *mut _, optimization, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_and_restrictions_async(&self, startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteWithOptimizationAndRestrictionsAsync)(self.as_abi() as *const _ as *mut _, startPoint.as_abi() as *const _ as *mut _, endPoint.as_abi() as *const _ as *mut _, optimization, restrictions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_optimization_restrictions_and_heading_async(&self, startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync)(self.as_abi() as *const _ as *mut _, startPoint.as_abi() as *const _ as *mut _, endPoint.as_abi() as *const _ as *mut _, optimization, restrictions, headingInDegrees, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_async(&self, wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteFromWaypointsAsync)(self.as_abi() as *const _ as *mut _, wayPoints.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_and_optimization_async(&self, wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>, optimization: MapRouteOptimization) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteFromWaypointsAndOptimizationAsync)(self.as_abi() as *const _ as *mut _, wayPoints.as_abi() as *const _ as *mut _, optimization, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_optimization_and_restrictions_async(&self, wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync)(self.as_abi() as *const _ as *mut _, wayPoints.as_abi() as *const _ as *mut _, optimization, restrictions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_from_waypoints_optimization_restrictions_and_heading_async(&self, wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingInDegrees: f64) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync)(self.as_abi() as *const _ as *mut _, wayPoints.as_abi() as *const _ as *mut _, optimization, restrictions, headingInDegrees, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_walking_route_async(&self, startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetWalkingRouteAsync)(self.as_abi() as *const _ as *mut _, startPoint.as_abi() as *const _ as *mut _, endPoint.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_walking_route_from_waypoints_async(&self, wayPoints: &ComPtr<foundation::collections::IIterable<super::super::devices::geolocation::Geopoint>>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetWalkingRouteFromWaypointsAsync)(self.as_abi() as *const _ as *mut _, wayPoints.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRouteFinderStatics2, 2949393523, 30560, 18863, 179, 189, 186, 241, 53, 183, 3, 225);
RT_INTERFACE!{static interface IMapRouteFinderStatics2(IMapRouteFinderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderStatics2] {
    #[cfg(feature="windows-devices")] fn GetDrivingRouteWithOptionsAsync(&self, startPoint: *mut super::super::devices::geolocation::Geopoint, endPoint: *mut super::super::devices::geolocation::Geopoint, options: *mut MapRouteDrivingOptions, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT
}}
impl ComPtr<IMapRouteFinderStatics2> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_driving_route_with_options_async(&self, startPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, endPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, options: &ComPtr<MapRouteDrivingOptions>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteWithOptionsAsync)(self.as_abi() as *const _ as *mut _, startPoint.as_abi() as *const _ as *mut _, endPoint.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRouteFinderStatics3, 4127818036, 22803, 4582, 139, 119, 134, 243, 12, 168, 147, 211);
RT_INTERFACE!{static interface IMapRouteFinderStatics3(IMapRouteFinderStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteFinderStatics3] {
    fn GetDrivingRouteFromEnhancedWaypointsAsync(&self, waypoints: *mut foundation::collections::IIterable<EnhancedWaypoint>, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT,
    fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(&self, waypoints: *mut foundation::collections::IIterable<EnhancedWaypoint>, options: *mut MapRouteDrivingOptions, out: *mut *mut foundation::IAsyncOperation<MapRouteFinderResult>) -> HRESULT
}}
impl ComPtr<IMapRouteFinderStatics3> {
    #[inline] pub fn get_driving_route_from_enhanced_waypoints_async(&self, waypoints: &ComPtr<foundation::collections::IIterable<EnhancedWaypoint>>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteFromEnhancedWaypointsAsync)(self.as_abi() as *const _ as *mut _, waypoints.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_driving_route_from_enhanced_waypoints_with_options_async(&self, waypoints: &ComPtr<foundation::collections::IIterable<EnhancedWaypoint>>, options: &ComPtr<MapRouteDrivingOptions>) -> Result<ComPtr<foundation::IAsyncOperation<MapRouteFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync)(self.as_abi() as *const _ as *mut _, waypoints.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum MapRouteFinderStatus: i32 {
    Success = 0, UnknownError = 1, InvalidCredentials = 2, NoRouteFound = 3, NoRouteFoundWithGivenOptions = 4, StartPointNotFound = 5, EndPointNotFound = 6, NoPedestrianRouteFound = 7, NetworkFailure = 8, NotSupported = 9,
}}
DEFINE_IID!(IID_IMapRouteLeg, 2532881142, 23482, 19735, 157, 182, 26, 38, 63, 236, 116, 113);
RT_INTERFACE!{interface IMapRouteLeg(IMapRouteLegVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteLeg] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_BoundingBox(&self, out: *mut *mut super::super::devices::geolocation::GeoboundingBox) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut super::super::devices::geolocation::Geopath) -> HRESULT,
    fn get_LengthInMeters(&self, out: *mut f64) -> HRESULT,
    fn get_EstimatedDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_Maneuvers(&self, out: *mut *mut foundation::collections::IVectorView<MapRouteManeuver>) -> HRESULT
}}
impl ComPtr<IMapRouteLeg> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_bounding_box(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::GeoboundingBox>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BoundingBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_path(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::Geopath>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_length_in_meters(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LengthInMeters)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_estimated_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EstimatedDuration)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_maneuvers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MapRouteManeuver>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Maneuvers)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapRouteLeg: IMapRouteLeg}
DEFINE_IID!(IID_IMapRouteLeg2, 48367149, 51654, 17848, 142, 84, 26, 16, 181, 122, 23, 232);
RT_INTERFACE!{interface IMapRouteLeg2(IMapRouteLeg2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteLeg2] {
    fn get_DurationWithoutTraffic(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_TrafficCongestion(&self, out: *mut TrafficCongestion) -> HRESULT
}}
impl ComPtr<IMapRouteLeg2> {
    #[inline] pub fn get_duration_without_traffic(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DurationWithoutTraffic)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_traffic_congestion(&self) -> Result<TrafficCongestion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrafficCongestion)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRouteManeuver, 3982235632, 42667, 19813, 160, 134, 250, 138, 126, 52, 13, 242);
RT_INTERFACE!{interface IMapRouteManeuver(IMapRouteManeuverVtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteManeuver] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_StartingPoint(&self, out: *mut *mut super::super::devices::geolocation::Geopoint) -> HRESULT,
    fn get_LengthInMeters(&self, out: *mut f64) -> HRESULT,
    fn get_InstructionText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut MapRouteManeuverKind) -> HRESULT,
    fn get_ExitNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ManeuverNotices(&self, out: *mut MapManeuverNotices) -> HRESULT
}}
impl ComPtr<IMapRouteManeuver> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_starting_point(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::Geopoint>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StartingPoint)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_length_in_meters(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LengthInMeters)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_instruction_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InstructionText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<MapRouteManeuverKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Kind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_exit_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExitNumber)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_maneuver_notices(&self) -> Result<MapManeuverNotices> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ManeuverNotices)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MapRouteManeuver: IMapRouteManeuver}
DEFINE_IID!(IID_IMapRouteManeuver2, 1568394652, 31899, 16863, 131, 139, 234, 226, 30, 75, 5, 169);
RT_INTERFACE!{interface IMapRouteManeuver2(IMapRouteManeuver2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteManeuver2] {
    fn get_StartHeading(&self, out: *mut f64) -> HRESULT,
    fn get_EndHeading(&self, out: *mut f64) -> HRESULT,
    fn get_StreetName(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IMapRouteManeuver2> {
    #[inline] pub fn get_start_heading(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_StartHeading)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_end_heading(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EndHeading)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_street_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StreetName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapRouteManeuver3, 2795583711, 1155, 16742, 133, 190, 185, 147, 54, 193, 24, 117);
RT_INTERFACE!{interface IMapRouteManeuver3(IMapRouteManeuver3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapRouteManeuver3] {
    fn get_Warnings(&self, out: *mut *mut foundation::collections::IVectorView<ManeuverWarning>) -> HRESULT
}}
impl ComPtr<IMapRouteManeuver3> {
    #[inline] pub fn get_warnings(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ManeuverWarning>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Warnings)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum MapRouteManeuverKind: i32 {
    None = 0, Start = 1, Stopover = 2, StopoverResume = 3, End = 4, GoStraight = 5, UTurnLeft = 6, UTurnRight = 7, TurnKeepLeft = 8, TurnKeepRight = 9, TurnLightLeft = 10, TurnLightRight = 11, TurnLeft = 12, TurnRight = 13, TurnHardLeft = 14, TurnHardRight = 15, FreewayEnterLeft = 16, FreewayEnterRight = 17, FreewayLeaveLeft = 18, FreewayLeaveRight = 19, FreewayContinueLeft = 20, FreewayContinueRight = 21, TrafficCircleLeft = 22, TrafficCircleRight = 23, TakeFerry = 24,
}}
RT_ENUM! { enum MapRouteOptimization: i32 {
    Time = 0, Distance = 1, TimeWithTraffic = 2, Scenic = 3,
}}
RT_ENUM! { enum MapRouteRestrictions: u32 {
    None = 0, Highways = 1, TollRoads = 2, Ferries = 4, Tunnels = 8, DirtRoads = 16, Motorail = 32,
}}
RT_CLASS!{static class MapService}
impl RtActivatable<IMapServiceStatics> for MapService {}
impl RtActivatable<IMapServiceStatics2> for MapService {}
impl RtActivatable<IMapServiceStatics3> for MapService {}
impl RtActivatable<IMapServiceStatics4> for MapService {}
impl MapService {
    #[inline] pub fn set_service_token(value: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IMapServiceStatics>>::get_activation_factory().set_service_token(value)
    }
    #[inline] pub fn get_service_token() -> Result<HString> {
        <Self as RtActivatable<IMapServiceStatics>>::get_activation_factory().get_service_token()
    }
    #[inline] pub fn get_world_view_region_code() -> Result<HString> {
        <Self as RtActivatable<IMapServiceStatics2>>::get_activation_factory().get_world_view_region_code()
    }
    #[inline] pub fn get_data_attributions() -> Result<HString> {
        <Self as RtActivatable<IMapServiceStatics3>>::get_activation_factory().get_data_attributions()
    }
    #[inline] pub fn set_data_usage_preference(value: MapServiceDataUsagePreference) -> Result<()> {
        <Self as RtActivatable<IMapServiceStatics4>>::get_activation_factory().set_data_usage_preference(value)
    }
    #[inline] pub fn get_data_usage_preference() -> Result<MapServiceDataUsagePreference> {
        <Self as RtActivatable<IMapServiceStatics4>>::get_activation_factory().get_data_usage_preference()
    }
}
DEFINE_CLSID!(MapService(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,77,97,112,83,101,114,118,105,99,101,0]) [CLSID_MapService]);
RT_ENUM! { enum MapServiceDataUsagePreference: i32 {
    Default = 0, OfflineMapDataOnly = 1,
}}
DEFINE_IID!(IID_IMapServiceStatics, 21278085, 49228, 19677, 135, 26, 160, 114, 109, 9, 124, 212);
RT_INTERFACE!{static interface IMapServiceStatics(IMapServiceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics] {
    fn put_ServiceToken(&self, value: HSTRING) -> HRESULT,
    fn get_ServiceToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IMapServiceStatics> {
    #[inline] pub fn set_service_token(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ServiceToken)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_service_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ServiceToken)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapServiceStatics2, 4162404077, 40069, 16553, 136, 150, 15, 195, 253, 43, 124, 42);
RT_INTERFACE!{static interface IMapServiceStatics2(IMapServiceStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics2] {
    fn get_WorldViewRegionCode(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IMapServiceStatics2> {
    #[inline] pub fn get_world_view_region_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_WorldViewRegionCode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapServiceStatics3, 168939040, 25511, 18516, 179, 85, 214, 220, 218, 34, 61, 27);
RT_INTERFACE!{static interface IMapServiceStatics3(IMapServiceStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics3] {
    fn get_DataAttributions(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IMapServiceStatics3> {
    #[inline] pub fn get_data_attributions(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DataAttributions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapServiceStatics4, 143272034, 27324, 16910, 148, 95, 76, 253, 137, 198, 115, 86);
RT_INTERFACE!{static interface IMapServiceStatics4(IMapServiceStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_IMapServiceStatics4] {
    fn put_DataUsagePreference(&self, value: MapServiceDataUsagePreference) -> HRESULT,
    fn get_DataUsagePreference(&self, out: *mut MapServiceDataUsagePreference) -> HRESULT
}}
impl ComPtr<IMapServiceStatics4> {
    #[inline] pub fn set_data_usage_preference(&self, value: MapServiceDataUsagePreference) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_DataUsagePreference)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_data_usage_preference(&self) -> Result<MapServiceDataUsagePreference> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DataUsagePreference)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPlaceInfo, 2584219830, 12744, 20330, 159, 24, 149, 11, 76, 56, 149, 26);
RT_INTERFACE!{interface IPlaceInfo(IPlaceInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfo] {
    fn Show(&self, selection: foundation::Rect) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-ui")] fn ShowWithPreferredPlacement(&self, selection: foundation::Rect, preferredPlacement: super::super::ui::popups::Placement) -> HRESULT,
    fn get_Identifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayAddress(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-devices")] fn get_Geoshape(&self, out: *mut *mut super::super::devices::geolocation::IGeoshape) -> HRESULT
}}
impl ComPtr<IPlaceInfo> {
    #[inline] pub fn show(&self, selection: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Show)(self.as_abi() as *const _ as *mut _, selection);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn show_with_preferred_placement(&self, selection: foundation::Rect, preferredPlacement: super::super::ui::popups::Placement) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ShowWithPreferredPlacement)(self.as_abi() as *const _ as *mut _, selection, preferredPlacement);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Identifier)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_address(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayAddress)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_geoshape(&self) -> Result<Option<ComPtr<super::super::devices::geolocation::IGeoshape>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Geoshape)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PlaceInfo: IPlaceInfo}
impl RtActivatable<IPlaceInfoStatics> for PlaceInfo {}
impl RtActivatable<IPlaceInfoStatics2> for PlaceInfo {}
impl PlaceInfo {
    #[cfg(feature="windows-devices")] #[inline] pub fn create(referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create(referencePoint)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn create_with_geopoint_and_options(referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>, options: &ComPtr<PlaceInfoCreateOptions>) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_with_geopoint_and_options(referencePoint, options)
    }
    #[inline] pub fn create_from_identifier(identifier: &HStringArg) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_from_identifier(identifier)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn create_from_identifier_with_options(identifier: &HStringArg, defaultPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, options: &ComPtr<PlaceInfoCreateOptions>) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_from_identifier_with_options(identifier, defaultPoint, options)
    }
    #[inline] pub fn create_from_map_location(location: &ComPtr<MapLocation>) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().create_from_map_location(location)
    }
    #[inline] pub fn get_is_show_supported() -> Result<bool> {
        <Self as RtActivatable<IPlaceInfoStatics>>::get_activation_factory().get_is_show_supported()
    }
    #[inline] pub fn create_from_address(displayAddress: &HStringArg) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics2>>::get_activation_factory().create_from_address(displayAddress)
    }
    #[inline] pub fn create_from_address_with_name(displayAddress: &HStringArg, displayName: &HStringArg) -> Result<Option<ComPtr<PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoStatics2>>::get_activation_factory().create_from_address_with_name(displayAddress, displayName)
    }
}
DEFINE_CLSID!(PlaceInfo(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,80,108,97,99,101,73,110,102,111,0]) [CLSID_PlaceInfo]);
DEFINE_IID!(IID_IPlaceInfoCreateOptions, 3442721061, 26609, 19379, 153, 7, 236, 206, 147, 155, 3, 153);
RT_INTERFACE!{interface IPlaceInfoCreateOptions(IPlaceInfoCreateOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoCreateOptions] {
    fn put_DisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayAddress(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayAddress(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IPlaceInfoCreateOptions> {
    #[inline] pub fn set_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_DisplayName)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_display_address(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_DisplayAddress)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_address(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayAddress)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PlaceInfoCreateOptions: IPlaceInfoCreateOptions}
impl RtActivatable<IActivationFactory> for PlaceInfoCreateOptions {}
DEFINE_CLSID!(PlaceInfoCreateOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,80,108,97,99,101,73,110,102,111,67,114,101,97,116,101,79,112,116,105,111,110,115,0]) [CLSID_PlaceInfoCreateOptions]);
DEFINE_IID!(IID_IPlaceInfoStatics, 2193227633, 27856, 18596, 175, 217, 94, 216, 32, 151, 147, 107);
RT_INTERFACE!{static interface IPlaceInfoStatics(IPlaceInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoStatics] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn Create(&self, referencePoint: *mut super::super::devices::geolocation::Geopoint, out: *mut *mut PlaceInfo) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-devices")] fn CreateWithGeopointAndOptions(&self, referencePoint: *mut super::super::devices::geolocation::Geopoint, options: *mut PlaceInfoCreateOptions, out: *mut *mut PlaceInfo) -> HRESULT,
    fn CreateFromIdentifier(&self, identifier: HSTRING, out: *mut *mut PlaceInfo) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-devices")] fn CreateFromIdentifierWithOptions(&self, identifier: HSTRING, defaultPoint: *mut super::super::devices::geolocation::Geopoint, options: *mut PlaceInfoCreateOptions, out: *mut *mut PlaceInfo) -> HRESULT,
    fn CreateFromMapLocation(&self, location: *mut MapLocation, out: *mut *mut PlaceInfo) -> HRESULT,
    fn get_IsShowSupported(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IPlaceInfoStatics> {
    #[cfg(feature="windows-devices")] #[inline] pub fn create(&self, referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, referencePoint.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn create_with_geopoint_and_options(&self, referencePoint: &ComPtr<super::super::devices::geolocation::Geopoint>, options: &ComPtr<PlaceInfoCreateOptions>) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateWithGeopointAndOptions)(self.as_abi() as *const _ as *mut _, referencePoint.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_identifier(&self, identifier: &HStringArg) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIdentifier)(self.as_abi() as *const _ as *mut _, identifier.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn create_from_identifier_with_options(&self, identifier: &HStringArg, defaultPoint: &ComPtr<super::super::devices::geolocation::Geopoint>, options: &ComPtr<PlaceInfoCreateOptions>) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIdentifierWithOptions)(self.as_abi() as *const _ as *mut _, identifier.get(), defaultPoint.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_map_location(&self, location: &ComPtr<MapLocation>) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromMapLocation)(self.as_abi() as *const _ as *mut _, location.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_show_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsShowSupported)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPlaceInfoStatics2, 1930363465, 16455, 17571, 143, 129, 37, 80, 165, 33, 99, 112);
RT_INTERFACE!{static interface IPlaceInfoStatics2(IPlaceInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoStatics2] {
    fn CreateFromAddress(&self, displayAddress: HSTRING, out: *mut *mut PlaceInfo) -> HRESULT,
    fn CreateFromAddressWithName(&self, displayAddress: HSTRING, displayName: HSTRING, out: *mut *mut PlaceInfo) -> HRESULT
}}
impl ComPtr<IPlaceInfoStatics2> {
    #[inline] pub fn create_from_address(&self, displayAddress: &HStringArg) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromAddress)(self.as_abi() as *const _ as *mut _, displayAddress.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_address_with_name(&self, displayAddress: &HStringArg, displayName: &HStringArg) -> Result<Option<ComPtr<PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromAddressWithName)(self.as_abi() as *const _ as *mut _, displayAddress.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TrafficCongestion: i32 {
    Unknown = 0, Light = 1, Mild = 2, Medium = 3, Heavy = 4,
}}
RT_ENUM! { enum WaypointKind: i32 {
    Stop = 0, Via = 1,
}}
pub mod guidance { // Windows.Services.Maps.Guidance
use crate::prelude::*;
RT_ENUM! { enum GuidanceAudioMeasurementSystem: i32 {
    Meters = 0, MilesAndYards = 1, MilesAndFeet = 2,
}}
RT_ENUM! { enum GuidanceAudioNotificationKind: i32 {
    Maneuver = 0, Route = 1, Gps = 2, SpeedLimit = 3, Traffic = 4, TrafficCamera = 5,
}}
DEFINE_IID!(IID_IGuidanceAudioNotificationRequestedEventArgs, 3391791690, 51138, 19788, 157, 124, 73, 149, 118, 188, 237, 219);
RT_INTERFACE!{interface IGuidanceAudioNotificationRequestedEventArgs(IGuidanceAudioNotificationRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceAudioNotificationRequestedEventArgs] {
    fn get_AudioNotification(&self, out: *mut GuidanceAudioNotificationKind) -> HRESULT,
    fn get_AudioFilePaths(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_AudioText(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IGuidanceAudioNotificationRequestedEventArgs> {
    #[inline] pub fn get_audio_notification(&self) -> Result<GuidanceAudioNotificationKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AudioNotification)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_audio_file_paths(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AudioFilePaths)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_audio_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AudioText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceAudioNotificationRequestedEventArgs: IGuidanceAudioNotificationRequestedEventArgs}
RT_ENUM! { enum GuidanceAudioNotifications: u32 {
    None = 0, Maneuver = 1, Route = 2, Gps = 4, SpeedLimit = 8, Traffic = 16, TrafficCamera = 32,
}}
DEFINE_IID!(IID_IGuidanceLaneInfo, 2214908180, 25985, 17335, 172, 21, 201, 7, 155, 249, 13, 241);
RT_INTERFACE!{interface IGuidanceLaneInfo(IGuidanceLaneInfoVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceLaneInfo] {
    fn get_LaneMarkers(&self, out: *mut GuidanceLaneMarkers) -> HRESULT,
    fn get_IsOnRoute(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGuidanceLaneInfo> {
    #[inline] pub fn get_lane_markers(&self) -> Result<GuidanceLaneMarkers> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LaneMarkers)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_on_route(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsOnRoute)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceLaneInfo: IGuidanceLaneInfo}
RT_ENUM! { enum GuidanceLaneMarkers: u32 {
    None = 0, LightRight = 1, Right = 2, HardRight = 4, Straight = 8, UTurnLeft = 16, HardLeft = 32, Left = 64, LightLeft = 128, UTurnRight = 256, Unknown = 4294967295,
}}
DEFINE_IID!(IID_IGuidanceManeuver, 4228461164, 60617, 18728, 162, 161, 114, 50, 185, 155, 148, 161);
RT_INTERFACE!{interface IGuidanceManeuver(IGuidanceManeuverVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceManeuver] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_StartLocation(&self, out: *mut *mut crate::windows::devices::geolocation::Geopoint) -> HRESULT,
    fn get_DistanceFromRouteStart(&self, out: *mut i32) -> HRESULT,
    fn get_DistanceFromPreviousManeuver(&self, out: *mut i32) -> HRESULT,
    fn get_DepartureRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NextRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DepartureShortRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NextShortRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut GuidanceManeuverKind) -> HRESULT,
    fn get_StartAngle(&self, out: *mut i32) -> HRESULT,
    fn get_EndAngle(&self, out: *mut i32) -> HRESULT,
    fn get_RoadSignpost(&self, out: *mut *mut GuidanceRoadSignpost) -> HRESULT,
    fn get_InstructionText(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IGuidanceManeuver> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_start_location(&self) -> Result<Option<ComPtr<crate::windows::devices::geolocation::Geopoint>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StartLocation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_distance_from_route_start(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DistanceFromRouteStart)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_distance_from_previous_maneuver(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DistanceFromPreviousManeuver)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_departure_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DepartureRoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NextRoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_departure_short_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DepartureShortRoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_short_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NextShortRoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<GuidanceManeuverKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Kind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_start_angle(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_StartAngle)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_end_angle(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EndAngle)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_road_signpost(&self) -> Result<Option<ComPtr<GuidanceRoadSignpost>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RoadSignpost)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_instruction_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InstructionText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceManeuver: IGuidanceManeuver}
RT_ENUM! { enum GuidanceManeuverKind: i32 {
    None = 0, GoStraight = 1, UTurnRight = 2, UTurnLeft = 3, TurnKeepRight = 4, TurnLightRight = 5, TurnRight = 6, TurnHardRight = 7, KeepMiddle = 8, TurnKeepLeft = 9, TurnLightLeft = 10, TurnLeft = 11, TurnHardLeft = 12, FreewayEnterRight = 13, FreewayEnterLeft = 14, FreewayLeaveRight = 15, FreewayLeaveLeft = 16, FreewayKeepRight = 17, FreewayKeepLeft = 18, TrafficCircleRight1 = 19, TrafficCircleRight2 = 20, TrafficCircleRight3 = 21, TrafficCircleRight4 = 22, TrafficCircleRight5 = 23, TrafficCircleRight6 = 24, TrafficCircleRight7 = 25, TrafficCircleRight8 = 26, TrafficCircleRight9 = 27, TrafficCircleRight10 = 28, TrafficCircleRight11 = 29, TrafficCircleRight12 = 30, TrafficCircleLeft1 = 31, TrafficCircleLeft2 = 32, TrafficCircleLeft3 = 33, TrafficCircleLeft4 = 34, TrafficCircleLeft5 = 35, TrafficCircleLeft6 = 36, TrafficCircleLeft7 = 37, TrafficCircleLeft8 = 38, TrafficCircleLeft9 = 39, TrafficCircleLeft10 = 40, TrafficCircleLeft11 = 41, TrafficCircleLeft12 = 42, Start = 43, End = 44, TakeFerry = 45, PassTransitStation = 46, LeaveTransitStation = 47,
}}
DEFINE_IID!(IID_IGuidanceMapMatchedCoordinate, 3081548136, 10514, 19097, 175, 241, 121, 134, 9, 185, 129, 254);
RT_INTERFACE!{interface IGuidanceMapMatchedCoordinate(IGuidanceMapMatchedCoordinateVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceMapMatchedCoordinate] {
    #[cfg(not(feature="windows-devices"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Location(&self, out: *mut *mut crate::windows::devices::geolocation::Geopoint) -> HRESULT,
    fn get_CurrentHeading(&self, out: *mut f64) -> HRESULT,
    fn get_CurrentSpeed(&self, out: *mut f64) -> HRESULT,
    fn get_IsOnStreet(&self, out: *mut bool) -> HRESULT,
    fn get_Road(&self, out: *mut *mut GuidanceRoadSegment) -> HRESULT
}}
impl ComPtr<IGuidanceMapMatchedCoordinate> {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_location(&self) -> Result<Option<ComPtr<crate::windows::devices::geolocation::Geopoint>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Location)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_heading(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_CurrentHeading)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_speed(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_CurrentSpeed)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_on_street(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsOnStreet)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_road(&self) -> Result<Option<ComPtr<GuidanceRoadSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Road)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceMapMatchedCoordinate: IGuidanceMapMatchedCoordinate}
RT_ENUM! { enum GuidanceMode: i32 {
    None = 0, Simulation = 1, Navigation = 2, Tracking = 3,
}}
DEFINE_IID!(IID_IGuidanceNavigator, 150044407, 36415, 19866, 190, 138, 16, 143, 154, 1, 44, 103);
RT_INTERFACE!{interface IGuidanceNavigator(IGuidanceNavigatorVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigator] {
    fn StartNavigating(&self, route: *mut GuidanceRoute) -> HRESULT,
    fn StartSimulating(&self, route: *mut GuidanceRoute, speedInMetersPerSecond: i32) -> HRESULT,
    fn StartTracking(&self) -> HRESULT,
    fn Pause(&self) -> HRESULT,
    fn Resume(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn RepeatLastAudioNotification(&self) -> HRESULT,
    fn get_AudioMeasurementSystem(&self, out: *mut GuidanceAudioMeasurementSystem) -> HRESULT,
    fn put_AudioMeasurementSystem(&self, value: GuidanceAudioMeasurementSystem) -> HRESULT,
    fn get_AudioNotifications(&self, out: *mut GuidanceAudioNotifications) -> HRESULT,
    fn put_AudioNotifications(&self, value: GuidanceAudioNotifications) -> HRESULT,
    fn add_GuidanceUpdated(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_GuidanceUpdated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DestinationReached(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DestinationReached(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Rerouting(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Rerouting(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Rerouted(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Rerouted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RerouteFailed(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RerouteFailed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UserLocationLost(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserLocationLost(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UserLocationRestored(&self, handler: *mut foundation::TypedEventHandler<GuidanceNavigator, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserLocationRestored(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn SetGuidanceVoice(&self, voiceId: i32, voiceFolder: HSTRING) -> HRESULT,
    #[cfg(feature="windows-devices")] fn UpdateUserLocation(&self, userLocation: *mut crate::windows::devices::geolocation::Geocoordinate) -> HRESULT,
    #[cfg(feature="windows-devices")] fn UpdateUserLocationWithPositionOverride(&self, userLocation: *mut crate::windows::devices::geolocation::Geocoordinate, positionOverride: crate::windows::devices::geolocation::BasicGeoposition) -> HRESULT
}}
impl ComPtr<IGuidanceNavigator> {
    #[inline] pub fn start_navigating(&self, route: &ComPtr<GuidanceRoute>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).StartNavigating)(self.as_abi() as *const _ as *mut _, route.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_simulating(&self, route: &ComPtr<GuidanceRoute>, speedInMetersPerSecond: i32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).StartSimulating)(self.as_abi() as *const _ as *mut _, route.as_abi() as *const _ as *mut _, speedInMetersPerSecond);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_tracking(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).StartTracking)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn pause(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Pause)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn resume(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Resume)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Stop)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn repeat_last_audio_notification(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).RepeatLastAudioNotification)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_audio_measurement_system(&self) -> Result<GuidanceAudioMeasurementSystem> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AudioMeasurementSystem)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_audio_measurement_system(&self, value: GuidanceAudioMeasurementSystem) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_AudioMeasurementSystem)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_audio_notifications(&self) -> Result<GuidanceAudioNotifications> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AudioNotifications)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_audio_notifications(&self, value: GuidanceAudioNotifications) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_AudioNotifications)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_guidance_updated(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, GuidanceUpdatedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_GuidanceUpdated)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_guidance_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_GuidanceUpdated)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_destination_reached(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_DestinationReached)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_destination_reached(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_DestinationReached)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_rerouting(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_Rerouting)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_rerouting(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_Rerouting)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_rerouted(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, GuidanceReroutedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_Rerouted)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_rerouted(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_Rerouted)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_reroute_failed(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_RerouteFailed)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_reroute_failed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_RerouteFailed)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_user_location_lost(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_UserLocationLost)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_user_location_lost(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_UserLocationLost)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_user_location_restored(&self, handler: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_UserLocationRestored)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_user_location_restored(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_UserLocationRestored)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_guidance_voice(&self, voiceId: i32, voiceFolder: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetGuidanceVoice)(self.as_abi() as *const _ as *mut _, voiceId, voiceFolder.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn update_user_location(&self, userLocation: &ComPtr<crate::windows::devices::geolocation::Geocoordinate>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).UpdateUserLocation)(self.as_abi() as *const _ as *mut _, userLocation.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn update_user_location_with_position_override(&self, userLocation: &ComPtr<crate::windows::devices::geolocation::Geocoordinate>, positionOverride: crate::windows::devices::geolocation::BasicGeoposition) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).UpdateUserLocationWithPositionOverride)(self.as_abi() as *const _ as *mut _, userLocation.as_abi() as *const _ as *mut _, positionOverride);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceNavigator: IGuidanceNavigator}
impl RtActivatable<IGuidanceNavigatorStatics> for GuidanceNavigator {}
impl RtActivatable<IGuidanceNavigatorStatics2> for GuidanceNavigator {}
impl GuidanceNavigator {
    #[inline] pub fn get_current() -> Result<Option<ComPtr<GuidanceNavigator>>> {
        <Self as RtActivatable<IGuidanceNavigatorStatics>>::get_activation_factory().get_current()
    }
    #[inline] pub fn get_use_app_provided_voice() -> Result<bool> {
        <Self as RtActivatable<IGuidanceNavigatorStatics2>>::get_activation_factory().get_use_app_provided_voice()
    }
}
DEFINE_CLSID!(GuidanceNavigator(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,71,117,105,100,97,110,99,101,46,71,117,105,100,97,110,99,101,78,97,118,105,103,97,116,111,114,0]) [CLSID_GuidanceNavigator]);
DEFINE_IID!(IID_IGuidanceNavigator2, 1826377937, 1052, 19443, 182, 51, 161, 1, 252, 47, 107, 87);
RT_INTERFACE!{interface IGuidanceNavigator2(IGuidanceNavigator2Vtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigator2] {
    fn add_AudioNotificationRequested(&self, value: *mut foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AudioNotificationRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_IsGuidanceAudioMuted(&self, out: *mut bool) -> HRESULT,
    fn put_IsGuidanceAudioMuted(&self, value: bool) -> HRESULT
}}
impl ComPtr<IGuidanceNavigator2> {
    #[inline] pub fn add_audio_notification_requested(&self, value: &ComPtr<foundation::TypedEventHandler<GuidanceNavigator, GuidanceAudioNotificationRequestedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_AudioNotificationRequested)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_audio_notification_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_AudioNotificationRequested)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_guidance_audio_muted(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsGuidanceAudioMuted)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_guidance_audio_muted(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_IsGuidanceAudioMuted)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGuidanceNavigatorStatics, 16618771, 17494, 20070, 161, 67, 58, 221, 107, 224, 132, 38);
RT_INTERFACE!{static interface IGuidanceNavigatorStatics(IGuidanceNavigatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigatorStatics] {
    fn GetCurrent(&self, out: *mut *mut GuidanceNavigator) -> HRESULT
}}
impl ComPtr<IGuidanceNavigatorStatics> {
    #[inline] pub fn get_current(&self) -> Result<Option<ComPtr<GuidanceNavigator>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCurrent)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGuidanceNavigatorStatics2, 1422246882, 30596, 19589, 140, 149, 208, 198, 239, 180, 57, 101);
RT_INTERFACE!{static interface IGuidanceNavigatorStatics2(IGuidanceNavigatorStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceNavigatorStatics2] {
    fn get_UseAppProvidedVoice(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGuidanceNavigatorStatics2> {
    #[inline] pub fn get_use_app_provided_voice(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_UseAppProvidedVoice)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGuidanceReroutedEventArgs, 291323912, 54568, 17742, 187, 148, 165, 3, 65, 210, 201, 241);
RT_INTERFACE!{interface IGuidanceReroutedEventArgs(IGuidanceReroutedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceReroutedEventArgs] {
    fn get_Route(&self, out: *mut *mut GuidanceRoute) -> HRESULT
}}
impl ComPtr<IGuidanceReroutedEventArgs> {
    #[inline] pub fn get_route(&self) -> Result<Option<ComPtr<GuidanceRoute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Route)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceReroutedEventArgs: IGuidanceReroutedEventArgs}
DEFINE_IID!(IID_IGuidanceRoadSegment, 3005700262, 48760, 19555, 175, 231, 108, 41, 87, 71, 155, 62);
RT_INTERFACE!{interface IGuidanceRoadSegment(IGuidanceRoadSegmentVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoadSegment] {
    fn get_RoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ShortRoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SpeedLimit(&self, out: *mut f64) -> HRESULT,
    fn get_TravelTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut crate::windows::devices::geolocation::Geopath) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsHighway(&self, out: *mut bool) -> HRESULT,
    fn get_IsTunnel(&self, out: *mut bool) -> HRESULT,
    fn get_IsTollRoad(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGuidanceRoadSegment> {
    #[inline] pub fn get_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_short_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ShortRoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_speed_limit(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SpeedLimit)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_travel_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TravelTime)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_path(&self) -> Result<Option<ComPtr<crate::windows::devices::geolocation::Geopath>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Id)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_highway(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsHighway)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_tunnel(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTunnel)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_toll_road(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTollRoad)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceRoadSegment: IGuidanceRoadSegment}
DEFINE_IID!(IID_IGuidanceRoadSegment2, 611624477, 5923, 18929, 137, 91, 71, 162, 196, 170, 156, 85);
RT_INTERFACE!{interface IGuidanceRoadSegment2(IGuidanceRoadSegment2Vtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoadSegment2] {
    fn get_IsScenic(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IGuidanceRoadSegment2> {
    #[inline] pub fn get_is_scenic(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsScenic)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGuidanceRoadSignpost, 4054263990, 63354, 18242, 131, 18, 83, 48, 15, 152, 69, 240);
RT_INTERFACE!{interface IGuidanceRoadSignpost(IGuidanceRoadSignpostVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoadSignpost] {
    fn get_ExitNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Exit(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_BackgroundColor(&self, out: *mut crate::windows::ui::Color) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_ForegroundColor(&self, out: *mut crate::windows::ui::Color) -> HRESULT,
    fn get_ExitDirections(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl ComPtr<IGuidanceRoadSignpost> {
    #[inline] pub fn get_exit_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExitNumber)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_exit(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Exit)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_background_color(&self) -> Result<crate::windows::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BackgroundColor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_foreground_color(&self) -> Result<crate::windows::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ForegroundColor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_exit_directions(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExitDirections)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceRoadSignpost: IGuidanceRoadSignpost}
DEFINE_IID!(IID_IGuidanceRoute, 974410845, 32794, 16573, 162, 134, 175, 178, 1, 12, 206, 108);
RT_INTERFACE!{interface IGuidanceRoute(IGuidanceRouteVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRoute] {
    fn get_Duration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_Distance(&self, out: *mut i32) -> HRESULT,
    fn get_Maneuvers(&self, out: *mut *mut foundation::collections::IVectorView<GuidanceManeuver>) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_BoundingBox(&self, out: *mut *mut crate::windows::devices::geolocation::GeoboundingBox) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Path(&self, out: *mut *mut crate::windows::devices::geolocation::Geopath) -> HRESULT,
    fn get_RoadSegments(&self, out: *mut *mut foundation::collections::IVectorView<GuidanceRoadSegment>) -> HRESULT,
    fn ConvertToMapRoute(&self, out: *mut *mut super::MapRoute) -> HRESULT
}}
impl ComPtr<IGuidanceRoute> {
    #[inline] pub fn get_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Duration)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_distance(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Distance)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_maneuvers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<GuidanceManeuver>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Maneuvers)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_bounding_box(&self) -> Result<Option<ComPtr<crate::windows::devices::geolocation::GeoboundingBox>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BoundingBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_path(&self) -> Result<Option<ComPtr<crate::windows::devices::geolocation::Geopath>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_road_segments(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<GuidanceRoadSegment>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RoadSegments)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn convert_to_map_route(&self) -> Result<Option<ComPtr<super::MapRoute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ConvertToMapRoute)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceRoute: IGuidanceRoute}
impl RtActivatable<IGuidanceRouteStatics> for GuidanceRoute {}
impl GuidanceRoute {
    #[inline] pub fn can_create_from_map_route(mapRoute: &ComPtr<super::MapRoute>) -> Result<bool> {
        <Self as RtActivatable<IGuidanceRouteStatics>>::get_activation_factory().can_create_from_map_route(mapRoute)
    }
    #[inline] pub fn try_create_from_map_route(mapRoute: &ComPtr<super::MapRoute>) -> Result<Option<ComPtr<GuidanceRoute>>> {
        <Self as RtActivatable<IGuidanceRouteStatics>>::get_activation_factory().try_create_from_map_route(mapRoute)
    }
}
DEFINE_CLSID!(GuidanceRoute(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,71,117,105,100,97,110,99,101,46,71,117,105,100,97,110,99,101,82,111,117,116,101,0]) [CLSID_GuidanceRoute]);
DEFINE_IID!(IID_IGuidanceRouteStatics, 4117598826, 21997, 18881, 176, 156, 75, 130, 35, 181, 13, 179);
RT_INTERFACE!{static interface IGuidanceRouteStatics(IGuidanceRouteStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceRouteStatics] {
    fn CanCreateFromMapRoute(&self, mapRoute: *mut super::MapRoute, out: *mut bool) -> HRESULT,
    fn TryCreateFromMapRoute(&self, mapRoute: *mut super::MapRoute, out: *mut *mut GuidanceRoute) -> HRESULT
}}
impl ComPtr<IGuidanceRouteStatics> {
    #[inline] pub fn can_create_from_map_route(&self, mapRoute: &ComPtr<super::MapRoute>) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).CanCreateFromMapRoute)(self.as_abi() as *const _ as *mut _, mapRoute.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_create_from_map_route(&self, mapRoute: &ComPtr<super::MapRoute>) -> Result<Option<ComPtr<GuidanceRoute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryCreateFromMapRoute)(self.as_abi() as *const _ as *mut _, mapRoute.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGuidanceTelemetryCollector, 3676278181, 47224, 19858, 152, 221, 52, 125, 35, 211, 130, 98);
RT_INTERFACE!{interface IGuidanceTelemetryCollector(IGuidanceTelemetryCollectorVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceTelemetryCollector] {
    fn get_Enabled(&self, out: *mut bool) -> HRESULT,
    fn put_Enabled(&self, value: bool) -> HRESULT,
    fn ClearLocalData(&self) -> HRESULT,
    fn get_SpeedTrigger(&self, out: *mut f64) -> HRESULT,
    fn put_SpeedTrigger(&self, value: f64) -> HRESULT,
    fn get_UploadFrequency(&self, out: *mut i32) -> HRESULT,
    fn put_UploadFrequency(&self, value: i32) -> HRESULT
}}
impl ComPtr<IGuidanceTelemetryCollector> {
    #[inline] pub fn get_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Enabled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Enabled)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear_local_data(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ClearLocalData)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_speed_trigger(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SpeedTrigger)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_speed_trigger(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_SpeedTrigger)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_upload_frequency(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_UploadFrequency)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_upload_frequency(&self, value: i32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_UploadFrequency)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceTelemetryCollector: IGuidanceTelemetryCollector}
impl RtActivatable<IGuidanceTelemetryCollectorStatics> for GuidanceTelemetryCollector {}
impl GuidanceTelemetryCollector {
    #[inline] pub fn get_current() -> Result<Option<ComPtr<GuidanceTelemetryCollector>>> {
        <Self as RtActivatable<IGuidanceTelemetryCollectorStatics>>::get_activation_factory().get_current()
    }
}
DEFINE_CLSID!(GuidanceTelemetryCollector(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,71,117,105,100,97,110,99,101,46,71,117,105,100,97,110,99,101,84,101,108,101,109,101,116,114,121,67,111,108,108,101,99,116,111,114,0]) [CLSID_GuidanceTelemetryCollector]);
DEFINE_IID!(IID_IGuidanceTelemetryCollectorStatics, 911417415, 61792, 17659, 181, 120, 148, 87, 124, 160, 89, 144);
RT_INTERFACE!{static interface IGuidanceTelemetryCollectorStatics(IGuidanceTelemetryCollectorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceTelemetryCollectorStatics] {
    fn GetCurrent(&self, out: *mut *mut GuidanceTelemetryCollector) -> HRESULT
}}
impl ComPtr<IGuidanceTelemetryCollectorStatics> {
    #[inline] pub fn get_current(&self) -> Result<Option<ComPtr<GuidanceTelemetryCollector>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCurrent)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGuidanceUpdatedEventArgs, 4255913483, 40589, 19939, 169, 250, 176, 99, 33, 209, 141, 185);
RT_INTERFACE!{interface IGuidanceUpdatedEventArgs(IGuidanceUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IGuidanceUpdatedEventArgs] {
    fn get_Mode(&self, out: *mut GuidanceMode) -> HRESULT,
    fn get_NextManeuver(&self, out: *mut *mut GuidanceManeuver) -> HRESULT,
    fn get_NextManeuverDistance(&self, out: *mut i32) -> HRESULT,
    fn get_AfterNextManeuver(&self, out: *mut *mut GuidanceManeuver) -> HRESULT,
    fn get_AfterNextManeuverDistance(&self, out: *mut i32) -> HRESULT,
    fn get_DistanceToDestination(&self, out: *mut i32) -> HRESULT,
    fn get_ElapsedDistance(&self, out: *mut i32) -> HRESULT,
    fn get_ElapsedTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_TimeToDestination(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_RoadName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Route(&self, out: *mut *mut GuidanceRoute) -> HRESULT,
    fn get_CurrentLocation(&self, out: *mut *mut GuidanceMapMatchedCoordinate) -> HRESULT,
    fn get_IsNewManeuver(&self, out: *mut bool) -> HRESULT,
    fn get_LaneInfo(&self, out: *mut *mut foundation::collections::IVectorView<GuidanceLaneInfo>) -> HRESULT
}}
impl ComPtr<IGuidanceUpdatedEventArgs> {
    #[inline] pub fn get_mode(&self) -> Result<GuidanceMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Mode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_next_maneuver(&self) -> Result<Option<ComPtr<GuidanceManeuver>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NextManeuver)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_maneuver_distance(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_NextManeuverDistance)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_after_next_maneuver(&self) -> Result<Option<ComPtr<GuidanceManeuver>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AfterNextManeuver)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_after_next_maneuver_distance(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AfterNextManeuverDistance)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_distance_to_destination(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DistanceToDestination)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_elapsed_distance(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ElapsedDistance)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_elapsed_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ElapsedTime)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_time_to_destination(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TimeToDestination)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_road_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RoadName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_route(&self) -> Result<Option<ComPtr<GuidanceRoute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Route)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_location(&self) -> Result<Option<ComPtr<GuidanceMapMatchedCoordinate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CurrentLocation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_new_maneuver(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsNewManeuver)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_lane_info(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<GuidanceLaneInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LaneInfo)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GuidanceUpdatedEventArgs: IGuidanceUpdatedEventArgs}
} // Windows.Services.Maps.Guidance
pub mod localsearch { // Windows.Services.Maps.LocalSearch
use crate::prelude::*;
RT_CLASS!{static class LocalCategories}
impl RtActivatable<ILocalCategoriesStatics> for LocalCategories {}
impl LocalCategories {
    #[inline] pub fn get_bank_and_credit_unions() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_bank_and_credit_unions()
    }
    #[inline] pub fn get_eat_drink() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_eat_drink()
    }
    #[inline] pub fn get_hospitals() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_hospitals()
    }
    #[inline] pub fn get_hotels_and_motels() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_hotels_and_motels()
    }
    #[inline] pub fn get_all() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_all()
    }
    #[inline] pub fn get_parking() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_parking()
    }
    #[inline] pub fn get_see_do() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_see_do()
    }
    #[inline] pub fn get_shop() -> Result<HString> {
        <Self as RtActivatable<ILocalCategoriesStatics>>::get_activation_factory().get_shop()
    }
}
DEFINE_CLSID!(LocalCategories(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,76,111,99,97,108,83,101,97,114,99,104,46,76,111,99,97,108,67,97,116,101,103,111,114,105,101,115,0]) [CLSID_LocalCategories]);
DEFINE_IID!(IID_ILocalCategoriesStatics, 4103313909, 33377, 17185, 153, 116, 239, 146, 212, 154, 141, 202);
RT_INTERFACE!{static interface ILocalCategoriesStatics(ILocalCategoriesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILocalCategoriesStatics] {
    fn get_BankAndCreditUnions(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EatDrink(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hospitals(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HotelsAndMotels(&self, out: *mut HSTRING) -> HRESULT,
    fn get_All(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Parking(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SeeDo(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Shop(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<ILocalCategoriesStatics> {
    #[inline] pub fn get_bank_and_credit_unions(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BankAndCreditUnions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_eat_drink(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_EatDrink)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hospitals(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Hospitals)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hotels_and_motels(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_HotelsAndMotels)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_all(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_All)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parking(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Parking)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_see_do(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SeeDo)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_shop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Shop)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILocalLocation, 3138382251, 17666, 20268, 148, 169, 13, 96, 222, 14, 33, 99);
RT_INTERFACE!{interface ILocalLocation(ILocalLocationVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocation] {
    fn get_Address(&self, out: *mut *mut super::MapAddress) -> HRESULT,
    fn get_Identifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Point(&self, out: *mut *mut crate::windows::devices::geolocation::Geopoint) -> HRESULT,
    fn get_PhoneNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DataAttribution(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<ILocalLocation> {
    #[inline] pub fn get_address(&self) -> Result<Option<ComPtr<super::MapAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Address)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Identifier)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_point(&self) -> Result<Option<ComPtr<crate::windows::devices::geolocation::Geopoint>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Point)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_phone_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PhoneNumber)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data_attribution(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DataAttribution)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LocalLocation: ILocalLocation}
DEFINE_IID!(IID_ILocalLocation2, 1855860860, 60597, 20476, 187, 140, 186, 80, 186, 140, 45, 198);
RT_INTERFACE!{interface ILocalLocation2(ILocalLocation2Vtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocation2] {
    fn get_Category(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RatingInfo(&self, out: *mut *mut LocalLocationRatingInfo) -> HRESULT,
    fn get_HoursOfOperation(&self, out: *mut *mut foundation::collections::IVectorView<LocalLocationHoursOfOperationItem>) -> HRESULT
}}
impl ComPtr<ILocalLocation2> {
    #[inline] pub fn get_category(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Category)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rating_info(&self) -> Result<Option<ComPtr<LocalLocationRatingInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RatingInfo)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hours_of_operation(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<LocalLocationHoursOfOperationItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_HoursOfOperation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class LocalLocationFinder}
impl RtActivatable<ILocalLocationFinderStatics> for LocalLocationFinder {}
impl LocalLocationFinder {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_local_locations_async(searchTerm: &HStringArg, searchArea: &ComPtr<crate::windows::devices::geolocation::Geocircle>, localCategory: &HStringArg, maxResults: u32) -> Result<ComPtr<foundation::IAsyncOperation<LocalLocationFinderResult>>> {
        <Self as RtActivatable<ILocalLocationFinderStatics>>::get_activation_factory().find_local_locations_async(searchTerm, searchArea, localCategory, maxResults)
    }
}
DEFINE_CLSID!(LocalLocationFinder(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,76,111,99,97,108,83,101,97,114,99,104,46,76,111,99,97,108,76,111,99,97,116,105,111,110,70,105,110,100,101,114,0]) [CLSID_LocalLocationFinder]);
DEFINE_IID!(IID_ILocalLocationFinderResult, 3499846854, 62264, 16785, 159, 216, 84, 64, 185, 166, 143, 82);
RT_INTERFACE!{interface ILocalLocationFinderResult(ILocalLocationFinderResultVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationFinderResult] {
    fn get_LocalLocations(&self, out: *mut *mut foundation::collections::IVectorView<LocalLocation>) -> HRESULT,
    fn get_Status(&self, out: *mut LocalLocationFinderStatus) -> HRESULT
}}
impl ComPtr<ILocalLocationFinderResult> {
    #[inline] pub fn get_local_locations(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<LocalLocation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LocalLocations)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<LocalLocationFinderStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class LocalLocationFinderResult: ILocalLocationFinderResult}
DEFINE_IID!(IID_ILocalLocationFinderStatics, 3538907972, 41182, 18634, 129, 168, 7, 199, 220, 253, 55, 171);
RT_INTERFACE!{static interface ILocalLocationFinderStatics(ILocalLocationFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationFinderStatics] {
    #[cfg(feature="windows-devices")] fn FindLocalLocationsAsync(&self, searchTerm: HSTRING, searchArea: *mut crate::windows::devices::geolocation::Geocircle, localCategory: HSTRING, maxResults: u32, out: *mut *mut foundation::IAsyncOperation<LocalLocationFinderResult>) -> HRESULT
}}
impl ComPtr<ILocalLocationFinderStatics> {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_local_locations_async(&self, searchTerm: &HStringArg, searchArea: &ComPtr<crate::windows::devices::geolocation::Geocircle>, localCategory: &HStringArg, maxResults: u32) -> Result<ComPtr<foundation::IAsyncOperation<LocalLocationFinderResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindLocalLocationsAsync)(self.as_abi() as *const _ as *mut _, searchTerm.get(), searchArea.as_abi() as *const _ as *mut _, localCategory.get(), maxResults, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum LocalLocationFinderStatus: i32 {
    Success = 0, UnknownError = 1, InvalidCredentials = 2, InvalidCategory = 3, InvalidSearchTerm = 4, InvalidSearchArea = 5, NetworkFailure = 6, NotSupported = 7,
}}
DEFINE_IID!(IID_ILocalLocationHoursOfOperationItem, 592743538, 41415, 17393, 164, 240, 16, 145, 195, 158, 198, 64);
RT_INTERFACE!{interface ILocalLocationHoursOfOperationItem(ILocalLocationHoursOfOperationItemVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationHoursOfOperationItem] {
    #[cfg(not(feature="windows-globalization"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-globalization")] fn get_Day(&self, out: *mut crate::windows::globalization::DayOfWeek) -> HRESULT,
    fn get_Start(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_Span(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl ComPtr<ILocalLocationHoursOfOperationItem> {
    #[cfg(feature="windows-globalization")] #[inline] pub fn get_day(&self) -> Result<crate::windows::globalization::DayOfWeek> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Day)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_start(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Start)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_span(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Span)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class LocalLocationHoursOfOperationItem: ILocalLocationHoursOfOperationItem}
DEFINE_IID!(IID_ILocalLocationRatingInfo, 3407719254, 13140, 17169, 139, 192, 162, 212, 213, 235, 128, 110);
RT_INTERFACE!{interface ILocalLocationRatingInfo(ILocalLocationRatingInfoVtbl): IInspectable(IInspectableVtbl) [IID_ILocalLocationRatingInfo] {
    fn get_AggregateRating(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_RatingCount(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ProviderIdentifier(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<ILocalLocationRatingInfo> {
    #[inline] pub fn get_aggregate_rating(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AggregateRating)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rating_count(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RatingCount)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ProviderIdentifier)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LocalLocationRatingInfo: ILocalLocationRatingInfo}
RT_CLASS!{static class PlaceInfoHelper}
impl RtActivatable<IPlaceInfoHelperStatics> for PlaceInfoHelper {}
impl PlaceInfoHelper {
    #[inline] pub fn create_from_local_location(location: &ComPtr<LocalLocation>) -> Result<Option<ComPtr<super::PlaceInfo>>> {
        <Self as RtActivatable<IPlaceInfoHelperStatics>>::get_activation_factory().create_from_local_location(location)
    }
}
DEFINE_CLSID!(PlaceInfoHelper(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,76,111,99,97,108,83,101,97,114,99,104,46,80,108,97,99,101,73,110,102,111,72,101,108,112,101,114,0]) [CLSID_PlaceInfoHelper]);
DEFINE_IID!(IID_IPlaceInfoHelperStatics, 3709643175, 43462, 18715, 188, 9, 232, 15, 206, 164, 142, 230);
RT_INTERFACE!{static interface IPlaceInfoHelperStatics(IPlaceInfoHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlaceInfoHelperStatics] {
    fn CreateFromLocalLocation(&self, location: *mut LocalLocation, out: *mut *mut super::PlaceInfo) -> HRESULT
}}
impl ComPtr<IPlaceInfoHelperStatics> {
    #[inline] pub fn create_from_local_location(&self, location: &ComPtr<LocalLocation>) -> Result<Option<ComPtr<super::PlaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromLocalLocation)(self.as_abi() as *const _ as *mut _, location.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.Services.Maps.LocalSearch
pub mod offlinemaps { // Windows.Services.Maps.OfflineMaps
use crate::prelude::*;
DEFINE_IID!(IID_IOfflineMapPackage, 2811717435, 42421, 16708, 181, 37, 230, 140, 136, 98, 102, 75);
RT_INTERFACE!{interface IOfflineMapPackage(IOfflineMapPackageVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackage] {
    fn get_Status(&self, out: *mut OfflineMapPackageStatus) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EnclosingRegionName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EstimatedSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn remove_StatusChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_StatusChanged(&self, value: *mut foundation::TypedEventHandler<OfflineMapPackage, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn RequestStartDownloadAsync(&self, out: *mut *mut foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>) -> HRESULT
}}
impl ComPtr<IOfflineMapPackage> {
    #[inline] pub fn get_status(&self) -> Result<OfflineMapPackageStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_enclosing_region_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_EnclosingRegionName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_estimated_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EstimatedSizeInBytes)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_status_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_StatusChanged)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_status_changed(&self, value: &ComPtr<foundation::TypedEventHandler<OfflineMapPackage, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_StatusChanged)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn request_start_download_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageStartDownloadResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestStartDownloadAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OfflineMapPackage: IOfflineMapPackage}
impl RtActivatable<IOfflineMapPackageStatics> for OfflineMapPackage {}
impl OfflineMapPackage {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_async(queryPoint: &ComPtr<crate::windows::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> {
        <Self as RtActivatable<IOfflineMapPackageStatics>>::get_activation_factory().find_packages_async(queryPoint)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_in_bounding_box_async(queryBoundingBox: &ComPtr<crate::windows::devices::geolocation::GeoboundingBox>) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> {
        <Self as RtActivatable<IOfflineMapPackageStatics>>::get_activation_factory().find_packages_in_bounding_box_async(queryBoundingBox)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_in_geocircle_async(queryCircle: &ComPtr<crate::windows::devices::geolocation::Geocircle>) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> {
        <Self as RtActivatable<IOfflineMapPackageStatics>>::get_activation_factory().find_packages_in_geocircle_async(queryCircle)
    }
}
DEFINE_CLSID!(OfflineMapPackage(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,77,97,112,115,46,79,102,102,108,105,110,101,77,97,112,115,46,79,102,102,108,105,110,101,77,97,112,80,97,99,107,97,103,101,0]) [CLSID_OfflineMapPackage]);
DEFINE_IID!(IID_IOfflineMapPackageQueryResult, 1431852049, 14817, 20033, 164, 225, 95, 72, 114, 190, 225, 153);
RT_INTERFACE!{interface IOfflineMapPackageQueryResult(IOfflineMapPackageQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackageQueryResult] {
    fn get_Status(&self, out: *mut OfflineMapPackageQueryStatus) -> HRESULT,
    fn get_Packages(&self, out: *mut *mut foundation::collections::IVectorView<OfflineMapPackage>) -> HRESULT
}}
impl ComPtr<IOfflineMapPackageQueryResult> {
    #[inline] pub fn get_status(&self) -> Result<OfflineMapPackageQueryStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_packages(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<OfflineMapPackage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Packages)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OfflineMapPackageQueryResult: IOfflineMapPackageQueryResult}
RT_ENUM! { enum OfflineMapPackageQueryStatus: i32 {
    Success = 0, UnknownError = 1, InvalidCredentials = 2, NetworkFailure = 3,
}}
DEFINE_IID!(IID_IOfflineMapPackageStartDownloadResult, 3647322392, 54486, 19198, 147, 120, 62, 199, 30, 241, 28, 61);
RT_INTERFACE!{interface IOfflineMapPackageStartDownloadResult(IOfflineMapPackageStartDownloadResultVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackageStartDownloadResult] {
    fn get_Status(&self, out: *mut OfflineMapPackageStartDownloadStatus) -> HRESULT
}}
impl ComPtr<IOfflineMapPackageStartDownloadResult> {
    #[inline] pub fn get_status(&self) -> Result<OfflineMapPackageStartDownloadStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class OfflineMapPackageStartDownloadResult: IOfflineMapPackageStartDownloadResult}
RT_ENUM! { enum OfflineMapPackageStartDownloadStatus: i32 {
    Success = 0, UnknownError = 1, InvalidCredentials = 2, DeniedWithoutCapability = 3,
}}
DEFINE_IID!(IID_IOfflineMapPackageStatics, 408844578, 43057, 19120, 148, 31, 105, 152, 250, 146, 146, 133);
RT_INTERFACE!{static interface IOfflineMapPackageStatics(IOfflineMapPackageStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IOfflineMapPackageStatics] {
    #[cfg(feature="windows-devices")] fn FindPackagesAsync(&self, queryPoint: *mut crate::windows::devices::geolocation::Geopoint, out: *mut *mut foundation::IAsyncOperation<OfflineMapPackageQueryResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindPackagesInBoundingBoxAsync(&self, queryBoundingBox: *mut crate::windows::devices::geolocation::GeoboundingBox, out: *mut *mut foundation::IAsyncOperation<OfflineMapPackageQueryResult>) -> HRESULT,
    #[cfg(feature="windows-devices")] fn FindPackagesInGeocircleAsync(&self, queryCircle: *mut crate::windows::devices::geolocation::Geocircle, out: *mut *mut foundation::IAsyncOperation<OfflineMapPackageQueryResult>) -> HRESULT
}}
impl ComPtr<IOfflineMapPackageStatics> {
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_async(&self, queryPoint: &ComPtr<crate::windows::devices::geolocation::Geopoint>) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindPackagesAsync)(self.as_abi() as *const _ as *mut _, queryPoint.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_in_bounding_box_async(&self, queryBoundingBox: &ComPtr<crate::windows::devices::geolocation::GeoboundingBox>) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindPackagesInBoundingBoxAsync)(self.as_abi() as *const _ as *mut _, queryBoundingBox.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn find_packages_in_geocircle_async(&self, queryCircle: &ComPtr<crate::windows::devices::geolocation::Geocircle>) -> Result<ComPtr<foundation::IAsyncOperation<OfflineMapPackageQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindPackagesInGeocircleAsync)(self.as_abi() as *const _ as *mut _, queryCircle.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum OfflineMapPackageStatus: i32 {
    NotDownloaded = 0, Downloading = 1, Downloaded = 2, Deleting = 3,
}}
} // Windows.Services.Maps.OfflineMaps
} // Windows.Services.Maps
pub mod store { // Windows.Services.Store
use crate::prelude::*;
DEFINE_IID!(IID_IStoreAcquireLicenseResult, 4225209453, 61504, 19635, 154, 57, 41, 188, 236, 219, 226, 45);
RT_INTERFACE!{interface IStoreAcquireLicenseResult(IStoreAcquireLicenseResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreAcquireLicenseResult] {
    fn get_StorePackageLicense(&self, out: *mut *mut StorePackageLicense) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStoreAcquireLicenseResult> {
    #[inline] pub fn get_store_package_license(&self) -> Result<Option<ComPtr<StorePackageLicense>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StorePackageLicense)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreAcquireLicenseResult: IStoreAcquireLicenseResult}
DEFINE_IID!(IID_IStoreAppLicense, 4085905886, 29632, 17870, 155, 171, 178, 254, 62, 94, 175, 211);
RT_INTERFACE!{interface IStoreAppLicense(IStoreAppLicenseVtbl): IInspectable(IInspectableVtbl) [IID_IStoreAppLicense] {
    fn get_SkuStoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsActive(&self, out: *mut bool) -> HRESULT,
    fn get_IsTrial(&self, out: *mut bool) -> HRESULT,
    fn get_ExpirationDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AddOnLicenses(&self, out: *mut *mut foundation::collections::IMapView<HString, StoreLicense>) -> HRESULT,
    fn get_TrialTimeRemaining(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_IsTrialOwnedByThisUser(&self, out: *mut bool) -> HRESULT,
    fn get_TrialUniqueId(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IStoreAppLicense> {
    #[inline] pub fn get_sku_store_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SkuStoreId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_active(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsActive)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_trial(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTrial)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_expiration_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExpirationDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_add_on_licenses(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, StoreLicense>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AddOnLicenses)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_trial_time_remaining(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrialTimeRemaining)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_trial_owned_by_this_user(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTrialOwnedByThisUser)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_trial_unique_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_TrialUniqueId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreAppLicense: IStoreAppLicense}
DEFINE_IID!(IID_IStoreAppLicense2, 3026611857, 17475, 16563, 153, 63, 40, 144, 68, 53, 189, 198);
RT_INTERFACE!{interface IStoreAppLicense2(IStoreAppLicense2Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreAppLicense2] {
    fn get_IsDiscLicense(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IStoreAppLicense2> {
    #[inline] pub fn get_is_disc_license(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsDiscLicense)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreAvailability, 4194698021, 4093, 17555, 173, 67, 241, 249, 145, 143, 105, 250);
RT_INTERFACE!{interface IStoreAvailability(IStoreAvailabilityVtbl): IInspectable(IInspectableVtbl) [IID_IStoreAvailability] {
    fn get_StoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EndDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_Price(&self, out: *mut *mut StorePrice) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn RequestPurchaseAsync(&self, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT
}}
impl ComPtr<IStoreAvailability> {
    #[inline] pub fn get_store_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StoreId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_end_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EndDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_price(&self) -> Result<Option<ComPtr<StorePrice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Price)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_with_purchase_properties_async(&self, storePurchaseProperties: &ComPtr<StorePurchaseProperties>) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self.as_abi() as *const _ as *mut _, storePurchaseProperties.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreAvailability: IStoreAvailability}
DEFINE_IID!(IID_IStoreCanAcquireLicenseResult, 979975603, 136, 18479, 134, 213, 189, 70, 82, 38, 99, 173);
RT_INTERFACE!{interface IStoreCanAcquireLicenseResult(IStoreCanAcquireLicenseResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreCanAcquireLicenseResult] {
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_LicensableSku(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Status(&self, out: *mut StoreCanLicenseStatus) -> HRESULT
}}
impl ComPtr<IStoreCanAcquireLicenseResult> {
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_licensable_sku(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LicensableSku)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<StoreCanLicenseStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreCanAcquireLicenseResult: IStoreCanAcquireLicenseResult}
RT_ENUM! { enum StoreCanLicenseStatus: i32 {
    NotLicensableToUser = 0, Licensable = 1, LicenseActionNotApplicableToProduct = 2, NetworkError = 3, ServerError = 4,
}}
DEFINE_IID!(IID_IStoreCollectionData, 2326053811, 23475, 17434, 42, 180, 77, 171, 115, 213, 206, 103);
RT_INTERFACE!{interface IStoreCollectionData(IStoreCollectionDataVtbl): IInspectable(IInspectableVtbl) [IID_IStoreCollectionData] {
    fn get_IsTrial(&self, out: *mut bool) -> HRESULT,
    fn get_CampaignId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeveloperOfferId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AcquiredDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_StartDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_EndDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_TrialTimeRemaining(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IStoreCollectionData> {
    #[inline] pub fn get_is_trial(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTrial)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_campaign_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CampaignId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_developer_offer_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DeveloperOfferId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_acquired_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AcquiredDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_start_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_StartDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_end_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_EndDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_trial_time_remaining(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrialTimeRemaining)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreCollectionData: IStoreCollectionData}
DEFINE_IID!(IID_IStoreConsumableResult, 3932007282, 27136, 16466, 190, 91, 191, 218, 180, 67, 51, 82);
RT_INTERFACE!{interface IStoreConsumableResult(IStoreConsumableResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreConsumableResult] {
    fn get_Status(&self, out: *mut StoreConsumableStatus) -> HRESULT,
    fn get_TrackingId(&self, out: *mut Guid) -> HRESULT,
    fn get_BalanceRemaining(&self, out: *mut u32) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStoreConsumableResult> {
    #[inline] pub fn get_status(&self) -> Result<StoreConsumableStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_tracking_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrackingId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_balance_remaining(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BalanceRemaining)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreConsumableResult: IStoreConsumableResult}
RT_ENUM! { enum StoreConsumableStatus: i32 {
    Succeeded = 0, InsufficentQuantity = 1, NetworkError = 2, ServerError = 3,
}}
DEFINE_IID!(IID_IStoreContext, 2895689406, 62717, 18706, 186, 189, 80, 53, 229, 232, 188, 171);
RT_INTERFACE!{interface IStoreContext(IStoreContextVtbl): IInspectable(IInspectableVtbl) [IID_IStoreContext] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT,
    fn add_OfflineLicensesChanged(&self, handler: *mut foundation::TypedEventHandler<StoreContext, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OfflineLicensesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn GetCustomerPurchaseIdAsync(&self, serviceTicket: HSTRING, publisherUserId: HSTRING, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetCustomerCollectionsIdAsync(&self, serviceTicket: HSTRING, publisherUserId: HSTRING, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetAppLicenseAsync(&self, out: *mut *mut foundation::IAsyncOperation<StoreAppLicense>) -> HRESULT,
    fn GetStoreProductForCurrentAppAsync(&self, out: *mut *mut foundation::IAsyncOperation<StoreProductResult>) -> HRESULT,
    fn GetStoreProductsAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, storeIds: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetAssociatedStoreProductsAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetAssociatedStoreProductsWithPagingAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, maxItemsToRetrievePerPage: u32, out: *mut *mut foundation::IAsyncOperation<StoreProductPagedQueryResult>) -> HRESULT,
    fn GetUserCollectionAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetUserCollectionWithPagingAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, maxItemsToRetrievePerPage: u32, out: *mut *mut foundation::IAsyncOperation<StoreProductPagedQueryResult>) -> HRESULT,
    fn ReportConsumableFulfillmentAsync(&self, productStoreId: HSTRING, quantity: u32, trackingId: Guid, out: *mut *mut foundation::IAsyncOperation<StoreConsumableResult>) -> HRESULT,
    fn GetConsumableBalanceRemainingAsync(&self, productStoreId: HSTRING, out: *mut *mut foundation::IAsyncOperation<StoreConsumableResult>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn AcquireStoreLicenseForOptionalPackageAsync(&self, optionalPackage: *mut super::super::applicationmodel::Package, out: *mut *mut foundation::IAsyncOperation<StoreAcquireLicenseResult>) -> HRESULT,
    fn RequestPurchaseAsync(&self, storeId: HSTRING, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storeId: HSTRING, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn GetAppAndOptionalStorePackageUpdatesAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<StorePackageUpdate>>) -> HRESULT,
    fn RequestDownloadStorePackageUpdatesAsync(&self, storePackageUpdates: *mut foundation::collections::IIterable<StorePackageUpdate>, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    fn RequestDownloadAndInstallStorePackageUpdatesAsync(&self, storePackageUpdates: *mut foundation::collections::IIterable<StorePackageUpdate>, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    fn RequestDownloadAndInstallStorePackagesAsync(&self, storeIds: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT
}}
impl ComPtr<IStoreContext> {
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::super::system::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_User)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_offline_licenses_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<StoreContext, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_OfflineLicensesChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_offline_licenses_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_OfflineLicensesChanged)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_customer_purchase_id_async(&self, serviceTicket: &HStringArg, publisherUserId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCustomerPurchaseIdAsync)(self.as_abi() as *const _ as *mut _, serviceTicket.get(), publisherUserId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_customer_collections_id_async(&self, serviceTicket: &HStringArg, publisherUserId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCustomerCollectionsIdAsync)(self.as_abi() as *const _ as *mut _, serviceTicket.get(), publisherUserId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_license_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StoreAppLicense>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAppLicenseAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_store_product_for_current_app_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetStoreProductForCurrentAppAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_store_products_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>, storeIds: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetStoreProductsAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, storeIds.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_associated_store_products_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAssociatedStoreProductsAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_associated_store_products_with_paging_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>, maxItemsToRetrievePerPage: u32) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductPagedQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAssociatedStoreProductsWithPagingAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, maxItemsToRetrievePerPage, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user_collection_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetUserCollectionAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user_collection_with_paging_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>, maxItemsToRetrievePerPage: u32) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductPagedQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetUserCollectionWithPagingAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, maxItemsToRetrievePerPage, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn report_consumable_fulfillment_async(&self, productStoreId: &HStringArg, quantity: u32, trackingId: Guid) -> Result<ComPtr<foundation::IAsyncOperation<StoreConsumableResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ReportConsumableFulfillmentAsync)(self.as_abi() as *const _ as *mut _, productStoreId.get(), quantity, trackingId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_consumable_balance_remaining_async(&self, productStoreId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StoreConsumableResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetConsumableBalanceRemainingAsync)(self.as_abi() as *const _ as *mut _, productStoreId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn acquire_store_license_for_optional_package_async(&self, optionalPackage: &ComPtr<super::super::applicationmodel::Package>) -> Result<ComPtr<foundation::IAsyncOperation<StoreAcquireLicenseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).AcquireStoreLicenseForOptionalPackageAsync)(self.as_abi() as *const _ as *mut _, optionalPackage.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_async(&self, storeId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseAsync)(self.as_abi() as *const _ as *mut _, storeId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_with_purchase_properties_async(&self, storeId: &HStringArg, storePurchaseProperties: &ComPtr<StorePurchaseProperties>) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self.as_abi() as *const _ as *mut _, storeId.get(), storePurchaseProperties.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_and_optional_store_package_updates_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<StorePackageUpdate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAppAndOptionalStorePackageUpdatesAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_download_store_package_updates_async(&self, storePackageUpdates: &ComPtr<foundation::collections::IIterable<StorePackageUpdate>>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestDownloadStorePackageUpdatesAsync)(self.as_abi() as *const _ as *mut _, storePackageUpdates.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_download_and_install_store_package_updates_async(&self, storePackageUpdates: &ComPtr<foundation::collections::IIterable<StorePackageUpdate>>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestDownloadAndInstallStorePackageUpdatesAsync)(self.as_abi() as *const _ as *mut _, storePackageUpdates.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_download_and_install_store_packages_async(&self, storeIds: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestDownloadAndInstallStorePackagesAsync)(self.as_abi() as *const _ as *mut _, storeIds.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreContext: IStoreContext}
impl RtActivatable<IStoreContextStatics> for StoreContext {}
impl StoreContext {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<StoreContext>>> {
        <Self as RtActivatable<IStoreContextStatics>>::get_activation_factory().get_default()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &ComPtr<super::super::system::User>) -> Result<Option<ComPtr<StoreContext>>> {
        <Self as RtActivatable<IStoreContextStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(StoreContext(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,67,111,110,116,101,120,116,0]) [CLSID_StoreContext]);
DEFINE_IID!(IID_IStoreContext2, 414995674, 31705, 17708, 145, 22, 59, 189, 6, 255, 198, 58);
RT_INTERFACE!{interface IStoreContext2(IStoreContext2Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreContext2] {
    #[cfg(feature="windows-applicationmodel")] fn FindStoreProductForPackageAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, package: *mut super::super::applicationmodel::Package, out: *mut *mut foundation::IAsyncOperation<StoreProductResult>) -> HRESULT
}}
impl ComPtr<IStoreContext2> {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_store_product_for_package_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>, package: &ComPtr<super::super::applicationmodel::Package>) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindStoreProductForPackageAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, package.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreContext3, 3798083274, 6657, 18224, 133, 166, 236, 200, 150, 228, 174, 56);
RT_INTERFACE!{interface IStoreContext3(IStoreContext3Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreContext3] {
    fn get_CanSilentlyDownloadStorePackageUpdates(&self, out: *mut bool) -> HRESULT,
    fn TrySilentDownloadStorePackageUpdatesAsync(&self, storePackageUpdates: *mut foundation::collections::IIterable<StorePackageUpdate>, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    fn TrySilentDownloadAndInstallStorePackageUpdatesAsync(&self, storePackageUpdates: *mut foundation::collections::IIterable<StorePackageUpdate>, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn CanAcquireStoreLicenseForOptionalPackageAsync(&self, optionalPackage: *mut super::super::applicationmodel::Package, out: *mut *mut foundation::IAsyncOperation<StoreCanAcquireLicenseResult>) -> HRESULT,
    fn CanAcquireStoreLicenseAsync(&self, productStoreId: HSTRING, out: *mut *mut foundation::IAsyncOperation<StoreCanAcquireLicenseResult>) -> HRESULT,
    fn GetStoreProductsWithOptionsAsync(&self, productKinds: *mut foundation::collections::IIterable<HString>, storeIds: *mut foundation::collections::IIterable<HString>, storeProductOptions: *mut StoreProductOptions, out: *mut *mut foundation::IAsyncOperation<StoreProductQueryResult>) -> HRESULT,
    fn GetAssociatedStoreQueueItemsAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<StoreQueueItem>>) -> HRESULT,
    fn GetStoreQueueItemsAsync(&self, storeIds: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<StoreQueueItem>>) -> HRESULT,
    fn RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync(&self, storeIds: *mut foundation::collections::IIterable<HString>, storePackageInstallOptions: *mut StorePackageInstallOptions, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    fn DownloadAndInstallStorePackagesAsync(&self, storeIds: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy10(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn RequestUninstallStorePackageAsync(&self, package: *mut super::super::applicationmodel::Package, out: *mut *mut foundation::IAsyncOperation<StoreUninstallStorePackageResult>) -> HRESULT,
    fn RequestUninstallStorePackageByStoreIdAsync(&self, storeId: HSTRING, out: *mut *mut foundation::IAsyncOperation<StoreUninstallStorePackageResult>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy12(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn UninstallStorePackageAsync(&self, package: *mut super::super::applicationmodel::Package, out: *mut *mut foundation::IAsyncOperation<StoreUninstallStorePackageResult>) -> HRESULT,
    fn UninstallStorePackageByStoreIdAsync(&self, storeId: HSTRING, out: *mut *mut foundation::IAsyncOperation<StoreUninstallStorePackageResult>) -> HRESULT
}}
impl ComPtr<IStoreContext3> {
    #[inline] pub fn get_can_silently_download_store_package_updates(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_CanSilentlyDownloadStorePackageUpdates)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_silent_download_store_package_updates_async(&self, storePackageUpdates: &ComPtr<foundation::collections::IIterable<StorePackageUpdate>>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TrySilentDownloadStorePackageUpdatesAsync)(self.as_abi() as *const _ as *mut _, storePackageUpdates.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_silent_download_and_install_store_package_updates_async(&self, storePackageUpdates: &ComPtr<foundation::collections::IIterable<StorePackageUpdate>>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TrySilentDownloadAndInstallStorePackageUpdatesAsync)(self.as_abi() as *const _ as *mut _, storePackageUpdates.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn can_acquire_store_license_for_optional_package_async(&self, optionalPackage: &ComPtr<super::super::applicationmodel::Package>) -> Result<ComPtr<foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CanAcquireStoreLicenseForOptionalPackageAsync)(self.as_abi() as *const _ as *mut _, optionalPackage.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn can_acquire_store_license_async(&self, productStoreId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StoreCanAcquireLicenseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CanAcquireStoreLicenseAsync)(self.as_abi() as *const _ as *mut _, productStoreId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_store_products_with_options_async(&self, productKinds: &ComPtr<foundation::collections::IIterable<HString>>, storeIds: &ComPtr<foundation::collections::IIterable<HString>>, storeProductOptions: &ComPtr<StoreProductOptions>) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetStoreProductsWithOptionsAsync)(self.as_abi() as *const _ as *mut _, productKinds.as_abi() as *const _ as *mut _, storeIds.as_abi() as *const _ as *mut _, storeProductOptions.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_associated_store_queue_items_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<StoreQueueItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAssociatedStoreQueueItemsAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_store_queue_items_async(&self, storeIds: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<StoreQueueItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetStoreQueueItemsAsync)(self.as_abi() as *const _ as *mut _, storeIds.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_download_and_install_store_packages_with_install_options_async(&self, storeIds: &ComPtr<foundation::collections::IIterable<HString>>, storePackageInstallOptions: &ComPtr<StorePackageInstallOptions>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestDownloadAndInstallStorePackagesWithInstallOptionsAsync)(self.as_abi() as *const _ as *mut _, storeIds.as_abi() as *const _ as *mut _, storePackageInstallOptions.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn download_and_install_store_packages_async(&self, storeIds: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<StorePackageUpdateResult, StorePackageUpdateStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).DownloadAndInstallStorePackagesAsync)(self.as_abi() as *const _ as *mut _, storeIds.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn request_uninstall_store_package_async(&self, package: &ComPtr<super::super::applicationmodel::Package>) -> Result<ComPtr<foundation::IAsyncOperation<StoreUninstallStorePackageResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestUninstallStorePackageAsync)(self.as_abi() as *const _ as *mut _, package.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_uninstall_store_package_by_store_id_async(&self, storeId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StoreUninstallStorePackageResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestUninstallStorePackageByStoreIdAsync)(self.as_abi() as *const _ as *mut _, storeId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn uninstall_store_package_async(&self, package: &ComPtr<super::super::applicationmodel::Package>) -> Result<ComPtr<foundation::IAsyncOperation<StoreUninstallStorePackageResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).UninstallStorePackageAsync)(self.as_abi() as *const _ as *mut _, package.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn uninstall_store_package_by_store_id_async(&self, storeId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StoreUninstallStorePackageResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).UninstallStorePackageByStoreIdAsync)(self.as_abi() as *const _ as *mut _, storeId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreContext4, 2946264937, 48801, 19444, 142, 116, 174, 3, 226, 6, 198, 176);
RT_INTERFACE!{interface IStoreContext4(IStoreContext4Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreContext4] {
    fn RequestRateAndReviewAppAsync(&self, out: *mut *mut foundation::IAsyncOperation<StoreRateAndReviewResult>) -> HRESULT,
    fn SetInstallOrderForAssociatedStoreQueueItemsAsync(&self, items: *mut foundation::collections::IIterable<StoreQueueItem>, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<StoreQueueItem>>) -> HRESULT
}}
impl ComPtr<IStoreContext4> {
    #[inline] pub fn request_rate_and_review_app_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StoreRateAndReviewResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestRateAndReviewAppAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_install_order_for_associated_store_queue_items_async(&self, items: &ComPtr<foundation::collections::IIterable<StoreQueueItem>>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<StoreQueueItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SetInstallOrderForAssociatedStoreQueueItemsAsync)(self.as_abi() as *const _ as *mut _, items.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreContextStatics, 2617699935, 5568, 20082, 147, 48, 214, 25, 28, 235, 209, 156);
RT_INTERFACE!{static interface IStoreContextStatics(IStoreContextStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStoreContextStatics] {
    fn GetDefault(&self, out: *mut *mut StoreContext) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut super::super::system::User, out: *mut *mut StoreContext) -> HRESULT
}}
impl ComPtr<IStoreContextStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<StoreContext>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDefault)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(&self, user: &ComPtr<super::super::system::User>) -> Result<Option<ComPtr<StoreContext>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetForUser)(self.as_abi() as *const _ as *mut _, user.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum StoreDurationUnit: i32 {
    Minute = 0, Hour = 1, Day = 2, Week = 3, Month = 4, Year = 5,
}}
DEFINE_IID!(IID_IStoreImage, 136303176, 44468, 19300, 169, 147, 120, 71, 137, 146, 110, 213);
RT_INTERFACE!{interface IStoreImage(IStoreImageVtbl): IInspectable(IInspectableVtbl) [IID_IStoreImage] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_ImagePurposeTag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Caption(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IStoreImage> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Uri)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image_purpose_tag(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ImagePurposeTag)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Width)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Height)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_caption(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Caption)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreImage: IStoreImage}
DEFINE_IID!(IID_IStoreLicense, 651990393, 19535, 20272, 188, 137, 100, 159, 96, 227, 96, 85);
RT_INTERFACE!{interface IStoreLicense(IStoreLicenseVtbl): IInspectable(IInspectableVtbl) [IID_IStoreLicense] {
    fn get_SkuStoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsActive(&self, out: *mut bool) -> HRESULT,
    fn get_ExpirationDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InAppOfferToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IStoreLicense> {
    #[inline] pub fn get_sku_store_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SkuStoreId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_active(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsActive)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_expiration_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExpirationDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_in_app_offer_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InAppOfferToken)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreLicense: IStoreLicense}
DEFINE_IID!(IID_IStorePackageInstallOptions, 490562316, 3277, 17629, 140, 89, 128, 129, 10, 114, 153, 115);
RT_INTERFACE!{interface IStorePackageInstallOptions(IStorePackageInstallOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageInstallOptions] {
    fn get_AllowForcedAppRestart(&self, out: *mut bool) -> HRESULT,
    fn put_AllowForcedAppRestart(&self, value: bool) -> HRESULT
}}
impl ComPtr<IStorePackageInstallOptions> {
    #[inline] pub fn get_allow_forced_app_restart(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AllowForcedAppRestart)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_forced_app_restart(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_AllowForcedAppRestart)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePackageInstallOptions: IStorePackageInstallOptions}
impl RtActivatable<IActivationFactory> for StorePackageInstallOptions {}
DEFINE_CLSID!(StorePackageInstallOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,80,97,99,107,97,103,101,73,110,115,116,97,108,108,79,112,116,105,111,110,115,0]) [CLSID_StorePackageInstallOptions]);
DEFINE_IID!(IID_IStorePackageLicense, 205936404, 5345, 18803, 189, 20, 247, 119, 36, 39, 30, 153);
RT_INTERFACE!{interface IStorePackageLicense(IStorePackageLicenseVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageLicense] {
    fn add_LicenseLost(&self, handler: *mut foundation::TypedEventHandler<StorePackageLicense, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LicenseLost(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Package(&self, out: *mut *mut super::super::applicationmodel::Package) -> HRESULT,
    fn get_IsValid(&self, out: *mut bool) -> HRESULT,
    fn ReleaseLicense(&self) -> HRESULT
}}
impl ComPtr<IStorePackageLicense> {
    #[inline] pub fn add_license_lost(&self, handler: &ComPtr<foundation::TypedEventHandler<StorePackageLicense, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_LicenseLost)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_license_lost(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_LicenseLost)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_package(&self) -> Result<Option<ComPtr<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Package)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_valid(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsValid)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn release_license(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ReleaseLicense)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePackageLicense: IStorePackageLicense}
DEFINE_IID!(IID_IStorePackageUpdate, 336568656, 15551, 18997, 185, 31, 72, 39, 28, 49, 176, 114);
RT_INTERFACE!{interface IStorePackageUpdate(IStorePackageUpdateVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageUpdate] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Package(&self, out: *mut *mut super::super::applicationmodel::Package) -> HRESULT,
    fn get_Mandatory(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IStorePackageUpdate> {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_package(&self) -> Result<Option<ComPtr<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Package)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mandatory(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Mandatory)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePackageUpdate: IStorePackageUpdate}
DEFINE_IID!(IID_IStorePackageUpdateResult, 3885056749, 25081, 18579, 180, 254, 207, 25, 22, 3, 175, 123);
RT_INTERFACE!{interface IStorePackageUpdateResult(IStorePackageUpdateResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageUpdateResult] {
    fn get_OverallState(&self, out: *mut StorePackageUpdateState) -> HRESULT,
    fn get_StorePackageUpdateStatuses(&self, out: *mut *mut foundation::collections::IVectorView<StorePackageUpdateStatus>) -> HRESULT
}}
impl ComPtr<IStorePackageUpdateResult> {
    #[inline] pub fn get_overall_state(&self) -> Result<StorePackageUpdateState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_OverallState)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_store_package_update_statuses(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StorePackageUpdateStatus>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StorePackageUpdateStatuses)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePackageUpdateResult: IStorePackageUpdateResult}
DEFINE_IID!(IID_IStorePackageUpdateResult2, 119341358, 48226, 20270, 135, 234, 153, 216, 1, 174, 175, 152);
RT_INTERFACE!{interface IStorePackageUpdateResult2(IStorePackageUpdateResult2Vtbl): IInspectable(IInspectableVtbl) [IID_IStorePackageUpdateResult2] {
    fn get_StoreQueueItems(&self, out: *mut *mut foundation::collections::IVectorView<StoreQueueItem>) -> HRESULT
}}
impl ComPtr<IStorePackageUpdateResult2> {
    #[inline] pub fn get_store_queue_items(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreQueueItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StoreQueueItems)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum StorePackageUpdateState: i32 {
    Pending = 0, Downloading = 1, Deploying = 2, Completed = 3, Canceled = 4, OtherError = 5, ErrorLowBattery = 6, ErrorWiFiRecommended = 7, ErrorWiFiRequired = 8,
}}
RT_STRUCT! { struct StorePackageUpdateStatus {
    PackageFamilyName: HSTRING, PackageDownloadSizeInBytes: u64, PackageBytesDownloaded: u64, PackageDownloadProgress: f64, TotalDownloadProgress: f64, PackageUpdateState: StorePackageUpdateState,
}}
DEFINE_IID!(IID_IStorePrice, 1438291140, 5617, 16508, 143, 6, 0, 99, 128, 244, 223, 11);
RT_INTERFACE!{interface IStorePrice(IStorePriceVtbl): IInspectable(IInspectableVtbl) [IID_IStorePrice] {
    fn get_FormattedBasePrice(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FormattedPrice(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsOnSale(&self, out: *mut bool) -> HRESULT,
    fn get_SaleEndDate(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_CurrencyCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FormattedRecurrencePrice(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IStorePrice> {
    #[inline] pub fn get_formatted_base_price(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_FormattedBasePrice)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_formatted_price(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_FormattedPrice)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_on_sale(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsOnSale)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_sale_end_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SaleEndDate)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_currency_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CurrencyCode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_formatted_recurrence_price(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_FormattedRecurrencePrice)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePrice: IStorePrice}
DEFINE_IID!(IID_IStoreProduct, 839789650, 55136, 17674, 164, 43, 103, 209, 233, 1, 172, 144);
RT_INTERFACE!{interface IStoreProduct(IStoreProductVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProduct] {
    fn get_StoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProductKind(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasDigitalDownload(&self, out: *mut bool) -> HRESULT,
    fn get_Keywords(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Images(&self, out: *mut *mut foundation::collections::IVectorView<StoreImage>) -> HRESULT,
    fn get_Videos(&self, out: *mut *mut foundation::collections::IVectorView<StoreVideo>) -> HRESULT,
    fn get_Skus(&self, out: *mut *mut foundation::collections::IVectorView<StoreSku>) -> HRESULT,
    fn get_IsInUserCollection(&self, out: *mut bool) -> HRESULT,
    fn get_Price(&self, out: *mut *mut StorePrice) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LinkUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn GetIsAnySkuInstalledAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestPurchaseAsync(&self, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn get_InAppOfferToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IStoreProduct> {
    #[inline] pub fn get_store_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StoreId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Language)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Title)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_product_kind(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ProductKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_digital_download(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HasDigitalDownload)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_keywords(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Keywords)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_images(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreImage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Images)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_videos(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreVideo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Videos)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_skus(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreSku>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Skus)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_in_user_collection(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsInUserCollection)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_price(&self) -> Result<Option<ComPtr<StorePrice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Price)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_link_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LinkUri)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_any_sku_installed_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetIsAnySkuInstalledAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_with_purchase_properties_async(&self, storePurchaseProperties: &ComPtr<StorePurchaseProperties>) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self.as_abi() as *const _ as *mut _, storePurchaseProperties.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_in_app_offer_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InAppOfferToken)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreProduct: IStoreProduct}
DEFINE_IID!(IID_IStoreProductOptions, 1530175737, 41235, 18449, 131, 38, 22, 25, 156, 146, 127, 49);
RT_INTERFACE!{interface IStoreProductOptions(IStoreProductOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductOptions] {
    fn get_ActionFilters(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT
}}
impl ComPtr<IStoreProductOptions> {
    #[inline] pub fn get_action_filters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ActionFilters)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreProductOptions: IStoreProductOptions}
impl RtActivatable<IActivationFactory> for StoreProductOptions {}
DEFINE_CLSID!(StoreProductOptions(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,80,114,111,100,117,99,116,79,112,116,105,111,110,115,0]) [CLSID_StoreProductOptions]);
DEFINE_IID!(IID_IStoreProductPagedQueryResult, 3374782661, 19925, 18537, 164, 98, 236, 198, 135, 46, 67, 197);
RT_INTERFACE!{interface IStoreProductPagedQueryResult(IStoreProductPagedQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductPagedQueryResult] {
    fn get_Products(&self, out: *mut *mut foundation::collections::IMapView<HString, StoreProduct>) -> HRESULT,
    fn get_HasMoreResults(&self, out: *mut bool) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn GetNextAsync(&self, out: *mut *mut foundation::IAsyncOperation<StoreProductPagedQueryResult>) -> HRESULT
}}
impl ComPtr<IStoreProductPagedQueryResult> {
    #[inline] pub fn get_products(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, StoreProduct>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Products)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_more_results(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HasMoreResults)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_next_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StoreProductPagedQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNextAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreProductPagedQueryResult: IStoreProductPagedQueryResult}
DEFINE_IID!(IID_IStoreProductQueryResult, 3624265413, 54358, 20470, 128, 73, 144, 118, 213, 22, 95, 115);
RT_INTERFACE!{interface IStoreProductQueryResult(IStoreProductQueryResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductQueryResult] {
    fn get_Products(&self, out: *mut *mut foundation::collections::IMapView<HString, StoreProduct>) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStoreProductQueryResult> {
    #[inline] pub fn get_products(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, StoreProduct>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Products)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreProductQueryResult: IStoreProductQueryResult}
DEFINE_IID!(IID_IStoreProductResult, 3077001075, 15495, 20193, 130, 1, 244, 40, 53, 155, 211, 175);
RT_INTERFACE!{interface IStoreProductResult(IStoreProductResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreProductResult] {
    fn get_Product(&self, out: *mut *mut StoreProduct) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStoreProductResult> {
    #[inline] pub fn get_product(&self) -> Result<Option<ComPtr<StoreProduct>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Product)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreProductResult: IStoreProductResult}
DEFINE_IID!(IID_IStorePurchaseProperties, 2204268787, 65415, 17252, 165, 180, 253, 33, 83, 235, 228, 59);
RT_INTERFACE!{interface IStorePurchaseProperties(IStorePurchasePropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IStorePurchaseProperties] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ExtendedJsonData(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IStorePurchaseProperties> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Name)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_extended_json_data(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePurchaseProperties: IStorePurchaseProperties}
impl RtActivatable<IStorePurchasePropertiesFactory> for StorePurchaseProperties {}
impl RtActivatable<IActivationFactory> for StorePurchaseProperties {}
impl StorePurchaseProperties {
    #[inline] pub fn create(name: &HStringArg) -> Result<ComPtr<StorePurchaseProperties>> {
        <Self as RtActivatable<IStorePurchasePropertiesFactory>>::get_activation_factory().create(name)
    }
}
DEFINE_CLSID!(StorePurchaseProperties(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,80,117,114,99,104,97,115,101,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_StorePurchaseProperties]);
DEFINE_IID!(IID_IStorePurchasePropertiesFactory, 2808673694, 65277, 18591, 154, 23, 34, 165, 147, 230, 139, 157);
RT_INTERFACE!{static interface IStorePurchasePropertiesFactory(IStorePurchasePropertiesFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IStorePurchasePropertiesFactory] {
    fn Create(&self, name: HSTRING, out: *mut *mut StorePurchaseProperties) -> HRESULT
}}
impl ComPtr<IStorePurchasePropertiesFactory> {
    #[inline] pub fn create(&self, name: &HStringArg) -> Result<ComPtr<StorePurchaseProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorePurchaseResult, 2916255058, 63850, 17981, 167, 187, 194, 11, 79, 202, 105, 82);
RT_INTERFACE!{interface IStorePurchaseResult(IStorePurchaseResultVtbl): IInspectable(IInspectableVtbl) [IID_IStorePurchaseResult] {
    fn get_Status(&self, out: *mut StorePurchaseStatus) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStorePurchaseResult> {
    #[inline] pub fn get_status(&self) -> Result<StorePurchaseStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StorePurchaseResult: IStorePurchaseResult}
RT_ENUM! { enum StorePurchaseStatus: i32 {
    Succeeded = 0, AlreadyPurchased = 1, NotPurchased = 2, NetworkError = 3, ServerError = 4,
}}
DEFINE_IID!(IID_IStoreQueueItem, 1456849707, 63536, 17043, 145, 136, 202, 210, 220, 222, 115, 87);
RT_INTERFACE!{interface IStoreQueueItem(IStoreQueueItemVtbl): IInspectable(IInspectableVtbl) [IID_IStoreQueueItem] {
    fn get_ProductId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InstallKind(&self, out: *mut StoreQueueItemKind) -> HRESULT,
    fn GetCurrentStatus(&self, out: *mut *mut StoreQueueItemStatus) -> HRESULT,
    fn add_Completed(&self, handler: *mut foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Completed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_StatusChanged(&self, handler: *mut foundation::TypedEventHandler<StoreQueueItem, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StatusChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<IStoreQueueItem> {
    #[inline] pub fn get_product_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ProductId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_package_family_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PackageFamilyName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_install_kind(&self) -> Result<StoreQueueItemKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_InstallKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_status(&self) -> Result<Option<ComPtr<StoreQueueItemStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCurrentStatus)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_completed(&self, handler: &ComPtr<foundation::TypedEventHandler<StoreQueueItem, StoreQueueItemCompletedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_Completed)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_Completed)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_status_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<StoreQueueItem, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_StatusChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_status_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_StatusChanged)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreQueueItem: IStoreQueueItem}
DEFINE_IID!(IID_IStoreQueueItem2, 1766399144, 6868, 17532, 173, 140, 169, 80, 53, 246, 77, 130);
RT_INTERFACE!{interface IStoreQueueItem2(IStoreQueueItem2Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreQueueItem2] {
    fn CancelInstallAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn PauseInstallAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ResumeInstallAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<IStoreQueueItem2> {
    #[inline] pub fn cancel_install_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CancelInstallAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pause_install_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).PauseInstallAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn resume_install_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ResumeInstallAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreQueueItemCompletedEventArgs, 306700140, 46154, 17307, 187, 7, 29, 48, 3, 208, 5, 194);
RT_INTERFACE!{interface IStoreQueueItemCompletedEventArgs(IStoreQueueItemCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IStoreQueueItemCompletedEventArgs] {
    fn get_Status(&self, out: *mut *mut StoreQueueItemStatus) -> HRESULT
}}
impl ComPtr<IStoreQueueItemCompletedEventArgs> {
    #[inline] pub fn get_status(&self) -> Result<Option<ComPtr<StoreQueueItemStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreQueueItemCompletedEventArgs: IStoreQueueItemCompletedEventArgs}
RT_ENUM! { enum StoreQueueItemExtendedState: i32 {
    ActivePending = 0, ActiveStarting = 1, ActiveAcquiringLicense = 2, ActiveDownloading = 3, ActiveRestoringData = 4, ActiveInstalling = 5, Completed = 6, Canceled = 7, Paused = 8, Error = 9, PausedPackagesInUse = 10, PausedLowBattery = 11, PausedWiFiRecommended = 12, PausedWiFiRequired = 13, PausedReadyToInstall = 14,
}}
RT_ENUM! { enum StoreQueueItemKind: i32 {
    Install = 0, Update = 1, Repair = 2,
}}
RT_ENUM! { enum StoreQueueItemState: i32 {
    Active = 0, Completed = 1, Canceled = 2, Error = 3, Paused = 4,
}}
DEFINE_IID!(IID_IStoreQueueItemStatus, 2614524271, 40131, 20163, 178, 239, 123, 228, 51, 179, 1, 116);
RT_INTERFACE!{interface IStoreQueueItemStatus(IStoreQueueItemStatusVtbl): IInspectable(IInspectableVtbl) [IID_IStoreQueueItemStatus] {
    fn get_PackageInstallState(&self, out: *mut StoreQueueItemState) -> HRESULT,
    fn get_PackageInstallExtendedState(&self, out: *mut StoreQueueItemExtendedState) -> HRESULT,
    fn get_UpdateStatus(&self, out: *mut StorePackageUpdateStatus) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStoreQueueItemStatus> {
    #[inline] pub fn get_package_install_state(&self) -> Result<StoreQueueItemState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_PackageInstallState)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_package_install_extended_state(&self) -> Result<StoreQueueItemExtendedState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_PackageInstallExtendedState)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_update_status(&self) -> Result<StorePackageUpdateStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_UpdateStatus)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreQueueItemStatus: IStoreQueueItemStatus}
DEFINE_IID!(IID_IStoreRateAndReviewResult, 2636160342, 42677, 16673, 155, 97, 238, 109, 15, 189, 189, 187);
RT_INTERFACE!{interface IStoreRateAndReviewResult(IStoreRateAndReviewResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreRateAndReviewResult] {
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_WasUpdated(&self, out: *mut bool) -> HRESULT,
    fn get_Status(&self, out: *mut StoreRateAndReviewStatus) -> HRESULT
}}
impl ComPtr<IStoreRateAndReviewResult> {
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_was_updated(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_WasUpdated)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<StoreRateAndReviewStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreRateAndReviewResult: IStoreRateAndReviewResult}
RT_ENUM! { enum StoreRateAndReviewStatus: i32 {
    Succeeded = 0, CanceledByUser = 1, NetworkError = 2, Error = 3,
}}
RT_CLASS!{static class StoreRequestHelper}
impl RtActivatable<IStoreRequestHelperStatics> for StoreRequestHelper {}
impl StoreRequestHelper {
    #[inline] pub fn send_request_async(context: &ComPtr<StoreContext>, requestKind: u32, parametersAsJson: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StoreSendRequestResult>>> {
        <Self as RtActivatable<IStoreRequestHelperStatics>>::get_activation_factory().send_request_async(context, requestKind, parametersAsJson)
    }
}
DEFINE_CLSID!(StoreRequestHelper(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,83,116,111,114,101,46,83,116,111,114,101,82,101,113,117,101,115,116,72,101,108,112,101,114,0]) [CLSID_StoreRequestHelper]);
DEFINE_IID!(IID_IStoreRequestHelperStatics, 1827005945, 41161, 19244, 150, 166, 161, 113, 198, 48, 3, 141);
RT_INTERFACE!{static interface IStoreRequestHelperStatics(IStoreRequestHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStoreRequestHelperStatics] {
    fn SendRequestAsync(&self, context: *mut StoreContext, requestKind: u32, parametersAsJson: HSTRING, out: *mut *mut foundation::IAsyncOperation<StoreSendRequestResult>) -> HRESULT
}}
impl ComPtr<IStoreRequestHelperStatics> {
    #[inline] pub fn send_request_async(&self, context: &ComPtr<StoreContext>, requestKind: u32, parametersAsJson: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<StoreSendRequestResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SendRequestAsync)(self.as_abi() as *const _ as *mut _, context.as_abi() as *const _ as *mut _, requestKind, parametersAsJson.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreSendRequestResult, 3342515808, 33394, 17666, 138, 105, 110, 117, 21, 58, 66, 153);
RT_INTERFACE!{interface IStoreSendRequestResult(IStoreSendRequestResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreSendRequestResult] {
    fn get_Response(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ComPtr<IStoreSendRequestResult> {
    #[inline] pub fn get_response(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Response)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreSendRequestResult: IStoreSendRequestResult}
DEFINE_IID!(IID_IStoreSendRequestResult2, 687941999, 49328, 18896, 142, 141, 170, 148, 10, 249, 193, 11);
RT_INTERFACE!{interface IStoreSendRequestResult2(IStoreSendRequestResult2Vtbl): IInspectable(IInspectableVtbl) [IID_IStoreSendRequestResult2] {
    #[cfg(feature="windows-web")] fn get_HttpStatusCode(&self, out: *mut super::super::web::http::HttpStatusCode) -> HRESULT
}}
impl ComPtr<IStoreSendRequestResult2> {
    #[cfg(feature="windows-web")] #[inline] pub fn get_http_status_code(&self) -> Result<super::super::web::http::HttpStatusCode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HttpStatusCode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStoreSku, 964587349, 17472, 20227, 134, 60, 145, 243, 254, 200, 61, 121);
RT_INTERFACE!{interface IStoreSku(IStoreSkuVtbl): IInspectable(IInspectableVtbl) [IID_IStoreSku] {
    fn get_StoreId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsTrial(&self, out: *mut bool) -> HRESULT,
    fn get_CustomDeveloperData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Images(&self, out: *mut *mut foundation::collections::IVectorView<StoreImage>) -> HRESULT,
    fn get_Videos(&self, out: *mut *mut foundation::collections::IVectorView<StoreVideo>) -> HRESULT,
    fn get_Availabilities(&self, out: *mut *mut foundation::collections::IVectorView<StoreAvailability>) -> HRESULT,
    fn get_Price(&self, out: *mut *mut StorePrice) -> HRESULT,
    fn get_ExtendedJsonData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsInUserCollection(&self, out: *mut bool) -> HRESULT,
    fn get_BundledSkus(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_CollectionData(&self, out: *mut *mut StoreCollectionData) -> HRESULT,
    fn GetIsInstalledAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestPurchaseAsync(&self, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn RequestPurchaseWithPurchasePropertiesAsync(&self, storePurchaseProperties: *mut StorePurchaseProperties, out: *mut *mut foundation::IAsyncOperation<StorePurchaseResult>) -> HRESULT,
    fn get_IsSubscription(&self, out: *mut bool) -> HRESULT,
    fn get_SubscriptionInfo(&self, out: *mut *mut StoreSubscriptionInfo) -> HRESULT
}}
impl ComPtr<IStoreSku> {
    #[inline] pub fn get_store_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_StoreId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Language)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Title)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_trial(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTrial)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_developer_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CustomDeveloperData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_images(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreImage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Images)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_videos(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreVideo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Videos)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_availabilities(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<StoreAvailability>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Availabilities)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_price(&self) -> Result<Option<ComPtr<StorePrice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Price)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_json_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedJsonData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_in_user_collection(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsInUserCollection)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bundled_skus(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_BundledSkus)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_collection_data(&self) -> Result<Option<ComPtr<StoreCollectionData>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CollectionData)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_installed_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetIsInstalledAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_purchase_with_purchase_properties_async(&self, storePurchaseProperties: &ComPtr<StorePurchaseProperties>) -> Result<ComPtr<foundation::IAsyncOperation<StorePurchaseResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestPurchaseWithPurchasePropertiesAsync)(self.as_abi() as *const _ as *mut _, storePurchaseProperties.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_subscription(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsSubscription)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_subscription_info(&self) -> Result<Option<ComPtr<StoreSubscriptionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SubscriptionInfo)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreSku: IStoreSku}
DEFINE_IID!(IID_IStoreSubscriptionInfo, 1099528042, 1369, 17324, 169, 198, 58, 176, 1, 31, 184, 235);
RT_INTERFACE!{interface IStoreSubscriptionInfo(IStoreSubscriptionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IStoreSubscriptionInfo] {
    fn get_BillingPeriod(&self, out: *mut u32) -> HRESULT,
    fn get_BillingPeriodUnit(&self, out: *mut StoreDurationUnit) -> HRESULT,
    fn get_HasTrialPeriod(&self, out: *mut bool) -> HRESULT,
    fn get_TrialPeriod(&self, out: *mut u32) -> HRESULT,
    fn get_TrialPeriodUnit(&self, out: *mut StoreDurationUnit) -> HRESULT
}}
impl ComPtr<IStoreSubscriptionInfo> {
    #[inline] pub fn get_billing_period(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BillingPeriod)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_billing_period_unit(&self) -> Result<StoreDurationUnit> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BillingPeriodUnit)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_trial_period(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HasTrialPeriod)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_trial_period(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrialPeriod)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_trial_period_unit(&self) -> Result<StoreDurationUnit> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrialPeriodUnit)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreSubscriptionInfo: IStoreSubscriptionInfo}
DEFINE_IID!(IID_IStoreUninstallStorePackageResult, 2680830461, 4719, 19674, 184, 1, 19, 70, 184, 208, 162, 96);
RT_INTERFACE!{interface IStoreUninstallStorePackageResult(IStoreUninstallStorePackageResultVtbl): IInspectable(IInspectableVtbl) [IID_IStoreUninstallStorePackageResult] {
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_Status(&self, out: *mut StoreUninstallStorePackageStatus) -> HRESULT
}}
impl ComPtr<IStoreUninstallStorePackageResult> {
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ExtendedError)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<StoreUninstallStorePackageStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreUninstallStorePackageResult: IStoreUninstallStorePackageResult}
RT_ENUM! { enum StoreUninstallStorePackageStatus: i32 {
    Succeeded = 0, CanceledByUser = 1, NetworkError = 2, UninstallNotApplicable = 3, Error = 4,
}}
DEFINE_IID!(IID_IStoreVideo, 4067209604, 28510, 19906, 136, 108, 60, 99, 8, 60, 47, 148);
RT_INTERFACE!{interface IStoreVideo(IStoreVideoVtbl): IInspectable(IInspectableVtbl) [IID_IStoreVideo] {
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_VideoPurposeTag(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Caption(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PreviewImage(&self, out: *mut *mut StoreImage) -> HRESULT
}}
impl ComPtr<IStoreVideo> {
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Uri)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_video_purpose_tag(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_VideoPurposeTag)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Width)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Height)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_caption(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Caption)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_preview_image(&self) -> Result<Option<ComPtr<StoreImage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PreviewImage)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StoreVideo: IStoreVideo}
} // Windows.Services.Store
pub mod targetedcontent { // Windows.Services.TargetedContent
use crate::prelude::*;
DEFINE_IID!(IID_ITargetedContentAction, 3613092126, 27862, 19616, 157, 143, 71, 40, 176, 183, 230, 182);
RT_INTERFACE!{interface ITargetedContentAction(ITargetedContentActionVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentAction] {
    fn InvokeAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<ITargetedContentAction> {
    #[inline] pub fn invoke_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).InvokeAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentAction: ITargetedContentAction}
RT_ENUM! { enum TargetedContentAppInstallationState: i32 {
    NotApplicable = 0, NotInstalled = 1, Installed = 2,
}}
RT_ENUM! { enum TargetedContentAvailability: i32 {
    None = 0, Partial = 1, All = 2,
}}
DEFINE_IID!(IID_ITargetedContentAvailabilityChangedEventArgs, 3774192934, 22823, 17488, 150, 92, 28, 235, 123, 236, 222, 101);
RT_INTERFACE!{interface ITargetedContentAvailabilityChangedEventArgs(ITargetedContentAvailabilityChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentAvailabilityChangedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl ComPtr<ITargetedContentAvailabilityChangedEventArgs> {
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDeferral)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentAvailabilityChangedEventArgs: ITargetedContentAvailabilityChangedEventArgs}
DEFINE_IID!(IID_ITargetedContentChangedEventArgs, 2580842697, 22654, 17798, 142, 247, 181, 76, 169, 69, 58, 22);
RT_INTERFACE!{interface ITargetedContentChangedEventArgs(ITargetedContentChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentChangedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT,
    fn get_HasPreviousContentExpired(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<ITargetedContentChangedEventArgs> {
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDeferral)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_previous_content_expired(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_HasPreviousContentExpired)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentChangedEventArgs: ITargetedContentChangedEventArgs}
DEFINE_IID!(IID_ITargetedContentCollection, 759916229, 61795, 17594, 159, 110, 225, 164, 194, 187, 85, 157);
RT_INTERFACE!{interface ITargetedContentCollection(ITargetedContentCollectionVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentCollection] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> HRESULT,
    fn ReportCustomInteraction(&self, customInteractionName: HSTRING) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut foundation::collections::IMapView<HString, TargetedContentValue>) -> HRESULT,
    fn get_Collections(&self, out: *mut *mut foundation::collections::IVectorView<TargetedContentCollection>) -> HRESULT,
    fn get_Items(&self, out: *mut *mut foundation::collections::IVectorView<TargetedContentItem>) -> HRESULT
}}
impl ComPtr<ITargetedContentCollection> {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Id)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn report_interaction(&self, interaction: TargetedContentInteraction) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ReportInteraction)(self.as_abi() as *const _ as *mut _, interaction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn report_custom_interaction(&self, customInteractionName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ReportCustomInteraction)(self.as_abi() as *const _ as *mut _, customInteractionName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, TargetedContentValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Properties)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_collections(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<TargetedContentCollection>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Collections)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_items(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<TargetedContentItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Items)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentCollection: ITargetedContentCollection}
DEFINE_IID!(IID_ITargetedContentContainer, 3156513993, 34871, 18370, 133, 15, 215, 157, 100, 89, 89, 38);
RT_INTERFACE!{interface ITargetedContentContainer(ITargetedContentContainerVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentContainer] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Timestamp(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_Availability(&self, out: *mut TargetedContentAvailability) -> HRESULT,
    fn get_Content(&self, out: *mut *mut TargetedContentCollection) -> HRESULT,
    fn SelectSingleObject(&self, path: HSTRING, out: *mut *mut TargetedContentObject) -> HRESULT
}}
impl ComPtr<ITargetedContentContainer> {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Id)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_timestamp(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Timestamp)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_availability(&self) -> Result<TargetedContentAvailability> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Availability)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_content(&self) -> Result<Option<ComPtr<TargetedContentCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Content)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_single_object(&self, path: &HStringArg) -> Result<Option<ComPtr<TargetedContentObject>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SelectSingleObject)(self.as_abi() as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentContainer: ITargetedContentContainer}
impl RtActivatable<ITargetedContentContainerStatics> for TargetedContentContainer {}
impl TargetedContentContainer {
    #[inline] pub fn get_async(contentId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<TargetedContentContainer>>> {
        <Self as RtActivatable<ITargetedContentContainerStatics>>::get_activation_factory().get_async(contentId)
    }
}
DEFINE_CLSID!(TargetedContentContainer(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,67,111,110,116,97,105,110,101,114,0]) [CLSID_TargetedContentContainer]);
DEFINE_IID!(IID_ITargetedContentContainerStatics, 1531439099, 8512, 19487, 167, 54, 197, 149, 131, 242, 39, 216);
RT_INTERFACE!{static interface ITargetedContentContainerStatics(ITargetedContentContainerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentContainerStatics] {
    fn GetAsync(&self, contentId: HSTRING, out: *mut *mut foundation::IAsyncOperation<TargetedContentContainer>) -> HRESULT
}}
impl ComPtr<ITargetedContentContainerStatics> {
    #[inline] pub fn get_async(&self, contentId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<TargetedContentContainer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsync)(self.as_abi() as *const _ as *mut _, contentId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
#[cfg(feature="windows-storage")] RT_CLASS!{class TargetedContentFile: super::super::storage::streams::IRandomAccessStreamReference}
#[cfg(not(feature="windows-storage"))] RT_CLASS!{class TargetedContentFile: IInspectable}
DEFINE_IID!(IID_ITargetedContentImage, 2812642777, 30623, 19230, 187, 177, 142, 175, 83, 251, 234, 178);
RT_INTERFACE!{interface ITargetedContentImage(ITargetedContentImageVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentImage] {
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT
}}
impl ComPtr<ITargetedContentImage> {
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Height)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Width)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentImage: ITargetedContentImage}
RT_ENUM! { enum TargetedContentInteraction: i32 {
    Impression = 0, ClickThrough = 1, Hover = 2, Like = 3, Dislike = 4, Dismiss = 5, Ineligible = 6, Accept = 7, Decline = 8, Defer = 9, Canceled = 10, Conversion = 11, Opportunity = 12,
}}
DEFINE_IID!(IID_ITargetedContentItem, 941002180, 10092, 19506, 150, 186, 86, 92, 110, 64, 110, 116);
RT_INTERFACE!{interface ITargetedContentItem(ITargetedContentItemVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentItem] {
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn ReportInteraction(&self, interaction: TargetedContentInteraction) -> HRESULT,
    fn ReportCustomInteraction(&self, customInteractionName: HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut *mut TargetedContentItemState) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut foundation::collections::IMapView<HString, TargetedContentValue>) -> HRESULT,
    fn get_Collections(&self, out: *mut *mut foundation::collections::IVectorView<TargetedContentCollection>) -> HRESULT
}}
impl ComPtr<ITargetedContentItem> {
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn report_interaction(&self, interaction: TargetedContentInteraction) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ReportInteraction)(self.as_abi() as *const _ as *mut _, interaction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn report_custom_interaction(&self, customInteractionName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ReportCustomInteraction)(self.as_abi() as *const _ as *mut _, customInteractionName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<Option<ComPtr<TargetedContentItemState>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_State)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, TargetedContentValue>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Properties)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_collections(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<TargetedContentCollection>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Collections)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentItem: ITargetedContentItem}
DEFINE_IID!(IID_ITargetedContentItemState, 1939035220, 19557, 19271, 164, 65, 71, 45, 229, 60, 121, 182);
RT_INTERFACE!{interface ITargetedContentItemState(ITargetedContentItemStateVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentItemState] {
    fn get_ShouldDisplay(&self, out: *mut bool) -> HRESULT,
    fn get_AppInstallationState(&self, out: *mut TargetedContentAppInstallationState) -> HRESULT
}}
impl ComPtr<ITargetedContentItemState> {
    #[inline] pub fn get_should_display(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ShouldDisplay)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_installation_state(&self) -> Result<TargetedContentAppInstallationState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AppInstallationState)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentItemState: ITargetedContentItemState}
DEFINE_IID!(IID_ITargetedContentObject, 69040489, 8722, 17105, 157, 250, 136, 168, 227, 3, 58, 163);
RT_INTERFACE!{interface ITargetedContentObject(ITargetedContentObjectVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentObject] {
    fn get_ObjectKind(&self, out: *mut TargetedContentObjectKind) -> HRESULT,
    fn get_Collection(&self, out: *mut *mut TargetedContentCollection) -> HRESULT,
    fn get_Item(&self, out: *mut *mut TargetedContentItem) -> HRESULT,
    fn get_Value(&self, out: *mut *mut TargetedContentValue) -> HRESULT
}}
impl ComPtr<ITargetedContentObject> {
    #[inline] pub fn get_object_kind(&self) -> Result<TargetedContentObjectKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ObjectKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_collection(&self) -> Result<Option<ComPtr<TargetedContentCollection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Collection)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item(&self) -> Result<Option<ComPtr<TargetedContentItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Item)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<TargetedContentValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Value)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentObject: ITargetedContentObject}
RT_ENUM! { enum TargetedContentObjectKind: i32 {
    Collection = 0, Item = 1, Value = 2,
}}
DEFINE_IID!(IID_ITargetedContentStateChangedEventArgs, 2585587517, 32883, 17430, 141, 242, 84, 104, 53, 166, 65, 79);
RT_INTERFACE!{interface ITargetedContentStateChangedEventArgs(ITargetedContentStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentStateChangedEventArgs] {
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl ComPtr<ITargetedContentStateChangedEventArgs> {
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDeferral)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentStateChangedEventArgs: ITargetedContentStateChangedEventArgs}
DEFINE_IID!(IID_ITargetedContentSubscription, 2284596297, 50770, 19578, 172, 173, 31, 127, 162, 152, 108, 115);
RT_INTERFACE!{interface ITargetedContentSubscription(ITargetedContentSubscriptionVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentSubscription] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn GetContentContainerAsync(&self, out: *mut *mut foundation::IAsyncOperation<TargetedContentContainer>) -> HRESULT,
    fn add_ContentChanged(&self, handler: *mut foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ContentChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AvailabilityChanged(&self, handler: *mut foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AvailabilityChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_StateChanged(&self, handler: *mut foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StateChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<ITargetedContentSubscription> {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Id)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_content_container_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<TargetedContentContainer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetContentContainerAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_content_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentChangedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_ContentChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_content_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_ContentChanged)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_availability_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentAvailabilityChangedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_AvailabilityChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_availability_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_AvailabilityChanged)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_state_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<TargetedContentSubscription, TargetedContentStateChangedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_StateChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_state_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_StateChanged)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentSubscription: ITargetedContentSubscription}
impl RtActivatable<ITargetedContentSubscriptionStatics> for TargetedContentSubscription {}
impl TargetedContentSubscription {
    #[inline] pub fn get_async(subscriptionId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<TargetedContentSubscription>>> {
        <Self as RtActivatable<ITargetedContentSubscriptionStatics>>::get_activation_factory().get_async(subscriptionId)
    }
    #[inline] pub fn get_options(subscriptionId: &HStringArg) -> Result<Option<ComPtr<TargetedContentSubscriptionOptions>>> {
        <Self as RtActivatable<ITargetedContentSubscriptionStatics>>::get_activation_factory().get_options(subscriptionId)
    }
}
DEFINE_CLSID!(TargetedContentSubscription(&[87,105,110,100,111,119,115,46,83,101,114,118,105,99,101,115,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,46,84,97,114,103,101,116,101,100,67,111,110,116,101,110,116,83,117,98,115,99,114,105,112,116,105,111,110,0]) [CLSID_TargetedContentSubscription]);
DEFINE_IID!(IID_ITargetedContentSubscriptionOptions, 1643014864, 11395, 16923, 132, 103, 65, 62, 175, 26, 235, 151);
RT_INTERFACE!{interface ITargetedContentSubscriptionOptions(ITargetedContentSubscriptionOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentSubscriptionOptions] {
    fn get_SubscriptionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AllowPartialContentAvailability(&self, out: *mut bool) -> HRESULT,
    fn put_AllowPartialContentAvailability(&self, value: bool) -> HRESULT,
    fn get_CloudQueryParameters(&self, out: *mut *mut foundation::collections::IMap<HString, HString>) -> HRESULT,
    fn get_LocalFilters(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn Update(&self) -> HRESULT
}}
impl ComPtr<ITargetedContentSubscriptionOptions> {
    #[inline] pub fn get_subscription_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SubscriptionId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_partial_content_availability(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AllowPartialContentAvailability)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_partial_content_availability(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_AllowPartialContentAvailability)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cloud_query_parameters(&self) -> Result<Option<ComPtr<foundation::collections::IMap<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CloudQueryParameters)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_filters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LocalFilters)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn update(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Update)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentSubscriptionOptions: ITargetedContentSubscriptionOptions}
DEFINE_IID!(IID_ITargetedContentSubscriptionStatics, 4208852608, 13837, 18710, 181, 60, 126, 162, 112, 144, 208, 42);
RT_INTERFACE!{static interface ITargetedContentSubscriptionStatics(ITargetedContentSubscriptionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentSubscriptionStatics] {
    fn GetAsync(&self, subscriptionId: HSTRING, out: *mut *mut foundation::IAsyncOperation<TargetedContentSubscription>) -> HRESULT,
    fn GetOptions(&self, subscriptionId: HSTRING, out: *mut *mut TargetedContentSubscriptionOptions) -> HRESULT
}}
impl ComPtr<ITargetedContentSubscriptionStatics> {
    #[inline] pub fn get_async(&self, subscriptionId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<TargetedContentSubscription>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsync)(self.as_abi() as *const _ as *mut _, subscriptionId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_options(&self, subscriptionId: &HStringArg) -> Result<Option<ComPtr<TargetedContentSubscriptionOptions>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetOptions)(self.as_abi() as *const _ as *mut _, subscriptionId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITargetedContentValue, 2868765875, 16917, 19448, 134, 127, 67, 240, 72, 101, 249, 191);
RT_INTERFACE!{interface ITargetedContentValue(ITargetedContentValueVtbl): IInspectable(IInspectableVtbl) [IID_ITargetedContentValue] {
    fn get_ValueKind(&self, out: *mut TargetedContentValueKind) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_String(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Uri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Number(&self, out: *mut f64) -> HRESULT,
    fn get_Boolean(&self, out: *mut bool) -> HRESULT,
    fn get_File(&self, out: *mut *mut TargetedContentFile) -> HRESULT,
    fn get_ImageFile(&self, out: *mut *mut TargetedContentImage) -> HRESULT,
    fn get_Action(&self, out: *mut *mut TargetedContentAction) -> HRESULT,
    fn get_Strings(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Uris(&self, out: *mut *mut foundation::collections::IVectorView<foundation::Uri>) -> HRESULT,
    fn get_Numbers(&self, out: *mut *mut foundation::collections::IVectorView<f64>) -> HRESULT,
    fn get_Booleans(&self, out: *mut *mut foundation::collections::IVectorView<bool>) -> HRESULT,
    fn get_Files(&self, out: *mut *mut foundation::collections::IVectorView<TargetedContentFile>) -> HRESULT,
    fn get_ImageFiles(&self, out: *mut *mut foundation::collections::IVectorView<TargetedContentImage>) -> HRESULT,
    fn get_Actions(&self, out: *mut *mut foundation::collections::IVectorView<TargetedContentAction>) -> HRESULT
}}
impl ComPtr<ITargetedContentValue> {
    #[inline] pub fn get_value_kind(&self) -> Result<TargetedContentValueKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ValueKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Path)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_String)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Uri)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_number(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Number)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_boolean(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Boolean)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_file(&self) -> Result<Option<ComPtr<TargetedContentFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_File)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image_file(&self) -> Result<Option<ComPtr<TargetedContentImage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ImageFile)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_action(&self) -> Result<Option<ComPtr<TargetedContentAction>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Action)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_strings(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Strings)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uris(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<foundation::Uri>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Uris)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_numbers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Numbers)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_booleans(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<bool>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Booleans)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<TargetedContentFile>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Files)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image_files(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<TargetedContentImage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ImageFiles)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_actions(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<TargetedContentAction>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Actions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetedContentValue: ITargetedContentValue}
RT_ENUM! { enum TargetedContentValueKind: i32 {
    String = 0, Uri = 1, Number = 2, Boolean = 3, File = 4, ImageFile = 5, Action = 6, Strings = 7, Uris = 8, Numbers = 9, Booleans = 10, Files = 11, ImageFiles = 12, Actions = 13,
}}
} // Windows.Services.TargetedContent
