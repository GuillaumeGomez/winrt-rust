pub mod machinelearning { // Windows.AI.MachineLearning
use crate::prelude::*;
DEFINE_IID!(IID_IImageFeatureDescriptor, 911574437, 5914, 18986, 152, 95, 38, 81, 89, 211, 137, 90);
RT_INTERFACE!{interface IImageFeatureDescriptor(IImageFeatureDescriptorVtbl): IInspectable [IID_IImageFeatureDescriptor] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapPixelFormat(&self, out: *mut super::super::graphics::imaging::BitmapPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapAlphaMode(&self, out: *mut super::super::graphics::imaging::BitmapAlphaMode) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT
}}
impl IImageFeatureDescriptor {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<super::super::graphics::imaging::BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_BitmapPixelFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_alpha_mode(&self) -> Result<super::super::graphics::imaging::BitmapAlphaMode> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_BitmapAlphaMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Width)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Height)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageFeatureDescriptor: IImageFeatureDescriptor}
DEFINE_IID!(IID_IImageFeatureValue, 4030812121, 51626, 17413, 183, 251, 148, 248, 124, 138, 48, 55);
RT_INTERFACE!{interface IImageFeatureValue(IImageFeatureValueVtbl): IInspectable [IID_IImageFeatureValue] {
    #[cfg(feature="windows-media")] fn get_VideoFrame(&self, out: *mut <super::super::media::VideoFrame as RtType>::Abi) -> HRESULT
}}
impl IImageFeatureValue {
    #[cfg(feature="windows-media")] #[inline] pub fn get_video_frame(&self) -> Result<Option<super::super::media::VideoFrame>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VideoFrame)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::media::VideoFrame::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageFeatureValue: IImageFeatureValue}
impl RtActivatable<IImageFeatureValueStatics> for ImageFeatureValue {}
impl ImageFeatureValue {
    #[cfg(feature="windows-media")] #[inline] pub fn create_from_video_frame(image: &super::super::media::VideoFrame) -> Result<Option<ImageFeatureValue>> {
        <Self as RtActivatable<IImageFeatureValueStatics>>::get_activation_factory().create_from_video_frame(image)
    }
}
DEFINE_CLSID!(ImageFeatureValue(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,73,109,97,103,101,70,101,97,116,117,114,101,86,97,108,117,101,0]) [CLSID_ImageFeatureValue]);
DEFINE_IID!(IID_IImageFeatureValueStatics, 465770493, 9163, 17936, 176, 133, 200, 225, 200, 126, 186, 160);
RT_INTERFACE!{static interface IImageFeatureValueStatics(IImageFeatureValueStaticsVtbl): IInspectable [IID_IImageFeatureValueStatics] {
    #[cfg(feature="windows-media")] fn CreateFromVideoFrame(&self, image: <super::super::media::VideoFrame as RtType>::Abi, out: *mut <ImageFeatureValue as RtType>::Abi) -> HRESULT
}}
impl IImageFeatureValueStatics {
    #[cfg(feature="windows-media")] #[inline] pub fn create_from_video_frame(&self, image: &super::super::media::VideoFrame) -> Result<Option<ImageFeatureValue>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromVideoFrame)(self.get_abi() as *const _ as *mut _, image.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ImageFeatureValue::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModel, 1536051488, 18591, 20102, 145, 40, 38, 90, 50, 123, 120, 250);
RT_INTERFACE!{interface ILearningModel(ILearningModelVtbl): IInspectable [IID_ILearningModel] {
    fn get_Author(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Domain(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Version(&self, out: *mut i64) -> HRESULT,
    fn get_Metadata(&self, out: *mut <foundation::collections::IMapView<HString, HString> as RtType>::Abi) -> HRESULT,
    fn get_InputFeatures(&self, out: *mut <foundation::collections::IVectorView<ILearningModelFeatureDescriptor> as RtType>::Abi) -> HRESULT,
    fn get_OutputFeatures(&self, out: *mut <foundation::collections::IVectorView<ILearningModelFeatureDescriptor> as RtType>::Abi) -> HRESULT
}}
impl ILearningModel {
    #[inline] pub fn get_author(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Author)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Domain)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Version)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<foundation::collections::IMapView<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Metadata)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_features(&self) -> Result<Option<foundation::collections::IVectorView<ILearningModelFeatureDescriptor>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_InputFeatures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_output_features(&self) -> Result<Option<foundation::collections::IVectorView<ILearningModelFeatureDescriptor>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_OutputFeatures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModel: ILearningModel}
impl RtActivatable<ILearningModelStatics> for LearningModel {}
impl LearningModel {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_async(modelFile: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_storage_file_async(modelFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(modelStream: &super::super::storage::streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream_async(modelStream)
    }
    #[inline] pub fn load_from_file_path(filePath: &HStringArg) -> Result<Option<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_file_path(filePath)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream(modelStream: &super::super::storage::streams::IRandomAccessStreamReference) -> Result<Option<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream(modelStream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_with_operator_provider_async(modelFile: &super::super::storage::IStorageFile, operatorProvider: &ILearningModelOperatorProvider) -> Result<foundation::IAsyncOperation<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_storage_file_with_operator_provider_async(modelFile, operatorProvider)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider_async(modelStream: &super::super::storage::streams::IRandomAccessStreamReference, operatorProvider: &ILearningModelOperatorProvider) -> Result<foundation::IAsyncOperation<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream_with_operator_provider_async(modelStream, operatorProvider)
    }
    #[inline] pub fn load_from_file_path_with_operator_provider(filePath: &HStringArg, operatorProvider: &ILearningModelOperatorProvider) -> Result<Option<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_file_path_with_operator_provider(filePath, operatorProvider)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider(modelStream: &super::super::storage::streams::IRandomAccessStreamReference, operatorProvider: &ILearningModelOperatorProvider) -> Result<Option<LearningModel>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream_with_operator_provider(modelStream, operatorProvider)
    }
}
DEFINE_CLSID!(LearningModel(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,0]) [CLSID_LearningModel]);
DEFINE_IID!(IID_ILearningModelBinding, 3929091872, 5775, 20364, 148, 254, 46, 122, 195, 27, 74, 168);
RT_INTERFACE!{interface ILearningModelBinding(ILearningModelBindingVtbl): IInspectable [IID_ILearningModelBinding] {
    fn Bind(&self, name: HSTRING, value: <IInspectable as RtType>::Abi) -> HRESULT,
    fn BindWithProperties(&self, name: HSTRING, value: <IInspectable as RtType>::Abi, props: <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ILearningModelBinding {
    #[inline] pub fn bind(&self, name: &HStringArg, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Bind)(self.get_abi() as *const _ as *mut _, name.get(), value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn bind_with_properties(&self, name: &HStringArg, value: &IInspectable, props: &foundation::collections::IPropertySet) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().BindWithProperties)(self.get_abi() as *const _ as *mut _, name.get(), value.get_abi() as *const _ as *mut _, props.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Clear)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelBinding: ILearningModelBinding}
impl RtActivatable<ILearningModelBindingFactory> for LearningModelBinding {}
impl LearningModelBinding {
    #[inline] pub fn create_from_session(session: &LearningModelSession) -> Result<LearningModelBinding> {
        <Self as RtActivatable<ILearningModelBindingFactory>>::get_activation_factory().create_from_session(session)
    }
}
DEFINE_CLSID!(LearningModelBinding(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,66,105,110,100,105,110,103,0]) [CLSID_LearningModelBinding]);
DEFINE_IID!(IID_ILearningModelBindingFactory, 3378477690, 59272, 18270, 137, 23, 35, 170, 56, 31, 175, 11);
RT_INTERFACE!{static interface ILearningModelBindingFactory(ILearningModelBindingFactoryVtbl): IInspectable [IID_ILearningModelBindingFactory] {
    fn CreateFromSession(&self, session: <LearningModelSession as RtType>::Abi, out: *mut <LearningModelBinding as RtType>::Abi) -> HRESULT
}}
impl ILearningModelBindingFactory {
    #[inline] pub fn create_from_session(&self, session: &LearningModelSession) -> Result<LearningModelBinding> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromSession)(self.get_abi() as *const _ as *mut _, session.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelBinding::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelDevice, 4123183358, 16214, 19084, 172, 95, 253, 185, 45, 139, 130, 82);
RT_INTERFACE!{interface ILearningModelDevice(ILearningModelDeviceVtbl): IInspectable [IID_ILearningModelDevice] {
    #[cfg(feature="windows-graphics")] fn get_AdapterId(&self, out: *mut super::super::graphics::DisplayAdapterId) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_Direct3D11Device(&self, out: *mut <super::super::graphics::directx::direct3d11::IDirect3DDevice as RtType>::Abi) -> HRESULT
}}
impl ILearningModelDevice {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_adapter_id(&self) -> Result<super::super::graphics::DisplayAdapterId> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AdapterId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_direct3d11_device(&self) -> Result<Option<super::super::graphics::directx::direct3d11::IDirect3DDevice>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Direct3D11Device)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::graphics::directx::direct3d11::IDirect3DDevice::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelDevice: ILearningModelDevice}
impl RtActivatable<ILearningModelDeviceFactory> for LearningModelDevice {}
impl RtActivatable<ILearningModelDeviceStatics> for LearningModelDevice {}
impl LearningModelDevice {
    #[inline] pub fn create(deviceKind: LearningModelDeviceKind) -> Result<LearningModelDevice> {
        <Self as RtActivatable<ILearningModelDeviceFactory>>::get_activation_factory().create(deviceKind)
    }
    #[cfg(feature="windows-graphics")] #[inline] pub fn create_from_direct3d11_device(device: &super::super::graphics::directx::direct3d11::IDirect3DDevice) -> Result<Option<LearningModelDevice>> {
        <Self as RtActivatable<ILearningModelDeviceStatics>>::get_activation_factory().create_from_direct3d11_device(device)
    }
}
DEFINE_CLSID!(LearningModelDevice(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,68,101,118,105,99,101,0]) [CLSID_LearningModelDevice]);
DEFINE_IID!(IID_ILearningModelDeviceFactory, 2634012493, 45541, 20256, 128, 173, 10, 86, 105, 13, 176, 107);
RT_INTERFACE!{static interface ILearningModelDeviceFactory(ILearningModelDeviceFactoryVtbl): IInspectable [IID_ILearningModelDeviceFactory] {
    fn Create(&self, deviceKind: LearningModelDeviceKind, out: *mut <LearningModelDevice as RtType>::Abi) -> HRESULT
}}
impl ILearningModelDeviceFactory {
    #[inline] pub fn create(&self, deviceKind: LearningModelDeviceKind) -> Result<LearningModelDevice> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, deviceKind, &mut out);
        if hr == S_OK { Ok(LearningModelDevice::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum LearningModelDeviceKind: i32 {
    Default = 0, Cpu = 1, DirectX = 2, DirectXHighPerformance = 3, DirectXMinPower = 4,
}}
DEFINE_IID!(IID_ILearningModelDeviceStatics, 1240670471, 43199, 17083, 146, 199, 16, 177, 45, 197, 210, 31);
RT_INTERFACE!{static interface ILearningModelDeviceStatics(ILearningModelDeviceStaticsVtbl): IInspectable [IID_ILearningModelDeviceStatics] {
    #[cfg(feature="windows-graphics")] fn CreateFromDirect3D11Device(&self, device: <super::super::graphics::directx::direct3d11::IDirect3DDevice as RtType>::Abi, out: *mut <LearningModelDevice as RtType>::Abi) -> HRESULT
}}
impl ILearningModelDeviceStatics {
    #[cfg(feature="windows-graphics")] #[inline] pub fn create_from_direct3d11_device(&self, device: &super::super::graphics::directx::direct3d11::IDirect3DDevice) -> Result<Option<LearningModelDevice>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromDirect3D11Device)(self.get_abi() as *const _ as *mut _, device.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelDevice::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelEvaluationResult, 3002712013, 38414, 18880, 133, 147, 235, 25, 10, 227, 238, 226);
RT_INTERFACE!{interface ILearningModelEvaluationResult(ILearningModelEvaluationResultVtbl): IInspectable [IID_ILearningModelEvaluationResult] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ErrorStatus(&self, out: *mut i32) -> HRESULT,
    fn get_Succeeded(&self, out: *mut bool) -> HRESULT,
    fn get_Outputs(&self, out: *mut <foundation::collections::IMapView<HString, IInspectable> as RtType>::Abi) -> HRESULT
}}
impl ILearningModelEvaluationResult {
    #[inline] pub fn get_correlation_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CorrelationId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_error_status(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ErrorStatus)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_succeeded(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Succeeded)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_outputs(&self) -> Result<Option<foundation::collections::IMapView<HString, IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Outputs)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelEvaluationResult: ILearningModelEvaluationResult}
DEFINE_IID!(IID_ILearningModelFeatureDescriptor, 3154694012, 28368, 16388, 151, 186, 185, 162, 238, 205, 43, 79);
RT_INTERFACE!{interface ILearningModelFeatureDescriptor(ILearningModelFeatureDescriptorVtbl): IInspectable [IID_ILearningModelFeatureDescriptor] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut LearningModelFeatureKind) -> HRESULT,
    fn get_IsRequired(&self, out: *mut bool) -> HRESULT
}}
impl ILearningModelFeatureDescriptor {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<LearningModelFeatureKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Kind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsRequired)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum LearningModelFeatureKind: i32 {
    Tensor = 0, Sequence = 1, Map = 2, Image = 3,
}}
DEFINE_IID!(IID_ILearningModelFeatureValue, 4111467995, 16517, 19966, 159, 237, 149, 235, 12, 12, 247, 92);
RT_INTERFACE!{interface ILearningModelFeatureValue(ILearningModelFeatureValueVtbl): IInspectable [IID_ILearningModelFeatureValue] {
    fn get_Kind(&self, out: *mut LearningModelFeatureKind) -> HRESULT
}}
impl ILearningModelFeatureValue {
    #[inline] pub fn get_kind(&self) -> Result<LearningModelFeatureKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Kind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelOperatorProvider, 706883165, 44977, 18413, 191, 173, 181, 179, 164, 89, 236, 4);
RT_INTERFACE!{interface ILearningModelOperatorProvider(ILearningModelOperatorProviderVtbl): IInspectable [IID_ILearningModelOperatorProvider] {
    
}}
DEFINE_IID!(IID_ILearningModelSession, 2388195574, 46983, 19473, 144, 240, 113, 41, 174, 202, 116, 169);
RT_INTERFACE!{interface ILearningModelSession(ILearningModelSessionVtbl): IInspectable [IID_ILearningModelSession] {
    fn get_Model(&self, out: *mut <LearningModel as RtType>::Abi) -> HRESULT,
    fn get_Device(&self, out: *mut <LearningModelDevice as RtType>::Abi) -> HRESULT,
    fn get_EvaluationProperties(&self, out: *mut <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT,
    fn EvaluateAsync(&self, bindings: <LearningModelBinding as RtType>::Abi, correlationId: HSTRING, out: *mut <foundation::IAsyncOperation<LearningModelEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn EvaluateFeaturesAsync(&self, features: <foundation::collections::IMap<HString, IInspectable> as RtType>::Abi, correlationId: HSTRING, out: *mut <foundation::IAsyncOperation<LearningModelEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn Evaluate(&self, bindings: <LearningModelBinding as RtType>::Abi, correlationId: HSTRING, out: *mut <LearningModelEvaluationResult as RtType>::Abi) -> HRESULT,
    fn EvaluateFeatures(&self, features: <foundation::collections::IMap<HString, IInspectable> as RtType>::Abi, correlationId: HSTRING, out: *mut <LearningModelEvaluationResult as RtType>::Abi) -> HRESULT
}}
impl ILearningModelSession {
    #[inline] pub fn get_model(&self) -> Result<Option<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Model)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModel::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device(&self) -> Result<Option<LearningModelDevice>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Device)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelDevice::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_evaluation_properties(&self) -> Result<Option<foundation::collections::IPropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EvaluationProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IPropertySet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_async(&self, bindings: &LearningModelBinding, correlationId: &HStringArg) -> Result<foundation::IAsyncOperation<LearningModelEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EvaluateAsync)(self.get_abi() as *const _ as *mut _, bindings.get_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features_async(&self, features: &foundation::collections::IMap<HString, IInspectable>, correlationId: &HStringArg) -> Result<foundation::IAsyncOperation<LearningModelEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EvaluateFeaturesAsync)(self.get_abi() as *const _ as *mut _, features.get_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate(&self, bindings: &LearningModelBinding, correlationId: &HStringArg) -> Result<Option<LearningModelEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Evaluate)(self.get_abi() as *const _ as *mut _, bindings.get_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(LearningModelEvaluationResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features(&self, features: &foundation::collections::IMap<HString, IInspectable>, correlationId: &HStringArg) -> Result<Option<LearningModelEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EvaluateFeatures)(self.get_abi() as *const _ as *mut _, features.get_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(LearningModelEvaluationResult::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelSession: ILearningModelSession}
impl RtActivatable<ILearningModelSessionFactory> for LearningModelSession {}
impl LearningModelSession {
    #[inline] pub fn create_from_model(model: &LearningModel) -> Result<LearningModelSession> {
        <Self as RtActivatable<ILearningModelSessionFactory>>::get_activation_factory().create_from_model(model)
    }
    #[inline] pub fn create_from_model_on_device(model: &LearningModel, deviceToRunOn: &LearningModelDevice) -> Result<LearningModelSession> {
        <Self as RtActivatable<ILearningModelSessionFactory>>::get_activation_factory().create_from_model_on_device(model, deviceToRunOn)
    }
}
DEFINE_CLSID!(LearningModelSession(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,83,101,115,115,105,111,110,0]) [CLSID_LearningModelSession]);
DEFINE_IID!(IID_ILearningModelSessionFactory, 258705437, 7323, 18358, 191, 224, 241, 207, 98, 166, 117, 121);
RT_INTERFACE!{static interface ILearningModelSessionFactory(ILearningModelSessionFactoryVtbl): IInspectable [IID_ILearningModelSessionFactory] {
    fn CreateFromModel(&self, model: <LearningModel as RtType>::Abi, out: *mut <LearningModelSession as RtType>::Abi) -> HRESULT,
    fn CreateFromModelOnDevice(&self, model: <LearningModel as RtType>::Abi, deviceToRunOn: <LearningModelDevice as RtType>::Abi, out: *mut <LearningModelSession as RtType>::Abi) -> HRESULT
}}
impl ILearningModelSessionFactory {
    #[inline] pub fn create_from_model(&self, model: &LearningModel) -> Result<LearningModelSession> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromModel)(self.get_abi() as *const _ as *mut _, model.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelSession::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_model_on_device(&self, model: &LearningModel, deviceToRunOn: &LearningModelDevice) -> Result<LearningModelSession> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromModelOnDevice)(self.get_abi() as *const _ as *mut _, model.get_abi() as *const _ as *mut _, deviceToRunOn.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelSession::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelStatics, 3820582888, 26962, 20039, 142, 244, 31, 127, 7, 137, 124, 109);
RT_INTERFACE!{static interface ILearningModelStatics(ILearningModelStaticsVtbl): IInspectable [IID_ILearningModelStatics] {
    #[cfg(feature="windows-storage")] fn LoadFromStorageFileAsync(&self, modelFile: <super::super::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<LearningModel> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamAsync(&self, modelStream: <super::super::storage::streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <foundation::IAsyncOperation<LearningModel> as RtType>::Abi) -> HRESULT,
    fn LoadFromFilePath(&self, filePath: HSTRING, out: *mut <LearningModel as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStream(&self, modelStream: <super::super::storage::streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <LearningModel as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStorageFileWithOperatorProviderAsync(&self, modelFile: <super::super::storage::IStorageFile as RtType>::Abi, operatorProvider: <ILearningModelOperatorProvider as RtType>::Abi, out: *mut <foundation::IAsyncOperation<LearningModel> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithOperatorProviderAsync(&self, modelStream: <super::super::storage::streams::IRandomAccessStreamReference as RtType>::Abi, operatorProvider: <ILearningModelOperatorProvider as RtType>::Abi, out: *mut <foundation::IAsyncOperation<LearningModel> as RtType>::Abi) -> HRESULT,
    fn LoadFromFilePathWithOperatorProvider(&self, filePath: HSTRING, operatorProvider: <ILearningModelOperatorProvider as RtType>::Abi, out: *mut <LearningModel as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithOperatorProvider(&self, modelStream: <super::super::storage::streams::IRandomAccessStreamReference as RtType>::Abi, operatorProvider: <ILearningModelOperatorProvider as RtType>::Abi, out: *mut <LearningModel as RtType>::Abi) -> HRESULT
}}
impl ILearningModelStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_async(&self, modelFile: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromStorageFileAsync)(self.get_abi() as *const _ as *mut _, modelFile.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(&self, modelStream: &super::super::storage::streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromStreamAsync)(self.get_abi() as *const _ as *mut _, modelStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_file_path(&self, filePath: &HStringArg) -> Result<Option<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromFilePath)(self.get_abi() as *const _ as *mut _, filePath.get(), &mut out);
        if hr == S_OK { Ok(LearningModel::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream(&self, modelStream: &super::super::storage::streams::IRandomAccessStreamReference) -> Result<Option<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromStream)(self.get_abi() as *const _ as *mut _, modelStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModel::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_with_operator_provider_async(&self, modelFile: &super::super::storage::IStorageFile, operatorProvider: &ILearningModelOperatorProvider) -> Result<foundation::IAsyncOperation<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromStorageFileWithOperatorProviderAsync)(self.get_abi() as *const _ as *mut _, modelFile.get_abi() as *const _ as *mut _, operatorProvider.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider_async(&self, modelStream: &super::super::storage::streams::IRandomAccessStreamReference, operatorProvider: &ILearningModelOperatorProvider) -> Result<foundation::IAsyncOperation<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromStreamWithOperatorProviderAsync)(self.get_abi() as *const _ as *mut _, modelStream.get_abi() as *const _ as *mut _, operatorProvider.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_file_path_with_operator_provider(&self, filePath: &HStringArg, operatorProvider: &ILearningModelOperatorProvider) -> Result<Option<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromFilePathWithOperatorProvider)(self.get_abi() as *const _ as *mut _, filePath.get(), operatorProvider.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModel::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider(&self, modelStream: &super::super::storage::streams::IRandomAccessStreamReference, operatorProvider: &ILearningModelOperatorProvider) -> Result<Option<LearningModel>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFromStreamWithOperatorProvider)(self.get_abi() as *const _ as *mut _, modelStream.get_abi() as *const _ as *mut _, operatorProvider.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModel::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapFeatureDescriptor, 1392780477, 41559, 17261, 158, 96, 194, 152, 31, 124, 197, 196);
RT_INTERFACE!{interface IMapFeatureDescriptor(IMapFeatureDescriptorVtbl): IInspectable [IID_IMapFeatureDescriptor] {
    fn get_KeyKind(&self, out: *mut TensorKind) -> HRESULT,
    fn get_ValueDescriptor(&self, out: *mut <ILearningModelFeatureDescriptor as RtType>::Abi) -> HRESULT
}}
impl IMapFeatureDescriptor {
    #[inline] pub fn get_key_kind(&self) -> Result<TensorKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value_descriptor(&self) -> Result<Option<ILearningModelFeatureDescriptor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ValueDescriptor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ILearningModelFeatureDescriptor::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapFeatureDescriptor: IMapFeatureDescriptor}
DEFINE_IID!(IID_ISequenceFeatureDescriptor, 2230752346, 22059, 19810, 168, 81, 115, 154, 206, 217, 102, 104);
RT_INTERFACE!{interface ISequenceFeatureDescriptor(ISequenceFeatureDescriptorVtbl): IInspectable [IID_ISequenceFeatureDescriptor] {
    fn get_ElementDescriptor(&self, out: *mut <ILearningModelFeatureDescriptor as RtType>::Abi) -> HRESULT
}}
impl ISequenceFeatureDescriptor {
    #[inline] pub fn get_element_descriptor(&self) -> Result<Option<ILearningModelFeatureDescriptor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ElementDescriptor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ILearningModelFeatureDescriptor::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SequenceFeatureDescriptor: ISequenceFeatureDescriptor}
DEFINE_IID!(IID_ITensor, 88642963, 41733, 18981, 173, 9, 68, 1, 25, 180, 183, 246);
RT_INTERFACE!{interface ITensor(ITensorVtbl): IInspectable [IID_ITensor] {
    fn get_TensorKind(&self, out: *mut TensorKind) -> HRESULT,
    fn get_Shape(&self, out: *mut <foundation::collections::IVectorView<i64> as RtType>::Abi) -> HRESULT
}}
impl ITensor {
    #[inline] pub fn get_tensor_kind(&self) -> Result<TensorKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_TensorKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<foundation::collections::IVectorView<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Shape)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorBoolean, 1358107117, 10729, 19036, 164, 77, 143, 197, 18, 88, 78, 237);
RT_INTERFACE!{interface ITensorBoolean(ITensorBooleanVtbl): IInspectable [IID_ITensorBoolean] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<bool> as RtType>::Abi) -> HRESULT
}}
impl ITensorBoolean {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorBoolean: ITensorBoolean}
impl RtActivatable<ITensorBooleanStatics> for TensorBoolean {}
impl TensorBoolean {
    #[inline] pub fn create() -> Result<Option<TensorBoolean>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorBoolean>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[bool]) -> Result<Option<TensorBoolean>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<bool>) -> Result<Option<TensorBoolean>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorBoolean(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,66,111,111,108,101,97,110,0]) [CLSID_TensorBoolean]);
DEFINE_IID!(IID_ITensorBooleanStatics, 664176172, 9047, 18855, 180, 118, 208, 170, 61, 254, 104, 102);
RT_INTERFACE!{static interface ITensorBooleanStatics(ITensorBooleanStaticsVtbl): IInspectable [IID_ITensorBooleanStatics] {
    fn Create(&self, out: *mut <TensorBoolean as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorBoolean as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut bool, out: *mut <TensorBoolean as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<bool> as RtType>::Abi, out: *mut <TensorBoolean as RtType>::Abi) -> HRESULT
}}
impl ITensorBooleanStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorBoolean>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorBoolean::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorBoolean>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorBoolean::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[bool]) -> Result<Option<TensorBoolean>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorBoolean::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<bool>) -> Result<Option<TensorBoolean>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorBoolean::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorDouble, 2447643218, 31375, 20238, 162, 143, 150, 55, 255, 200, 163, 208);
RT_INTERFACE!{interface ITensorDouble(ITensorDoubleVtbl): IInspectable [IID_ITensorDouble] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<f64> as RtType>::Abi) -> HRESULT
}}
impl ITensorDouble {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorDouble: ITensorDouble}
impl RtActivatable<ITensorDoubleStatics> for TensorDouble {}
impl TensorDouble {
    #[inline] pub fn create() -> Result<Option<TensorDouble>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorDouble>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[f64]) -> Result<Option<TensorDouble>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<f64>) -> Result<Option<TensorDouble>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorDouble(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,68,111,117,98,108,101,0]) [CLSID_TensorDouble]);
DEFINE_IID!(IID_ITensorDoubleStatics, 2825294789, 38200, 17639, 163, 202, 93, 243, 116, 165, 167, 12);
RT_INTERFACE!{static interface ITensorDoubleStatics(ITensorDoubleStaticsVtbl): IInspectable [IID_ITensorDoubleStatics] {
    fn Create(&self, out: *mut <TensorDouble as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorDouble as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut f64, out: *mut <TensorDouble as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<f64> as RtType>::Abi, out: *mut <TensorDouble as RtType>::Abi) -> HRESULT
}}
impl ITensorDoubleStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorDouble>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorDouble::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorDouble>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorDouble::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[f64]) -> Result<Option<TensorDouble>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorDouble::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<f64>) -> Result<Option<TensorDouble>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorDouble::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorFeatureDescriptor, 1950702720, 37994, 17168, 161, 156, 238, 10, 240, 40, 252, 228);
RT_INTERFACE!{interface ITensorFeatureDescriptor(ITensorFeatureDescriptorVtbl): IInspectable [IID_ITensorFeatureDescriptor] {
    fn get_TensorKind(&self, out: *mut TensorKind) -> HRESULT,
    fn get_Shape(&self, out: *mut <foundation::collections::IVectorView<i64> as RtType>::Abi) -> HRESULT
}}
impl ITensorFeatureDescriptor {
    #[inline] pub fn get_tensor_kind(&self) -> Result<TensorKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_TensorKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<foundation::collections::IVectorView<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Shape)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorFeatureDescriptor: ITensorFeatureDescriptor}
DEFINE_IID!(IID_ITensorFloat, 4062719362, 43522, 17096, 160, 200, 223, 30, 252, 150, 118, 225);
RT_INTERFACE!{interface ITensorFloat(ITensorFloatVtbl): IInspectable [IID_ITensorFloat] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<f32> as RtType>::Abi) -> HRESULT
}}
impl ITensorFloat {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<f32>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorFloat: ITensorFloat}
impl RtActivatable<ITensorFloatStatics> for TensorFloat {}
impl TensorFloat {
    #[inline] pub fn create() -> Result<Option<TensorFloat>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorFloat>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[f32]) -> Result<Option<TensorFloat>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<f32>) -> Result<Option<TensorFloat>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorFloat(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,70,108,111,97,116,0]) [CLSID_TensorFloat]);
DEFINE_IID!(IID_ITensorFloat16Bit, 179934460, 23433, 19516, 181, 228, 82, 130, 165, 49, 108, 10);
RT_INTERFACE!{interface ITensorFloat16Bit(ITensorFloat16BitVtbl): IInspectable [IID_ITensorFloat16Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<f32> as RtType>::Abi) -> HRESULT
}}
impl ITensorFloat16Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<f32>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorFloat16Bit: ITensorFloat16Bit}
impl RtActivatable<ITensorFloat16BitStatics> for TensorFloat16Bit {}
impl TensorFloat16Bit {
    #[inline] pub fn create() -> Result<Option<TensorFloat16Bit>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorFloat16Bit>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[f32]) -> Result<Option<TensorFloat16Bit>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<f32>) -> Result<Option<TensorFloat16Bit>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorFloat16Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,70,108,111,97,116,49,54,66,105,116,0]) [CLSID_TensorFloat16Bit]);
DEFINE_IID!(IID_ITensorFloat16BitStatics, 2771236597, 12682, 17620, 130, 11, 12, 220, 112, 84, 168, 74);
RT_INTERFACE!{static interface ITensorFloat16BitStatics(ITensorFloat16BitStaticsVtbl): IInspectable [IID_ITensorFloat16BitStatics] {
    fn Create(&self, out: *mut <TensorFloat16Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorFloat16Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut f32, out: *mut <TensorFloat16Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<f32> as RtType>::Abi, out: *mut <TensorFloat16Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorFloat16BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorFloat16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorFloat16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[f32]) -> Result<Option<TensorFloat16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<f32>) -> Result<Option<TensorFloat16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat16Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorFloatStatics, 3687659867, 15267, 17711, 177, 13, 60, 19, 94, 87, 63, 169);
RT_INTERFACE!{static interface ITensorFloatStatics(ITensorFloatStaticsVtbl): IInspectable [IID_ITensorFloatStatics] {
    fn Create(&self, out: *mut <TensorFloat as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorFloat as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut f32, out: *mut <TensorFloat as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<f32> as RtType>::Abi, out: *mut <TensorFloat as RtType>::Abi) -> HRESULT
}}
impl ITensorFloatStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorFloat>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorFloat>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[f32]) -> Result<Option<TensorFloat>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<f32>) -> Result<Option<TensorFloat>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorFloat::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt16Bit, 2560830777, 59094, 17583, 138, 250, 186, 235, 196, 77, 192, 32);
RT_INTERFACE!{interface ITensorInt16Bit(ITensorInt16BitVtbl): IInspectable [IID_ITensorInt16Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<i16> as RtType>::Abi) -> HRESULT
}}
impl ITensorInt16Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<i16>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt16Bit: ITensorInt16Bit}
impl RtActivatable<ITensorInt16BitStatics> for TensorInt16Bit {}
impl TensorInt16Bit {
    #[inline] pub fn create() -> Result<Option<TensorInt16Bit>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt16Bit>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[i16]) -> Result<Option<TensorInt16Bit>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<i16>) -> Result<Option<TensorInt16Bit>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt16Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,49,54,66,105,116,0]) [CLSID_TensorInt16Bit]);
DEFINE_IID!(IID_ITensorInt16BitStatics, 2556715667, 9838, 19226, 130, 31, 230, 13, 112, 137, 139, 145);
RT_INTERFACE!{static interface ITensorInt16BitStatics(ITensorInt16BitStaticsVtbl): IInspectable [IID_ITensorInt16BitStatics] {
    fn Create(&self, out: *mut <TensorInt16Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorInt16Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut i16, out: *mut <TensorInt16Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<i16> as RtType>::Abi, out: *mut <TensorInt16Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorInt16BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[i16]) -> Result<Option<TensorInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<i16>) -> Result<Option<TensorInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt16Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt32Bit, 738994387, 8316, 17542, 167, 210, 136, 69, 34, 197, 229, 137);
RT_INTERFACE!{interface ITensorInt32Bit(ITensorInt32BitVtbl): IInspectable [IID_ITensorInt32Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<i32> as RtType>::Abi) -> HRESULT
}}
impl ITensorInt32Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<i32>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt32Bit: ITensorInt32Bit}
impl RtActivatable<ITensorInt32BitStatics> for TensorInt32Bit {}
impl TensorInt32Bit {
    #[inline] pub fn create() -> Result<Option<TensorInt32Bit>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt32Bit>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[i32]) -> Result<Option<TensorInt32Bit>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<i32>) -> Result<Option<TensorInt32Bit>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt32Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,51,50,66,105,116,0]) [CLSID_TensorInt32Bit]);
DEFINE_IID!(IID_ITensorInt32BitStatics, 1698268747, 21242, 20021, 144, 124, 131, 76, 172, 65, 123, 80);
RT_INTERFACE!{static interface ITensorInt32BitStatics(ITensorInt32BitStaticsVtbl): IInspectable [IID_ITensorInt32BitStatics] {
    fn Create(&self, out: *mut <TensorInt32Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorInt32Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut i32, out: *mut <TensorInt32Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<i32> as RtType>::Abi, out: *mut <TensorInt32Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorInt32BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt32Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt32Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[i32]) -> Result<Option<TensorInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt32Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<i32>) -> Result<Option<TensorInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt32Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt64Bit, 1234593210, 8098, 17837, 175, 37, 160, 189, 155, 218, 76, 135);
RT_INTERFACE!{interface ITensorInt64Bit(ITensorInt64BitVtbl): IInspectable [IID_ITensorInt64Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<i64> as RtType>::Abi) -> HRESULT
}}
impl ITensorInt64Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt64Bit: ITensorInt64Bit}
impl RtActivatable<ITensorInt64BitStatics> for TensorInt64Bit {}
impl TensorInt64Bit {
    #[inline] pub fn create() -> Result<Option<TensorInt64Bit>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt64Bit>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[i64]) -> Result<Option<TensorInt64Bit>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt64Bit>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt64Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,54,52,66,105,116,0]) [CLSID_TensorInt64Bit]);
DEFINE_IID!(IID_ITensorInt64BitStatics, 2521345437, 4504, 19828, 149, 23, 120, 58, 182, 43, 156, 194);
RT_INTERFACE!{static interface ITensorInt64BitStatics(ITensorInt64BitStaticsVtbl): IInspectable [IID_ITensorInt64BitStatics] {
    fn Create(&self, out: *mut <TensorInt64Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorInt64Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut i64, out: *mut <TensorInt64Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorInt64Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorInt64BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt64Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt64Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[i64]) -> Result<Option<TensorInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt64Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt64Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt8Bit, 3453851589, 65496, 20463, 174, 251, 48, 225, 164, 133, 178, 238);
RT_INTERFACE!{interface ITensorInt8Bit(ITensorInt8BitVtbl): IInspectable [IID_ITensorInt8Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<u8> as RtType>::Abi) -> HRESULT
}}
impl ITensorInt8Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<u8>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt8Bit: ITensorInt8Bit}
impl RtActivatable<ITensorInt8BitStatics> for TensorInt8Bit {}
impl TensorInt8Bit {
    #[inline] pub fn create() -> Result<Option<TensorInt8Bit>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt8Bit>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[u8]) -> Result<Option<TensorInt8Bit>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u8>) -> Result<Option<TensorInt8Bit>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt8Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,56,66,105,116,0]) [CLSID_TensorInt8Bit]);
DEFINE_IID!(IID_ITensorInt8BitStatics, 2980127364, 2396, 19574, 166, 97, 172, 76, 238, 31, 62, 139);
RT_INTERFACE!{static interface ITensorInt8BitStatics(ITensorInt8BitStaticsVtbl): IInspectable [IID_ITensorInt8BitStatics] {
    fn Create(&self, out: *mut <TensorInt8Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorInt8Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut u8, out: *mut <TensorInt8Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<u8> as RtType>::Abi, out: *mut <TensorInt8Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorInt8BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt8Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt8Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[u8]) -> Result<Option<TensorInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt8Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u8>) -> Result<Option<TensorInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorInt8Bit::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TensorKind: i32 {
    Undefined = 0, Float = 1, UInt8 = 2, Int8 = 3, UInt16 = 4, Int16 = 5, Int32 = 6, Int64 = 7, String = 8, Boolean = 9, Float16 = 10, Double = 11, UInt32 = 12, UInt64 = 13, Complex64 = 14, Complex128 = 15,
}}
DEFINE_IID!(IID_ITensorString, 1478702536, 48561, 17936, 188, 117, 53, 233, 203, 240, 9, 183);
RT_INTERFACE!{interface ITensorString(ITensorStringVtbl): IInspectable [IID_ITensorString] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl ITensorString {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorString: ITensorString}
impl RtActivatable<ITensorStringStatics> for TensorString {}
impl TensorString {
    #[inline] pub fn create() -> Result<Option<TensorString>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorString>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[&HStringArg]) -> Result<Option<TensorString>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<HString>) -> Result<Option<TensorString>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorString(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,83,116,114,105,110,103,0]) [CLSID_TensorString]);
DEFINE_IID!(IID_ITensorStringStatics, 2204250916, 53030, 20247, 162, 212, 32, 239, 141, 9, 125, 83);
RT_INTERFACE!{static interface ITensorStringStatics(ITensorStringStaticsVtbl): IInspectable [IID_ITensorStringStatics] {
    fn Create(&self, out: *mut <TensorString as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorString as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut HSTRING, out: *mut <TensorString as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <TensorString as RtType>::Abi) -> HRESULT
}}
impl ITensorStringStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[&HStringArg]) -> Result<Option<TensorString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<HString>) -> Result<Option<TensorString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt16Bit, 1746145099, 9152, 17139, 129, 246, 168, 145, 192, 17, 188, 63);
RT_INTERFACE!{interface ITensorUInt16Bit(ITensorUInt16BitVtbl): IInspectable [IID_ITensorUInt16Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<u16> as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt16Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<u16>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt16Bit: ITensorUInt16Bit}
impl RtActivatable<ITensorUInt16BitStatics> for TensorUInt16Bit {}
impl TensorUInt16Bit {
    #[inline] pub fn create() -> Result<Option<TensorUInt16Bit>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt16Bit>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[u16]) -> Result<Option<TensorUInt16Bit>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u16>) -> Result<Option<TensorUInt16Bit>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt16Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,49,54,66,105,116,0]) [CLSID_TensorUInt16Bit]);
DEFINE_IID!(IID_ITensorUInt16BitStatics, 1576486365, 650, 18458, 162, 124, 199, 230, 67, 94, 82, 221);
RT_INTERFACE!{static interface ITensorUInt16BitStatics(ITensorUInt16BitStaticsVtbl): IInspectable [IID_ITensorUInt16BitStatics] {
    fn Create(&self, out: *mut <TensorUInt16Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorUInt16Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut u16, out: *mut <TensorUInt16Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<u16> as RtType>::Abi, out: *mut <TensorUInt16Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt16BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorUInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[u16]) -> Result<Option<TensorUInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt16Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u16>) -> Result<Option<TensorUInt16Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt16Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt32Bit, 3637101311, 29969, 17827, 191, 172, 195, 143, 55, 13, 34, 55);
RT_INTERFACE!{interface ITensorUInt32Bit(ITensorUInt32BitVtbl): IInspectable [IID_ITensorUInt32Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<u32> as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt32Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt32Bit: ITensorUInt32Bit}
impl RtActivatable<ITensorUInt32BitStatics> for TensorUInt32Bit {}
impl TensorUInt32Bit {
    #[inline] pub fn create() -> Result<Option<TensorUInt32Bit>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt32Bit>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[u32]) -> Result<Option<TensorUInt32Bit>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u32>) -> Result<Option<TensorUInt32Bit>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt32Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,51,50,66,105,116,0]) [CLSID_TensorUInt32Bit]);
DEFINE_IID!(IID_ITensorUInt32BitStatics, 1098659895, 59251, 17272, 142, 127, 12, 195, 61, 190, 166, 151);
RT_INTERFACE!{static interface ITensorUInt32BitStatics(ITensorUInt32BitStaticsVtbl): IInspectable [IID_ITensorUInt32BitStatics] {
    fn Create(&self, out: *mut <TensorUInt32Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorUInt32Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut u32, out: *mut <TensorUInt32Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<u32> as RtType>::Abi, out: *mut <TensorUInt32Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt32BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorUInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt32Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt32Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[u32]) -> Result<Option<TensorUInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt32Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u32>) -> Result<Option<TensorUInt32Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt32Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt64Bit, 779157421, 1215, 18469, 131, 154, 130, 186, 239, 140, 120, 134);
RT_INTERFACE!{interface ITensorUInt64Bit(ITensorUInt64BitVtbl): IInspectable [IID_ITensorUInt64Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<u64> as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt64Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<u64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt64Bit: ITensorUInt64Bit}
impl RtActivatable<ITensorUInt64BitStatics> for TensorUInt64Bit {}
impl TensorUInt64Bit {
    #[inline] pub fn create() -> Result<Option<TensorUInt64Bit>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt64Bit>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[u64]) -> Result<Option<TensorUInt64Bit>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u64>) -> Result<Option<TensorUInt64Bit>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt64Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,54,52,66,105,116,0]) [CLSID_TensorUInt64Bit]);
DEFINE_IID!(IID_ITensorUInt64BitStatics, 2055086315, 9263, 18379, 169, 198, 246, 2, 236, 251, 254, 228);
RT_INTERFACE!{static interface ITensorUInt64BitStatics(ITensorUInt64BitStaticsVtbl): IInspectable [IID_ITensorUInt64BitStatics] {
    fn Create(&self, out: *mut <TensorUInt64Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorUInt64Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut u64, out: *mut <TensorUInt64Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<u64> as RtType>::Abi, out: *mut <TensorUInt64Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt64BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorUInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt64Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt64Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[u64]) -> Result<Option<TensorUInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt64Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u64>) -> Result<Option<TensorUInt64Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt64Bit::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt8Bit, 1491185191, 25131, 18659, 190, 34, 216, 103, 174, 209, 218, 172);
RT_INTERFACE!{interface ITensorUInt8Bit(ITensorUInt8BitVtbl): IInspectable [IID_ITensorUInt8Bit] {
    fn GetAsVectorView(&self, out: *mut <foundation::collections::IVectorView<u8> as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt8Bit {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<foundation::collections::IVectorView<u8>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAsVectorView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt8Bit: ITensorUInt8Bit}
impl RtActivatable<ITensorUInt8BitStatics> for TensorUInt8Bit {}
impl TensorUInt8Bit {
    #[inline] pub fn create() -> Result<Option<TensorUInt8Bit>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt8Bit>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &foundation::collections::IIterable<i64>, data: &[u8]) -> Result<Option<TensorUInt8Bit>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u8>) -> Result<Option<TensorUInt8Bit>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt8Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,56,66,105,116,0]) [CLSID_TensorUInt8Bit]);
DEFINE_IID!(IID_ITensorUInt8BitStatics, 100038019, 48164, 16928, 138, 65, 45, 205, 140, 94, 211, 60);
RT_INTERFACE!{static interface ITensorUInt8BitStatics(ITensorUInt8BitStaticsVtbl): IInspectable [IID_ITensorUInt8BitStatics] {
    fn Create(&self, out: *mut <TensorUInt8Bit as RtType>::Abi) -> HRESULT,
    fn Create2(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, out: *mut <TensorUInt8Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromArray(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, dataSize: u32, data: *mut u8, out: *mut <TensorUInt8Bit as RtType>::Abi) -> HRESULT,
    fn CreateFromIterable(&self, shape: <foundation::collections::IIterable<i64> as RtType>::Abi, data: <foundation::collections::IIterable<u8> as RtType>::Abi, out: *mut <TensorUInt8Bit as RtType>::Abi) -> HRESULT
}}
impl ITensorUInt8BitStatics {
    #[inline] pub fn create(&self) -> Result<Option<TensorUInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt8Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &foundation::collections::IIterable<i64>) -> Result<Option<TensorUInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create2)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt8Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &foundation::collections::IIterable<i64>, data: &[u8]) -> Result<Option<TensorUInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromArray)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt8Bit::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &foundation::collections::IIterable<i64>, data: &foundation::collections::IIterable<u8>) -> Result<Option<TensorUInt8Bit>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromIterable)(self.get_abi() as *const _ as *mut _, shape.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TensorUInt8Bit::wrap(out)) } else { err(hr) }
    }}
}
pub mod preview { // Windows.AI.MachineLearning.Preview
use crate::prelude::*;
RT_ENUM! { enum FeatureElementKindPreview: i32 {
    Undefined = 0, Float = 1, UInt8 = 2, Int8 = 3, UInt16 = 4, Int16 = 5, Int32 = 6, Int64 = 7, String = 8, Boolean = 9, Float16 = 10, Double = 11, UInt32 = 12, UInt64 = 13, Complex64 = 14, Complex128 = 15,
}}
DEFINE_IID!(IID_IImageVariableDescriptorPreview, 2061630066, 670, 19909, 162, 248, 95, 183, 99, 21, 65, 80);
RT_INTERFACE!{interface IImageVariableDescriptorPreview(IImageVariableDescriptorPreviewVtbl): IInspectable [IID_IImageVariableDescriptorPreview] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapPixelFormat(&self, out: *mut crate::windows::graphics::imaging::BitmapPixelFormat) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT
}}
impl IImageVariableDescriptorPreview {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<crate::windows::graphics::imaging::BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_BitmapPixelFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Width)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Height)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageVariableDescriptorPreview: IImageVariableDescriptorPreview}
DEFINE_IID!(IID_IInferencingOptionsPreview, 1203536389, 19766, 18345, 143, 104, 255, 203, 51, 157, 208, 252);
RT_INTERFACE!{interface IInferencingOptionsPreview(IInferencingOptionsPreviewVtbl): IInspectable [IID_IInferencingOptionsPreview] {
    fn get_PreferredDeviceKind(&self, out: *mut LearningModelDeviceKindPreview) -> HRESULT,
    fn put_PreferredDeviceKind(&self, value: LearningModelDeviceKindPreview) -> HRESULT,
    fn get_IsTracingEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsTracingEnabled(&self, value: bool) -> HRESULT,
    fn get_MaxBatchSize(&self, out: *mut i32) -> HRESULT,
    fn put_MaxBatchSize(&self, value: i32) -> HRESULT,
    fn get_MinimizeMemoryAllocation(&self, out: *mut bool) -> HRESULT,
    fn put_MinimizeMemoryAllocation(&self, value: bool) -> HRESULT,
    fn get_ReclaimMemoryAfterEvaluation(&self, out: *mut bool) -> HRESULT,
    fn put_ReclaimMemoryAfterEvaluation(&self, value: bool) -> HRESULT
}}
impl IInferencingOptionsPreview {
    #[inline] pub fn get_preferred_device_kind(&self) -> Result<LearningModelDeviceKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PreferredDeviceKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_preferred_device_kind(&self, value: LearningModelDeviceKindPreview) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_PreferredDeviceKind)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_tracing_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsTracingEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_tracing_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IsTracingEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_batch_size(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MaxBatchSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_batch_size(&self, value: i32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_MaxBatchSize)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_minimize_memory_allocation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MinimizeMemoryAllocation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_minimize_memory_allocation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_MinimizeMemoryAllocation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_reclaim_memory_after_evaluation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ReclaimMemoryAfterEvaluation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_reclaim_memory_after_evaluation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ReclaimMemoryAfterEvaluation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class InferencingOptionsPreview: IInferencingOptionsPreview}
DEFINE_IID!(IID_ILearningModelBindingPreview, 2479423976, 27768, 19279, 174, 193, 166, 187, 158, 105, 22, 36);
RT_INTERFACE!{interface ILearningModelBindingPreview(ILearningModelBindingPreviewVtbl): IInspectable [IID_ILearningModelBindingPreview] {
    fn Bind(&self, name: HSTRING, value: <IInspectable as RtType>::Abi) -> HRESULT,
    fn BindWithProperties(&self, name: HSTRING, value: <IInspectable as RtType>::Abi, metadata: <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ILearningModelBindingPreview {
    #[inline] pub fn bind(&self, name: &HStringArg, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Bind)(self.get_abi() as *const _ as *mut _, name.get(), value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn bind_with_properties(&self, name: &HStringArg, value: &IInspectable, metadata: &foundation::collections::IPropertySet) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().BindWithProperties)(self.get_abi() as *const _ as *mut _, name.get(), value.get_abi() as *const _ as *mut _, metadata.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Clear)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelBindingPreview: ILearningModelBindingPreview}
impl RtActivatable<ILearningModelBindingPreviewFactory> for LearningModelBindingPreview {}
impl LearningModelBindingPreview {
    #[inline] pub fn create_from_model(model: &LearningModelPreview) -> Result<LearningModelBindingPreview> {
        <Self as RtActivatable<ILearningModelBindingPreviewFactory>>::get_activation_factory().create_from_model(model)
    }
}
DEFINE_CLSID!(LearningModelBindingPreview(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,80,114,101,118,105,101,119,46,76,101,97,114,110,105,110,103,77,111,100,101,108,66,105,110,100,105,110,103,80,114,101,118,105,101,119,0]) [CLSID_LearningModelBindingPreview]);
DEFINE_IID!(IID_ILearningModelBindingPreviewFactory, 1220026783, 7761, 19831, 174, 80, 62, 193, 100, 173, 52, 128);
RT_INTERFACE!{static interface ILearningModelBindingPreviewFactory(ILearningModelBindingPreviewFactoryVtbl): IInspectable [IID_ILearningModelBindingPreviewFactory] {
    fn CreateFromModel(&self, model: <LearningModelPreview as RtType>::Abi, out: *mut <LearningModelBindingPreview as RtType>::Abi) -> HRESULT
}}
impl ILearningModelBindingPreviewFactory {
    #[inline] pub fn create_from_model(&self, model: &LearningModelPreview) -> Result<LearningModelBindingPreview> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromModel)(self.get_abi() as *const _ as *mut _, model.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelBindingPreview::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelDescriptionPreview, 4113304006, 34321, 16557, 142, 89, 222, 63, 215, 3, 10, 64);
RT_INTERFACE!{interface ILearningModelDescriptionPreview(ILearningModelDescriptionPreviewVtbl): IInspectable [IID_ILearningModelDescriptionPreview] {
    fn get_Author(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Domain(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Version(&self, out: *mut i64) -> HRESULT,
    fn get_Metadata(&self, out: *mut <foundation::collections::IMapView<HString, HString> as RtType>::Abi) -> HRESULT,
    fn get_InputFeatures(&self, out: *mut <foundation::collections::IIterable<ILearningModelVariableDescriptorPreview> as RtType>::Abi) -> HRESULT,
    fn get_OutputFeatures(&self, out: *mut <foundation::collections::IIterable<ILearningModelVariableDescriptorPreview> as RtType>::Abi) -> HRESULT
}}
impl ILearningModelDescriptionPreview {
    #[inline] pub fn get_author(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Author)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Domain)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Version)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<foundation::collections::IMapView<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Metadata)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_features(&self) -> Result<Option<foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_InputFeatures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_output_features(&self) -> Result<Option<foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_OutputFeatures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelDescriptionPreview: ILearningModelDescriptionPreview}
RT_ENUM! { enum LearningModelDeviceKindPreview: i32 {
    LearningDeviceAny = 0, LearningDeviceCpu = 1, LearningDeviceGpu = 2, LearningDeviceNpu = 3, LearningDeviceDsp = 4, LearningDeviceFpga = 5,
}}
DEFINE_IID!(IID_ILearningModelEvaluationResultPreview, 3743804063, 39011, 16520, 132, 152, 135, 161, 244, 104, 111, 146);
RT_INTERFACE!{interface ILearningModelEvaluationResultPreview(ILearningModelEvaluationResultPreviewVtbl): IInspectable [IID_ILearningModelEvaluationResultPreview] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Outputs(&self, out: *mut <foundation::collections::IMapView<HString, IInspectable> as RtType>::Abi) -> HRESULT
}}
impl ILearningModelEvaluationResultPreview {
    #[inline] pub fn get_correlation_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CorrelationId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_outputs(&self) -> Result<Option<foundation::collections::IMapView<HString, IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Outputs)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelEvaluationResultPreview: ILearningModelEvaluationResultPreview}
RT_ENUM! { enum LearningModelFeatureKindPreview: i32 {
    Undefined = 0, Tensor = 1, Sequence = 2, Map = 3, Image = 4,
}}
DEFINE_IID!(IID_ILearningModelPreview, 77342314, 37812, 18316, 174, 184, 112, 21, 123, 240, 255, 148);
RT_INTERFACE!{interface ILearningModelPreview(ILearningModelPreviewVtbl): IInspectable [IID_ILearningModelPreview] {
    fn EvaluateAsync(&self, binding: <LearningModelBindingPreview as RtType>::Abi, correlationId: HSTRING, out: *mut <foundation::IAsyncOperation<LearningModelEvaluationResultPreview> as RtType>::Abi) -> HRESULT,
    fn EvaluateFeaturesAsync(&self, features: <foundation::collections::IMap<HString, IInspectable> as RtType>::Abi, correlationId: HSTRING, out: *mut <foundation::IAsyncOperation<LearningModelEvaluationResultPreview> as RtType>::Abi) -> HRESULT,
    fn get_Description(&self, out: *mut <LearningModelDescriptionPreview as RtType>::Abi) -> HRESULT,
    fn get_InferencingOptions(&self, out: *mut <InferencingOptionsPreview as RtType>::Abi) -> HRESULT,
    fn put_InferencingOptions(&self, value: <InferencingOptionsPreview as RtType>::Abi) -> HRESULT
}}
impl ILearningModelPreview {
    #[inline] pub fn evaluate_async(&self, binding: &LearningModelBindingPreview, correlationId: &HStringArg) -> Result<foundation::IAsyncOperation<LearningModelEvaluationResultPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EvaluateAsync)(self.get_abi() as *const _ as *mut _, binding.get_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features_async(&self, features: &foundation::collections::IMap<HString, IInspectable>, correlationId: &HStringArg) -> Result<foundation::IAsyncOperation<LearningModelEvaluationResultPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EvaluateFeaturesAsync)(self.get_abi() as *const _ as *mut _, features.get_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<Option<LearningModelDescriptionPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(LearningModelDescriptionPreview::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inferencing_options(&self) -> Result<Option<InferencingOptionsPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_InferencingOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(InferencingOptionsPreview::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_inferencing_options(&self, value: &InferencingOptionsPreview) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_InferencingOptions)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelPreview: ILearningModelPreview}
impl RtActivatable<ILearningModelPreviewStatics> for LearningModelPreview {}
impl LearningModelPreview {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_storage_file_async(modelFile: &crate::windows::storage::IStorageFile) -> Result<foundation::IAsyncOperation<LearningModelPreview>> {
        <Self as RtActivatable<ILearningModelPreviewStatics>>::get_activation_factory().load_model_from_storage_file_async(modelFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_stream_async(modelStream: &crate::windows::storage::streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<LearningModelPreview>> {
        <Self as RtActivatable<ILearningModelPreviewStatics>>::get_activation_factory().load_model_from_stream_async(modelStream)
    }
}
DEFINE_CLSID!(LearningModelPreview(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,80,114,101,118,105,101,119,46,76,101,97,114,110,105,110,103,77,111,100,101,108,80,114,101,118,105,101,119,0]) [CLSID_LearningModelPreview]);
DEFINE_IID!(IID_ILearningModelPreviewStatics, 374061920, 33893, 18310, 139, 147, 44, 22, 168, 146, 137, 215);
RT_INTERFACE!{static interface ILearningModelPreviewStatics(ILearningModelPreviewStaticsVtbl): IInspectable [IID_ILearningModelPreviewStatics] {
    #[cfg(feature="windows-storage")] fn LoadModelFromStorageFileAsync(&self, modelFile: <crate::windows::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<LearningModelPreview> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadModelFromStreamAsync(&self, modelStream: <crate::windows::storage::streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <foundation::IAsyncOperation<LearningModelPreview> as RtType>::Abi) -> HRESULT
}}
impl ILearningModelPreviewStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_storage_file_async(&self, modelFile: &crate::windows::storage::IStorageFile) -> Result<foundation::IAsyncOperation<LearningModelPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadModelFromStorageFileAsync)(self.get_abi() as *const _ as *mut _, modelFile.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_stream_async(&self, modelStream: &crate::windows::storage::streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<LearningModelPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadModelFromStreamAsync)(self.get_abi() as *const _ as *mut _, modelStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelVariableDescriptorPreview, 2973628034, 64560, 18731, 142, 160, 237, 31, 83, 192, 176, 56);
RT_INTERFACE!{interface ILearningModelVariableDescriptorPreview(ILearningModelVariableDescriptorPreviewVtbl): IInspectable [IID_ILearningModelVariableDescriptorPreview] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ModelFeatureKind(&self, out: *mut LearningModelFeatureKindPreview) -> HRESULT,
    fn get_IsRequired(&self, out: *mut bool) -> HRESULT
}}
impl ILearningModelVariableDescriptorPreview {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_model_feature_kind(&self) -> Result<LearningModelFeatureKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ModelFeatureKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsRequired)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelVariableDescriptorPreview: ILearningModelVariableDescriptorPreview}
DEFINE_IID!(IID_IMapVariableDescriptorPreview, 1018397552, 49195, 16950, 179, 232, 107, 220, 164, 156, 49, 41);
RT_INTERFACE!{interface IMapVariableDescriptorPreview(IMapVariableDescriptorPreviewVtbl): IInspectable [IID_IMapVariableDescriptorPreview] {
    fn get_KeyKind(&self, out: *mut FeatureElementKindPreview) -> HRESULT,
    fn get_ValidStringKeys(&self, out: *mut <foundation::collections::IIterable<HString> as RtType>::Abi) -> HRESULT,
    fn get_ValidIntegerKeys(&self, out: *mut <foundation::collections::IIterable<i64> as RtType>::Abi) -> HRESULT,
    fn get_Fields(&self, out: *mut <ILearningModelVariableDescriptorPreview as RtType>::Abi) -> HRESULT
}}
impl IMapVariableDescriptorPreview {
    #[inline] pub fn get_key_kind(&self) -> Result<FeatureElementKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_string_keys(&self) -> Result<Option<foundation::collections::IIterable<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ValidStringKeys)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_integer_keys(&self) -> Result<Option<foundation::collections::IIterable<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ValidIntegerKeys)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_fields(&self) -> Result<Option<ILearningModelVariableDescriptorPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Fields)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ILearningModelVariableDescriptorPreview::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapVariableDescriptorPreview: IMapVariableDescriptorPreview}
DEFINE_IID!(IID_ISequenceVariableDescriptorPreview, 2631463570, 39090, 17712, 161, 182, 45, 237, 95, 236, 188, 38);
RT_INTERFACE!{interface ISequenceVariableDescriptorPreview(ISequenceVariableDescriptorPreviewVtbl): IInspectable [IID_ISequenceVariableDescriptorPreview] {
    fn get_ElementType(&self, out: *mut <ILearningModelVariableDescriptorPreview as RtType>::Abi) -> HRESULT
}}
impl ISequenceVariableDescriptorPreview {
    #[inline] pub fn get_element_type(&self) -> Result<Option<ILearningModelVariableDescriptorPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ElementType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ILearningModelVariableDescriptorPreview::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SequenceVariableDescriptorPreview: ISequenceVariableDescriptorPreview}
DEFINE_IID!(IID_ITensorVariableDescriptorPreview, 2819575834, 39596, 16947, 151, 132, 172, 234, 249, 37, 16, 181);
RT_INTERFACE!{interface ITensorVariableDescriptorPreview(ITensorVariableDescriptorPreviewVtbl): IInspectable [IID_ITensorVariableDescriptorPreview] {
    fn get_DataType(&self, out: *mut FeatureElementKindPreview) -> HRESULT,
    fn get_Shape(&self, out: *mut <foundation::collections::IIterable<i64> as RtType>::Abi) -> HRESULT
}}
impl ITensorVariableDescriptorPreview {
    #[inline] pub fn get_data_type(&self) -> Result<FeatureElementKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_DataType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<foundation::collections::IIterable<i64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Shape)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorVariableDescriptorPreview: ITensorVariableDescriptorPreview}
} // Windows.AI.MachineLearning.Preview
} // Windows.AI.MachineLearning
