// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11sdklayers::{ID3D11InfoQueue, D3D11_INFO_QUEUE_FILTER};

/// Debug messages for setting up an info-queue filter (see [`D3D11_INFO_QUEUE_FILTER`]); use these
/// messages to allow or deny message categories to pass through the storage and retrieval filters.
/// These IDs are used by methods such as [`ID3D11InfoQueue::get_message`] or
/// [`ID3D11InfoQueue::add_message`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_MESSAGE_ID {
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
    CreateInputLayoutTooManyElementS,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidFormat,

    #[allow(missing_docs)]
    CreateInputLayoutIncompatibleFormat,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidSlot,

    #[allow(missing_docs)]
    CreateInputLayoutInvalidInputSlotClass,

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
    CreateInputLayoutUnparseableInputsignature,

    #[allow(missing_docs)]
    CreateInputLayoutNullSemantic,

    #[allow(missing_docs)]
    CreateInputLayoutMissingElement,

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
    CreateGeometryShaderWithStreamOutputOutputStreamStrideUnused,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputUnexpectedDecl,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputExpectedDecl,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputOutputSlot0Expected,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidOutputSlot,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputOnlyOneElementPerSlot,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidComponentCount,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidStartComponentAndComponentCount,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidGapDefinitionn,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputRepeatedOutput,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputInvalidOutputStreamStride,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputMissingsemantic,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputMaskMismatch,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputCantHaveOnlyGaps,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputDeclTooComplex,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputMissingOutputSignature,

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
    CreateSamplerStateInvalidFilter,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidAddressU,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidAddressV,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidAddressW,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidMipLodBias,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidMaxAnisotropy,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidComparisonFunc,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidMinLod,

    #[allow(missing_docs)]
    CreateSamplerStateInvalidMaxLod,

    #[allow(missing_docs)]
    CreateSamplerStateTooManyObjects,

    #[allow(missing_docs)]
    CreateSamplerStateNullDesc,

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
    DeviceResolveSubresourceDestinationInvalid,

    #[allow(missing_docs)]
    DeviceResolveSubresourceDestinationSubresourceInvalid,

    #[allow(missing_docs)]
    DeviceResolveSubresourceSourceInvalid,

    #[allow(missing_docs)]
    DeviceResolveSubresourceSourceSubresourceInvalid,

    #[allow(missing_docs)]
    DeviceResolveSubresourceFormatInvalid,

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
    RefSimulatingInfinitelyFastHardware,

    #[allow(missing_docs)]
    RefThreadingMode,

    #[allow(missing_docs)]
    RefUmDriverException,

    #[allow(missing_docs)]
    RefKmDriverException,

    #[allow(missing_docs)]
    RefHardwareException,

    #[allow(missing_docs)]
    RefAccessingIndexableTempOutOfRange,

    #[allow(missing_docs)]
    RefProblemParsInGShader,

    #[allow(missing_docs)]
    RefOutOfMemory,

    #[allow(missing_docs)]
    RefInFo,

    #[allow(missing_docs)]
    DeviceDrawVertexPosOverflow,

    #[allow(missing_docs)]
    DeviceDrawIndexedIndexPosOverflow,

    #[allow(missing_docs)]
    DeviceDrawInstancedVertexPosOverflow,

    #[allow(missing_docs)]
    DeviceDrawInstancedInstancePosOverflow,

    #[allow(missing_docs)]
    DeviceDrawIndexedInstancedInstancePosOverflow,

    #[allow(missing_docs)]
    DeviceDrawIndexedInstancedIndexPosOverflow,

    #[allow(missing_docs)]
    DeviceDrawVertexShaderNotSet,

    #[allow(missing_docs)]
    DeviceShaderLinkageSemanticNameNotFound,

    #[allow(missing_docs)]
    DeviceShaderLinkageRegisterIndex,

    #[allow(missing_docs)]
    DeviceShaderLinkageComponentType,

    #[allow(missing_docs)]
    DeviceShaderLinkageRegisterMask,

    #[allow(missing_docs)]
    DeviceShaderLinkageSystemValue,

    #[allow(missing_docs)]
    DeviceShaderLinkageNeverWrittenAlwaysReads,

    #[allow(missing_docs)]
    DeviceDrawVertexBufferNotSet,

    #[allow(missing_docs)]
    DeviceDrawInputLayoutNotSet,

    #[allow(missing_docs)]
    DeviceDrawConstantBufferNotSet,

    #[allow(missing_docs)]
    DeviceDrawConstantBufferTooSmall,

    #[allow(missing_docs)]
    DeviceDrawSamplerNotSet,

    #[allow(missing_docs)]
    DeviceDrawShaderResourceViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawViewDimensionMismatch,

    #[allow(missing_docs)]
    DeviceDrawVertexBufferStrideTooSmall,

    #[allow(missing_docs)]
    DeviceDrawVertexBufferTooSmall,

    #[allow(missing_docs)]
    DeviceDrawIndexBufferNotSet,

    #[allow(missing_docs)]
    DeviceDrawIndexBufferFormatInvalid,

    #[allow(missing_docs)]
    DeviceDrawIndexBufferTooSmall,

    #[allow(missing_docs)]
    DeviceDrawGsInputPrimitiveMismatch,

    #[allow(missing_docs)]
    DeviceDrawResourceReturnTypeMismatch,

    #[allow(missing_docs)]
    DeviceDrawPositionNotPresent,

    #[allow(missing_docs)]
    DeviceDrawOutputStreamNotSet,

    #[allow(missing_docs)]
    DeviceDrawBoundResourceMapped,

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
    DeviceDrawResourceFormatLdUnsupported,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatSampleUnsupported,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatSampleCUnsupported,

    #[allow(missing_docs)]
    DeviceDrawResourceMultiSampleUnsupported,

    #[allow(missing_docs)]
    DeviceDrawSoTargetsBoundWithoutSource,

    #[allow(missing_docs)]
    DeviceDrawSoStrideLargerThanBuffer,

    #[allow(missing_docs)]
    DeviceDrawOmRenderTargetDoesNotSupportBlendInG,

    #[allow(missing_docs)]
    DeviceDrawOmDualSourceBlendingCanonlyHaveRenderTarget0,

    #[allow(missing_docs)]
    DeviceRemovalProcessAtFault,

    #[allow(missing_docs)]
    DeviceRemovalProcessPossiblyAtFault,

    #[allow(missing_docs)]
    DeviceRemovalProcessNotAtFault,

    #[allow(missing_docs)]
    DeviceOpenSharedResourceInvalidArgReturn,

    #[allow(missing_docs)]
    DeviceOpenSharedResourceOutOfMemoryReturn,

    #[allow(missing_docs)]
    DeviceOpenSharedResourceBadInterfaceReturn,

    #[allow(missing_docs)]
    DeviceDrawViewportNotSet,

    #[allow(missing_docs)]
    CreateInputLayoutTrailingDigitInSemantic,

    #[allow(missing_docs)]
    CreateGeometryShaderWithStreamOutputTrailingDigitInSemantic,

    #[allow(missing_docs)]
    DeviceRsSetViewportsDenormFlush,

    #[allow(missing_docs)]
    OmSetRenderTargetsInvalidView,

    #[allow(missing_docs)]
    DeviceSetTextFilterSizeInvalidDimensions,

    #[allow(missing_docs)]
    DeviceDrawSamplerMismatch,

    #[allow(missing_docs)]
    CreateInputLayoutTypeMismatch,

    #[allow(missing_docs)]
    BlendStateGetDescLegacy,

    #[allow(missing_docs)]
    ShaderResourceViewGetDescLegacy,

    #[allow(missing_docs)]
    CreateQueryOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreatePredicateOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateCounterOutOfRangeCounter,

    #[allow(missing_docs)]
    CreateCounterSimultaneousActiveCountersExhausted,

    #[allow(missing_docs)]
    CreateCounterUnsupportedWellKnownCounter,

    #[allow(missing_docs)]
    CreateCounterOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateCounterNonExclusiveReturn,

    #[allow(missing_docs)]
    CreateCounterNullDesc,

    #[allow(missing_docs)]
    CheckCounterOutOfRangeCounter,

    #[allow(missing_docs)]
    CheckCounterUnsupportedWellKnownCounter,

    #[allow(missing_docs)]
    SetPredicationInvalidPredicateState,

    #[allow(missing_docs)]
    QueryBeginUnsupported,

    #[allow(missing_docs)]
    PredicateBeginDuringPredication,

    #[allow(missing_docs)]
    QueryBeginDuplicate,

    #[allow(missing_docs)]
    QueryBeginAbandoningPreviousResults,

    #[allow(missing_docs)]
    PredicateEndDuringPredication,

    #[allow(missing_docs)]
    QueryEndAbandoningPreviousResults,

    #[allow(missing_docs)]
    QueryEndWithoutBegin,

    #[allow(missing_docs)]
    QueryGetDataInvalidDataSize,

    #[allow(missing_docs)]
    QueryGetDataInvalidFlags,

    #[allow(missing_docs)]
    QueryGetDataInvalidCall,

    #[allow(missing_docs)]
    DeviceDrawPsOutputTypeMismatch,

    #[allow(missing_docs)]
    DeviceDrawResourceFormatGatherUnsupported,

    #[allow(missing_docs)]
    DeviceDrawInvalidUseOfCenterMultiSamplePattern,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersStrideTooLarge,

    #[allow(missing_docs)]
    DeviceIaSetVertexBuffersInvalidRange,

    #[allow(missing_docs)]
    CreateInputLayoutEmptyLayout,

    #[allow(missing_docs)]
    DeviceDrawResourceSampleCountMismatch,

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
    LiveSampler,

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
    CreateSamplerStateNoComparisonSupport,

    #[allow(missing_docs)]
    CreateSamplerStateExcessiveAnisotropy,

    #[allow(missing_docs)]
    CreateSamplerStateBorderOutOfRange,

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
    CopyResourceOnlyTexture2DWithInGpuMemory,

    #[allow(missing_docs)]
    CopyResourceNoTexture3DReadBack,

    #[allow(missing_docs)]
    CopyResourceNoTextureOnlyReadBack,

    #[allow(missing_docs)]
    CreateInputLayoutUnsupportedFormat,

    #[allow(missing_docs)]
    CreateBlendStateNoAlphaToCoverage,

    #[allow(missing_docs)]
    CreateRasterizerStateDepthClipEnableMustBeTrue,

    #[allow(missing_docs)]
    DrawIndexedStartIndexLocationMustBePositive,

    #[allow(missing_docs)]
    CreateShaderResourceViewMustUseLowestLod,

    #[allow(missing_docs)]
    CreateSamplerStateMinLodMustNotBeFractional,

    #[allow(missing_docs)]
    CreateSamplerStateMaxLodMustBeFltMax,

    #[allow(missing_docs)]
    CreateShaderResourceViewFirstArraySliceMustBeZero,

    #[allow(missing_docs)]
    CreateShaderResourceViewCubesMustHave6Sides,

    #[allow(missing_docs)]
    CreateResourceNotBindableAsRenderTarget,

    #[allow(missing_docs)]
    CreateResourceNoDwordIndexBuffer,

    #[allow(missing_docs)]
    CreateResourceMsaaPrecludesShaderResource,

    #[allow(missing_docs)]
    CreateResourcePresentationPrecludesShaderResource,

    #[allow(missing_docs)]
    CreateBlendStateNoIndependentBlendEnable,

    #[allow(missing_docs)]
    CreateBlendStateNoIndependentWriteMasks,

    #[allow(missing_docs)]
    CreateResourceNoStreamOut,

    #[allow(missing_docs)]
    CreateResourceOnlyVBIBForBuffers,

    #[allow(missing_docs)]
    CreateResourceNoAutoGenForVolumes,

    #[allow(missing_docs)]
    CreateResourceDxgiFormatR8G8B8A8CannotBeShared,

    #[allow(missing_docs)]
    VsShaderResourcesNotSupported,

    #[allow(missing_docs)]
    GeometryShaderNotSupported,

    #[allow(missing_docs)]
    StreamOutNotSupported,

    #[allow(missing_docs)]
    TextFilterNotSupported,

    #[allow(missing_docs)]
    CreateBlendStateNoSeparateAlphaBlend,

    #[allow(missing_docs)]
    CreateBlendStateNoMrtBlend,

    #[allow(missing_docs)]
    CreateBlendStateOperationNotSupported,

    #[allow(missing_docs)]
    CreateSamplerStateNoMirrorOnce,

    #[allow(missing_docs)]
    DrawInstancedNotSupported,

    #[allow(missing_docs)]
    DrawIndexedInstancedNotSupportedBelow93,

    #[allow(missing_docs)]
    DrawIndexedPointListUnsupported,

    #[allow(missing_docs)]
    SetBlendStateSampleMaskCannotBeZero,

    #[allow(missing_docs)]
    CreateResourceDimensionExceedsFeatureLevelDefinitionn,

    #[allow(missing_docs)]
    CreateResourceOnlySingleMipLevelDepthstencilSupported,

    #[allow(missing_docs)]
    DeviceRsSetScissorRectsNegativeScissor,

    #[allow(missing_docs)]
    SlotZeroMustBeD3D10InputPerVertexData,

    #[allow(missing_docs)]
    CreateResourceNonPow2MipMap,

    #[allow(missing_docs)]
    CreateSamplerStateBorderNotSupported,

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
    DeviceHsSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceHsSetConstantBuffersHazard,

    #[allow(missing_docs)]
    HsSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    HsSetConstantBuffersUnbindDeletingObject,

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
    DeviceHsSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    HsSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceHsSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceHsSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceHsGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceHsGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceHsGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceDsSetShaderResourcesHazard,

    #[allow(missing_docs)]
    DeviceDsSetConstantBuffersHazard,

    #[allow(missing_docs)]
    DsSetShaderResourcesUnbindDeletingObject,

    #[allow(missing_docs)]
    DsSetConstantBuffersUnbindDeletingObject,

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
    DeviceDsSetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DsSetConstantBuffersInvalidBuffer,

    #[allow(missing_docs)]
    DeviceDsSetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceDsSetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceDsGetShaderResourcesViewsEmpty,

    #[allow(missing_docs)]
    DeviceDsGetConstantBuffersBuffersEmpty,

    #[allow(missing_docs)]
    DeviceDsGetSamplersSamplersEmpty,

    #[allow(missing_docs)]
    DeviceDrawHsXorDsMismatch,

    #[allow(missing_docs)]
    DeferredContextRemovalProcessAtFault,

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
    CreateSampler,

    #[allow(missing_docs)]
    LiveSamplerWin7,

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
    DeviceSetShaderInvalidInstancedata,

    #[allow(missing_docs)]
    DeviceSetShaderUnboundInstancedata,

    #[allow(missing_docs)]
    DeviceSetShaderInstancedataBindings,

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
    DeviceUnorderedAccessViewAppendUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicsUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewStructureStrideMismatch,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewBufferTypeMismatch,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewRawUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewFormatLdUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewFormatStoreUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicAddUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicBitwiseOpsUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicCmpStoreCmpExchangeUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicExchangeUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicSignedMinMaxUnsupported,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewAtomicUnsignedMinMaxUnsupported,

    #[allow(missing_docs)]
    DeviceDispatchBoundResourceMapped,

    #[allow(missing_docs)]
    DeviceDispatchThreadGroupCountOverflow,

    #[allow(missing_docs)]
    DeviceDispatchThreadGroupCountZero,

    #[allow(missing_docs)]
    DeviceShaderResourceViewStructureStrideMismatch,

    #[allow(missing_docs)]
    DeviceShaderResourceViewBufferTypeMismatch,

    #[allow(missing_docs)]
    DeviceShaderResourceViewRawUnsupported,

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
    RefWarning,

    #[allow(missing_docs)]
    DeviceDrawPixelShaderWithoutRtvOrDsv,

    #[allow(missing_docs)]
    ShaderAbort,

    #[allow(missing_docs)]
    ShaderMessage,

    #[allow(missing_docs)]
    ShaderError,

    #[allow(missing_docs)]
    OfferResourcesInvalidResource,

    #[allow(missing_docs)]
    HsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    DsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    CsSetSamplersUnbindDeletingObject,

    #[allow(missing_docs)]
    HsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    DsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    CsSetShaderUnbindDeletingObject,

    #[allow(missing_docs)]
    EnqueueSetEventInvalidArgReturn,

    #[allow(missing_docs)]
    EnqueueSetEventoutOfMemoryReturn,

    #[allow(missing_docs)]
    EnqueueSetEventAccessDeniedReturn,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsNumUavsInvalidRange,

    #[allow(missing_docs)]
    UseOfZeroRefCountObject,

    #[allow(missing_docs)]
    D3D11MessageSend,

    #[allow(missing_docs)]
    D3D111MessagesStart = 0x300000,

    #[allow(missing_docs)]
    CreateVideoDecoder,

    #[allow(missing_docs)]
    CreateVideoProcessorEnum,

    #[allow(missing_docs)]
    CreateVideoProcessor,

    #[allow(missing_docs)]
    CreateDecoderOutputView,

    #[allow(missing_docs)]
    CreateProcessorInputView,

    #[allow(missing_docs)]
    CreateProcessorOutputView,

    #[allow(missing_docs)]
    CreateDeviceContextState,

    #[allow(missing_docs)]
    LiveVideoDecoder,

    #[allow(missing_docs)]
    LiveVideoProcessorEnum,

    #[allow(missing_docs)]
    LiveVideoProcessor,

    #[allow(missing_docs)]
    LiveDecoderOutputView,

    #[allow(missing_docs)]
    LiveProcessorInputView,

    #[allow(missing_docs)]
    LiveProcessorOutputView,

    #[allow(missing_docs)]
    LiveDeviceContextState,

    #[allow(missing_docs)]
    DestroyVideoDecoder,

    #[allow(missing_docs)]
    DestroyVideoProcessorEnum,

    #[allow(missing_docs)]
    DestroyVideoProcessor,

    #[allow(missing_docs)]
    DestroyDecoderOutputView,

    #[allow(missing_docs)]
    DestroyProcessorInputView,

    #[allow(missing_docs)]
    DestroyProcessorOutputView,

    #[allow(missing_docs)]
    DestroyDeviceContextState,

    #[allow(missing_docs)]
    CreateDeviceContextStateInvalidFlags,

    #[allow(missing_docs)]
    CreateDeviceContextStateInvalidFeatureLevel,

    #[allow(missing_docs)]
    CreateDeviceContextStateFeatureLevelsNotSupported,

    #[allow(missing_docs)]
    CreateDeviceContextStateInvalidRefIid,

    #[allow(missing_docs)]
    DeviceDiscardViewInvalidView,

    #[allow(missing_docs)]
    CopySubresourceRegion1InvalidCopyFlags,

    #[allow(missing_docs)]
    UpdateSubresource1InvalidCopyFlags,

    #[allow(missing_docs)]
    CreateRasterizerStateInvalidForCedSampleCount,

    #[allow(missing_docs)]
    CreateVideoDecoderOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoDecoderNullParam,

    #[allow(missing_docs)]
    CreateVideoDecoderInvalidFormat,

    #[allow(missing_docs)]
    CreateVideoDecoderZeroWidthHeight,

    #[allow(missing_docs)]
    CreateVideoDecoderDriverInvalidBufferSize,

    #[allow(missing_docs)]
    CreateVideoDecoderDriverInvalidBufferUsage,

    #[allow(missing_docs)]
    GetVideoDecoderProfileCountOutOfMemory,

    #[allow(missing_docs)]
    GetVideoDecoderProfileNullParam,

    #[allow(missing_docs)]
    GetVideoDecoderProfileInvalidIndex,

    #[allow(missing_docs)]
    GetVideoDecoderProfileOutOfMemoryReturn,

    #[allow(missing_docs)]
    CheckVideoDecoderFormatNullParam,

    #[allow(missing_docs)]
    CheckVideoDecoderFormatOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetVideoDecoderConfigCountNullParam,

    #[allow(missing_docs)]
    GetVideoDecoderConfigCountOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetVideoDecoderConfigNullParam,

    #[allow(missing_docs)]
    GetVideoDecoderConfigInvalidIndex,

    #[allow(missing_docs)]
    GetVideoDecoderConfigOutOfMemoryReturn,

    #[allow(missing_docs)]
    GetDecoderCreationParamsNullParam,

    #[allow(missing_docs)]
    GetDecoderDriverHandleNullParam,

    #[allow(missing_docs)]
    GetDecoderBufferNullParam,

    #[allow(missing_docs)]
    GetDecoderBufferInvalidBuffer,

    #[allow(missing_docs)]
    GetDecoderBufferInvalidType,

    #[allow(missing_docs)]
    GetDecoderBufferLocked,

    #[allow(missing_docs)]
    ReleaseDecoderBufferNullParam,

    #[allow(missing_docs)]
    ReleaseDecoderBufferInvalidType,

    #[allow(missing_docs)]
    ReleaseDecoderBufferNotLocked,

    #[allow(missing_docs)]
    DecoderbegInFrameNullParam,

    #[allow(missing_docs)]
    DecoderbegInFrameHazard,

    #[allow(missing_docs)]
    DecoderEndFrameNullParam,

    #[allow(missing_docs)]
    SubmitDecoderBuffersNullParam,

    #[allow(missing_docs)]
    SubmitDecoderBuffersInvalidType,

    #[allow(missing_docs)]
    DecoderExtensionNullParam,

    #[allow(missing_docs)]
    DecoderExtensionInvalidResource,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorNullParam,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorInvalidFrameFormat,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorInvalidUsage,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorInvalidInputFrameRate,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorInvalidOutputFrameRate,

    #[allow(missing_docs)]
    CreateVideoProcessorEnumeratorInvalidWidthHeight,

    #[allow(missing_docs)]
    GetVideoProcessorContentDescNullParam,

    #[allow(missing_docs)]
    CheckVideoProcessorFormatNullParam,

    #[allow(missing_docs)]
    GetVideoProcessorCapsNullParam,

    #[allow(missing_docs)]
    GetVideoProcessorRateConversionCapsNullParam,

    #[allow(missing_docs)]
    GetVideoProcessorRateConversionCapsInvalidIndex,

    #[allow(missing_docs)]
    GetVideoProcessorCustomRateNullParam,

    #[allow(missing_docs)]
    GetVideoProcessorCustomRateInvalidIndex,

    #[allow(missing_docs)]
    GetVideoProcessorFilterRangeNullParam,

    #[allow(missing_docs)]
    GetVideoProcessorFilterRangeUnsupported,

    #[allow(missing_docs)]
    CreateVideoProcessorOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessorNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputTargetRectNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputBackgroundColorNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputBackgroundColorInvalidAlpha,

    #[allow(missing_docs)]
    VideoProcessorSetOutputColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputAlphaFillModeNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputAlphaFillModeUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetOutputAlphaFillModeInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetOutputAlphaFillModeInvalidFillMode,

    #[allow(missing_docs)]
    VideoProcessorSetOutputConstrictionNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputStereoModeNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputStereoModeUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetOutputExtensionNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputTargetRectNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputBackgroundColorNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputAlphaFillModeNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputConstrictionNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputConstrictionUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetOutputConstrictionInvalidSize,

    #[allow(missing_docs)]
    VideoProcessorGetOutputStereoModeNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputExtensionNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFrameFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFrameFormatInvalidFormat,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFrameFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamColorSpaceInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamOutputRateNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamOutputRateInvalidRate,

    #[allow(missing_docs)]
    VideoProcessorSetStreamOutputRateInvalidFlag,

    #[allow(missing_docs)]
    VideoProcessorSetStreamOutputRateInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamSourceRectNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamSourceRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamSourceRectInvalidRect,

    #[allow(missing_docs)]
    VideoProcessorSetStreamDestRectNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamDestRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamDestRectInvalidRect,

    #[allow(missing_docs)]
    VideoProcessorSetStreamAlphaNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamAlphaInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamAlphaInvalidAlpha,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPaletteNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPaletteInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPaletteInvalidCount,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPaletteInvalidAlpha,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPixelAspectRationullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPixelAspectRatioInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPixelAspectRatioInvalidRatio,

    #[allow(missing_docs)]
    VideoProcessorSetStreamLumaKeyNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamLumaKeyInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamLumaKeyInvalidRange,

    #[allow(missing_docs)]
    VideoProcessorSetStreamLumaKeyUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatFlipUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatMonoOffsetUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatFormatUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamStereoFormatInvalidFormat,

    #[allow(missing_docs)]
    VideoProcessorSetStreamAutoProcessingModeNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamAutoProcessingModeInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFilterNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFilterInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFilterInvalidFilter,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFilterUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamFilterInvalidLevel,

    #[allow(missing_docs)]
    VideoProcessorSetStreamExtensionNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamExtensionInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamFrameFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamColorSpaceNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamOutputRateNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamSourceRectNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamDestRectNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamAlphaNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamPaletteNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamPixelAspectRationullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamLumaKeyNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamStereoFormatNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamAutoProcessingModeNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamFilterNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamExtensionNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamExtensionInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorBltNullParam,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidStreamCount,

    #[allow(missing_docs)]
    VideoProcessorBltTargetRect,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidOutput,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidPastFrames,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidFutureFrames,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidSourceRect,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidDestRect,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidInputResource,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidArraySize,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidArray,

    #[allow(missing_docs)]
    VideoProcessorBltRightExpected,

    #[allow(missing_docs)]
    VideoProcessorBltRightNotExpected,

    #[allow(missing_docs)]
    VideoProcessorBltStereoNotEnabled,

    #[allow(missing_docs)]
    VideoProcessorBltInvalidRightResource,

    #[allow(missing_docs)]
    VideoProcessorBltNoStereoStreams,

    #[allow(missing_docs)]
    VideoProcessorBltInputHazard,

    #[allow(missing_docs)]
    VideoProcessorBltOutputHazard,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewNullParam,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewInvalidType,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewInvalidBind,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewUnsupportedFormat,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewInvalidMip,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewUnsupportedMip,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewInvalidArraySize,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewInvalidArray,

    #[allow(missing_docs)]
    CreateVideoDecoderOutputViewInvalidDimension,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewNullParam,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidType,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidBind,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidMisc,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidUsage,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidFormat,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidFourCc,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidMip,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewUnsupportedMip,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidArraySize,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidArray,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidDimension,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewOutOfMemoryReturn,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewNullParam,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidType,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidBind,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidFormat,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidMip,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewUnsupportedMip,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewUnsupportedArray,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidArray,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidDimension,

    #[allow(missing_docs)]
    DeviceDrawInvalidUseOffOrCedsampleCount,

    #[allow(missing_docs)]
    CreateBlendStateInvalidLogicOps,

    #[allow(missing_docs)]
    CreateShaderResourceViewInvalidDArrayWithDecoder,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidDArrayWithDecoder,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidDArrayWithDecoder,

    #[allow(missing_docs)]
    DeviceLockedOutInterface,

    #[allow(missing_docs)]
    RefWarningAtomicInconsistent,

    #[allow(missing_docs)]
    RefWarningReadingUninitializedResource,

    #[allow(missing_docs)]
    RefWarningRawHazard,

    #[allow(missing_docs)]
    RefWarningWarHazard,

    #[allow(missing_docs)]
    RefWarningWawHazard,

    #[allow(missing_docs)]
    CreateCryptoSessionNullParam,

    #[allow(missing_docs)]
    CreateCryptoSessionoutOfMemoryReturn,

    #[allow(missing_docs)]
    GetCryptoTypeNullParam,

    #[allow(missing_docs)]
    GetDecoderProfileNullParam,

    #[allow(missing_docs)]
    GetCryptoSessionCertificateSizeNullParam,

    #[allow(missing_docs)]
    GetCryptoSessionCertificateNullParam,

    #[allow(missing_docs)]
    GetCryptoSessionCertificateWrongSize,

    #[allow(missing_docs)]
    GetCryptoSessionHandleWrongSize,

    #[allow(missing_docs)]
    NegotiateCryptoSessionKeyExchangeNullParam,

    #[allow(missing_docs)]
    EncryptionBltUnsupported,

    #[allow(missing_docs)]
    EncryptionBltNullParam,

    #[allow(missing_docs)]
    EncryptionBltSrcWrongDevice,

    #[allow(missing_docs)]
    EncryptionBltDstWrongDevice,

    #[allow(missing_docs)]
    EncryptionBltFormatMismatch,

    #[allow(missing_docs)]
    EncryptionBltSizeMismatch,

    #[allow(missing_docs)]
    EncryptionBltSrcMultiSampleD,

    #[allow(missing_docs)]
    EncryptionBltDstNotStaging,

    #[allow(missing_docs)]
    EncryptionBltSrcMapped,

    #[allow(missing_docs)]
    EncryptionBltDstMapped,

    #[allow(missing_docs)]
    EncryptionBltSrcOffered,

    #[allow(missing_docs)]
    EncryptionBltDstOffered,

    #[allow(missing_docs)]
    EncryptionBltSrcContentUndefined,

    #[allow(missing_docs)]
    DecryptionBltUnsupported,

    #[allow(missing_docs)]
    DecryptionBltNullParam,

    #[allow(missing_docs)]
    DecryptionBltSrcWrongDevice,

    #[allow(missing_docs)]
    DecryptionBltDstWrongDevice,

    #[allow(missing_docs)]
    DecryptionBltFormatMismatch,

    #[allow(missing_docs)]
    DecryptionBltSizeMismatch,

    #[allow(missing_docs)]
    DecryptionBltDstMultiSampleD,

    #[allow(missing_docs)]
    DecryptionBltSrcNotStaging,

    #[allow(missing_docs)]
    DecryptionBltDstNotRenderTarget,

    #[allow(missing_docs)]
    DecryptionBltSrcMapped,

    #[allow(missing_docs)]
    DecryptionBltDstMapped,

    #[allow(missing_docs)]
    DecryptionBltSrcOffered,

    #[allow(missing_docs)]
    DecryptionBltDstOffered,

    #[allow(missing_docs)]
    DecryptionBltSrcContentUndefined,

    #[allow(missing_docs)]
    StartSessionKeyRefreshNullParam,

    #[allow(missing_docs)]
    StartSessionKeyRefreshInvalidSize,

    #[allow(missing_docs)]
    FinishSessionKeyRefreshNullParam,

    #[allow(missing_docs)]
    GetEncryptionBltKeyNullParam,

    #[allow(missing_docs)]
    GetEncryptionBltKeyInvalidSize,

    #[allow(missing_docs)]
    GetContentProtectionCapsNullParam,

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
    GetAuthenticatedChannelCertificateSizeInvalidChannel,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCertificateSizeNullParam,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCertificateInvalidChannel,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCertificateNullParam,

    #[allow(missing_docs)]
    GetAuthenticatedChannelCertificateWrongSize,

    #[allow(missing_docs)]
    NegotiateAuthenticatedChannelKeyExchangeInvalidChannel,

    #[allow(missing_docs)]
    NegotiateAuthenticatedChannelKeyExchangeNullParam,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelNullParam,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelWrongChannel,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelUnsupportedQuery,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelWrongSize,

    #[allow(missing_docs)]
    QueryAuthenticatedChannelInvalidProcessIndex,

    #[allow(missing_docs)]
    ConfigureAuthenticatedChannelNullParam,

    #[allow(missing_docs)]
    ConfigureAuthenticatedChannelWrongChannel,

    #[allow(missing_docs)]
    ConfigureAuthenticatedChannelUnsupportedConfigure,

    #[allow(missing_docs)]
    ConfigureAuthenticatedChannelWrongSize,

    #[allow(missing_docs)]
    ConfigureAuthenticatedChannelInvalidProcessIdType,

    #[allow(missing_docs)]
    VsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    DsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    HsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    GsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    PsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    CsSetConstantBuffersInvalidBufferOffsetOrCount,

    #[allow(missing_docs)]
    NegotiateCryptoSessionKeyExchangeInvalidSize,

    #[allow(missing_docs)]
    NegotiateAuthenticatedChannelKeyExchangeInvalidSize,

    #[allow(missing_docs)]
    OfferResourcesInvalidPriority,

    #[allow(missing_docs)]
    GetCryptoSessionHandleOutOfMemory,

    #[allow(missing_docs)]
    AcquireHandleForCaptureNullParam,

    #[allow(missing_docs)]
    AcquireHandleForCaptureInvalidType,

    #[allow(missing_docs)]
    AcquireHandleForCaptureInvalidBind,

    #[allow(missing_docs)]
    AcquireHandleForCaptureInvalidArray,

    #[allow(missing_docs)]
    VideoProcessorSetStreamRotationNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamRotationInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamRotationInvalid,

    #[allow(missing_docs)]
    VideoProcessorSetStreamRotationUnsupported,

    #[allow(missing_docs)]
    VideoProcessorGetStreamRotationNullParam,

    #[allow(missing_docs)]
    DeviceClearViewInvalidView,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderDoubleExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderShaderExtensionsNotSupported,

    #[allow(missing_docs)]
    DeviceShaderLinkageMinPrecision,

    #[allow(missing_docs)]
    VideoProcessorSetStreamAlphaUnsupported,

    #[allow(missing_docs)]
    VideoProcessorSetStreamPixelAspectRatioUnsupported,

    #[allow(missing_docs)]
    DeviceCreateVertexShaderUavsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateHullShaderUavsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateDomainShaderUavsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderUavsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateGeometryShaderWithStreamOutputUavsNotSupported,

    #[allow(missing_docs)]
    DeviceCreatePixelShaderUavsNotSupported,

    #[allow(missing_docs)]
    DeviceCreateComputeShaderUavsNotSupported,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsInvalidOffset,

    #[allow(missing_docs)]
    DeviceOmSetRenderTargetsAndUnorderedAccessViewsTooManyViews,

    #[allow(missing_docs)]
    DeviceClearViewNotSupported,

    #[allow(missing_docs)]
    SwapDeviceContextStateNotSupported,

    #[allow(missing_docs)]
    UpdateSubresourcePreferUpdateSubresource1,

    #[allow(missing_docs)]
    GetDcInaccessible,

    #[allow(missing_docs)]
    DeviceClearViewInvalidRect,

    #[allow(missing_docs)]
    DeviceDrawSampleMaskIgnoredOnFl9,

    #[allow(missing_docs)]
    DeviceOpenSharedResource1NotSupported,

    #[allow(missing_docs)]
    DeviceOpenSharedResourceByNameNotSupported,

    #[allow(missing_docs)]
    EnqueueSetEventNotSupported,

    #[allow(missing_docs)]
    OfferReleaseNotSupported,

    #[allow(missing_docs)]
    OfferResourcesInaccessible,

    #[allow(missing_docs)]
    CreateVideoProcessorInputViewInvalidMsaa,

    #[allow(missing_docs)]
    CreateVideoProcessorOutputViewInvalidMsaa,

    #[allow(missing_docs)]
    DeviceClearViewInvalidSourceRect,

    #[allow(missing_docs)]
    DeviceClearViewEmptyRect,

    #[allow(missing_docs)]
    UpdateSubresourceEmptyDestBox,

    #[allow(missing_docs)]
    CopySubresourceRegionEmptySourceBox,

    #[allow(missing_docs)]
    DeviceDrawOmRenderTargetDoesNotSupportLogicOps,

    #[allow(missing_docs)]
    DeviceDrawDepthstencilViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawRenderTargetViewNotSet,

    #[allow(missing_docs)]
    DeviceDrawRenderTargetViewNotSetDueToFlipPresent,

    #[allow(missing_docs)]
    DeviceUnorderedAccessViewNotSetDueToFlipPresent,

    #[allow(missing_docs)]
    GetDataForNewHardwareKeyNullParam,

    #[allow(missing_docs)]
    CheckCryptoSessionStatusNullParam,

    #[allow(missing_docs)]
    GetCryptoSessionPrivateDataSizeNullParam,

    #[allow(missing_docs)]
    GetVideoDecoderCapsNullParam,

    #[allow(missing_docs)]
    GetVideoDecoderCapsZeroWidthHeight,

    #[allow(missing_docs)]
    CheckVideoDecoderDownSamplingNullParam,

    #[allow(missing_docs)]
    CheckVideoDecoderDownSamplingInvalidColorSpace,

    #[allow(missing_docs)]
    CheckVideoDecoderDownSamplingZeroWidthHeight,

    #[allow(missing_docs)]
    VideoDecoderEnabledownSamplingNullParam,

    #[allow(missing_docs)]
    VideoDecoderEnabledownSamplingUnsupported,

    #[allow(missing_docs)]
    VideoDecoderUpdateDownSamplingNullParam,

    #[allow(missing_docs)]
    VideoDecoderUpdateDownSamplingUnsupported,

    #[allow(missing_docs)]
    CheckVideoProcessorFormatConversionNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamColorSpace1InvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamMirrorNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamMirrorInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamMirrorUnsupported,

    #[allow(missing_docs)]
    VideoProcessorGetStreamColorSpace1NullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamMirrorNullParam,

    #[allow(missing_docs)]
    RecommendVideoDecoderDownSamplingNullParam,

    #[allow(missing_docs)]
    RecommendVideoDecoderDownSamplingInvalidColorSpace,

    #[allow(missing_docs)]
    RecommendVideoDecoderDownSamplingZeroWidthHeight,

    #[allow(missing_docs)]
    VideoProcessorSetOutputShaderUsageNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputShaderUsageNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetBehaviorHintsNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetBehaviorHintsInvalidStreamCount,

    #[allow(missing_docs)]
    VideoProcessorGetBehaviorHintsTargetRect,

    #[allow(missing_docs)]
    VideoProcessorGetBehaviorHintsInvalidSourceRect,

    #[allow(missing_docs)]
    VideoProcessorGetBehaviorHintsInvalidDestRect,

    #[allow(missing_docs)]
    GetCryptoSessionPrivateDataSizeInvalidKeyExchangeType,

    #[allow(missing_docs)]
    DeviceOpenSharedResource1AccessDenied,

    #[allow(missing_docs)]
    D3D111MessageSend,

    #[allow(missing_docs)]
    D3D112MessagesStart,

    #[allow(missing_docs)]
    CreateBufferInvalidUsage,

    #[allow(missing_docs)]
    CreateTexture1DInvalidUsage,

    #[allow(missing_docs)]
    CreateTexture2DInvalidUsage,

    #[allow(missing_docs)]
    CreateInputLayoutLevel9StepRateNot1,

    #[allow(missing_docs)]
    CreateInputLayoutLevel9InstancingNotSupported,

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
    ClearUnorderedAccessViewUIntHazard,

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
    D3D113MessagesStart,

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
    CreateShaderResourceViewInvalidVideoPlaneIndex,

    #[allow(missing_docs)]
    CreateShaderResourceViewAmbiguousVideoPlaneIndex,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidPlaneIndex,

    #[allow(missing_docs)]
    CreateRenderTargetViewInvalidVideoPlaneIndex,

    #[allow(missing_docs)]
    CreateRenderTargetViewAmbiguousVideoPlaneIndex,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidPlaneIndex,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewInvalidVideoPlaneIndex,

    #[allow(missing_docs)]
    CreateUnorderedAccessViewAmbiguousVideoPlaneIndex,

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
    VideoProcessorSetOutputHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetOutputHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessorGetOutputHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetOutputHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessorSetStreamHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessorSetStreamHdrMetadataInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorSetStreamHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessorGetStreamHdrMetadataNullParam,

    #[allow(missing_docs)]
    VideoProcessorGetStreamHdrMetadataInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamHdrMetadataInvalidSize,

    #[allow(missing_docs)]
    VideoProcessorGetStreamFrameFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamColorSpaceInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamOutputRateInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamSourceRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamDestRectInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamAlphaInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamPaletteInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamPixelAspectRatioInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamLumaKeyInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamStereoFormatInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamAutoProcessingModeInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamFilterInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamRotationInvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamColorSpace1InvalidStream,

    #[allow(missing_docs)]
    VideoProcessorGetStreamMirrorInvalidStream,

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
    D3D115MessagesStart,

    #[allow(missing_docs)]
    NegotiateCryptoSessionKeyExchangeMtInvalidKeyExchangeType,

    #[allow(missing_docs)]
    NegotiateCryptoSessionKeyExchangeMtNotSupported,

    #[allow(missing_docs)]
    DecoderbegInFrameInvalidHistogramComponentCount,

    #[allow(missing_docs)]
    DecoderbegInFrameInvalidHistogramComponent,

    #[allow(missing_docs)]
    DecoderbegInFrameInvalidHistogramBufferSize,

    #[allow(missing_docs)]
    DecoderbegInFrameInvalidHistogramBufferUsage,

    #[allow(missing_docs)]
    DecoderbegInFrameInvalidHistogramBufferMiscFlags,

    #[allow(missing_docs)]
    DecoderbegInFrameInvalidHistogramBufferOffset,

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
    IncompleteTrackedWorkloadPair,

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
