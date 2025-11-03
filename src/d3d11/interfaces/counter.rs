use crate::{
    com_interface,
    d3d11::{
        ID3D11Asynchronous, ID3D11DeviceChild,
        D3D11_COUNTER_DESC,
    },
    unknwn::{IUnknown},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext, D3D11_COUNTER};

com_interface!(
    /// This interface encapsulates methods for measuring GPU performance.
    ///
    /// # Remarks
    /// A counter can be created with [`ID3D11Device::create_counter`].
    ///
    /// This is a derived class of [`ID3D11Asynchronous`].
    ///
    /// Counter data is gathered by issuing an [`ID3D11DeviceContext::begin`] command, issuing some
    /// graphics commands, issuing an [`ID3D11DeviceContext::end`] command, and then calling
    /// [`ID3D11DeviceContext::get_data`] to get data about what happened in between the
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`] calls. The data returned by
    /// [`ID3D11DeviceContext::get_data`] will be different depending on the type of counter. The
    /// call to [`ID3D11DeviceContext::end`] causes the data returned by
    /// [`ID3D11DeviceContext::get_data`] to be accurate up until the last call to
    /// [`ID3D11DeviceContext::end`].
    ///
    /// Counters are best suited for profiling.
    ///
    /// For a list of the types of performance counters, see [`D3D11_COUNTER`].
    pub abstract ID3D11Counter(ID3D11CounterVTable):
        ID3D11Asynchronous(asynchronous) +
        ID3D11DeviceChild +
        IUnknown {
        const IID = 0x6E8C49FB-0xA371-0x4770-0xB440-0x29086022B741;

        /// Get a counter description.
        ///
        /// # Parameters
        ///  * `desc` - Pointer to a counter description (see [`D3D11_COUNTER_DESC`]).
        fn get_desc(&mut self, desc: *mut D3D11_COUNTER_DESC);
    }
);
