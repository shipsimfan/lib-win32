use crate::d3dcommon::{
    ID3D10Blob, D3D_PRIMITIVE, D3D_TESSELLATOR_DOMAIN, D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    D3D_TESSELLATOR_PARTITIONING,
};

// rustdoc imports
#[cfg(feature = "d3dcompiler")]
#[allow(unused_imports)]
use crate::d3dcompiler::D3DCreateBlob;

#[allow(missing_docs, non_camel_case_types)]
pub type D3D11_PRIMITIVE = D3D_PRIMITIVE;

#[allow(missing_docs, non_camel_case_types)]
pub type D3D11_TESSELLATOR_OUTPUT_PRIMITIVE = D3D_TESSELLATOR_OUTPUT_PRIMITIVE;

#[allow(missing_docs, non_camel_case_types)]
pub type D3D11_TESSELLATOR_PARTITIONING = D3D_TESSELLATOR_PARTITIONING;

#[allow(missing_docs, non_camel_case_types)]
pub type D3D11_TESSELLATOR_DOMAIN = D3D_TESSELLATOR_DOMAIN;

/// This interface is used to return data of arbitrary length.
///
/// # Remarks
/// An [`ID3DBlob`] is obtained by calling the [`D3DCreateBlob`] function.
///
/// [`ID3DBlob`] is version-neutral and can be used in code for any Direct3D version.
///
/// Blobs can be used as data buffers. Blobs can also be used for storing vertex, adjacency, and
/// material information during mesh optimization, and for loading operations. Also, these objects
/// are used to return object code and error messages in APIs that compile vertex, geometry, and
/// pixel shaders.
pub type ID3DBlob = ID3D10Blob;
