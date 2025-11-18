use crate::{
    com_interface,
    dxgidebug::{
        DXGI_DEBUG_ID, DXGI_INFO_QUEUE_FILTER, DXGI_INFO_QUEUE_MESSAGE,
        DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_ID,
        DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    },
    unknwn::IUnknown,
    BOOL, HRESULT, LPCSTR, SIZE_T, UINT, UINT64,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, S_OK, TRUE};
#[allow(unused_imports)]
use std::ptr::null_mut;

com_interface!(
    /// This interface controls the debug information queue, and can only be used if the debug
    /// layer is turned on.
    ///
    /// # Remarks
    /// This interface is obtained by calling the [`DXGIGetDebugInterface`] function.
    pub abstract IDXGIInfoQueue(IDXGIInfoQueueVTable): IUnknown(unknown) {
        const IID = 0xD67441C7-0x672A-0x476f-0x9E82-0xCD55B44949CE;

        /// Sets the maximum number of messages that can be added to the message queue.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that sets the
        ///                 limit on the number of messages.
        ///  * `message_count_limit` - The maximum number of messages that can be added to the
        ///                            queue. –1 means no limit.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn set_message_count_limit(
            &mut self,
            producer: DXGI_DEBUG_ID,
            message_count_limit: UINT64
        ) -> HRESULT;

        /// Clears all messages from the message queue.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that clears the
        ///                 messages.
        fn clear_stored_messages(&mut self, producer: DXGI_DEBUG_ID);

        /// Gets a message from the message queue.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 message.
        ///  * `message_index` - An index into the message queue after an optional retrieval filter
        ///                      has been applied. This can be between 0 and the number of messages
        ///                      in the message queue that pass through the retrieval filter. Call
        ///                [`IDXGIInfoQueue::get_num_stored_messages_allowed_by_retrieval_filters`]
        ///                      to obtain this number. 0 is the message at the beginning of the
        ///                      message queue.
        ///  * `message` - A pointer to a [`DXGI_INFO_QUEUE_MESSAGE`] structure that describes the
        ///                message.
        ///  * `message_byte_length` - A pointer to a variable that receives the size, in bytes, of
        ///                            the message description that pMessage points to. This size
        ///                            includes the size of the [`DXGI_INFO_QUEUE_MESSAGE`]
        ///                            structure in bytes.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// This method doesn't remove any messages from the message queue.
        ///
        /// This method gets a message from the message queue after an optional retrieval filter
        /// has been applied.
        ///
        /// Call this method twice to retrieve a message, first to obtain the size of the message
        /// and second to get the message.
        fn get_message(
            &mut self,
            producer: DXGI_DEBUG_ID,
            message_index: UINT64,
            message: *mut DXGI_INFO_QUEUE_MESSAGE,
            message_byte_length: *mut SIZE_T
        )-> HRESULT;

        /// Gets the number of messages that can pass through a retrieval filter.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 number.
        ///
        /// # Return Value
        /// Returns the number of messages that can pass through a retrieval filter.
        fn get_num_stored_messages_allowed_by_retrieval_filters(
            &mut self,
            producer: DXGI_DEBUG_ID
        ) -> UINT64;

        /// Gets the number of messages currently stored in the message queue.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 number.
        ///
        /// # Return Value
        /// Returns the number of messages currently stored in the message queue.
        fn get_num_stored_messages(&mut self, producer: DXGI_DEBUG_ID) -> UINT64;

        /// Gets the number of messages that were discarded due to the message count limit.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 number.
        ///
        /// # Return Value
        /// Returns the number of messages that were discarded.
        ///
        /// # Remarks
        /// Get and set the message count limit with [`IDXGIInfoQueue::get_message_count_limit`]
        /// and [`IDXGIInfoQueue::set_message_count_limit`], respectively.
        fn get_num_messages_discarded_by_message_count_limit(
            &mut self,
            producer: DXGI_DEBUG_ID
        ) -> UINT64;

        /// Gets the maximum number of messages that can be added to the message queue.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 number.
        ///
        /// # Return Value
        /// Returns the maximum number of messages that can be added to the queue. –1 means no
        /// limit.
        ///
        /// # Remarks
        /// When the number of messages in the message queue reaches the maximum limit, new
        /// messages coming in push old messages out.
        fn get_message_count_limit(&mut self, producer: DXGI_DEBUG_ID) -> UINT64;

        /// Gets the number of messages that a storage filter allowed to pass through.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 number.
        ///
        /// # Return Value
        /// Returns the number of messages allowed by a storage filter.
        fn get_num_messages_allowed_by_storage_filter(
            &mut self,
            producer: DXGI_DEBUG_ID
        ) -> UINT64;

        /// Gets the number of messages that were denied passage through a storage filter.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 number.
        ///
        /// # Return Value
        /// Returns the number of messages denied by a storage filter.
        fn get_num_messages_denied_by_storage_filter(&mut self, producer: DXGI_DEBUG_ID) -> UINT64;

        /// Adds storage filters to the top of the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that produced the
        ///                 filters.
        ///  * `filter` - An array of [`DXGI_INFO_QUEUE_FILTER`] structures that describe the
        ///               filters.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn add_storage_filter_entries(
            &mut self,
            producer: DXGI_DEBUG_ID,
            filter: *mut DXGI_INFO_QUEUE_FILTER
        ) -> HRESULT;

        /// Gets the storage filter at the top of the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 filter.
        ///  * `filter` - A pointer to a [`DXGI_INFO_QUEUE_FILTER`] structure that describes the
        ///               filter.
        ///  * `filter_byte_length` - A pointer to a variable that receives the size, in bytes, of
        ///                           the filter description to which `filter` points. If `filter`
        ///                           is [`null_mut`], [`IDXGIInfoQueue::get_storage_filter`]
        ///                           outputs the size of the storage filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn get_storage_filter(
            &mut self,
            producer: DXGI_DEBUG_ID,
            filter: *mut DXGI_INFO_QUEUE_FILTER,
            filter_byte_length: *mut SIZE_T
        ) -> HRESULT;

        /// Removes a storage filter from the top of the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that removes the
        ///                 filter.
        fn clear_storage_filter(&mut self, producer: DXGI_DEBUG_ID);

        /// Pushes an empty storage filter onto the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 empty storage filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// An empty storage filter allows all messages to pass through.
        fn push_empty_storage_filter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT;

        /// Pushes a deny-all storage filter onto the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// A deny-all storage filter prevents all messages from passing through.
        fn push_deny_all_storage_filter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT;

        /// Pushes a copy of the storage filter that is currently on the top of the storage-filter
        /// stack onto the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 copy of the storage filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn push_copy_of_storage_filter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT;

        /// Pushes a storage filter onto the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 filter.
        ///  * `filter` - A pointer to a [`DXGI_INFO_QUEUE_FILTER`] structure that describes the
        ///               filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn push_storage_filter(
            &mut self,
            producer: DXGI_DEBUG_ID,
            filter: *mut DXGI_INFO_QUEUE_FILTER
        ) -> HRESULT;

        /// Pops a storage filter from the top of the storage-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pops the
        ///                 filter.
        fn pop_storage_filter(&mut self, producer: DXGI_DEBUG_ID);

        /// Gets the size of the storage-filter stack in bytes.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 size.
        ///
        /// # Return Value
        /// Returns the size of the storage-filter stack in bytes.
        fn get_storage_filter_stack_size(&mut self, producer: DXGI_DEBUG_ID) -> UINT;

        /// Adds retrieval filters to the top of the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that produced the
        ///                 filters.
        ///  * `filter` - An array of [`DXGI_INFO_QUEUE_FILTER`] structures that describe the
        ///               filters.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn add_retrieval_filter_entries(
            &mut self,
            producer: DXGI_DEBUG_ID,
            filter: *mut DXGI_INFO_QUEUE_FILTER
        ) -> HRESULT;

        /// Gets the retrieval filter at the top of the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 filter.
        ///  * `filter` - A pointer to a [`DXGI_INFO_QUEUE_FILTER`] structure that describes the
        ///               filter.
        ///  * `filter_byte_range` - A pointer to a variable that receives the size, in bytes, of
        ///                          the filter description to which `filter` points. If `filter`
        ///                          is [`null_mut`], [`IDXGIInfoQueue::get_retrieval_filter`]
        ///                          outputs the size of the retrieval filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn get_retrieval_filter(
            &mut self,
            producer: DXGI_DEBUG_ID,
            filter: *mut DXGI_INFO_QUEUE_FILTER,
            filter_byte_length: *mut SIZE_T
        ) -> HRESULT;

        /// Removes a retrieval filter from the top of the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that removes the
        ///                 filter.
        fn clear_retrieval_filter(&mut self, producer: DXGI_DEBUG_ID);

        /// Pushes an empty retrieval filter onto the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 empty retrieval filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// An empty retrieval filter allows all messages to pass through.
        fn push_empty_retrieval_filter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT;

        /// Pushes a deny-all retrieval filter onto the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 deny-all retrieval filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        ///
        /// # Remarks
        /// A deny-all retrieval filter prevents all messages from passing through.
        fn push_deny_all_retrieval_filter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT;

        /// Pushes a copy of the retrieval filter that is currently on the top of the
        /// retrieval-filter stack onto the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 copy of the retrieval filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn push_copy_of_retrieval_filter(&mut self, producer: DXGI_DEBUG_ID) -> HRESULT;

        /// Pushes a retrieval filter onto the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pushes the
        ///                 filter.
        ///  * `filter` - A pointer to a [`DXGI_INFO_QUEUE_FILTER`] structure that describes the
        ///               filter.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn push_retrieval_filter(
            &mut self,
            producer: DXGI_DEBUG_ID,
            filter: *mut DXGI_INFO_QUEUE_FILTER
        ) -> HRESULT;

        /// Pops a retrieval filter from the top of the retrieval-filter stack.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that pops the
        ///                 filter.
        fn pop_retrieval_filter(&mut self, producer: DXGI_DEBUG_ID);

        /// Gets the size of the retrieval-filter stack in bytes.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 size.
        ///
        /// # Return Value
        /// Returns the size of the retrieval-filter stack in bytes.
        fn get_retrieval_filter_stack_size(&mut self, producer: DXGI_DEBUG_ID) -> UINT;

        /// Adds a debug message to the message queue and sends that message to the debug output.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that produced the
        ///                 message.
        ///  * `category` - A [`DXGI_INFO_QUEUE_MESSAGE_CATEGORY`]-typed value that specifies the
        ///                 category of the message.
        ///  * `severity` - A [`DXGI_INFO_QUEUE_MESSAGE_SEVERITY`]-typed value that specifies the
        ///                 severity of the message.
        ///  * `id` - An integer that uniquely identifies the message.
        ///  * `description` - The message string.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn add_message(
            &mut self,
            producer: DXGI_DEBUG_ID,
            category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
            severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
            id: DXGI_INFO_QUEUE_MESSAGE_ID,
            description: LPCSTR
        ) -> HRESULT;

        /// Adds a user-defined message to the message queue and sends that message to the debug
        /// output.
        ///
        /// # Parameters
        ///  * `severity` - A [`DXGI_INFO_QUEUE_MESSAGE_SEVERITY`]-typed value that specifies the
        ///                 severity of the message.
        ///  * `description` - The message string.
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn add_application_message(
            &mut self,
            severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
            description: LPCSTR
        ) -> HRESULT;

        /// Sets a message category to break on when a message with that category passes through
        /// the storage filter.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that sets the
        ///                 breaking condition.
        ///  * `category` - A [`DXGI_INFO_QUEUE_MESSAGE_CATEGORY`]-typed value that specifies the
        ///                 category of the message.
        ///  * `enable` - A Boolean value that specifies whether
        ///               [`IDXGIInfoQueue::set_break_on_category`] turns on or off this breaking
        ///               condition ([`TRUE`] for on, [`FALSE`] for off).
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn set_break_on_category(
            &mut self,
            producer: DXGI_DEBUG_ID,
            category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
            enable: BOOL
        ) -> HRESULT;

        /// Sets a message severity level to break on when a message with that severity level
        /// passes through the storage filter.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that sets the
        ///                 breaking condition.
        ///  * `severity` - A [`DXGI_INFO_QUEUE_MESSAGE_SEVERITY`]-typed value that specifies the
        ///                 severity of the message.
        ///  * `enable` - A Boolean value that specifies whether
        ///               [`IDXGIInfoQueue::set_break_on_severity`] turns on or off this breaking
        ///               condition ([`TRUE`] for on, [`FALSE`] for off).
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn set_break_on_severity(
            &mut self,
            producer: DXGI_DEBUG_ID,
            severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
            enable: BOOL
        ) -> HRESULT;

        /// Sets a message identifier to break on when a message with that identifier passes
        /// through the storage filter.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that sets the
        ///                 breaking condition.
        ///  * `id` - An integer value that specifies the identifier of the message.
        ///  * `enable` - A Boolean value that specifies whether
        ///               [`IDXGIInfoQueue::set_break_on_id`] turns on or off this breaking
        ///               condition ([`TRUE`] for on, [`FALSE`] for off).
        ///
        /// # Return Value
        /// Returns [`S_OK`] if successful; an error code otherwise.
        fn set_break_on_id(
            &mut self,
            producer: DXGI_DEBUG_ID,
            id: DXGI_INFO_QUEUE_MESSAGE_ID,
            enable: BOOL
        ) -> HRESULT;

        /// Determines whether the break on a message category is turned on or off.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 breaking status.
        ///  * `category` - A [`DXGI_INFO_QUEUE_MESSAGE_CATEGORY`]-typed value that specifies the
        ///                 category of the message.
        ///
        /// # Return Value
        /// Returns a Boolean value that specifies whether this category of breaking condition is
        /// turned on or off ([`TRUE`] for on, [`FALSE`] for off).
        fn get_break_on_category(
            &mut self,
            producer: DXGI_DEBUG_ID,
            category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY
        ) -> BOOL;

        /// Determines whether the break on a message severity level is turned on or off.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 breaking status.
        ///  * `severity` - A [`DXGI_INFO_QUEUE_MESSAGE_SEVERITY`]-typed value that specifies the
        ///                 severity of the message.
        ///
        /// # Return Value
        /// Returns a Boolean value that specifies whether this severity of breaking condition is
        /// turned on or off ([`TRUE`] for on, [`FALSE`] for off).
        fn get_break_on_severity(
            &mut self,
            producer: DXGI_DEBUG_ID,
            severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY
        ) -> BOOL;

        /// Determines whether the break on a message identifier is turned on or off.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the
        ///                 breaking status.
        ///  * `id` - An integer value that specifies the identifier of the message.
        ///
        /// # Return Value
        /// Returns a Boolean value that specifies whether this break on a message identifier is
        /// turned on or off ([`TRUE`] for on, [`FALSE`] for off).
        fn get_break_on_id(
            &mut self,
            producer: DXGI_DEBUG_ID,
            id: DXGI_INFO_QUEUE_MESSAGE_ID
        ) -> BOOL;

        /// Turns the debug output on or off.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the mute
        ///                 status.
        ///  * `mute` - A Boolean value that specifies whether to turn the debug output on or off
        ///             ([`TRUE`] for on, [`FALSE`] for off).
        fn set_mute_debug_output(&mut self, producer: DXGI_DEBUG_ID, mute: BOOL);

        /// Determines whether the debug output is turned on or off.
        ///
        /// # Parameters
        ///  * `producer` - A [`DXGI_DEBUG_ID`] value that identifies the entity that gets the mute
        ///                 status.
        ///
        /// # Return Value
        /// Returns a Boolean value that specifies whether the debug output is turned on or off
        /// ([`TRUE`] for on, [`FALSE`] for off).
        fn get_mute_debug_output(&mut self, producer: DXGI_DEBUG_ID) -> BOOL;
    }
);
