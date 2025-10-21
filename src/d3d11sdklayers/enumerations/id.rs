#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_Message_ID {
    #[allow(missing_docs)]
    Unknown = 0,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersHazard,

    #[allow(missing_docs)]
    DeviceIaSetIndexBufferHazard,

    #[allow(missing_docs)]
    DeviceVsSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceVsSetConstantBuffersHazard,

    #[allow(missing_docs)]
    DeviceGsSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceGsSetConstantBuffersHazard,

    #[allow(missing_docs)]
    DevicePsSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DevicePsSetConstantBuffersHazard,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsHazard,

    #[allow(missing_docs)]
    DeviceSoSetTargetsHazard,

    #[allow(missing_docs)]
    StringFromApplication,

    #[allow(missing_docs)]
    CorruptedThis,

    #[allow(missing_docs)]
    CorruptedParameter1,

    #[allow(missing_docs)]
    CorruptedParameter2,

    #[allow(missing_docs)]
    CorruptedParameter3,

    #[allow(missing_docs)]
    CorruptedParameter4,

    #[allow(missing_docs)]
    CorruptedParameter5,

    #[allow(missing_docs)]
    CorruptedParameter6,

    #[allow(missing_docs)]
    CorruptedParameter7,

    #[allow(missing_docs)]
    CorruptedParameter8,

    #[allow(missing_docs)]
    CorruptedParameter9,

    #[allow(missing_docs)]
    CorruptedParameter10,

    #[allow(missing_docs)]
    CorruptedParameter11,

    #[allow(missing_docs)]
    CorruptedParameter12,

    #[allow(missing_docs)]
    CorruptedParameter13,

    #[allow(missing_docs)]
    CorruptedParameter14,

    #[allow(missing_docs)]
    CorruptedParameter15,

    #[allow(missing_docs)]
    CorruptedMultiThreading,

    #[allow(missing_docs)]
    MessageReportingOutOfMemory,

    #[allow(missing_docs)]
    IaSetInputLayoutUnbindDeletingObject,

    #[allow(missing_docs)]
    IaSetVertexBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    IaSetIndexBufferUnbindDeletingObject,

    #[allow(missing_docs)]
    VsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    VsSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    VsSetConstantBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    VsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    GsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    GsSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    GsSetConstantBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    GsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    SoSetTargetsUnbindDeletingObject,

    #[allow(missing_docs)]
    PsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    PsSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    PsSetConstantBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    PsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    RsSetStateUnbindDeletingObject,

    #[allow(missing_docs)]
    OmSetBlendStateUnbindDeletingObject,

    #[allow(missing_docs)]
    OmSetDepthstencilStateUnbindDeletingObject,

    #[allow(missing_docs)]
    OmSetRenderTargetsUnbindDeletingObject,

    #[allow(missing_docs)]
    SetPredicationUnbindDeletingObject,

    #[allow(missing_docs)]
    GetPrivateDataMoreData,

    #[allow(missing_docs)]
    SetPrivateDataInvalidFreeData,

    #[allow(missing_docs)]
    SetPrivateDataInvalidIUnknown,

    #[allow(missing_docs)]
    SetPrivateDataInvalidFlags,

    #[allow(missing_docs)]
    SetPrivateDataChangingParams,

    #[allow(missing_docs)]
    SetPrivateDataOutOfMemory,

    #[allow(missing_docs)]
    CreateBufferUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateBufferInvalidSamples,

    #[allow(missing_docs)]
    CreateBufferUnrecognizedUsage,

    #[allow(missing_docs)]
    CreateBufferUnrecognizedBindFlags,

    #[allow(missing_docs)]
    CreateBufferUnrecognizedCpuAccessFlags,

    #[allow(missing_docs)]
    CreateBufferUnrecognizedMiscFlags,

    #[allow(missing_docs)]
    CreateBufferInvalidCpuAccessFlags,

    #[allow(missing_docs)]
    CreateBufferInvalidBindFlags,

    #[allow(missing_docs)]
    CreateBufferInvalidInitialData,

    #[allow(missing_docs)]
    CreateBufferInvalidDimensions,

    #[allow(missing_docs)]
    CreateBufferInvalidMipLevels,

    #[allow(missing_docs)]
    CreateBufferInvalidMiscFlags,

    #[allow(missing_docs)]
    CreateBufferInvalidArgReturn,

    #[allow(missing_docs)]
    CreateBufferOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateBufferNullDesc,

    #[allow(missing_docs)]
    CreateBufferInvalidConstantBufferBindings,

    #[allow(missing_docs)]
    CreateBufferLargeAllocation,

    #[allow(missing_docs)]
    CreateTexture1DUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateTexture1DUnsupportedFormat,

    #[allow(missing_docs)]
    CreateTexture1DInvalidSamples,

    #[allow(missing_docs)]
    CreateTexture1DUnrecognizedUsage,

    #[allow(missing_docs)]
    CreateTexture1DUnrecognizedBindFlags,

    #[allow(missing_docs)]
    CreateTexture1DUnrecognizedCpuAccessFlags,

    #[allow(missing_docs)]
    CreateTexture1DUnrecognizedMiscFlags,

    #[allow(missing_docs)]
    CreateTexture1DInvalidCpuAccessFlags,

    #[allow(missing_docs)]
    CreateTexture1DInvalidBindFlags,

    #[allow(missing_docs)]
    CreateTexture1DInvalidInitialData,

    #[allow(missing_docs)]
    CreateTexture1DInvalidDimensions,

    #[allow(missing_docs)]
    CreateTexture1DInvalidMipLevels,

    #[allow(missing_docs)]
    CreateTexture1DInvalidMiscFlags,

    #[allow(missing_docs)]
    CreateTexture1DInvalidArgReturn,

    #[allow(missing_docs)]
    CreateTexture1DOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateTexture1DNullDesc,

    #[allow(missing_docs)]
    CreateTexture1DLargeAllocation,

    #[allow(missing_docs)]
    CreateTexture2DUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateTexture2DUnsupportedFormat,

    #[allow(missing_docs)]
    CreateTexture2DInvalidSamples,

    #[allow(missing_docs)]
    CreateTexture2DUnrecognizedUsage,

    #[allow(missing_docs)]
    CreateTexture2DUnrecognizedBindFlags,

    #[allow(missing_docs)]
    CreateTexture2DUnrecognizedCpuAccessFlags,

    #[allow(missing_docs)]
    CreateTexture2DUnrecognizedMiscFlags,

    #[allow(missing_docs)]
    CreateTexture2DInvalidCpuAccessFlags,

    #[allow(missing_docs)]
    CreateTexture2DInvalidBindFlags,

    #[allow(missing_docs)]
    CreateTexture2DInvalidInitialData,

    #[allow(missing_docs)]
    CreateTexture2DInvalidDimensions,

    #[allow(missing_docs)]
    CreateTexture2DInvalidMipLevels,

    #[allow(missing_docs)]
    CreateTexture2DInvalidMiscFlags,

    #[allow(missing_docs)]
    CreateTexture2DInvalidArgReturn,

    #[allow(missing_docs)]
    CreateTexture2DOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateTexture2DNullDesc,

    #[allow(missing_docs)]
    CreateTexture2DLargeAllocation,

    #[allow(missing_docs)]
    CreateTexture3DUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateTexture3DUnsupportedFormat,

    #[allow(missing_docs)]
    CreateTexture3DInvalidSamples,

    #[allow(missing_docs)]
    CreateTexture3DUnrecognizedUsage,

    #[allow(missing_docs)]
    CreateTexture3DUnrecognizedBindFlags,

    #[allow(missing_docs)]
    CreateTexture3DUnrecognizedCpuAccessFlags,

    #[allow(missing_docs)]
    CreateTexture3DUnrecognizedMiscFlags,

    #[allow(missing_docs)]
    CreateTexture3DInvalidCpuAccessFlags,

    #[allow(missing_docs)]
    CreateTexture3DInvalidBindFlags,

    #[allow(missing_docs)]
    CreateTexture3DInvalidInitialData,

    #[allow(missing_docs)]
    CreateTexture3DInvalidDimensions,

    #[allow(missing_docs)]
    CreateTexture3DInvalidMipLevels,

    #[allow(missing_docs)]
    CreateTexture3DInvalidMiscFlags,

    #[allow(missing_docs)]
    CreateTexture3DInvalidArgReturn,

    #[allow(missing_docs)]
    CreateTexture3DOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateTexture3DNullDesc,

    #[allow(missing_docs)]
    CreateTexture3DLargeAllocation,

    #[allow(missing_docs)]
    CreateShaderResourceViewUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidDesc,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidFormat,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidDimensions,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidResource,

    #[allow(missing_docs)]
    CreateShaderResourceViewTooManyObjects,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidArgReturn,

    #[allow(missing_docs)]
    CreateShaderResourceViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateRenderTargetViewUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateRenderTargetViewUnsupportedFormat,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidDesc,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidFormat,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidDimensions,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidResource,

    #[allow(missing_docs)]
    CreateRenderTargetViewTooManyObjects,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidArgReturn,

    #[allow(missing_docs)]
    CreateRenderTargetViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateDepthstencilViewUnrecognizedFormat,

    #[allow(missing_docs)]
    CreateDepthstencilViewInvalidDesc,

    #[allow(missing_docs)]
    CreateDepthstencilViewInvalidFormat,

    #[allow(missing_docs)]
    CreateDepthstencilViewInvalidDimensions,

    #[allow(missing_docs)]
    CreateDepthstencilViewInvalidResource,

    #[allow(missing_docs)]
    CreateDepthstencilViewTooManyObjects,

    #[allow(missing_docs)]
    CreateDepthstencilViewInvalidArgReturn,

    #[allow(missing_docs)]
    CreateDepthstencilViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateInputLayoutOutOfMemory,

    #[allow(missing_docs)]
    CreateInputLayoutTooManyELEMENTS,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidFormat,

    #[allow(missing_docs)]
    CreateInputLayoutIncompatibleFormat,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidSlot,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidInputsLOTClass,

    #[allow(missing_docs)]
    CreateInputLayoutStepRateSlotClassMismatch,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidSlotClassChange,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidStepRateChange,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidAlignment,

    #[allow(missing_docs)]
    CreateInputLayoutDuplicateSemantic,

    #[allow(missing_docs)]
    CreateInputLayoutUNPARsEABLEInputsignature,

    #[allow(missing_docs)]
    CreateInputLayoutNullSemantic,

    #[allow(missing_docs)]
    CreateInputLayoutMISSInGELEMENT,

    #[allow(missing_docs)]
    CreateInputLayoutNullDesc,

    #[allow(missing_docs)]
    CreateVertexShaderOutOfMemory,

    #[allow(missing_docs)]
    CreateVertexShaderInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreateVertexShaderInvalidShaderType,

    #[allow(missing_docs)]
    CreateGeometryShaderOutOfMemory,

    #[allow(missing_docs)]
    CreateGeometryShaderInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreateGeometryShaderInvalidShaderType,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputOutOfMemory,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidShaderType,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidNumEntries,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputOutputStreamsTRIDEUNUSED,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputUnexpectedDECL,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputEXPECTEDDECL,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputOutputSlot0EXPECTED,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidOutputSlot,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputOnlyOnEELEMENTPERsLOT,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidComponentCount,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidStartComponentAndComponentCount,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidGAPDEFInITIOn,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputREPEATEDOutput,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidOutputStreamsTRIDE,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputMISSInGsemantic,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputMaskMismatch,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputCANTHAVEOnlyGAPs,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputDECLTooCOmPLEX,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputMISSInGOutputSignature,

    #[allow(missing_docs)]
    CreatePixelShaderOutOfMemory,

    #[allow(missing_docs)]
    CreatePixelShaderInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreatePixelShaderInvalidShaderType,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidFillMode,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidCullMode,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidDepthBiasClamp,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidSlopeScaledDepthBias,

    #[allow(missing_docs)]
    CreateRasterizerStateTooManyObjects,

    #[allow(missing_docs)]
    CreateRasterizerStateNullDesc,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidDepthWriteMask,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidDepthFunc,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidFrontFaceStencilFailOp,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidFrontFaceStencilZFailOp,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidFrontFaceStencilPassOp,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidFrontFaceStencilFunc,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidBackFaceStencilFailOp,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidBackFaceStencilZFailOp,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidBackFaceStencilPassOp,

    #[allow(missing_docs)]
    CreateDepthstencilStateInvalidBackFaceStencilFunc,

    #[allow(missing_docs)]
    CreateDepthstencilStateTooManyObjects,

    #[allow(missing_docs)]
    CreateDepthstencilStateNullDesc,

    #[allow(missing_docs)]
    CreateBlendStateInvalidSrcBlend,

    #[allow(missing_docs)]
    CreateBlendStateInvalidDestBlend,

    #[allow(missing_docs)]
    CreateBlendStateInvalidBlendOp,

    #[allow(missing_docs)]
    CreateBlendStateInvalidSrcBlendAlpha,

    #[allow(missing_docs)]
    CreateBlendStateInvalidDestBlendAlpha,

    #[allow(missing_docs)]
    CreateBlendStateInvalidBlendOpAlpha,

    #[allow(missing_docs)]
    CreateBlendStateInvalidRenderTargetWriteMask,

    #[allow(missing_docs)]
    CreateBlendStateTooManyObjects,

    #[allow(missing_docs)]
    CreateBlendStateNullDesc,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidFilter,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidAddressU,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidAddressV,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidAddressW,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidMipLodBias,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidMaxAnisotropy,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidComparisonFunc,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidMinLod,

    #[allow(missing_docs)]
    CreateSamplerstateInvalidMaxLod,

    #[allow(missing_docs)]
    CreateSamplerstateTooManyObjects,

    #[allow(missing_docs)]
    CreateSamplerstateNullDesc,

    #[allow(missing_docs)]
    CreateQueryOrPredicateInvalidQuery,

    #[allow(missing_docs)]
    CreateQueryOrPredicateInvalidMiscFlags,

    #[allow(missing_docs)]
    CreateQueryOrPredicateUnexpectedMiscFlag,

    #[allow(missing_docs)]
    CreateQueryOrPredicateNullDesc,

    #[allow(missing_docs)]
    DeviceIaSetPrimitiveTopologyTopologyUnrecognized,

    #[allow(missing_docs)]
    DeviceIaSetPrimitiveTopologyTopologyUndefined,

    #[allow(missing_docs)]
    IaSetVertexBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersOffsetTooLarge,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersBuffersEmpty,

    #[allow(missing_docs)]
    IaSetIndexBufferInvalidBuffer,

    #[allow(missing_docs)]
    DeviceIaSetIndexBufferFormatInvalid,

    #[allow(missing_docs)]
    DeviceIaSetIndexBufferOffsetTooLarge,

    #[allow(missing_docs)]
    DeviceIaSetIndexBufferOffsetUnaligned,

    #[allow(missing_docs)]
    DeviceVsSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    VsSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceVsSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceVsSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceGsSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    GsSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceGsSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceGsSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    SoSetTargetsInvalidBuffer,

    #[allow(missing_docs)]
    DeviceSoSetTargetsOffsetUnaligned,

    #[allow(missing_docs)]
    DevicePsSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    PsSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DevicePsSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DevicePsSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceRsSetViewportsInvalidViewport,

    #[allow(missing_docs)]
    DeviceRsSetScissorRectsInvalidScissor,

    #[allow(missing_docs)]
    ClearRenderTargetViewDenormFlush,

    #[allow(missing_docs)]
    ClearDepthstencilViewDenormFlush,

    #[allow(missing_docs)]
    ClearDepthstencilViewInvalid,

    #[allow(missing_docs)]
    DeviceIaGetVertexBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceVsGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceVsGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceVsGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceGsGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceGsGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceGsGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceSoGetTargetsBuffersEmpty,

    #[allow(missing_docs)]
    DevicePsGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DevicePsGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DevicePsGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceRsGetViewportsViewportsEmpty,

    #[allow(missing_docs)]
    DeviceRsGetScissorRectsRectsEmpty,

    #[allow(missing_docs)]
    DeviceGenerateMipsResourceInvalid,

    #[allow(missing_docs)]
    CopySubresourceRegionInvalidDestinationSubresource,

    #[allow(missing_docs)]
    CopySubresourceRegionInvalidSourceSubresource,

    #[allow(missing_docs)]
    CopySubresourceRegionInvalidSourceBox,

    #[allow(missing_docs)]
    CopySubresourceRegionInvalidSource,

    #[allow(missing_docs)]
    CopySubresourceRegionInvalidDestinationState,

    #[allow(missing_docs)]
    CopySubresourceRegionInvalidSourceState,

    #[allow(missing_docs)]
    CopyResourceInvalidSource,

    #[allow(missing_docs)]
    CopyResourceInvalidDestinationState,

    #[allow(missing_docs)]
    CopyResourceInvalidSourceState,

    #[allow(missing_docs)]
    UpdateSubresourceInvalidDestinationSubresource,

    #[allow(missing_docs)]
    UpdateSubresourceInvalidDestinationBox,

    #[allow(missing_docs)]
    UpdateSubresourceInvalidDestinationState,

    #[allow(missing_docs)]
    DeviceRESoLVESubresourceDestinationInvalid,

    #[allow(missing_docs)]
    DeviceRESoLVESubresourceDestinationSubresourceInvalid,

    #[allow(missing_docs)]
    DeviceRESoLVESubresourcesOURCEInvalid,

    #[allow(missing_docs)]
    DeviceRESoLVESubresourcesOURCESubresourceInvalid,

    #[allow(missing_docs)]
    DeviceRESoLVESubresourceFormatInvalid,

    #[allow(missing_docs)]
    BufferMapInvalidMapType,

    #[allow(missing_docs)]
    BufferMapInvalidFlags,

    #[allow(missing_docs)]
    BufferMapALReadYMapped,

    #[allow(missing_docs)]
    BufferMapDeviceRemovedReturn,

    #[allow(missing_docs)]
    BufferUnmapNotMapped,

    #[allow(missing_docs)]
    Texture1DMapInvalidMapType,

    #[allow(missing_docs)]
    Texture1DMapInvalidSubresource,

    #[allow(missing_docs)]
    Texture1DMapInvalidFlags,

    #[allow(missing_docs)]
    Texture1DMapALReadYMapped,

    #[allow(missing_docs)]
    Texture1DMapDeviceRemovedReturn,

    #[allow(missing_docs)]
    Texture1DUnmapInvalidSubresource,

    #[allow(missing_docs)]
    Texture1DUnmapNotMapped,

    #[allow(missing_docs)]
    Texture2DMapInvalidMapType,

    #[allow(missing_docs)]
    Texture2DMapInvalidSubresource,

    #[allow(missing_docs)]
    Texture2DMapInvalidFlags,

    #[allow(missing_docs)]
    Texture2DMapALReadYMapped,

    #[allow(missing_docs)]
    Texture2DMapDeviceRemovedReturn,

    #[allow(missing_docs)]
    Texture2DUnmapInvalidSubresource,

    #[allow(missing_docs)]
    Texture2DUnmapNotMapped,

    #[allow(missing_docs)]
    Texture3DMapInvalidMapType,

    #[allow(missing_docs)]
    Texture3DMapInvalidSubresource,

    #[allow(missing_docs)]
    Texture3DMapInvalidFlags,

    #[allow(missing_docs)]
    Texture3DMapALReadYMapped,

    #[allow(missing_docs)]
    Texture3DMapDeviceRemovedReturn,

    #[allow(missing_docs)]
    Texture3DUnmapInvalidSubresource,

    #[allow(missing_docs)]
    Texture3DUnmapNotMapped,

    #[allow(missing_docs)]
    CheckFormatSupportFormatDeprecated,

    #[allow(missing_docs)]
    CheckMultiSampleQualityLevelsFormatDeprecated,

    #[allow(missing_docs)]
    SetExceptionModeUnrecognizedFlags,

    #[allow(missing_docs)]
    SetExceptionModeInvalidArgReturn,

    #[allow(missing_docs)]
    SetExceptionModeDeviceRemovedReturn,

    #[allow(missing_docs)]
    REFSIMULATInGInFInITELYFAsTHardware,

    #[allow(missing_docs)]
    REFThreadingMode,

    #[allow(missing_docs)]
    REFUMDRIVERException,

    #[allow(missing_docs)]
    REFKMDRIVERException,

    #[allow(missing_docs)]
    REFHardwareException,

    #[allow(missing_docs)]
    REFAccessInGIndexABLETEMPOutOfRange,

    #[allow(missing_docs)]
    REFPROBLEMPARsInGshader,

    #[allow(missing_docs)]
    REFOutOfMemory,

    #[allow(missing_docs)]
    REFInFO,

    #[allow(missing_docs)]
    DeviceDrawVertexPOSoVERFLOW,

    #[allow(missing_docs)]
    DeviceDrawIndexEDIndexPOSoVERFLOW,

    #[allow(missing_docs)]
    DeviceDrawInstanceDVertexPOSoVERFLOW,

    #[allow(missing_docs)]
    DeviceDrawInstanceDInstancePOSoVERFLOW,

    #[allow(missing_docs)]
    DeviceDrawIndexEDInstanceDInstancePOSoVERFLOW,

    #[allow(missing_docs)]
    DeviceDrawIndexEDInstanceDIndexPOSoVERFLOW,

    #[allow(missing_docs)]
    DeviceDrawVertexShaderNotSet,

    #[allow(missing_docs)]
    DeviceShaderLinkageSemanticNAMENotFound,

    #[allow(missing_docs)]
    DeviceShaderLinkageREGISTERIndex,

    #[allow(missing_docs)]
    DeviceShaderLinkageComponentType,

    #[allow(missing_docs)]
    DeviceShaderLinkageREGISTERMask,

    #[allow(missing_docs)]
    DeviceShaderLinkageSystemValue,

    #[allow(missing_docs)]
    DeviceShaderLinkageNEVERWRITTENALWAYSReadS,

    #[allow(missing_docs)]
    DeviceDrawVertexBufferNotSet,

    #[allow(missing_docs)]
    DeviceDrawInputLayoutNotSet,

    #[allow(missing_docs)]
    DeviceDrawConstantBufferNotSet,

    #[allow(missing_docs)]
    DeviceDrawConstantBufferTooSMALL,

    #[allow(missing_docs)]
    DeviceDrawSampleRNotSet,

    #[allow(missing_docs)]
    DeviceDrawShaderResourceViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawViewDimensionMismatch,

    #[allow(missing_docs)]
    DeviceDrawVertexBuffersTRIDETooSMALL,

    #[allow(missing_docs)]
    DeviceDrawVertexBufferTooSMALL,

    #[allow(missing_docs)]
    DeviceDrawIndexBufferNotSet,

    #[allow(missing_docs)]
    DeviceDrawIndexBufferFormatInvalid,

    #[allow(missing_docs)]
    DeviceDrawIndexBufferTooSMALL,

    #[allow(missing_docs)]
    DeviceDrawGsInputPrimitiveMismatch,

    #[allow(missing_docs)]
    DeviceDrawResourceReturnTypeMismatch,

    #[allow(missing_docs)]
    DeviceDrawPOSITIOnNotPRESENT,

    #[allow(missing_docs)]
    DeviceDrawOutputStreamNotSet,

    #[allow(missing_docs)]
    DeviceDrawBOUNDResourceMapped,

    #[allow(missing_docs)]
    DeviceDrawInvalidPrimitiveTopology,

    #[allow(missing_docs)]
    DeviceDrawVertexOffsetUnaligned,

    #[allow(missing_docs)]
    DeviceDrawVertexStrideUnaligned,

    #[allow(missing_docs)]
    DeviceDrawIndexOffsetUnaligned,

    #[allow(missing_docs)]
    DeviceDrawOutputStreamOffsetUnaligned,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatLDUnsupported,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatSampleUnsupported,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatSampleCUnsupported,

    #[allow(missing_docs)]
    DeviceDrawResourceMultiSampleUnsupported,

    #[allow(missing_docs)]
    DeviceDrawSoTargetsBOUNDWithoutSource,

    #[allow(missing_docs)]
    DeviceDrawSoStrideLargeRTHANBuffer,

    #[allow(missing_docs)]
    DeviceDrawOmRenderTargetDOESNotSupportBlendInG,

    #[allow(missing_docs)]
    DeviceDrawOmDUALSourceBlendInGCANonlyHAVERenderTarget0,

    #[allow(missing_docs)]
    DeviceRemovalProcessATFault,

    #[allow(missing_docs)]
    DeviceRemovalProcessPOSSIBLYATFault,

    #[allow(missing_docs)]
    DeviceRemovalProcessNotATFault,

    #[allow(missing_docs)]
    DeviceOpENSharedResourceInvalidArgReturn,

    #[allow(missing_docs)]
    DeviceOpENSharedResourceOutOfMemoryReturn,

    #[allow(missing_docs)]
    DeviceOpENSharedResourceBadInterfaceReturn,

    #[allow(missing_docs)]
    DeviceDrawViewportNotSet,

    #[allow(missing_docs)]
    CreateInputLayoutTRAILInGDIGITInSemantic,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputTRAILInGDIGITInSemantic,

    #[allow(missing_docs)]
    DeviceRsSetViewportsDenormFlush,

    #[allow(missing_docs)]
    OmSetRenderTargetsInvalidView,

    #[allow(missing_docs)]
    DeviceSetTEXTFilterSizeInvalidDimensions,

    #[allow(missing_docs)]
    DeviceDrawSampleRMismatch,

    #[allow(missing_docs)]
    CreateInputLayoutTypeMismatch,

    #[allow(missing_docs)]
    BlendStateGetDescLEGACY,

    #[allow(missing_docs)]
    ShaderResourceViewGetDescLEGACY,

    #[allow(missing_docs)]
    CreateQueryOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreatePredicateOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateCounterOutOfRangeCounter,

    #[allow(missing_docs)]
    CreateCountersIMULTANEOUSACTIVECountersEXHAUSTED,

    #[allow(missing_docs)]
    CreateCounterUnsupportedWELLKNoWNCounter,

    #[allow(missing_docs)]
    CreateCounterOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateCounterNonEXCLUSIVEReturn,

    #[allow(missing_docs)]
    CreateCounterNullDesc,

    #[allow(missing_docs)]
    CheckCounterOutOfRangeCounter,

    #[allow(missing_docs)]
    CheckCounterUnsupportedWELLKNoWNCounter,

    #[allow(missing_docs)]
    SetPredicationInvalidPredicateState,

    #[allow(missing_docs)]
    QueryBEGInUnsupported,

    #[allow(missing_docs)]
    PredicateBEGInDURInGPredication,

    #[allow(missing_docs)]
    QueryBEGInDuplicate,

    #[allow(missing_docs)]
    QueryBEGInABAndOnInGPREVIOUSRESULTS,

    #[allow(missing_docs)]
    PredicateEndDURInGPredication,

    #[allow(missing_docs)]
    QueryEndABAndOnInGPREVIOUSRESULTS,

    #[allow(missing_docs)]
    QueryEndWithoutBEGIn,

    #[allow(missing_docs)]
    QueryGetDataInvalidDataSize,

    #[allow(missing_docs)]
    QueryGetDataInvalidFlags,

    #[allow(missing_docs)]
    QueryGetDataInvalidCall,

    #[allow(missing_docs)]
    DeviceDrawPsOutputTypeMismatch,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatGATHERUnsupported,

    #[allow(missing_docs)]
    DeviceDrawInvalidUSEOFCENTERMultiSamplePATTERN,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersStrideTooLarge,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersInvalidRange,

    #[allow(missing_docs)]
    CreateInputLayoutEmptyLayout,

    #[allow(missing_docs)]
    DeviceDrawResourcesAMPLECountMismatch,

    #[allow(missing_docs)]
    LiveObjectsUnmarry,

    #[allow(missing_docs)]
    LiveBuffer,

    #[allow(missing_docs)]
    LiveTexture1D,

    #[allow(missing_docs)]
    LiveTexture2D,

    #[allow(missing_docs)]
    LiveTexture3D,

    #[allow(missing_docs)]
    LiveShaderResourceView,

    #[allow(missing_docs)]
    LiveRenderTargetView,

    #[allow(missing_docs)]
    LiveDepthstencilView,

    #[allow(missing_docs)]
    LiveVertexShader,

    #[allow(missing_docs)]
    LiveGeometryShader,

    #[allow(missing_docs)]
    LivePixelShader,

    #[allow(missing_docs)]
    LiveInputLayout,

    #[allow(missing_docs)]
    LiveSampleR,

    #[allow(missing_docs)]
    LiveBlendState,

    #[allow(missing_docs)]
    LiveDepthstencilState,

    #[allow(missing_docs)]
    LiveRasterizerState,

    #[allow(missing_docs)]
    LiveQuery,

    #[allow(missing_docs)]
    LivePredicate,

    #[allow(missing_docs)]
    LiveCounter,

    #[allow(missing_docs)]
    LiveDevice,

    #[allow(missing_docs)]
    LiveSwapchain,

    #[allow(missing_docs)]
    D3D10MessageSend,

    #[allow(missing_docs)]
    D3D10L9MessagesStart = 0x100000,

    #[allow(missing_docs)]
    CreateDepthstencilStateStencilNotWoSided,

    #[allow(missing_docs)]
    CreateRasterizerStateDepthBiasClampNotSupported,

    #[allow(missing_docs)]
    CreateSamplerstateNoComparisonSupport,

    #[allow(missing_docs)]
    CreateSamplerstateExcessiveAnisotropy,

    #[allow(missing_docs)]
    CreateSamplerstateBorderOutOfRange,

    #[allow(missing_docs)]
    VsSetSamplersNotSupported,

    #[allow(missing_docs)]
    VsSetSamplersTooManySamplers,

    #[allow(missing_docs)]
    PsSetSamplersTooManySamplers,

    #[allow(missing_docs)]
    CreateResourceNoArrays,

    #[allow(missing_docs)]
    CreateResourceNoVBandIbBind,

    #[allow(missing_docs)]
    CreateResourceNoTexture1D,

    #[allow(missing_docs)]
    CreateResourceDimensionoutOfRange,

    #[allow(missing_docs)]
    CreateResourceNotBindableAsShaderResource,

    #[allow(missing_docs)]
    OmSetRenderTargetsTooManyRenderTargets,

    #[allow(missing_docs)]
    OmSetRenderTargetsNoDifferingBitDepths,

    #[allow(missing_docs)]
    IaSetVertexBuffersBadBufferIndex,

    #[allow(missing_docs)]
    DeviceRsSetViewportsTooManyViewports,

    #[allow(missing_docs)]
    DeviceIaSetPrimitiveTopologyAdjacencyUnsupported,

    #[allow(missing_docs)]
    DeviceRsSetScissorRectsTooManyScissors,

    #[allow(missing_docs)]
    CopyResourceOnlyTexture2DWithInGPUMemory,

    #[allow(missing_docs)]
    CopyResourceNoTexture3DReadBack,

    #[allow(missing_docs)]
    CopyResourceNoTextureOnlyReadBack,

    #[allow(missing_docs)]
    CreateInputLayoutUnsupportedFormat,

    #[allow(missing_docs)]
    CreateBlendStateNoAlphaToCOVERAGE,

    #[allow(missing_docs)]
    CreateRasterizerStateDepthClipEnableMUSTBETRUE,

    #[allow(missing_docs)]
    DrawIndexEDStartIndexLOCATIOnMUSTBEPOSITIVE,

    #[allow(missing_docs)]
    CreateShaderResourceViewMUSTUSELOWESTLod,

    #[allow(missing_docs)]
    CreateSamplerstateMinLodMUSTNotBEFractional,

    #[allow(missing_docs)]
    CreateSamplerstateMaxLodMUSTBEFLTMax,

    #[allow(missing_docs)]
    CreateShaderResourceViewFIRsTArraysLICEMUSTBEZERO,

    #[allow(missing_docs)]
    CreateShaderResourceViewCUBESMUSTHAVE6SIDES,

    #[allow(missing_docs)]
    CreateResourceNotBindableAsRenderTarget,

    #[allow(missing_docs)]
    CreateResourceNoDWOrDIndexBuffer,

    #[allow(missing_docs)]
    CreateResourceMSAAPRECLUDESShaderResource,

    #[allow(missing_docs)]
    CreateResourcePRESENTATIOnPRECLUDESShaderResource,

    #[allow(missing_docs)]
    CreateBlendStateNoInDEPEndENTBlendENABLE,

    #[allow(missing_docs)]
    CreateBlendStateNoInDEPEndENTWriteMaskS,

    #[allow(missing_docs)]
    CreateResourceNoStreamOUT,

    #[allow(missing_docs)]
    CreateResourceOnlyVBIBForBuffers,

    #[allow(missing_docs)]
    CreateResourceNoAutoGENForVOLUMES,

    #[allow(missing_docs)]
    CreateResourceDXGIFormatR8G8B8A8CannotBEShared,

    #[allow(missing_docs)]
    VsShaderResourcesNotSupported,

    #[allow(missing_docs)]
    GeometryShaderNotSupported,

    #[allow(missing_docs)]
    StreamOUTNotSupported,

    #[allow(missing_docs)]
    TEXTFilterNotSupported,

    #[allow(missing_docs)]
    CreateBlendStateNoSEPARateAlphaBlend,

    #[allow(missing_docs)]
    CreateBlendStateNomRTBlend,

    #[allow(missing_docs)]
    CreateBlendStateOpERationNotSupported,

    #[allow(missing_docs)]
    CreateSamplerstateNomIRROrOnCE,

    #[allow(missing_docs)]
    DrawInstanceDNotSupported,

    #[allow(missing_docs)]
    DrawIndexEDInstanceDNotSupportedBELOW93,

    #[allow(missing_docs)]
    DrawIndexEDPointListUnsupported,

    #[allow(missing_docs)]
    SetBlendStateSampleMaskCannotBEZERO,

    #[allow(missing_docs)]
    CreateResourceDimensionEXCEEDSFeatureLevelDEFInITIOn,

    #[allow(missing_docs)]
    CreateResourceOnlySingleMipLevelDepthstencilSupported,

    #[allow(missing_docs)]
    DeviceRsSetScissorRectsNEGATIVEScissor,

    #[allow(missing_docs)]
    SlotZEROmUSTBED3D10InputPERVertexData,

    #[allow(missing_docs)]
    CreateResourceNonPow2MipMap,

    #[allow(missing_docs)]
    CreateSamplerstateBorderNotSupported,

    #[allow(missing_docs)]
    OmSetRenderTargetsNoSrgbMrt,

    #[allow(missing_docs)]
    CopyResourceNo3DMismatchedUpdates,

    #[allow(missing_docs)]
    D3D10L9MessageSend,

    #[allow(missing_docs)]
    D3D11MessagesStart = 0x200000,

    #[allow(missing_docs)]
    CreateDepthstencilViewInvalidFlags,

    #[allow(missing_docs)]
    CreateVertexShaderInvalidClassLinkage,

    #[allow(missing_docs)]
    CreateGeometryShaderInvalidClassLinkage,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidNumStreams,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidStreamToRasterizer,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputUnexpectedStreams,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidClassLinkage,

    #[allow(missing_docs)]
    CreatePixelShaderInvalidClassLinkage,

    #[allow(missing_docs)]
    CreateDeferredContextInvalidCommandListFlags,

    #[allow(missing_docs)]
    CreateDeferredContextSingleThreaded,

    #[allow(missing_docs)]
    CreateDeferredContextInvalidArgReturn,

    #[allow(missing_docs)]
    CreateDeferredContextInvalidCallReturn,

    #[allow(missing_docs)]
    CreateDeferredContextOutOfMemoryReturn,

    #[allow(missing_docs)]
    FinishDisplayListOnImmediateContext,

    #[allow(missing_docs)]
    FinishDisplayListOutOfMemoryReturn,

    #[allow(missing_docs)]
    FinishDisplayListInvalidCallReturn,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidStream,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputUnexpectedEntries,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputUnexpectedStrides,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidNumStrides,

    #[allow(missing_docs)]
    DeviceHSSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceHSSetConstantBuffersHazard,

    #[allow(missing_docs)]
    HSSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    HSSetConstantBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    CreateHullShaderInvalidCall,

    #[allow(missing_docs)]
    CreateHullShaderOutOfMemory,

    #[allow(missing_docs)]
    CreateHullShaderInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreateHullShaderInvalidShaderType,

    #[allow(missing_docs)]
    CreateHullShaderInvalidClassLinkage,

    #[allow(missing_docs)]
    DeviceHSSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    HSSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceHSSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceHSSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceHSGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceHSGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceHSGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceDSSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceDSSetConstantBuffersHazard,

    #[allow(missing_docs)]
    DSSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    DSSetConstantBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    CreateDomainShaderInvalidCall,

    #[allow(missing_docs)]
    CreateDomainShaderOutOfMemory,

    #[allow(missing_docs)]
    CreateDomainShaderInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreateDomainShaderInvalidShaderType,

    #[allow(missing_docs)]
    CreateDomainShaderInvalidClassLinkage,

    #[allow(missing_docs)]
    DeviceDSSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DSSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceDSSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceDSSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceDSGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceDSGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceDSGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceDrawHSXOrDSMismatch,

    #[allow(missing_docs)]
    DeferredContextRemovalProcessATFault,

    #[allow(missing_docs)]
    DeviceDrawIndirectInvalidArgBuffer,

    #[allow(missing_docs)]
    DeviceDrawIndirectOffsetUnaligned,

    #[allow(missing_docs)]
    DeviceDrawIndirectOffsetOverflow,

    #[allow(missing_docs)]
    ResourceMapInvalidMapType,

    #[allow(missing_docs)]
    ResourceMapInvalidSubresource,

    #[allow(missing_docs)]
    ResourceMapInvalidFlags,

    #[allow(missing_docs)]
    ResourceMapALReadYMapped,

    #[allow(missing_docs)]
    ResourceMapDeviceRemovedReturn,

    #[allow(missing_docs)]
    ResourceMapOutOfMemoryReturn,

    #[allow(missing_docs)]
    ResourceMapWithoutInitialDiscard,

    #[allow(missing_docs)]
    ResourceUnmapInvalidSubresource,

    #[allow(missing_docs)]
    ResourceUnmapNotMapped,

    #[allow(missing_docs)]
    DeviceDrawRasterizingControlPoints,

    #[allow(missing_docs)]
    DeviceIaSetPrimitiveTopologyTopologyUnsupported,

    #[allow(missing_docs)]
    DeviceDrawHsdsSignatureMismatch,

    #[allow(missing_docs)]
    DeviceDrawHullShaderInputTopologyMismatch,

    #[allow(missing_docs)]
    DeviceDrawHsdsControlPointCountMismatch,

    #[allow(missing_docs)]
    DeviceDrawHsdsTessellatorDomainismatch,

    #[allow(missing_docs)]
    CreateContext,

    #[allow(missing_docs)]
    LiveContext,

    #[allow(missing_docs)]
    DestroyContext,

    #[allow(missing_docs)]
    CreateBuffer,

    #[allow(missing_docs)]
    LiveBufferWin7,

    #[allow(missing_docs)]
    DestroyBuffer,

    #[allow(missing_docs)]
    CreateTexture1D,

    #[allow(missing_docs)]
    LiveTexture1DWin7,

    #[allow(missing_docs)]
    DestroyTexture1D,

    #[allow(missing_docs)]
    CreateTexture2D,

    #[allow(missing_docs)]
    LiveTexture2DWin7,

    #[allow(missing_docs)]
    DestroyTexture2D,

    #[allow(missing_docs)]
    CreateTexture3D,

    #[allow(missing_docs)]
    LiveTexture3DWin7,

    #[allow(missing_docs)]
    DestroyTexture3D,

    #[allow(missing_docs)]
    CreateShaderResourceView,

    #[allow(missing_docs)]
    LiveShaderResourceViewWin7,

    #[allow(missing_docs)]
    DestroyShaderResourceView,

    #[allow(missing_docs)]
    CreateRenderTargetView,

    #[allow(missing_docs)]
    LiveRenderTargetViewWin7,

    #[allow(missing_docs)]
    DestroyRenderTargetView,

    #[allow(missing_docs)]
    CreateDepthstencilView,

    #[allow(missing_docs)]
    LiveDepthstencilViewWin7,

    #[allow(missing_docs)]
    DestroyDepthstencilView,

    #[allow(missing_docs)]
    CreateVertexShader,

    #[allow(missing_docs)]
    LiveVertexShaderWin7,

    #[allow(missing_docs)]
    DestroyVertexShader,

    #[allow(missing_docs)]
    CreateHullShader,

    #[allow(missing_docs)]
    LiveHullShader,

    #[allow(missing_docs)]
    DestroyHullShader,

    #[allow(missing_docs)]
    CreateDomainShader,

    #[allow(missing_docs)]
    LiveDomainShader,

    #[allow(missing_docs)]
    DestroyDomainShader,

    #[allow(missing_docs)]
    CreateGeometryShader,

    #[allow(missing_docs)]
    LiveGeometryShaderWin7,

    #[allow(missing_docs)]
    DestroyGeometryShader,

    #[allow(missing_docs)]
    CreatePixelShader,

    #[allow(missing_docs)]
    LivePixelShaderWin7,

    #[allow(missing_docs)]
    DestroyPixelShader,

    #[allow(missing_docs)]
    CreateInputLayout,

    #[allow(missing_docs)]
    LiveInputLayoutWin7,

    #[allow(missing_docs)]
    DestroyInputLayout,

    #[allow(missing_docs)]
    CreateSampleR,

    #[allow(missing_docs)]
    LiveSampleRWin7,

    #[allow(missing_docs)]
    DestroySampleR,

    #[allow(missing_docs)]
    CreateBlendState,

    #[allow(missing_docs)]
    LiveBlendStateWin7,

    #[allow(missing_docs)]
    DestroyBlendState,

    #[allow(missing_docs)]
    CreateDepthstencilState,

    #[allow(missing_docs)]
    LiveDepthstencilStateWin7,

    #[allow(missing_docs)]
    DestroyDepthstencilState,

    #[allow(missing_docs)]
    CreateRasterizerState,

    #[allow(missing_docs)]
    LiveRasterizerStateWin7,

    #[allow(missing_docs)]
    DestroyRasterizerState,

    #[allow(missing_docs)]
    CreateQuery,

    #[allow(missing_docs)]
    LiveQueryWin7,

    #[allow(missing_docs)]
    DestroyQuery,

    #[allow(missing_docs)]
    CreatePredicate,

    #[allow(missing_docs)]
    LivePredicateWin7,

    #[allow(missing_docs)]
    DestroyPredicate,

    #[allow(missing_docs)]
    CreateCounter,

    #[allow(missing_docs)]
    DestroyCounter,

    #[allow(missing_docs)]
    CreateCommandList,

    #[allow(missing_docs)]
    LiveCommandList,

    #[allow(missing_docs)]
    DestroyCommandList,

    #[allow(missing_docs)]
    CreateClassInstance,

    #[allow(missing_docs)]
    LiveClassInstance,

    #[allow(missing_docs)]
    DestroyClassInstance,

    #[allow(missing_docs)]
    CreateClassLinkage,

    #[allow(missing_docs)]
    LiveClassLinkage,

    #[allow(missing_docs)]
    DestroyClassLinkage,

    #[allow(missing_docs)]
    LiveDeviceWin7,

    #[allow(missing_docs)]
    LiveObjectsUnmarryWin7,

    #[allow(missing_docs)]
    CreateComputeShader,

    #[allow(missing_docs)]
    LiveComputeShader,

    #[allow(missing_docs)]
    DestroyComputeShader,

    #[allow(missing_docs)]
    CreateUnorderedAccessView,

    #[allow(missing_docs)]
    LiveUnorderedAccessView,

    #[allow(missing_docs)]
    DestroyUnorderedAccessView,

    #[allow(missing_docs)]
    DeviceSetShaderInterfacesFeatureLevel,

    #[allow(missing_docs)]
    DeviceSetShaderInterfaceCountMismatch,

    #[allow(missing_docs)]
    DeviceSetShaderInvalidInstance,

    #[allow(missing_docs)]
    DeviceSetShaderInvalidInstanceIndex,

    #[allow(missing_docs)]
    DeviceSetShaderInvalidInstanceType,

    #[allow(missing_docs)]
    DeviceSetShaderInvalidInstanceData,

    #[allow(missing_docs)]
    DeviceSetShaderUnboundInstanceData,

    #[allow(missing_docs)]
    DeviceSetShaderInstanceDataBindings,

    #[allow(missing_docs)]
    DeviceCreateShaderClassLinkageFull,

    #[allow(missing_docs)]
    DeviceCheckFeatureSupportUnrecognizedFeature,

    #[allow(missing_docs)]
    DeviceCheckFeatureSupportMismatchedDataSize,

    #[allow(missing_docs)]
    DeviceCheckFeatureSupportInvalidArgReturn,

    #[allow(missing_docs)]
    DeviceCsSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceCsSetConstantBuffersHazard,

    #[allow(missing_docs)]
    CsSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    CsSetConstantBuffersUnbindDeletingObject,

    #[allow(missing_docs)]
    CreateComputeShaderInvalidCall,

    #[allow(missing_docs)]
    CreateComputeShaderOutOfMemory,

    #[allow(missing_docs)]
    CreateComputeShaderInvalidShaderByteCode,

    #[allow(missing_docs)]
    CreateComputeShaderInvalidShaderType,

    #[allow(missing_docs)]
    CreateComputeShaderInvalidClassLinkage,

    #[allow(missing_docs)]
    DeviceCsSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    CsSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceCsSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceCsSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceCsGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceCsGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceCsGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderDoubleFloatOpsNotSupported,

    #[allow(missing_docs)]
    CreateBufferInvalidStructureStride,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidFlags,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidResource,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidDesc,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidFormat,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidDimensions,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewUnrecognizedFormat,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsHazard,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsOverlappingOldSlots,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsNoOp,

    #[allow(missing_docs)]
    CsSetUnorderedAccessViewsUnbindDeletingObject,

    #[allow(missing_docs)]
    PsSetUnorderedAccessViewsUnbindDeletingObject,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidArgReturn,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewTooManyObjects,

    #[allow(missing_docs)]
    DeviceCsSetUnorderedAccessViewsHazard,

    #[allow(missing_docs)]
    ClearUnorderedAccessViewDenormFlush,

    #[allow(missing_docs)]
    DeviceCsSetUnorderedAccessViewsEmpty,

    #[allow(missing_docs)]
    DeviceCsGetUnorderedAccessViewsEmpty,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidFlags,

    #[allow(missing_docs)]
    CreateShaderResesourceViewTooManyObjects,

    #[allow(missing_docs)]
    DeviceDispatchIndirectInvalidArgBuffer,

    #[allow(missing_docs)]
    DeviceDispatchIndirectOffsetUnaligned,

    #[allow(missing_docs)]
    DeviceDispatchIndirectOffsetOverflow,

    #[allow(missing_docs)]
    DeviceSetResourceMinLodInvalidContext,

    #[allow(missing_docs)]
    DeviceSetResourceMinLodInvalidResource,

    #[allow(missing_docs)]
    DeviceSetResourceMinLodInvalidMinLod,

    #[allow(missing_docs)]
    DeviceGetResourceMinLodInvalidContext,

    #[allow(missing_docs)]
    DeviceGetResourceMinLodInvalidResource,

    #[allow(missing_docs)]
    OmSetDepthstencilUnbindDeletingObject,

    #[allow(missing_docs)]
    ClearDepthstencilViewDepthReadOnly,

    #[allow(missing_docs)]
    ClearDepthstencilViewStencilReadOnly,

    #[allow(missing_docs)]
    CheckFeatureSupportFormatDeprecated,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewReturnTypeMismatch,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawUnorderedAccessViewRenderTargetViewOverlap,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewDimensionMismatch,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAPPEndUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICsUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewsTRUCTUREStrideMismatch,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewBufferTypeMismatch,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewRAWUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewFormatLDUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewFormatSTorEUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICAddUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICBitWISEOpsUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICCMPsTorECMPExchangeUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICExchangeUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICsIGNEDMInMaxUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewATomICUNSIGNEDMInMaxUnsupported,

    #[allow(missing_docs)]
    DeviceDispatchBOUNDResourceMapped,

    #[allow(missing_docs)]
    DeviceDispatchThreadGROUPCountOverflow,

    #[allow(missing_docs)]
    DeviceDispatchThreadGROUPCountZERO,

    #[allow(missing_docs)]
    DeviceShaderResourceViewsTRUCTUREStrideMismatch,

    #[allow(missing_docs)]
    DeviceShaderResourceViewBufferTypeMismatch,

    #[allow(missing_docs)]
    DeviceShaderResourceViewRAWUnsupported,

    #[allow(missing_docs)]
    DeviceDispatchUnsupported,

    #[allow(missing_docs)]
    DeviceDispatchIndirectUnsupported,

    #[allow(missing_docs)]
    CopyStructureCountInvalidOffset,

    #[allow(missing_docs)]
    CopyStructureCountLargeOffset,

    #[allow(missing_docs)]
    CopyStructureCountInvalidDestinationState,

    #[allow(missing_docs)]
    CopyStructureCountInvalidSourceState,

    #[allow(missing_docs)]
    CheckFormatSupportFormatNotSupported,

    #[allow(missing_docs)]
    DeviceCsSetUnorderedAccessViewsInvalidView,

    #[allow(missing_docs)]
    DeviceCsSetUnorderedAccessViewsInvalidOffset,

    #[allow(missing_docs)]
    DeviceCsSetUnorderedAccessViewsTooManyViews,

    #[allow(missing_docs)]
    ClearUnorderedAccessViewFloatInvalidFormat,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewCounterUnsupported,

    #[allow(missing_docs)]
    REFWarning,

    #[allow(missing_docs)]
    DeviceDrawPixelShaderWithoutRTVOrDSV,

    #[allow(missing_docs)]
    ShaderABOrT,

    #[allow(missing_docs)]
    ShaderMessage,

    #[allow(missing_docs)]
    ShaderError,

    #[allow(missing_docs)]
    OffERResourcesInvalidResource,

    #[allow(missing_docs)]
    HSSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    DSSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    CsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    HSSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    DSSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    CsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    ENQUEUESetEVENTInvalidArgReturn,

    #[allow(missing_docs)]
    ENQUEUESetEVENToutOfMemoryReturn,

    #[allow(missing_docs)]
    ENQUEUESetEVENTAccessDENIEDReturn,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsNumUAVsInvalidRange,

    #[allow(missing_docs)]
    USEOFZEROrEFCountObject,

    #[allow(missing_docs)]
    D3D11MessageSend,

    #[allow(missing_docs)]
    D3D111MessageSStart = 0x300000,

    #[allow(missing_docs)]
    CreateVideoDecodeR,

    #[allow(missing_docs)]
    CreateVideoProcessOrENum,

    #[allow(missing_docs)]
    CreateVideoProcessOr,

    #[allow(missing_docs)]
    CreateDecodeROutputView,

    #[allow(missing_docs)]
    CreateProcessOrInputView,

    #[allow(missing_docs)]
    CreateProcessOrOutputView,

    #[allow(missing_docs)]
    CreateDeviceContextState,

    #[allow(missing_docs)]
    LiveVideoDecodeR,

    #[allow(missing_docs)]
    LiveVideoProcessOrENum,

    #[allow(missing_docs)]
    LiveVideoProcessOr,

    #[allow(missing_docs)]
    LiveDecodeROutputView,

    #[allow(missing_docs)]
    LiveProcessOrInputView,

    #[allow(missing_docs)]
    LiveProcessOrOutputView,

    #[allow(missing_docs)]
    LiveDeviceContextState,

    #[allow(missing_docs)]
    DestroyVideoDecodeR,

    #[allow(missing_docs)]
    DestroyVideoProcessOrENum,

    #[allow(missing_docs)]
    DestroyVideoProcessOr,

    #[allow(missing_docs)]
    DestroyDecodeROutputView,

    #[allow(missing_docs)]
    DestroyProcessOrInputView,

    #[allow(missing_docs)]
    DestroyProcessOrOutputView,

    #[allow(missing_docs)]
    DestroyDeviceContextState,

    #[allow(missing_docs)]
    CreateDeviceContextStateInvalidFlags,

    #[allow(missing_docs)]
    CreateDeviceContextStateInvalidFeatureLevel,

    #[allow(missing_docs)]
    CreateDeviceContextStateFeatureLevelsNotSupported,

    #[allow(missing_docs)]
    CreateDeviceContextStateInvalidREFIID,

    #[allow(missing_docs)]
    DeviceDiscardViewInvalidView,

    #[allow(missing_docs)]
    CopySubresourceRegion1InvalidCopyFlags,

    #[allow(missing_docs)]
    UpdateSubresource1InvalidCopyFlags,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidForCEDSampleCount,

    #[allow(missing_docs)]
    CreateVideoDecodeROutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoDecodeRNullParam,

    #[allow(missing_docs)]
    CreateVideoDecodeRInvalidFormat,

    #[allow(missing_docs)]
    CreateVideoDecodeRZEROWIDTHHEIGHT,

    #[allow(missing_docs)]
    CreateVideoDecodeRDRIVERInvalidBufferSize,

    #[allow(missing_docs)]
    CreateVideoDecodeRDRIVERInvalidBufferUsage,

    #[allow(missing_docs)]
    GetVideoDecodeRPROFILECountOutOfMemory,

    #[allow(missing_docs)]
    GetVideoDecodeRPROFILENullParam,

    #[allow(missing_docs)]
    GetVideoDecodeRPROFILEInvalidIndex,

    #[allow(missing_docs)]
    GetVideoDecodeRPROFILEOutOfMemoryReturn,

    #[allow(missing_docs)]
    CheckVideoDecodeRFormatNullParam,

    #[allow(missing_docs)]
    CheckVideoDecodeRFormatOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetVideoDecodeRCOnFIGCountNullParam,

    #[allow(missing_docs)]
    GetVideoDecodeRCOnFIGCountOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetVideoDecodeRCOnFIGNullParam,

    #[allow(missing_docs)]
    GetVideoDecodeRCOnFIGInvalidIndex,

    #[allow(missing_docs)]
    GetVideoDecodeRCOnFIGOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetDecodeRCREATIOnParamSNullParam,

    #[allow(missing_docs)]
    GetDecodeRDRIVERHAndLENullParam,

    #[allow(missing_docs)]
    GetDecodeRBufferNullParam,

    #[allow(missing_docs)]
    GetDecodeRBufferInvalidBuffer,

    #[allow(missing_docs)]
    GetDecodeRBufferInvalidType,

    #[allow(missing_docs)]
    GetDecodeRBufferLOCKED,

    #[allow(missing_docs)]
    RELEAsEDecodeRBufferNullParam,

    #[allow(missing_docs)]
    RELEAsEDecodeRBufferInvalidType,

    #[allow(missing_docs)]
    RELEAsEDecodeRBufferNotLOCKED,

    #[allow(missing_docs)]
    DecodeRbegInFrameNullParam,

    #[allow(missing_docs)]
    DecodeRbegInFrameHazard,

    #[allow(missing_docs)]
    DecodeREndFrameNullParam,

    #[allow(missing_docs)]
    SubMITDecodeRBuffersNullParam,

    #[allow(missing_docs)]
    SubMITDecodeRBuffersInvalidType,

    #[allow(missing_docs)]
    DecodeREXTENSIOnNullParam,

    #[allow(missing_docs)]
    DecodeREXTENSIOnInvalidResource,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorNullParam,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorInvalidFrameFormat,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorInvalidUsage,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorInvalidInputFrameRate,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorInvalidOutputFrameRate,

    #[allow(missing_docs)]
    CreateVideoProcessOrENumERATorInvalidWIDTHHEIGHT,

    #[allow(missing_docs)]
    GetVideoProcessOrCOnTENTDescNullParam,

    #[allow(missing_docs)]
    CheckVideoProcessOrFormatNullParam,

    #[allow(missing_docs)]
    GetVideoProcessOrCAPsNullParam,

    #[allow(missing_docs)]
    GetVideoProcessOrRateCOnVERsIOnCAPsNullParam,

    #[allow(missing_docs)]
    GetVideoProcessOrRateCOnVERsIOnCAPsInvalidIndex,

    #[allow(missing_docs)]
    GetVideoProcessOrCUSTomRateNullParam,

    #[allow(missing_docs)]
    GetVideoProcessOrCUSTomRateInvalidIndex,

    #[allow(missing_docs)]
    GetVideoProcessOrFilterRangeNullParam,

    #[allow(missing_docs)]
    GetVideoProcessOrFilterRangeUnsupported,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessOrNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputTargetRectNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputBackGROUNDColorNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputBackGROUNDColorInvalidAlpha,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputAlphaFillModeNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputAlphaFillModeUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputAlphaFillModeInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputAlphaFillModeInvalidFillMode,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputCOnSTRICTIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputSTEREOmODENullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputSTEREOmODEUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputEXTENSIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputTargetRectNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputBackGROUNDColorNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputAlphaFillModeNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputCOnSTRICTIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputCOnSTRICTIOnUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputCOnSTRICTIOnInvalidSize,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputSTEREOmODENullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputEXTENSIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFrameFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFrameFormatInvalidFormat,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFrameFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamColorSpaceInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamOutputRateNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamOutputRateInvalidRate,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamOutputRateInvalidFlag,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamOutputRateInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamsourceRectNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamsourceRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamsourceRectInvalidRect,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamDestRectNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamDestRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamDestRectInvalidRect,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamAlphaNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamAlphaInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamAlphaInvalidAlpha,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPaletteNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPaletteInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPaletteInvalidCount,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPaletteInvalidAlpha,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPixelAspectRationullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPixelAspectRatioInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPixelAspectRatioInvalidRatio,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamLumaKeyNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamLumaKeyInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamLumaKeyInvalidRange,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamLumaKeyUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatFLIPUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatMOnoOffsetUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatFormatUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamStereoFormatInvalidFormat,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamAutoProcessingModeNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamAutoProcessingModeInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFilterNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFilterInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFilterInvalidFilter,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFilterUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamFilterInvalidLevel,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamEXTENSIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamEXTENSIOnInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamFrameFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamOutputRateNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamsourceRectNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamDestRectNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamAlphaNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamPaletteNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamPixelAspectRationullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamLumaKeyNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamStereoFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamAutoProcessingModeNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamFilterNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamEXTENSIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamEXTENSIOnInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrBLTNullParam,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidStreamCount,

    #[allow(missing_docs)]
    VideoProcessOrBLTTargetRect,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidOutput,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidPastFrameS,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidFUTUREFrameS,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidSourceRect,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidDestRect,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidInputResource,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidArraySize,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidARRAY,

    #[allow(missing_docs)]
    VideoProcessOrBLTRIGHTEXPECTED,

    #[allow(missing_docs)]
    VideoProcessOrBLTRIGHTNotEXPECTED,

    #[allow(missing_docs)]
    VideoProcessOrBLTSTEREOnotENABLED,

    #[allow(missing_docs)]
    VideoProcessOrBLTInvalidRIGHTResource,

    #[allow(missing_docs)]
    VideoProcessOrBLTNoSTEREOStreams,

    #[allow(missing_docs)]
    VideoProcessOrBLTInputHazard,

    #[allow(missing_docs)]
    VideoProcessOrBLToutputHazard,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewNullParam,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewInvalidType,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewInvalidBind,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewUnsupportedFormat,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewInvalidMip,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewUNSupportEMip,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewInvalidArraySize,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewInvalidARRAY,

    #[allow(missing_docs)]
    CreateVideoDecodeROutputViewInvalidDimension,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewNullParam,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidType,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidBind,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidMisc,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidUsage,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidFormat,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidFOURCC,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidMip,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewUnsupportedMip,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidArraySize,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidARRAY,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidDimension,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewNullParam,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidType,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidBind,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidFormat,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidMip,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewUnsupportedMip,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewUnsupportedARRAY,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidARRAY,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidDimension,

    #[allow(missing_docs)]
    DeviceDrawInvalidUSEOffOrCEDSampleCount,

    #[allow(missing_docs)]
    CreateBlendStateInvalidLOGICOps,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidDARRAYWithDecodeR,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidDARRAYWithDecodeR,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidDARRAYWithDecodeR,

    #[allow(missing_docs)]
    DeviceLOCKEDOUTInterface,

    #[allow(missing_docs)]
    REFWarningATomICInCOnSISTENT,

    #[allow(missing_docs)]
    REFWarningReadInGUNInitialIZEDResource,

    #[allow(missing_docs)]
    REFWarningRAWHazard,

    #[allow(missing_docs)]
    REFWarningWARHazard,

    #[allow(missing_docs)]
    REFWarningWAWHazard,

    #[allow(missing_docs)]
    CreateCryptoSessionNullParam,

    #[allow(missing_docs)]
    CreateCryptoSessionoutOfMemoryReturn,

    #[allow(missing_docs)]
    GetCryptoTypeNullParam,

    #[allow(missing_docs)]
    GetDecodeRPROFILENullParam,

    #[allow(missing_docs)]
    GetCryptoSessionCERTIFICATESizeNullParam,

    #[allow(missing_docs)]
    GetCryptoSessionCERTIFICATENullParam,

    #[allow(missing_docs)]
    GetCryptoSessionCERTIFICATEWROnGSize,

    #[allow(missing_docs)]
    GetCryptoSessionHAndLEWROnGSize,

    #[allow(missing_docs)]
    NegotiateCRPYToSessionKeyExchangeNullParam,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTUnsupported,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTNullParam,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTSrcWROnGDevice,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTDstWROnGDevice,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTFormatMismatch,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTSizeMismatch,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTSrcMultiSampleD,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTDstNotSTAGInG,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTSrcMapped,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTDstMapped,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTSrcOffERED,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTDstoffERED,

    #[allow(missing_docs)]
    ENCRYPTIOnBLTSrcCOnTENTUndefined,

    #[allow(missing_docs)]
    DECRYPTIOnBLTUnsupported,

    #[allow(missing_docs)]
    DECRYPTIOnBLTNullParam,

    #[allow(missing_docs)]
    DECRYPTIOnBLTSrcWROnGDevice,

    #[allow(missing_docs)]
    DECRYPTIOnBLTDstWROnGDevice,

    #[allow(missing_docs)]
    DECRYPTIOnBLTFormatMismatch,

    #[allow(missing_docs)]
    DECRYPTIOnBLTSizeMismatch,

    #[allow(missing_docs)]
    DECRYPTIOnBLTDstMultiSampleD,

    #[allow(missing_docs)]
    DECRYPTIOnBLTSrcNotSTAGInG,

    #[allow(missing_docs)]
    DECRYPTIOnBLTDstNotRenderTarget,

    #[allow(missing_docs)]
    DECRYPTIOnBLTSrcMapped,

    #[allow(missing_docs)]
    DECRYPTIOnBLTDstMapped,

    #[allow(missing_docs)]
    DECRYPTIOnBLTSrcOffERED,

    #[allow(missing_docs)]
    DECRYPTIOnBLTDstoffERED,

    #[allow(missing_docs)]
    DECRYPTIOnBLTSrcCOnTENTUndefined,

    #[allow(missing_docs)]
    StartSessionKeyREFRESHNullParam,

    #[allow(missing_docs)]
    StartSessionKeyREFRESHInvalidSize,

    #[allow(missing_docs)]
    FinishSessionKeyREFRESHNullParam,

    #[allow(missing_docs)]
    GetENCRYPTIOnBLTKeyNullParam,

    #[allow(missing_docs)]
    GetENCRYPTIOnBLTKeyInvalidSize,

    #[allow(missing_docs)]
    GetCOnTENTProtectionCAPsNullParam,

    #[allow(missing_docs)]
    CheckCryptoKeyExchangeNullParam,

    #[allow(missing_docs)]
    CheckCryptoKeyExchangeInvalidIndex,

    #[allow(missing_docs)]
    CreateAuthenticatedChannelNullParam,

    #[allow(missing_docs)]
    CreateAuthenticatedChannelUnsupported,

    #[allow(missing_docs)]
    CreateAuthenticatedChannelInvalidType,

    #[allow(missing_docs)]
    CreateAuthenticatedChannelOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCERTIFICATESizeInvalidChannel,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCERTIFICATESizeNullParam,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCERTIFICATEInvalidChannel,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCERTIFICATENullParam,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCERTIFICATEWROnGSize,

    #[allow(missing_docs)]
    NegotiateAuthenticatedChannelKeyExchangeInvalidChannel,

    #[allow(missing_docs)]
    NegotiateAuthenticatedChannelKeyExchangeNullParam,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelNullParam,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelWROnGChannel,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelUnsupportedQuery,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelWROnGSize,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelInvalidProcessIndex,

    #[allow(missing_docs)]
    COnFIGUREAuthenticatedChannelNullParam,

    #[allow(missing_docs)]
    COnFIGUREAuthenticatedChannelWROnGChannel,

    #[allow(missing_docs)]
    COnFIGUREAuthenticatedChannelUnsupportedCOnFIGURE,

    #[allow(missing_docs)]
    COnFIGUREAuthenticatedChannelWROnGSize,

    #[allow(missing_docs)]
    COnFIGUREAuthenticatedChannelInvalidProcessIDType,

    #[allow(missing_docs)]
    VsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    DSSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    HSSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    GsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    PsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    CsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    NegotiateCRPYToSessionKeyExchangeInvalidSize,

    #[allow(missing_docs)]
    NegotiateAuthenticatedChannelKeyExchangeInvalidSize,

    #[allow(missing_docs)]
    OffERResourcesInvalidPRIOrITY,

    #[allow(missing_docs)]
    GetCryptoSessionHAndLEOutOfMemory,

    #[allow(missing_docs)]
    ACQUIREHAndLEForCAPTURENullParam,

    #[allow(missing_docs)]
    ACQUIREHAndLEForCAPTUREInvalidType,

    #[allow(missing_docs)]
    ACQUIREHAndLEForCAPTUREInvalidBind,

    #[allow(missing_docs)]
    ACQUIREHAndLEForCAPTUREInvalidARRAY,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamRotationNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamRotationInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamRotationInvalid,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamRotationUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamRotationNullParam,

    #[allow(missing_docs)]
    DeviceClearViewInvalidView,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderDoubleEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderShaderEXTENSIOnSNotSupported,

    #[allow(missing_docs)]
    DeviceShaderLinkageMInPRECISIOn,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamAlphaUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamPixelAspectRatioUnsupported,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderUAVsNotSupported,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsInvalidOffset,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsTooManyViews,

    #[allow(missing_docs)]
    DeviceClearViewNotSupported,

    #[allow(missing_docs)]
    SWAPDeviceContextStateNotSupported,

    #[allow(missing_docs)]
    UpdateSubresourcePREFERUpdateSubresource1,

    #[allow(missing_docs)]
    GetDCInAccessIBLE,

    #[allow(missing_docs)]
    DeviceClearViewInvalidRect,

    #[allow(missing_docs)]
    DeviceDrawSampleMaskIGNorEDOnFL9,

    #[allow(missing_docs)]
    DeviceOpENSharedResource1NotSupported,

    #[allow(missing_docs)]
    DeviceOpENSharedResourceBYNAMENotSupported,

    #[allow(missing_docs)]
    ENQUEUESetEVENTNotSupported,

    #[allow(missing_docs)]
    OffERRELEAsENotSupported,

    #[allow(missing_docs)]
    OffERResourcesInAccessIBLE,

    #[allow(missing_docs)]
    CreateVideoProcessOrInputViewInvalidMSAA,

    #[allow(missing_docs)]
    CreateVideoProcessOrOutputViewInvalidMSAA,

    #[allow(missing_docs)]
    DeviceClearViewInvalidSourceRect,

    #[allow(missing_docs)]
    DeviceClearViewEmptyRect,

    #[allow(missing_docs)]
    UpdateSubresourceEmptyDestBox,

    #[allow(missing_docs)]
    CopySubresourceRegionEmptySourceBox,

    #[allow(missing_docs)]
    DeviceDrawOmRenderTargetDOESNotSupportLOGICOps,

    #[allow(missing_docs)]
    DeviceDrawDepthstencilViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawRenderTargetViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawRenderTargetViewNotSetDUEToFLIPPRESENT,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewNotSetDUEToFLIPPRESENT,

    #[allow(missing_docs)]
    GetDataForNEWHardwareKeyNullParam,

    #[allow(missing_docs)]
    CheckCryptoSessionSTATUSNullParam,

    #[allow(missing_docs)]
    GetCryptoSessionPrivateDataSizeNullParam,

    #[allow(missing_docs)]
    GetVideoDecodeRCAPsNullParam,

    #[allow(missing_docs)]
    GetVideoDecodeRCAPsZEROWIDTHHEIGHT,

    #[allow(missing_docs)]
    CheckVideoDecodeRDownSAMPLInGNullParam,

    #[allow(missing_docs)]
    CheckVideoDecodeRDownSAMPLInGInvalidColorSpace,

    #[allow(missing_docs)]
    CheckVideoDecodeRDownSAMPLInGZEROWIDTHHEIGHT,

    #[allow(missing_docs)]
    VideoDecodeRENABLEDownSAMPLInGNullParam,

    #[allow(missing_docs)]
    VideoDecodeRENABLEDownSAMPLInGUnsupported,

    #[allow(missing_docs)]
    VideoDecodeRUpdateDownSAMPLInGNullParam,

    #[allow(missing_docs)]
    VideoDecodeRUpdateDownSAMPLInGUnsupported,

    #[allow(missing_docs)]
    CheckVideoProcessOrFormatCOnVERsIOnNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamColorSpace1InvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamMirrorNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamMirrorInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamMirrorUnsupported,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamMirrorNullParam,

    #[allow(missing_docs)]
    RECOmMEndVideoDecodeRDownSAMPLInGNullParam,

    #[allow(missing_docs)]
    RECOmMEndVideoDecodeRDownSAMPLInGInvalidColorSpace,

    #[allow(missing_docs)]
    RECOmMEndVideoDecodeRDownSAMPLInGZEROWIDTHHEIGHT,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputShaderUsageNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputShaderUsageNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetBEHAVIOrHInTSNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetBEHAVIOrHInTSInvalidStreamCount,

    #[allow(missing_docs)]
    VideoProcessOrGetBEHAVIOrHInTSTargetRect,

    #[allow(missing_docs)]
    VideoProcessOrGetBEHAVIOrHInTSInvalidSourceRect,

    #[allow(missing_docs)]
    VideoProcessOrGetBEHAVIOrHInTSInvalidDestRect,

    #[allow(missing_docs)]
    GetCryptoSessionPrivateDataSizeInvalidKeyExchangeType,

    #[allow(missing_docs)]
    DeviceOpENSharedResource1AccessDENIED,

    #[allow(missing_docs)]
    D3D111MessageSend,

    #[allow(missing_docs)]
    D3D112MessageSStart,

    #[allow(missing_docs)]
    CreateBufferInvalidUsage,

    #[allow(missing_docs)]
    CreateTexture1DInvalidUsage,

    #[allow(missing_docs)]
    CreateTexture2DInvalidUsage,

    #[allow(missing_docs)]
    CreateInputLayoutLevel9StepRateNot1,

    #[allow(missing_docs)]
    CreateInputLayoutLevel9InSTANCInGNotSupported,

    #[allow(missing_docs)]
    UpdateTileMappingsInvalidParameter,

    #[allow(missing_docs)]
    CopyTileMappingsInvalidParameter,

    #[allow(missing_docs)]
    CopyTilesInvalidParameter,

    #[allow(missing_docs)]
    UpdateTilesInvalidParameter,

    #[allow(missing_docs)]
    ResizeTilePoolInvalidParameter,

    #[allow(missing_docs)]
    TiledResourceBarrierInvalidParameter,

    #[allow(missing_docs)]
    NullTileMappingAccessWarning,

    #[allow(missing_docs)]
    NullTileMappingAccessError,

    #[allow(missing_docs)]
    DirtyTileMappingAccess,

    #[allow(missing_docs)]
    DuplicateTileMappingsInCoveredArea,

    #[allow(missing_docs)]
    TileMappingsInCoveredAreaDuplicatedOutside,

    #[allow(missing_docs)]
    TileMappingsSharedBetweenIncompatibleResources,

    #[allow(missing_docs)]
    TileMappingsSharedBetweenInputAndOutput,

    #[allow(missing_docs)]
    CheckMultiSampleQualityLevelsInvalidFlags,

    #[allow(missing_docs)]
    GetResourceTilingNonTiledResource,

    #[allow(missing_docs)]
    ResizeTilePoolShrinkWithMappingsStillDefinedPastEnd,

    #[allow(missing_docs)]
    NeedToCallTiledResourceBarrier,

    #[allow(missing_docs)]
    CreateDeviceInvalidArgs,

    #[allow(missing_docs)]
    CreateDeviceWarning,

    #[allow(missing_docs)]
    ClearUnorderedAccessViewUInTHazard,

    #[allow(missing_docs)]
    ClearUnorderedAccessViewFloatHazard,

    #[allow(missing_docs)]
    TiledResourceTier1BufferTextureMismatch,

    #[allow(missing_docs)]
    CreateCryptoSession,

    #[allow(missing_docs)]
    CreateAuthenticatedChannel,

    #[allow(missing_docs)]
    LiveCryptoSession,

    #[allow(missing_docs)]
    LiveAuthenticatedChannel,

    #[allow(missing_docs)]
    DestroyCryptoSession,

    #[allow(missing_docs)]
    DestroyAuthenticatedChannel,

    #[allow(missing_docs)]
    D3D112MessageSend,

    #[allow(missing_docs)]
    D3D113MessageSStart,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidConservativeRasterMode,

    #[allow(missing_docs)]
    DeviceDrawInvalidSystemValue,

    #[allow(missing_docs)]
    CreateQueryOrPredicateInvalidContextType,

    #[allow(missing_docs)]
    CreateQueryOrPredicateDecodeNotSupported,

    #[allow(missing_docs)]
    CreateQueryOrPredicateEncodeNotSupported,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidPlaneIndex,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidVideopLaneIndex,

    #[allow(missing_docs)]
    CreateShaderResourceViewAmbiguousVideopLaneIndex,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidPlaneIndex,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidVideopLaneIndex,

    #[allow(missing_docs)]
    CreateRenderTargetViewAmbiguousVideopLaneIndex,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidPlaneIndex,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidVideopLaneIndex,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewAmbiguousVideopLaneIndex,

    #[allow(missing_docs)]
    JpegDecodeInvalidScanDataOffset,

    #[allow(missing_docs)]
    JpegDecodeNotSupported,

    #[allow(missing_docs)]
    JpegDecodeDimensionsTooLarge,

    #[allow(missing_docs)]
    JpegDecodeInvalidComponents,

    #[allow(missing_docs)]
    JpegDecodeDestinationNot2D,

    #[allow(missing_docs)]
    JpegDecodeTiledResourcesUnsupported,

    #[allow(missing_docs)]
    JpegDecodeGuardRectsUnsupported,

    #[allow(missing_docs)]
    JpegDecodeFormatUnsupported,

    #[allow(missing_docs)]
    JpegDecodeInvalidSubresource,

    #[allow(missing_docs)]
    JpegDecodeInvalidMipLevel,

    #[allow(missing_docs)]
    JpegDecodeEmptyDestBox,

    #[allow(missing_docs)]
    JpegDecodeDestBoxNot2D,

    #[allow(missing_docs)]
    JpegDecodeDestBoxNotSub,

    #[allow(missing_docs)]
    JpegDecodeDestBoxesIntersect,

    #[allow(missing_docs)]
    JpegDecodeXSubSampleMismatch,

    #[allow(missing_docs)]
    JpegDecodeYSubSampleMismatch,

    #[allow(missing_docs)]
    JpegDecodeXSubSampleOdd,

    #[allow(missing_docs)]
    JpegDecodeYSubSampleOdd,

    #[allow(missing_docs)]
    JpegDecodeOutputDimensionsTooLarge,

    #[allow(missing_docs)]
    JpegDecodeNonPow2ScaleUnsupported,

    #[allow(missing_docs)]
    JpegDecodeFractionalDownscaleToLarge,

    #[allow(missing_docs)]
    JpegDecodeChromaSizeMismatch,

    #[allow(missing_docs)]
    JpegDecodeLumaChromaSizeMismatch,

    #[allow(missing_docs)]
    JpegDecodeInvalidNumDestinationS,

    #[allow(missing_docs)]
    JpegDecodeSubBoxUnsupported,

    #[allow(missing_docs)]
    JpegDecode1DestUnsupportedFormat,

    #[allow(missing_docs)]
    JpegDecode3DestUnsupportedFormat,

    #[allow(missing_docs)]
    JpegDecodeScaleUnsupported,

    #[allow(missing_docs)]
    JpegDecodeInvalidSourceSize,

    #[allow(missing_docs)]
    JpegDecodeInvalidCopyFlags,

    #[allow(missing_docs)]
    JpegDecodeHazard,

    #[allow(missing_docs)]
    JpegDecodeUnsupportedSrcBufferUsage,

    #[allow(missing_docs)]
    JpegDecodeUnsupportedSrcBufferMiscFlags,

    #[allow(missing_docs)]
    JpegDecodeUnsupportedDstTextureUsage,

    #[allow(missing_docs)]
    JpegDecodeBackBufferNotSupported,

    #[allow(missing_docs)]
    JpegDecodeUnsupprtedCopyFlags,

    #[allow(missing_docs)]
    JpegEncodeNotSupported,

    #[allow(missing_docs)]
    JpegEncodeInvalidScanDataOffset,

    #[allow(missing_docs)]
    JpegEncodeInvalidComponents,

    #[allow(missing_docs)]
    JpegEncodeSourceNot2D,

    #[allow(missing_docs)]
    JpegEncodeTiledResourcesUnsupported,

    #[allow(missing_docs)]
    JpegEncodeGuardRectsUnsupported,

    #[allow(missing_docs)]
    JpegEncodeXSubSampleMismatch,

    #[allow(missing_docs)]
    JpegEncodeYSubSampleMismatch,

    #[allow(missing_docs)]
    JpegEncodeFormatUnsupported,

    #[allow(missing_docs)]
    JpegEncodeInvalidSubresource,

    #[allow(missing_docs)]
    JpegEncodeInvalidMipLevel,

    #[allow(missing_docs)]
    JpegEncodeDimensionsTooLarge,

    #[allow(missing_docs)]
    JpegEncodeHazard,

    #[allow(missing_docs)]
    JpegEncodeUnsupportedDstBufferUsage,

    #[allow(missing_docs)]
    JpegEncodeUnsupportedDstBufferMiscFlags,

    #[allow(missing_docs)]
    JpegEncodeUnsupportedSrcTextureUsage,

    #[allow(missing_docs)]
    JpegEncodeBackBufferNotSupported,

    #[allow(missing_docs)]
    CreateQueryOrPredicateUnsupportedContextTTypeForQuery,

    #[allow(missing_docs)]
    Flush1InvalidContextType,

    #[allow(missing_docs)]
    DeviceSetHardwareProtectionInvalidContext,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetOutputHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetOutputHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamHdrMetadataInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrSetStreamHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamHdrMetadataInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamFrameFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamColorSpaceInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamOutputRateInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamsourceRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamDestRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamAlphaInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamPaletteInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamPixelAspectRatioInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamLumaKeyInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamStereoFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamAutoProcessingModeInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamFilterInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamRotationInvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamColorSpace1InvalidStream,

    #[allow(missing_docs)]
    VideoProcessOrGetStreamMirrorInvalidStream,

    #[allow(missing_docs)]
    CreateFence,

    #[allow(missing_docs)]
    LiveFence,

    #[allow(missing_docs)]
    DestroyFence,

    #[allow(missing_docs)]
    CreateSynchronizedChannel,

    #[allow(missing_docs)]
    LiveSynchronizedChannel,

    #[allow(missing_docs)]
    DestroySynchronizedChannel,

    #[allow(missing_docs)]
    CreateFenceInvalidFlags,

    #[allow(missing_docs)]
    D3D113MessageSend,

    #[allow(missing_docs)]
    D3D115MessageSStart,

    #[allow(missing_docs)]
    NegotiateCryptoSessionKeyExchangeMtInvalidKeyExchangeType,

    #[allow(missing_docs)]
    NegotiateCryptoSessionKeyExchangeMtNotSupported,

    #[allow(missing_docs)]
    DecodeRbegInFrameInvalidHistogramComponentCount,

    #[allow(missing_docs)]
    DecodeRbegInFrameInvalidHistogramComponent,

    #[allow(missing_docs)]
    DecodeRbegInFrameInvalidHistogramBufferSize,

    #[allow(missing_docs)]
    DecodeRbegInFrameInvalidHistogramBufferUsage,

    #[allow(missing_docs)]
    DecodeRbegInFrameInvalidHistogramBufferMiscFlags,

    #[allow(missing_docs)]
    DecodeRbegInFrameInvalidHistogramBufferOffset,

    #[allow(missing_docs)]
    CreateTrackedWorkload,

    #[allow(missing_docs)]
    LiveTrackedWorkload,

    #[allow(missing_docs)]
    DestroyTrackedWorkload,

    #[allow(missing_docs)]
    CreateTrackedWorkloadNullParam,

    #[allow(missing_docs)]
    CreateTrackedWorkloadInvalidMaxInstanceS,

    #[allow(missing_docs)]
    CreateTrackedWorkloadInvalidDeadlineType,

    #[allow(missing_docs)]
    CreateTrackedWorkloadInvalidEngineType,

    #[allow(missing_docs)]
    MutipleTrackedWorkloads,

    #[allow(missing_docs)]
    MutipleTrackedWorkloadPairs,

    #[allow(missing_docs)]
    InCOmPLETETrackedWorkloadPair,

    #[allow(missing_docs)]
    OutOfOrderTrackedWorkloadPair,

    #[allow(missing_docs)]
    CannotAddTrackedWorkload,

    #[allow(missing_docs)]
    TrackedWorkloadNotSupported,

    #[allow(missing_docs)]
    TrackedWorkloadEngineTypeNotFound,

    #[allow(missing_docs)]
    NoTrackedWorkloadSlotAvailable,

    #[allow(missing_docs)]
    EndTrackedWorkloadInvalidArg,

    #[allow(missing_docs)]
    TrackedWorkloadDisjointFailure,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatAndWriteMaskMismatch,

    #[allow(missing_docs)]
    D3D115MessageSend,
}
