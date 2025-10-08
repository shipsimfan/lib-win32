use crate::{
    com_interface,
    d3d11::{ID3D11ClassInstance, ID3D11DeviceChild, ID3D11DeviceChildTrait},
    unknwn::{IUnknown, IUnknownTrait},
    HRESULT, LPCSTR, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11Device, S_OK};

com_interface!(
    /// This interface encapsulates an HLSL dynamic linkage.
    ///
    /// # Remarks
    /// A class linkage object can hold up to 64K gotten instances. A gotten instance is a handle
    /// that references a variable name in any shader that is created with that linkage object.
    /// When you create a shader with a class linkage object, the runtime gathers these instances
    /// and stores them in the class linkage object.
    ///
    /// An [`ID3D11ClassLinkage`] object is created using the
    /// [`ID3D11Device::create_class_linkage`] method.
    pub abstract ID3D11ClassLinkage(ID3D11ClassLinkageVTable/ID3D11ClassLinkageTrait):
        ID3D11DeviceChild/ID3D11DeviceChildTrait(device_child) +
        IUnknown/IUnknownTrait(device_child.unknown) {
        const IID = 0xDDF57CBA-0x9543-0x46E4-0xA12B-0xF207A0FE7FED;

        /// Gets the class-instance object that represents the specified HLSL class.
        ///
        /// # Parameters
        ///  * `class_instance_name` - The name of a class for which to get the class instance.
        ///  * `instance_index` - The index of the class instance.
        ///  * `instance` - The address of a pointer to an [`ID3D11ClassInstance`] interface to
        ///                 initialize.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// A class instance must have at least 1 data member in order to be available for the
        /// runtime to use with [`ID3D11ClassLinkage::get_class_instance`]. Any instance with no
        /// members will be optimized out of a compiled shader blob as a zero-sized object. If you
        /// have a class with no data members, use [`ID3D11ClassLinkage::create_class_instance`]
        /// instead.
        fn get_class_instance(
            &mut self,
            class_instance_name: LPCSTR,
            instance_index: UINT,
            instance: *mut *mut ID3D11ClassInstance
        ) -> HRESULT;

        /// Initializes a class-instance object that represents an HLSL class instance.
        ///
        /// # Parameters
        ///  * `class_type_name` - The type name of a class to initialize.
        ///  * `constant_buffer_offset` - Identifies the constant buffer that contains the class
        ///                               data.
        ///  * `constant_vector_offset` - The four-component vector offset from the start of the
        ///                               constant buffer where the class data will begin.
        ///                               Consequently, this is not a byte offset.
        ///  * `texture_offset` - The texture slot for the first texture; there may be multiple
        ///                       textures following the offset.
        ///  * `sampler_offset` - The sampler slot for the first sampler; there may be multiple
        ///                       samplers following the offset.
        ///  * `instance` - The address of a pointer to an [`ID3D11ClassInstance`] interface to
        ///                 initialize.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; otherwise, returns one of the Direct3D 11 Return Codes.
        ///
        /// # Remarks
        /// Instances can be created (or gotten) before or after a shader is created. Use the same
        /// shader linkage object to acquire a class instance and create the shader the instance is
        /// going to be used in.
        fn create_class_instance(
            &mut self,
            class_type_name: LPCSTR,
            constant_buffer_offset: UINT,
            constant_vector_offset: UINT,
            texture_offset: UINT,
            sampler_offset: UINT,
            instance: *mut *mut ID3D11ClassLinkage
        ) -> HRESULT;
    }
);
