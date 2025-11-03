use crate::{
    com_interface,
    d3d11::{
        ID3D11ClassLinkage, ID3D11DeviceChild, D3D11_CLASS_INSTANCE_DESC,
    },
    unknwn::{IUnknown},
    LPSTR, SIZE_T,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11DeviceContext;

com_interface!(
    /// This interface encapsulates an HLSL class.
    ///
    /// # Remarks
    /// This interface is created by calling [`ID3D11ClassLinkage::create_class_instance`]. The
    /// interface is used when binding shader resources to the pipeline using APIs such as
    /// [`ID3D11DeviceContext::vs_set_shader`].
    pub abstract ID3D11ClassInstance(ID3D11ClassInstanceVTable):
        ID3D11DeviceChild(device_child) +
        IUnknown {
        const IID = 0xA6CD7FAA-0xB0B7-0x4A2F-0x9436-0x8662A65797CB;

        /// Gets the '`ID3D11ClassLinkage`' object associated with the current HLSL class.
        ///
        /// # Parameters
        ///  * `linkage` - A pointer to a [`ID3D11ClassLinkage`] interface pointer.
        fn get_class_linkage(&mut self, linkage: *mut *mut ID3D11ClassLinkage);

        /// Gets a description of the current HLSL class.
        ///
        /// # Parameters
        ///  * `desc` - A pointer to a [`D3D11_CLASS_INSTANCE_DESC`] structure that describes the
        ///             current HLSL class.
        ///
        /// # Remarks
        /// An instance is not restricted to being used for a single type in a single shader. An
        /// instance is flexible and can be used for any shader that used the same type name or
        /// instance name when the instance was generated.
        ///  - A created instance will work for any shader that contains a type of the same type
        ///    name. For instance, a class instance created with the type name "DefaultShader"
        ///    would work in any shader that contained a type "DefaultShader" even though several
        ///    shaders could describe a different type.
        ///  - A gotten instance maps directly to an instance name/index in a shader. A class
        ///    instance acquired using [`ID3D11ClassLinkage::get_class_instance`] will work for any
        ///    shader that contains a class instance of the name used to generate the runtime
        ///    instance, the instance does not have to be the same type in all of the shaders it's
        ///    used in.
        ///
        /// An instance does not replace the importance of reflection for a particular shader since
        /// a gotten instance will not know its slot location and a created instance only specifies
        /// a type name.
        fn get_desc(&mut self, desc: *mut D3D11_CLASS_INSTANCE_DESC);

        /// Gets the instance name of the current HLSL class.
        ///
        /// # Parameters
        ///  * `instance_name` - The instance name of the current HLSL class.
        ///  * `buffer_length` - The length of the `instance_name` parameter.
        ///
        /// # Remarks
        /// [`ID3D11ClassInstance::get_instance_name`] will return a valid name only for instances
        /// acquired using [`ID3D11ClassLinkage::get_class_instance`].
        fn get_instance_name(&mut self, instance_name: LPSTR, buffer_length: *mut SIZE_T);

        /// Gets the type of the current HLSL class.
        ///
        /// # Parameters
        ///  * `type_name` - Type of the current HLSL class.
        ///  * `buffer_length` - The length of the `type_name` parameter.
        ///
        /// # Remarks
        /// [`ID3D11ClassInstance::get_type_name`] will return a valid name only for instances
        /// acquired using [`ID3D11ClassLinkage::get_class_instance`].
        fn get_type_name(&mut self, type_name: LPSTR, buffer_length: *mut SIZE_T);
    }
);
