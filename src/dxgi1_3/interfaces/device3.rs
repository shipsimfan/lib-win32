use crate::{
    com_interface,
    dxgi::{IDXGIDevice, IDXGIDevice1, IDXGIObject},
    dxgi1_2::IDXGIDevice2,
    unknwn::IUnknown,
};

// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::ID3D11DeviceContext;

com_interface!(
    /// The [`IDXGIDevice3`] interface implements a derived class for DXGI objects that produce
    /// image data. The interface exposes a method to trim graphics memory usage by the DXGI
    /// device.
    ///
    /// # Remarks
    /// The [`IDXGIDevice3`] interface is designed for use by DXGI objects that need access to
    /// other DXGI objects. This interface is useful to applications that do not use Direct3D to
    /// communicate with DXGI.
    ///
    /// The Direct3D create device functions return a Direct3D device object. This Direct3D device
    /// object implements the [`IUnknown`] interface. You can query this Direct3D device object for
    /// the device's corresponding [`IDXGIDevice3`] interface.
    pub abstract IDXGIDevice3(IDXGIDevice3VTable):
        IDXGIDevice2(device2) +
        IDXGIDevice1 +
        IDXGIDevice +
        IDXGIObject +
        IUnknown {
        const IID = 0x6007896C-0x3244-0x4AFD-0xBF18-0xA6D3BEDA502;

        /// Trims the graphics memory allocated by the [`IDXGIDevice3`] DXGI device on the app's
        /// behalf.
        ///
        /// For apps that render with DirectX, graphics drivers periodically allocate internal
        /// memory buffers in order to speed up subsequent rendering requests. These memory
        /// allocations count against the app's memory usage for PLM and in general lead to
        /// increased memory usage by the overall system.
        ///
        /// Starting in Windows 8.1, apps that render with Direct2D and/or Direct3D (including
        /// CoreWindow and XAML interop) must call [`IDXGIDevice3::trim`] in response to the PLM
        /// suspend callback. The Direct3D runtime and the graphics driver will discard internal
        /// memory buffers allocated for the app, reducing its memory footprint.
        ///
        /// Calling this method does not change the rendering state of the graphics device and it
        /// has no effect on rendering operations. There is a brief performance hit when internal
        /// buffers are reallocated during the first rendering operations after the
        /// [`IDXGIDevice3::trim`] call, therefore apps should only call [`IDXGIDevice3::trim`]
        /// when going idle for a period of time (in response to PLM suspend, for example).
        ///
        /// Apps should ensure that they call [`IDXGIDevice3::trim`] as one of the last D3D
        /// operations done before going idle. Direct3D will normally defer the destruction of D3D
        /// objects. Calling [`IDXGIDevice3::trim`], however, forces Direct3D to destroy objects
        /// immediately. For this reason, it is not guaranteed that releasing the final reference
        /// on Direct3D objects after calling [`IDXGIDevice3::trim`] will cause the object to be
        /// destroyed and memory to be deallocated before the app suspends.
        ///
        /// Similar to [`ID3D11DeviceContext::flush`], apps should call
        /// [`ID3D11DeviceContext::clear_state`] before calling [`IDXGIDevice3::trim`].
        /// [`ID3D11DeviceContext::clear_state`] clears the Direct3D pipeline bindings, ensuring
        /// that Direct3D does not hold any references to the Direct3D objects you are trying to
        /// release.
        ///
        /// It is also prudent to release references on middleware before calling
        /// [`IDXGIDevice3::trim`], as that middleware may also need to release references to
        /// Direct3D objects.
        fn trim(&mut self);
    }
);
