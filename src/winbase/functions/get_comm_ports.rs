use crate::{PULONG, ULONG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA, ERROR_SUCCESS};

#[link(name = "OneCore")]
unsafe extern "system" {
    /// Gets an array that contains the well-formed COM ports.
    ///
    /// This function obtains the COM port numbers from the `HKLM\Hardware\DeviceMap\SERIALCOMM`
    /// registry key and then writes them to a caller-supplied array. If the array is too small,
    /// the function gets the necessary size.
    ///
    /// # Parameters
    ///  * `port_numbers` - An array for the port numbers.
    ///  * `port_numbers_count` - The length of the array in the `port_numbers` parameter.
    ///  * `port_numbers_found` - The number of port numbers written to the `port_numbers` or the
    ///                           length of the array required for the port numbers.
    ///
    /// # Return Value
    ///  * [`ERROR_SUCCESS`] - The call succeeded. The `port_numbers` array was large enough for
    ///                        the result.
    ///  * [`ERROR_MORE_DATA`] - The `port_numbers` array was too small to contain all available
    ///                          port numbers.
    ///  * [`ERROR_FILE_NOT_FOUND`] - There are no comm ports available.
    pub fn GetCommPorts(
        port_numbers: PULONG,
        port_numbers_count: ULONG,
        port_numbers_found: PULONG,
    ) -> ULONG;
}
