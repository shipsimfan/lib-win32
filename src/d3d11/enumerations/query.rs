// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11Device, ID3D11DeviceContext, D3D11_QUERY_DATA_SO_STATISTICS,
        D3D11_QUERY_DATA_TIMESTAMP_DISJOINT,
    },
    BOOL, FALSE, S_OK, TRUE,
};

/// Query types.
///
/// # Remarks
/// Create a query with [`ID3D11Device::create_query`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_QUERY {
    /// Determines whether or not the GPU is finished processing commands. When the GPU is finished
    /// processing commands [`ID3D11DeviceContext::get_data`] will return [`S_OK`], and `data` will
    /// point to a [`BOOL`] with a value of [`TRUE`]. When using this type of query,
    /// [`ID3D11DeviceContext::begin`] is disabled.
    Event = 0,

    /// Get the number of samples that passed the depth and stencil tests in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] returns a [`UINT64`]. If a depth or stencil test is
    /// disabled, then each of those tests will be counted as a pass.
    Occlusion,

    /// Get a timestamp value where [`ID3D11DeviceContext::get_data`] returns a [`UINT64`]. This
    /// kind of query is only useful if two timestamp queries are done in the middle of a
    /// [`D3D11_QUERY::TimestampDisjoint`] query. The difference of two timestamps can be used to
    /// determine how many ticks have elapsed, and the [`D3D11_QUERY::TimestampDisjoint`] query
    /// will determine if that difference is a reliable value and also has a value that shows how
    /// to convert the number of ticks into seconds. See [`D3D11_QUERY::TimestampDisjoint`].
    /// When using this type of query, [`ID3D11DeviceContext::begin`] is disabled.
    Timestamp,

    /// Determines whether or not a [`D3D11_QUERY::Timestamp`] is returning reliable values, and
    /// also gives the frequency of the processor enabling you to convert the number of elapsed
    /// ticks into seconds. [`ID3D11DeviceContext::get_data`] will return a
    /// [`D3D11_QUERY::TimestampDisjoint`]. This type of query should only be invoked once per
    /// frame or less.
    TimestampDisjoint,

    /// Get pipeline statistics, such as the number of pixel shader invocations in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] will return a [`D3D11_QUERY_DATA_PIPELINE_STATISTICS`].
    PipelineStatistics,

    /// Similar to [`D3D11_QUERY::Occlusion`], except [`ID3D11DeviceContext::get_data`] returns a
    /// [`BOOL`] indicating whether or not any samples passed the depth and stencil tests -
    /// [`TRUE`] meaning at least one passed, [`FALSE`] meaning none passed.
    OcclusionPredicate,

    /// Get streaming output statistics, such as the number of primitives streamed out in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] will return a [`D3D11_QUERY_DATA_SO_STATISTICS`]
    /// structure.
    SoStatistics,

    /// Determines whether or not any of the streaming output buffers overflowed in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] returns a [`BOOL`] - [`TRUE`] meaning there was an
    /// overflow, [`FALSE`] meaning there was not an overflow. If streaming output writes to
    /// multiple buffers, and one of the buffers overflows, then it will stop writing to all the
    /// output buffers. When an overflow is detected by Direct3D it is prevented from happening -
    /// no memory is corrupted. This predication may be used in conjunction with an
    /// [`D3D11_QUERY::SoStatistics`] query so that when an overflow occurs the
    /// [`D3D11_QUERY::SoStatistics`] query will let the application know how much memory was
    /// needed to prevent an overflow.
    SoOverflowPredicate,

    /// Get streaming output statistics for stream 0, such as the number of primitives streamed out
    /// in between [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] will return a [`D3D11_QUERY_DATA_SO_STATISTICS`]
    /// structure.
    SoStatisticsStream0,

    /// Determines whether or not the stream 0 output buffers overflowed in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] returns a [`BOOL`] - [`TRUE`] meaning there was an
    /// overflow, [`FALSE`] meaning there was not an overflow. If streaming output writes to
    /// multiple buffers, and one of the buffers overflows, then it will stop writing to all the
    /// output buffers. When an overflow is detected by Direct3D it is prevented from happening -
    /// no memory is corrupted. This predication may be used in conjunction with an
    /// [`D3D11_QUERY::SoStatistics`] query so that when an overflow occurs the
    /// [`D3D11_QUERY::SoStatistics`] query will let the application know how much memory was
    /// needed to prevent an overflow.
    SoOverflowPredicateStream0,

    /// Get streaming output statistics for stream 1, such as the number of primitives streamed out
    /// in between [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] will return a [`D3D11_QUERY_DATA_SO_STATISTICS`]
    /// structure.
    SoStatisticsStream1,

    /// Determines whether or not the stream 1 output buffers overflowed in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] returns a [`BOOL`] - [`TRUE`] meaning there was an
    /// overflow, [`FALSE`] meaning there was not an overflow. If streaming output writes to
    /// multiple buffers, and one of the buffers overflows, then it will stop writing to all the
    /// output buffers. When an overflow is detected by Direct3D it is prevented from happening -
    /// no memory is corrupted. This predication may be used in conjunction with an
    /// [`D3D11_QUERY::SoStatistics`] query so that when an overflow occurs the
    /// [`D3D11_QUERY::SoStatistics`] query will let the application know how much memory was
    /// needed to prevent an overflow.
    SoOverflowPredicateStream1,

    /// Get streaming output statistics for stream 2, such as the number of primitives streamed out
    /// in between [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] will return a [`D3D11_QUERY_DATA_SO_STATISTICS`]
    /// structure.
    SoStatisticsStream2,

    /// Determines whether or not the stream 2 output buffers overflowed in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] returns a [`BOOL`] - [`TRUE`] meaning there was an
    /// overflow, [`FALSE`] meaning there was not an overflow. If streaming output writes to
    /// multiple buffers, and one of the buffers overflows, then it will stop writing to all the
    /// output buffers. When an overflow is detected by Direct3D it is prevented from happening -
    /// no memory is corrupted. This predication may be used in conjunction with an
    /// [`D3D11_QUERY::SoStatistics`] query so that when an overflow occurs the
    /// [`D3D11_QUERY::SoStatistics`] query will let the application know how much memory was
    /// needed to prevent an overflow.
    SoOverflowPredicateStream2,

    /// Get streaming output statistics for stream 3, such as the number of primitives streamed out
    ///  in between [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`].
    /// [`ID3D11DeviceContext::get_data`] will return a [`D3D11_QUERY_DATA_SO_STATISTICS`]
    /// structure.
    SoStatisticsStream3,

    /// Determines whether or not the stream 3 output buffers overflowed in between
    /// [`ID3D11DeviceContext::begin`] and [`ID3D11DeviceContext::end`]. [
    /// `ID3D11DeviceContext::get_data`] returns a [`BOOL`] - [`TRUE`] meaning there was an
    /// overflow, [`FALSE`] meaning there was not an overflow. If streaming output writes to
    /// multiple buffers, and one of the buffers overflows, then it will stop writing to all the
    /// output buffers. When an overflow is detected by Direct3D it is prevented from happening -
    /// no memory is corrupted. This predication may be used in conjunction with an
    /// [`D3D11_QUERY::SoStatistics`] query so that when an overflow occurs the
    /// [`D3D11_QUERY::SoStatistics`] query will let the application know how much memory was
    /// needed to prevent an overflow.
    SoOverflowPredicateStream3,
}
