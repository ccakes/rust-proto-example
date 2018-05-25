// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ethernet_controller_driver_bag_v2_type_KEYS {
    // message fields
    pub interface_name: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ethernet_controller_driver_bag_v2_type_KEYS {}

impl ethernet_controller_driver_bag_v2_type_KEYS {
    pub fn new() -> ethernet_controller_driver_bag_v2_type_KEYS {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ethernet_controller_driver_bag_v2_type_KEYS {
        static mut instance: ::protobuf::lazy::Lazy<ethernet_controller_driver_bag_v2_type_KEYS> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ethernet_controller_driver_bag_v2_type_KEYS,
        };
        unsafe {
            instance.get(ethernet_controller_driver_bag_v2_type_KEYS::new)
        }
    }

    // string interface_name = 1;

    pub fn clear_interface_name(&mut self) {
        self.interface_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_interface_name(&mut self, v: ::std::string::String) {
        self.interface_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interface_name(&mut self) -> &mut ::std::string::String {
        &mut self.interface_name
    }

    // Take field
    pub fn take_interface_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.interface_name, ::std::string::String::new())
    }

    pub fn get_interface_name(&self) -> &str {
        &self.interface_name
    }

    fn get_interface_name_for_reflect(&self) -> &::std::string::String {
        &self.interface_name
    }

    fn mut_interface_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.interface_name
    }
}

impl ::protobuf::Message for ethernet_controller_driver_bag_v2_type_KEYS {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.interface_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.interface_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.interface_name);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.interface_name.is_empty() {
            os.write_string(1, &self.interface_name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ethernet_controller_driver_bag_v2_type_KEYS {
    fn new() -> ethernet_controller_driver_bag_v2_type_KEYS {
        ethernet_controller_driver_bag_v2_type_KEYS::new()
    }

    fn descriptor_static(_: ::std::option::Option<ethernet_controller_driver_bag_v2_type_KEYS>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "interface_name",
                    ethernet_controller_driver_bag_v2_type_KEYS::get_interface_name_for_reflect,
                    ethernet_controller_driver_bag_v2_type_KEYS::mut_interface_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ethernet_controller_driver_bag_v2_type_KEYS>(
                    "ethernet_controller_driver_bag_v2_type_KEYS",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ethernet_controller_driver_bag_v2_type_KEYS {
    fn clear(&mut self) {
        self.clear_interface_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ethernet_controller_driver_bag_v2_type_KEYS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ethernet_controller_driver_bag_v2_type_KEYS {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ethernet_controller_driver_bag_v2_type {
    // message fields
    pub admin_state: ::std::string::String,
    pub oper_state_up: u32,
    pub phy_info: ::protobuf::SingularPtrField<eth_ctrlr_phy_info>,
    pub layer1_info: ::protobuf::SingularPtrField<eth_ctrlr_l1_info>,
    pub mac_info: ::protobuf::SingularPtrField<eth_ctrlr_mac_info>,
    pub transport_info: ::protobuf::SingularPtrField<eth_ctlr_transport_info>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ethernet_controller_driver_bag_v2_type {}

impl ethernet_controller_driver_bag_v2_type {
    pub fn new() -> ethernet_controller_driver_bag_v2_type {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ethernet_controller_driver_bag_v2_type {
        static mut instance: ::protobuf::lazy::Lazy<ethernet_controller_driver_bag_v2_type> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ethernet_controller_driver_bag_v2_type,
        };
        unsafe {
            instance.get(ethernet_controller_driver_bag_v2_type::new)
        }
    }

    // string admin_state = 50;

    pub fn clear_admin_state(&mut self) {
        self.admin_state.clear();
    }

    // Param is passed by value, moved
    pub fn set_admin_state(&mut self, v: ::std::string::String) {
        self.admin_state = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin_state(&mut self) -> &mut ::std::string::String {
        &mut self.admin_state
    }

    // Take field
    pub fn take_admin_state(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.admin_state, ::std::string::String::new())
    }

    pub fn get_admin_state(&self) -> &str {
        &self.admin_state
    }

    fn get_admin_state_for_reflect(&self) -> &::std::string::String {
        &self.admin_state
    }

    fn mut_admin_state_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.admin_state
    }

    // uint32 oper_state_up = 51;

    pub fn clear_oper_state_up(&mut self) {
        self.oper_state_up = 0;
    }

    // Param is passed by value, moved
    pub fn set_oper_state_up(&mut self, v: u32) {
        self.oper_state_up = v;
    }

    pub fn get_oper_state_up(&self) -> u32 {
        self.oper_state_up
    }

    fn get_oper_state_up_for_reflect(&self) -> &u32 {
        &self.oper_state_up
    }

    fn mut_oper_state_up_for_reflect(&mut self) -> &mut u32 {
        &mut self.oper_state_up
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_info phy_info = 52;

    pub fn clear_phy_info(&mut self) {
        self.phy_info.clear();
    }

    pub fn has_phy_info(&self) -> bool {
        self.phy_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phy_info(&mut self, v: eth_ctrlr_phy_info) {
        self.phy_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_phy_info(&mut self) -> &mut eth_ctrlr_phy_info {
        if self.phy_info.is_none() {
            self.phy_info.set_default();
        }
        self.phy_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_phy_info(&mut self) -> eth_ctrlr_phy_info {
        self.phy_info.take().unwrap_or_else(|| eth_ctrlr_phy_info::new())
    }

    pub fn get_phy_info(&self) -> &eth_ctrlr_phy_info {
        self.phy_info.as_ref().unwrap_or_else(|| eth_ctrlr_phy_info::default_instance())
    }

    fn get_phy_info_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_phy_info> {
        &self.phy_info
    }

    fn mut_phy_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_phy_info> {
        &mut self.phy_info
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_l1_info layer1_info = 53;

    pub fn clear_layer1_info(&mut self) {
        self.layer1_info.clear();
    }

    pub fn has_layer1_info(&self) -> bool {
        self.layer1_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_layer1_info(&mut self, v: eth_ctrlr_l1_info) {
        self.layer1_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_layer1_info(&mut self) -> &mut eth_ctrlr_l1_info {
        if self.layer1_info.is_none() {
            self.layer1_info.set_default();
        }
        self.layer1_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_layer1_info(&mut self) -> eth_ctrlr_l1_info {
        self.layer1_info.take().unwrap_or_else(|| eth_ctrlr_l1_info::new())
    }

    pub fn get_layer1_info(&self) -> &eth_ctrlr_l1_info {
        self.layer1_info.as_ref().unwrap_or_else(|| eth_ctrlr_l1_info::default_instance())
    }

    fn get_layer1_info_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_l1_info> {
        &self.layer1_info
    }

    fn mut_layer1_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_l1_info> {
        &mut self.layer1_info
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_mac_info mac_info = 54;

    pub fn clear_mac_info(&mut self) {
        self.mac_info.clear();
    }

    pub fn has_mac_info(&self) -> bool {
        self.mac_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mac_info(&mut self, v: eth_ctrlr_mac_info) {
        self.mac_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac_info(&mut self) -> &mut eth_ctrlr_mac_info {
        if self.mac_info.is_none() {
            self.mac_info.set_default();
        }
        self.mac_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_mac_info(&mut self) -> eth_ctrlr_mac_info {
        self.mac_info.take().unwrap_or_else(|| eth_ctrlr_mac_info::new())
    }

    pub fn get_mac_info(&self) -> &eth_ctrlr_mac_info {
        self.mac_info.as_ref().unwrap_or_else(|| eth_ctrlr_mac_info::default_instance())
    }

    fn get_mac_info_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_mac_info> {
        &self.mac_info
    }

    fn mut_mac_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_mac_info> {
        &mut self.mac_info
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctlr_transport_info transport_info = 55;

    pub fn clear_transport_info(&mut self) {
        self.transport_info.clear();
    }

    pub fn has_transport_info(&self) -> bool {
        self.transport_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transport_info(&mut self, v: eth_ctlr_transport_info) {
        self.transport_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transport_info(&mut self) -> &mut eth_ctlr_transport_info {
        if self.transport_info.is_none() {
            self.transport_info.set_default();
        }
        self.transport_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_transport_info(&mut self) -> eth_ctlr_transport_info {
        self.transport_info.take().unwrap_or_else(|| eth_ctlr_transport_info::new())
    }

    pub fn get_transport_info(&self) -> &eth_ctlr_transport_info {
        self.transport_info.as_ref().unwrap_or_else(|| eth_ctlr_transport_info::default_instance())
    }

    fn get_transport_info_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctlr_transport_info> {
        &self.transport_info
    }

    fn mut_transport_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctlr_transport_info> {
        &mut self.transport_info
    }
}

impl ::protobuf::Message for ethernet_controller_driver_bag_v2_type {
    fn is_initialized(&self) -> bool {
        for v in &self.phy_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.layer1_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.mac_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.transport_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                50 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.admin_state)?;
                },
                51 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.oper_state_up = tmp;
                },
                52 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.phy_info)?;
                },
                53 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.layer1_info)?;
                },
                54 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mac_info)?;
                },
                55 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transport_info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.admin_state.is_empty() {
            my_size += ::protobuf::rt::string_size(50, &self.admin_state);
        }
        if self.oper_state_up != 0 {
            my_size += ::protobuf::rt::value_size(51, self.oper_state_up, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.phy_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.layer1_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.mac_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.transport_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.admin_state.is_empty() {
            os.write_string(50, &self.admin_state)?;
        }
        if self.oper_state_up != 0 {
            os.write_uint32(51, self.oper_state_up)?;
        }
        if let Some(ref v) = self.phy_info.as_ref() {
            os.write_tag(52, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.layer1_info.as_ref() {
            os.write_tag(53, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.mac_info.as_ref() {
            os.write_tag(54, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.transport_info.as_ref() {
            os.write_tag(55, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ethernet_controller_driver_bag_v2_type {
    fn new() -> ethernet_controller_driver_bag_v2_type {
        ethernet_controller_driver_bag_v2_type::new()
    }

    fn descriptor_static(_: ::std::option::Option<ethernet_controller_driver_bag_v2_type>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "admin_state",
                    ethernet_controller_driver_bag_v2_type::get_admin_state_for_reflect,
                    ethernet_controller_driver_bag_v2_type::mut_admin_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "oper_state_up",
                    ethernet_controller_driver_bag_v2_type::get_oper_state_up_for_reflect,
                    ethernet_controller_driver_bag_v2_type::mut_oper_state_up_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_info>>(
                    "phy_info",
                    ethernet_controller_driver_bag_v2_type::get_phy_info_for_reflect,
                    ethernet_controller_driver_bag_v2_type::mut_phy_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_l1_info>>(
                    "layer1_info",
                    ethernet_controller_driver_bag_v2_type::get_layer1_info_for_reflect,
                    ethernet_controller_driver_bag_v2_type::mut_layer1_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_mac_info>>(
                    "mac_info",
                    ethernet_controller_driver_bag_v2_type::get_mac_info_for_reflect,
                    ethernet_controller_driver_bag_v2_type::mut_mac_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctlr_transport_info>>(
                    "transport_info",
                    ethernet_controller_driver_bag_v2_type::get_transport_info_for_reflect,
                    ethernet_controller_driver_bag_v2_type::mut_transport_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ethernet_controller_driver_bag_v2_type>(
                    "ethernet_controller_driver_bag_v2_type",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ethernet_controller_driver_bag_v2_type {
    fn clear(&mut self) {
        self.clear_admin_state();
        self.clear_oper_state_up();
        self.clear_phy_info();
        self.clear_layer1_info();
        self.clear_mac_info();
        self.clear_transport_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ethernet_controller_driver_bag_v2_type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ethernet_controller_driver_bag_v2_type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ether_autoneg_ {
    // message fields
    pub autoneg_enabled: i32,
    pub mask: u32,
    pub speed: ::std::string::String,
    pub duplex: ::std::string::String,
    pub flowcontrol: ::std::string::String,
    pub config_override: i32,
    pub fec: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ether_autoneg_ {}

impl ether_autoneg_ {
    pub fn new() -> ether_autoneg_ {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ether_autoneg_ {
        static mut instance: ::protobuf::lazy::Lazy<ether_autoneg_> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ether_autoneg_,
        };
        unsafe {
            instance.get(ether_autoneg_::new)
        }
    }

    // sint32 autoneg_enabled = 1;

    pub fn clear_autoneg_enabled(&mut self) {
        self.autoneg_enabled = 0;
    }

    // Param is passed by value, moved
    pub fn set_autoneg_enabled(&mut self, v: i32) {
        self.autoneg_enabled = v;
    }

    pub fn get_autoneg_enabled(&self) -> i32 {
        self.autoneg_enabled
    }

    fn get_autoneg_enabled_for_reflect(&self) -> &i32 {
        &self.autoneg_enabled
    }

    fn mut_autoneg_enabled_for_reflect(&mut self) -> &mut i32 {
        &mut self.autoneg_enabled
    }

    // uint32 mask = 2;

    pub fn clear_mask(&mut self) {
        self.mask = 0;
    }

    // Param is passed by value, moved
    pub fn set_mask(&mut self, v: u32) {
        self.mask = v;
    }

    pub fn get_mask(&self) -> u32 {
        self.mask
    }

    fn get_mask_for_reflect(&self) -> &u32 {
        &self.mask
    }

    fn mut_mask_for_reflect(&mut self) -> &mut u32 {
        &mut self.mask
    }

    // string speed = 3;

    pub fn clear_speed(&mut self) {
        self.speed.clear();
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: ::std::string::String) {
        self.speed = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_speed(&mut self) -> &mut ::std::string::String {
        &mut self.speed
    }

    // Take field
    pub fn take_speed(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.speed, ::std::string::String::new())
    }

    pub fn get_speed(&self) -> &str {
        &self.speed
    }

    fn get_speed_for_reflect(&self) -> &::std::string::String {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.speed
    }

    // string duplex = 4;

    pub fn clear_duplex(&mut self) {
        self.duplex.clear();
    }

    // Param is passed by value, moved
    pub fn set_duplex(&mut self, v: ::std::string::String) {
        self.duplex = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_duplex(&mut self) -> &mut ::std::string::String {
        &mut self.duplex
    }

    // Take field
    pub fn take_duplex(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.duplex, ::std::string::String::new())
    }

    pub fn get_duplex(&self) -> &str {
        &self.duplex
    }

    fn get_duplex_for_reflect(&self) -> &::std::string::String {
        &self.duplex
    }

    fn mut_duplex_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.duplex
    }

    // string flowcontrol = 5;

    pub fn clear_flowcontrol(&mut self) {
        self.flowcontrol.clear();
    }

    // Param is passed by value, moved
    pub fn set_flowcontrol(&mut self, v: ::std::string::String) {
        self.flowcontrol = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_flowcontrol(&mut self) -> &mut ::std::string::String {
        &mut self.flowcontrol
    }

    // Take field
    pub fn take_flowcontrol(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.flowcontrol, ::std::string::String::new())
    }

    pub fn get_flowcontrol(&self) -> &str {
        &self.flowcontrol
    }

    fn get_flowcontrol_for_reflect(&self) -> &::std::string::String {
        &self.flowcontrol
    }

    fn mut_flowcontrol_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.flowcontrol
    }

    // sint32 config_override = 6;

    pub fn clear_config_override(&mut self) {
        self.config_override = 0;
    }

    // Param is passed by value, moved
    pub fn set_config_override(&mut self, v: i32) {
        self.config_override = v;
    }

    pub fn get_config_override(&self) -> i32 {
        self.config_override
    }

    fn get_config_override_for_reflect(&self) -> &i32 {
        &self.config_override
    }

    fn mut_config_override_for_reflect(&mut self) -> &mut i32 {
        &mut self.config_override
    }

    // string fec = 7;

    pub fn clear_fec(&mut self) {
        self.fec.clear();
    }

    // Param is passed by value, moved
    pub fn set_fec(&mut self, v: ::std::string::String) {
        self.fec = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fec(&mut self) -> &mut ::std::string::String {
        &mut self.fec
    }

    // Take field
    pub fn take_fec(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.fec, ::std::string::String::new())
    }

    pub fn get_fec(&self) -> &str {
        &self.fec
    }

    fn get_fec_for_reflect(&self) -> &::std::string::String {
        &self.fec
    }

    fn mut_fec_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.fec
    }
}

impl ::protobuf::Message for ether_autoneg_ {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.autoneg_enabled = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mask = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.speed)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.duplex)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.flowcontrol)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.config_override = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.fec)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.autoneg_enabled != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.autoneg_enabled);
        }
        if self.mask != 0 {
            my_size += ::protobuf::rt::value_size(2, self.mask, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.speed.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.speed);
        }
        if !self.duplex.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.duplex);
        }
        if !self.flowcontrol.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.flowcontrol);
        }
        if self.config_override != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, self.config_override);
        }
        if !self.fec.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.fec);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.autoneg_enabled != 0 {
            os.write_sint32(1, self.autoneg_enabled)?;
        }
        if self.mask != 0 {
            os.write_uint32(2, self.mask)?;
        }
        if !self.speed.is_empty() {
            os.write_string(3, &self.speed)?;
        }
        if !self.duplex.is_empty() {
            os.write_string(4, &self.duplex)?;
        }
        if !self.flowcontrol.is_empty() {
            os.write_string(5, &self.flowcontrol)?;
        }
        if self.config_override != 0 {
            os.write_sint32(6, self.config_override)?;
        }
        if !self.fec.is_empty() {
            os.write_string(7, &self.fec)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ether_autoneg_ {
    fn new() -> ether_autoneg_ {
        ether_autoneg_::new()
    }

    fn descriptor_static(_: ::std::option::Option<ether_autoneg_>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "autoneg_enabled",
                    ether_autoneg_::get_autoneg_enabled_for_reflect,
                    ether_autoneg_::mut_autoneg_enabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mask",
                    ether_autoneg_::get_mask_for_reflect,
                    ether_autoneg_::mut_mask_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "speed",
                    ether_autoneg_::get_speed_for_reflect,
                    ether_autoneg_::mut_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "duplex",
                    ether_autoneg_::get_duplex_for_reflect,
                    ether_autoneg_::mut_duplex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "flowcontrol",
                    ether_autoneg_::get_flowcontrol_for_reflect,
                    ether_autoneg_::mut_flowcontrol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "config_override",
                    ether_autoneg_::get_config_override_for_reflect,
                    ether_autoneg_::mut_config_override_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fec",
                    ether_autoneg_::get_fec_for_reflect,
                    ether_autoneg_::mut_fec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ether_autoneg_>(
                    "ether_autoneg_",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ether_autoneg_ {
    fn clear(&mut self) {
        self.clear_autoneg_enabled();
        self.clear_mask();
        self.clear_speed();
        self.clear_duplex();
        self.clear_flowcontrol();
        self.clear_config_override();
        self.clear_fec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ether_autoneg_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ether_autoneg_ {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct mac_addr_type {
    // message fields
    pub value: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for mac_addr_type {}

impl mac_addr_type {
    pub fn new() -> mac_addr_type {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static mac_addr_type {
        static mut instance: ::protobuf::lazy::Lazy<mac_addr_type> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const mac_addr_type,
        };
        unsafe {
            instance.get(mac_addr_type::new)
        }
    }

    // string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::string::String {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.value
    }
}

impl ::protobuf::Message for mac_addr_type {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_string(1, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for mac_addr_type {
    fn new() -> mac_addr_type {
        mac_addr_type::new()
    }

    fn descriptor_static(_: ::std::option::Option<mac_addr_type>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    mac_addr_type::get_value_for_reflect,
                    mac_addr_type::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<mac_addr_type>(
                    "mac_addr_type",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for mac_addr_type {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for mac_addr_type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for mac_addr_type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ether_mcast_mac_type_ {
    // message fields
    pub mac_address: ::std::string::String,
    pub mask: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ether_mcast_mac_type_ {}

impl ether_mcast_mac_type_ {
    pub fn new() -> ether_mcast_mac_type_ {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ether_mcast_mac_type_ {
        static mut instance: ::protobuf::lazy::Lazy<ether_mcast_mac_type_> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ether_mcast_mac_type_,
        };
        unsafe {
            instance.get(ether_mcast_mac_type_::new)
        }
    }

    // string mac_address = 1;

    pub fn clear_mac_address(&mut self) {
        self.mac_address.clear();
    }

    // Param is passed by value, moved
    pub fn set_mac_address(&mut self, v: ::std::string::String) {
        self.mac_address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mac_address(&mut self) -> &mut ::std::string::String {
        &mut self.mac_address
    }

    // Take field
    pub fn take_mac_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mac_address, ::std::string::String::new())
    }

    pub fn get_mac_address(&self) -> &str {
        &self.mac_address
    }

    fn get_mac_address_for_reflect(&self) -> &::std::string::String {
        &self.mac_address
    }

    fn mut_mac_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.mac_address
    }

    // string mask = 2;

    pub fn clear_mask(&mut self) {
        self.mask.clear();
    }

    // Param is passed by value, moved
    pub fn set_mask(&mut self, v: ::std::string::String) {
        self.mask = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mask(&mut self) -> &mut ::std::string::String {
        &mut self.mask
    }

    // Take field
    pub fn take_mask(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.mask, ::std::string::String::new())
    }

    pub fn get_mask(&self) -> &str {
        &self.mask
    }

    fn get_mask_for_reflect(&self) -> &::std::string::String {
        &self.mask
    }

    fn mut_mask_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.mask
    }
}

impl ::protobuf::Message for ether_mcast_mac_type_ {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mac_address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.mask)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.mac_address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.mac_address);
        }
        if !self.mask.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.mask);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.mac_address.is_empty() {
            os.write_string(1, &self.mac_address)?;
        }
        if !self.mask.is_empty() {
            os.write_string(2, &self.mask)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ether_mcast_mac_type_ {
    fn new() -> ether_mcast_mac_type_ {
        ether_mcast_mac_type_::new()
    }

    fn descriptor_static(_: ::std::option::Option<ether_mcast_mac_type_>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mac_address",
                    ether_mcast_mac_type_::get_mac_address_for_reflect,
                    ether_mcast_mac_type_::mut_mac_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mask",
                    ether_mcast_mac_type_::get_mask_for_reflect,
                    ether_mcast_mac_type_::mut_mask_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ether_mcast_mac_type_>(
                    "ether_mcast_mac_type_",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ether_mcast_mac_type_ {
    fn clear(&mut self) {
        self.clear_mac_address();
        self.clear_mask();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ether_mcast_mac_type_ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ether_mcast_mac_type_ {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_dom_lane_alarms {
    // message fields
    pub transmit_laser_power: ::std::string::String,
    pub received_laser_power: ::std::string::String,
    pub laser_bias_current: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_dom_lane_alarms {}

impl eth_ctrlr_phy_dom_lane_alarms {
    pub fn new() -> eth_ctrlr_phy_dom_lane_alarms {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_dom_lane_alarms {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_dom_lane_alarms> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_dom_lane_alarms,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_dom_lane_alarms::new)
        }
    }

    // string transmit_laser_power = 1;

    pub fn clear_transmit_laser_power(&mut self) {
        self.transmit_laser_power.clear();
    }

    // Param is passed by value, moved
    pub fn set_transmit_laser_power(&mut self, v: ::std::string::String) {
        self.transmit_laser_power = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transmit_laser_power(&mut self) -> &mut ::std::string::String {
        &mut self.transmit_laser_power
    }

    // Take field
    pub fn take_transmit_laser_power(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.transmit_laser_power, ::std::string::String::new())
    }

    pub fn get_transmit_laser_power(&self) -> &str {
        &self.transmit_laser_power
    }

    fn get_transmit_laser_power_for_reflect(&self) -> &::std::string::String {
        &self.transmit_laser_power
    }

    fn mut_transmit_laser_power_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.transmit_laser_power
    }

    // string received_laser_power = 2;

    pub fn clear_received_laser_power(&mut self) {
        self.received_laser_power.clear();
    }

    // Param is passed by value, moved
    pub fn set_received_laser_power(&mut self, v: ::std::string::String) {
        self.received_laser_power = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_received_laser_power(&mut self) -> &mut ::std::string::String {
        &mut self.received_laser_power
    }

    // Take field
    pub fn take_received_laser_power(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.received_laser_power, ::std::string::String::new())
    }

    pub fn get_received_laser_power(&self) -> &str {
        &self.received_laser_power
    }

    fn get_received_laser_power_for_reflect(&self) -> &::std::string::String {
        &self.received_laser_power
    }

    fn mut_received_laser_power_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.received_laser_power
    }

    // string laser_bias_current = 3;

    pub fn clear_laser_bias_current(&mut self) {
        self.laser_bias_current.clear();
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_current(&mut self, v: ::std::string::String) {
        self.laser_bias_current = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_laser_bias_current(&mut self) -> &mut ::std::string::String {
        &mut self.laser_bias_current
    }

    // Take field
    pub fn take_laser_bias_current(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.laser_bias_current, ::std::string::String::new())
    }

    pub fn get_laser_bias_current(&self) -> &str {
        &self.laser_bias_current
    }

    fn get_laser_bias_current_for_reflect(&self) -> &::std::string::String {
        &self.laser_bias_current
    }

    fn mut_laser_bias_current_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.laser_bias_current
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_dom_lane_alarms {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.transmit_laser_power)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.received_laser_power)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.laser_bias_current)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.transmit_laser_power.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.transmit_laser_power);
        }
        if !self.received_laser_power.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.received_laser_power);
        }
        if !self.laser_bias_current.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.laser_bias_current);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.transmit_laser_power.is_empty() {
            os.write_string(1, &self.transmit_laser_power)?;
        }
        if !self.received_laser_power.is_empty() {
            os.write_string(2, &self.received_laser_power)?;
        }
        if !self.laser_bias_current.is_empty() {
            os.write_string(3, &self.laser_bias_current)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_dom_lane_alarms {
    fn new() -> eth_ctrlr_phy_dom_lane_alarms {
        eth_ctrlr_phy_dom_lane_alarms::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_dom_lane_alarms>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "transmit_laser_power",
                    eth_ctrlr_phy_dom_lane_alarms::get_transmit_laser_power_for_reflect,
                    eth_ctrlr_phy_dom_lane_alarms::mut_transmit_laser_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "received_laser_power",
                    eth_ctrlr_phy_dom_lane_alarms::get_received_laser_power_for_reflect,
                    eth_ctrlr_phy_dom_lane_alarms::mut_received_laser_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "laser_bias_current",
                    eth_ctrlr_phy_dom_lane_alarms::get_laser_bias_current_for_reflect,
                    eth_ctrlr_phy_dom_lane_alarms::mut_laser_bias_current_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_dom_lane_alarms>(
                    "eth_ctrlr_phy_dom_lane_alarms",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_dom_lane_alarms {
    fn clear(&mut self) {
        self.clear_transmit_laser_power();
        self.clear_received_laser_power();
        self.clear_laser_bias_current();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_dom_lane_alarms {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_dom_lane_alarms {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_lane_opt_mon_validity {
    // message fields
    pub wavelength_valid: i32,
    pub transmit_power_valid: i32,
    pub receive_power_valid: i32,
    pub laser_bias_valid: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_lane_opt_mon_validity {}

impl eth_ctrlr_phy_lane_opt_mon_validity {
    pub fn new() -> eth_ctrlr_phy_lane_opt_mon_validity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_lane_opt_mon_validity {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_lane_opt_mon_validity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_lane_opt_mon_validity,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_lane_opt_mon_validity::new)
        }
    }

    // sint32 wavelength_valid = 1;

    pub fn clear_wavelength_valid(&mut self) {
        self.wavelength_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_wavelength_valid(&mut self, v: i32) {
        self.wavelength_valid = v;
    }

    pub fn get_wavelength_valid(&self) -> i32 {
        self.wavelength_valid
    }

    fn get_wavelength_valid_for_reflect(&self) -> &i32 {
        &self.wavelength_valid
    }

    fn mut_wavelength_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.wavelength_valid
    }

    // sint32 transmit_power_valid = 2;

    pub fn clear_transmit_power_valid(&mut self) {
        self.transmit_power_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_transmit_power_valid(&mut self, v: i32) {
        self.transmit_power_valid = v;
    }

    pub fn get_transmit_power_valid(&self) -> i32 {
        self.transmit_power_valid
    }

    fn get_transmit_power_valid_for_reflect(&self) -> &i32 {
        &self.transmit_power_valid
    }

    fn mut_transmit_power_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.transmit_power_valid
    }

    // sint32 receive_power_valid = 3;

    pub fn clear_receive_power_valid(&mut self) {
        self.receive_power_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_receive_power_valid(&mut self, v: i32) {
        self.receive_power_valid = v;
    }

    pub fn get_receive_power_valid(&self) -> i32 {
        self.receive_power_valid
    }

    fn get_receive_power_valid_for_reflect(&self) -> &i32 {
        &self.receive_power_valid
    }

    fn mut_receive_power_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.receive_power_valid
    }

    // sint32 laser_bias_valid = 4;

    pub fn clear_laser_bias_valid(&mut self) {
        self.laser_bias_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_valid(&mut self, v: i32) {
        self.laser_bias_valid = v;
    }

    pub fn get_laser_bias_valid(&self) -> i32 {
        self.laser_bias_valid
    }

    fn get_laser_bias_valid_for_reflect(&self) -> &i32 {
        &self.laser_bias_valid
    }

    fn mut_laser_bias_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.laser_bias_valid
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_lane_opt_mon_validity {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.wavelength_valid = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transmit_power_valid = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.receive_power_valid = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.laser_bias_valid = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.wavelength_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.wavelength_valid);
        }
        if self.transmit_power_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.transmit_power_valid);
        }
        if self.receive_power_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, self.receive_power_valid);
        }
        if self.laser_bias_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, self.laser_bias_valid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.wavelength_valid != 0 {
            os.write_sint32(1, self.wavelength_valid)?;
        }
        if self.transmit_power_valid != 0 {
            os.write_sint32(2, self.transmit_power_valid)?;
        }
        if self.receive_power_valid != 0 {
            os.write_sint32(3, self.receive_power_valid)?;
        }
        if self.laser_bias_valid != 0 {
            os.write_sint32(4, self.laser_bias_valid)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_lane_opt_mon_validity {
    fn new() -> eth_ctrlr_phy_lane_opt_mon_validity {
        eth_ctrlr_phy_lane_opt_mon_validity::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_lane_opt_mon_validity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "wavelength_valid",
                    eth_ctrlr_phy_lane_opt_mon_validity::get_wavelength_valid_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon_validity::mut_wavelength_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transmit_power_valid",
                    eth_ctrlr_phy_lane_opt_mon_validity::get_transmit_power_valid_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon_validity::mut_transmit_power_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "receive_power_valid",
                    eth_ctrlr_phy_lane_opt_mon_validity::get_receive_power_valid_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon_validity::mut_receive_power_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "laser_bias_valid",
                    eth_ctrlr_phy_lane_opt_mon_validity::get_laser_bias_valid_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon_validity::mut_laser_bias_valid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_lane_opt_mon_validity>(
                    "eth_ctrlr_phy_lane_opt_mon_validity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_lane_opt_mon_validity {
    fn clear(&mut self) {
        self.clear_wavelength_valid();
        self.clear_transmit_power_valid();
        self.clear_receive_power_valid();
        self.clear_laser_bias_valid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_lane_opt_mon_validity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_lane_opt_mon_validity {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_lane_opt_mon {
    // message fields
    pub center_wavelength: u32,
    pub transmit_laser_power: i32,
    pub received_laser_power: i32,
    pub laser_bias_current: u32,
    pub dig_opt_mon_alarm: ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_lane_alarms>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_lane_opt_mon {}

impl eth_ctrlr_phy_lane_opt_mon {
    pub fn new() -> eth_ctrlr_phy_lane_opt_mon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_lane_opt_mon {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_lane_opt_mon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_lane_opt_mon,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_lane_opt_mon::new)
        }
    }

    // uint32 center_wavelength = 1;

    pub fn clear_center_wavelength(&mut self) {
        self.center_wavelength = 0;
    }

    // Param is passed by value, moved
    pub fn set_center_wavelength(&mut self, v: u32) {
        self.center_wavelength = v;
    }

    pub fn get_center_wavelength(&self) -> u32 {
        self.center_wavelength
    }

    fn get_center_wavelength_for_reflect(&self) -> &u32 {
        &self.center_wavelength
    }

    fn mut_center_wavelength_for_reflect(&mut self) -> &mut u32 {
        &mut self.center_wavelength
    }

    // sint32 transmit_laser_power = 2;

    pub fn clear_transmit_laser_power(&mut self) {
        self.transmit_laser_power = 0;
    }

    // Param is passed by value, moved
    pub fn set_transmit_laser_power(&mut self, v: i32) {
        self.transmit_laser_power = v;
    }

    pub fn get_transmit_laser_power(&self) -> i32 {
        self.transmit_laser_power
    }

    fn get_transmit_laser_power_for_reflect(&self) -> &i32 {
        &self.transmit_laser_power
    }

    fn mut_transmit_laser_power_for_reflect(&mut self) -> &mut i32 {
        &mut self.transmit_laser_power
    }

    // sint32 received_laser_power = 3;

    pub fn clear_received_laser_power(&mut self) {
        self.received_laser_power = 0;
    }

    // Param is passed by value, moved
    pub fn set_received_laser_power(&mut self, v: i32) {
        self.received_laser_power = v;
    }

    pub fn get_received_laser_power(&self) -> i32 {
        self.received_laser_power
    }

    fn get_received_laser_power_for_reflect(&self) -> &i32 {
        &self.received_laser_power
    }

    fn mut_received_laser_power_for_reflect(&mut self) -> &mut i32 {
        &mut self.received_laser_power
    }

    // uint32 laser_bias_current = 4;

    pub fn clear_laser_bias_current(&mut self) {
        self.laser_bias_current = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_current(&mut self, v: u32) {
        self.laser_bias_current = v;
    }

    pub fn get_laser_bias_current(&self) -> u32 {
        self.laser_bias_current
    }

    fn get_laser_bias_current_for_reflect(&self) -> &u32 {
        &self.laser_bias_current
    }

    fn mut_laser_bias_current_for_reflect(&mut self) -> &mut u32 {
        &mut self.laser_bias_current
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_dom_lane_alarms dig_opt_mon_alarm = 5;

    pub fn clear_dig_opt_mon_alarm(&mut self) {
        self.dig_opt_mon_alarm.clear();
    }

    pub fn has_dig_opt_mon_alarm(&self) -> bool {
        self.dig_opt_mon_alarm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dig_opt_mon_alarm(&mut self, v: eth_ctrlr_phy_dom_lane_alarms) {
        self.dig_opt_mon_alarm = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dig_opt_mon_alarm(&mut self) -> &mut eth_ctrlr_phy_dom_lane_alarms {
        if self.dig_opt_mon_alarm.is_none() {
            self.dig_opt_mon_alarm.set_default();
        }
        self.dig_opt_mon_alarm.as_mut().unwrap()
    }

    // Take field
    pub fn take_dig_opt_mon_alarm(&mut self) -> eth_ctrlr_phy_dom_lane_alarms {
        self.dig_opt_mon_alarm.take().unwrap_or_else(|| eth_ctrlr_phy_dom_lane_alarms::new())
    }

    pub fn get_dig_opt_mon_alarm(&self) -> &eth_ctrlr_phy_dom_lane_alarms {
        self.dig_opt_mon_alarm.as_ref().unwrap_or_else(|| eth_ctrlr_phy_dom_lane_alarms::default_instance())
    }

    fn get_dig_opt_mon_alarm_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_phy_dom_lane_alarms> {
        &self.dig_opt_mon_alarm
    }

    fn mut_dig_opt_mon_alarm_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_lane_alarms> {
        &mut self.dig_opt_mon_alarm
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_lane_opt_mon {
    fn is_initialized(&self) -> bool {
        for v in &self.dig_opt_mon_alarm {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.center_wavelength = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transmit_laser_power = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.received_laser_power = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.laser_bias_current = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dig_opt_mon_alarm)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.center_wavelength != 0 {
            my_size += ::protobuf::rt::value_size(1, self.center_wavelength, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.transmit_laser_power != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.transmit_laser_power);
        }
        if self.received_laser_power != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, self.received_laser_power);
        }
        if self.laser_bias_current != 0 {
            my_size += ::protobuf::rt::value_size(4, self.laser_bias_current, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.dig_opt_mon_alarm.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.center_wavelength != 0 {
            os.write_uint32(1, self.center_wavelength)?;
        }
        if self.transmit_laser_power != 0 {
            os.write_sint32(2, self.transmit_laser_power)?;
        }
        if self.received_laser_power != 0 {
            os.write_sint32(3, self.received_laser_power)?;
        }
        if self.laser_bias_current != 0 {
            os.write_uint32(4, self.laser_bias_current)?;
        }
        if let Some(ref v) = self.dig_opt_mon_alarm.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_lane_opt_mon {
    fn new() -> eth_ctrlr_phy_lane_opt_mon {
        eth_ctrlr_phy_lane_opt_mon::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_lane_opt_mon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "center_wavelength",
                    eth_ctrlr_phy_lane_opt_mon::get_center_wavelength_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon::mut_center_wavelength_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transmit_laser_power",
                    eth_ctrlr_phy_lane_opt_mon::get_transmit_laser_power_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon::mut_transmit_laser_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "received_laser_power",
                    eth_ctrlr_phy_lane_opt_mon::get_received_laser_power_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon::mut_received_laser_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "laser_bias_current",
                    eth_ctrlr_phy_lane_opt_mon::get_laser_bias_current_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon::mut_laser_bias_current_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_dom_lane_alarms>>(
                    "dig_opt_mon_alarm",
                    eth_ctrlr_phy_lane_opt_mon::get_dig_opt_mon_alarm_for_reflect,
                    eth_ctrlr_phy_lane_opt_mon::mut_dig_opt_mon_alarm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_lane_opt_mon>(
                    "eth_ctrlr_phy_lane_opt_mon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_lane_opt_mon {
    fn clear(&mut self) {
        self.clear_center_wavelength();
        self.clear_transmit_laser_power();
        self.clear_received_laser_power();
        self.clear_laser_bias_current();
        self.clear_dig_opt_mon_alarm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_lane_opt_mon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_lane_opt_mon {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_dom_alarms {
    // message fields
    pub transceiver_temperature: ::std::string::String,
    pub transceiver_voltage: ::std::string::String,
    pub transmit_laser_power: ::std::string::String,
    pub received_laser_power: ::std::string::String,
    pub laser_bias_current: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_dom_alarms {}

impl eth_ctrlr_phy_dom_alarms {
    pub fn new() -> eth_ctrlr_phy_dom_alarms {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_dom_alarms {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_dom_alarms> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_dom_alarms,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_dom_alarms::new)
        }
    }

    // string transceiver_temperature = 1;

    pub fn clear_transceiver_temperature(&mut self) {
        self.transceiver_temperature.clear();
    }

    // Param is passed by value, moved
    pub fn set_transceiver_temperature(&mut self, v: ::std::string::String) {
        self.transceiver_temperature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transceiver_temperature(&mut self) -> &mut ::std::string::String {
        &mut self.transceiver_temperature
    }

    // Take field
    pub fn take_transceiver_temperature(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.transceiver_temperature, ::std::string::String::new())
    }

    pub fn get_transceiver_temperature(&self) -> &str {
        &self.transceiver_temperature
    }

    fn get_transceiver_temperature_for_reflect(&self) -> &::std::string::String {
        &self.transceiver_temperature
    }

    fn mut_transceiver_temperature_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.transceiver_temperature
    }

    // string transceiver_voltage = 2;

    pub fn clear_transceiver_voltage(&mut self) {
        self.transceiver_voltage.clear();
    }

    // Param is passed by value, moved
    pub fn set_transceiver_voltage(&mut self, v: ::std::string::String) {
        self.transceiver_voltage = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transceiver_voltage(&mut self) -> &mut ::std::string::String {
        &mut self.transceiver_voltage
    }

    // Take field
    pub fn take_transceiver_voltage(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.transceiver_voltage, ::std::string::String::new())
    }

    pub fn get_transceiver_voltage(&self) -> &str {
        &self.transceiver_voltage
    }

    fn get_transceiver_voltage_for_reflect(&self) -> &::std::string::String {
        &self.transceiver_voltage
    }

    fn mut_transceiver_voltage_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.transceiver_voltage
    }

    // string transmit_laser_power = 3;

    pub fn clear_transmit_laser_power(&mut self) {
        self.transmit_laser_power.clear();
    }

    // Param is passed by value, moved
    pub fn set_transmit_laser_power(&mut self, v: ::std::string::String) {
        self.transmit_laser_power = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transmit_laser_power(&mut self) -> &mut ::std::string::String {
        &mut self.transmit_laser_power
    }

    // Take field
    pub fn take_transmit_laser_power(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.transmit_laser_power, ::std::string::String::new())
    }

    pub fn get_transmit_laser_power(&self) -> &str {
        &self.transmit_laser_power
    }

    fn get_transmit_laser_power_for_reflect(&self) -> &::std::string::String {
        &self.transmit_laser_power
    }

    fn mut_transmit_laser_power_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.transmit_laser_power
    }

    // string received_laser_power = 4;

    pub fn clear_received_laser_power(&mut self) {
        self.received_laser_power.clear();
    }

    // Param is passed by value, moved
    pub fn set_received_laser_power(&mut self, v: ::std::string::String) {
        self.received_laser_power = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_received_laser_power(&mut self) -> &mut ::std::string::String {
        &mut self.received_laser_power
    }

    // Take field
    pub fn take_received_laser_power(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.received_laser_power, ::std::string::String::new())
    }

    pub fn get_received_laser_power(&self) -> &str {
        &self.received_laser_power
    }

    fn get_received_laser_power_for_reflect(&self) -> &::std::string::String {
        &self.received_laser_power
    }

    fn mut_received_laser_power_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.received_laser_power
    }

    // string laser_bias_current = 5;

    pub fn clear_laser_bias_current(&mut self) {
        self.laser_bias_current.clear();
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_current(&mut self, v: ::std::string::String) {
        self.laser_bias_current = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_laser_bias_current(&mut self) -> &mut ::std::string::String {
        &mut self.laser_bias_current
    }

    // Take field
    pub fn take_laser_bias_current(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.laser_bias_current, ::std::string::String::new())
    }

    pub fn get_laser_bias_current(&self) -> &str {
        &self.laser_bias_current
    }

    fn get_laser_bias_current_for_reflect(&self) -> &::std::string::String {
        &self.laser_bias_current
    }

    fn mut_laser_bias_current_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.laser_bias_current
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_dom_alarms {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.transceiver_temperature)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.transceiver_voltage)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.transmit_laser_power)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.received_laser_power)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.laser_bias_current)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.transceiver_temperature.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.transceiver_temperature);
        }
        if !self.transceiver_voltage.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.transceiver_voltage);
        }
        if !self.transmit_laser_power.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.transmit_laser_power);
        }
        if !self.received_laser_power.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.received_laser_power);
        }
        if !self.laser_bias_current.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.laser_bias_current);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.transceiver_temperature.is_empty() {
            os.write_string(1, &self.transceiver_temperature)?;
        }
        if !self.transceiver_voltage.is_empty() {
            os.write_string(2, &self.transceiver_voltage)?;
        }
        if !self.transmit_laser_power.is_empty() {
            os.write_string(3, &self.transmit_laser_power)?;
        }
        if !self.received_laser_power.is_empty() {
            os.write_string(4, &self.received_laser_power)?;
        }
        if !self.laser_bias_current.is_empty() {
            os.write_string(5, &self.laser_bias_current)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_dom_alarms {
    fn new() -> eth_ctrlr_phy_dom_alarms {
        eth_ctrlr_phy_dom_alarms::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_dom_alarms>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "transceiver_temperature",
                    eth_ctrlr_phy_dom_alarms::get_transceiver_temperature_for_reflect,
                    eth_ctrlr_phy_dom_alarms::mut_transceiver_temperature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "transceiver_voltage",
                    eth_ctrlr_phy_dom_alarms::get_transceiver_voltage_for_reflect,
                    eth_ctrlr_phy_dom_alarms::mut_transceiver_voltage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "transmit_laser_power",
                    eth_ctrlr_phy_dom_alarms::get_transmit_laser_power_for_reflect,
                    eth_ctrlr_phy_dom_alarms::mut_transmit_laser_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "received_laser_power",
                    eth_ctrlr_phy_dom_alarms::get_received_laser_power_for_reflect,
                    eth_ctrlr_phy_dom_alarms::mut_received_laser_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "laser_bias_current",
                    eth_ctrlr_phy_dom_alarms::get_laser_bias_current_for_reflect,
                    eth_ctrlr_phy_dom_alarms::mut_laser_bias_current_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_dom_alarms>(
                    "eth_ctrlr_phy_dom_alarms",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_dom_alarms {
    fn clear(&mut self) {
        self.clear_transceiver_temperature();
        self.clear_transceiver_voltage();
        self.clear_transmit_laser_power();
        self.clear_received_laser_power();
        self.clear_laser_bias_current();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_dom_alarms {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_dom_alarms {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_dom_threshold_validity {
    // message fields
    pub temperature_valid: i32,
    pub voltage_valid: i32,
    pub laser_bias_valid: i32,
    pub transmit_power_valid: i32,
    pub receive_power_valid: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_dom_threshold_validity {}

impl eth_ctrlr_phy_dom_threshold_validity {
    pub fn new() -> eth_ctrlr_phy_dom_threshold_validity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_dom_threshold_validity {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_dom_threshold_validity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_dom_threshold_validity,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_dom_threshold_validity::new)
        }
    }

    // sint32 temperature_valid = 1;

    pub fn clear_temperature_valid(&mut self) {
        self.temperature_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_temperature_valid(&mut self, v: i32) {
        self.temperature_valid = v;
    }

    pub fn get_temperature_valid(&self) -> i32 {
        self.temperature_valid
    }

    fn get_temperature_valid_for_reflect(&self) -> &i32 {
        &self.temperature_valid
    }

    fn mut_temperature_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.temperature_valid
    }

    // sint32 voltage_valid = 2;

    pub fn clear_voltage_valid(&mut self) {
        self.voltage_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_voltage_valid(&mut self, v: i32) {
        self.voltage_valid = v;
    }

    pub fn get_voltage_valid(&self) -> i32 {
        self.voltage_valid
    }

    fn get_voltage_valid_for_reflect(&self) -> &i32 {
        &self.voltage_valid
    }

    fn mut_voltage_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.voltage_valid
    }

    // sint32 laser_bias_valid = 3;

    pub fn clear_laser_bias_valid(&mut self) {
        self.laser_bias_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_valid(&mut self, v: i32) {
        self.laser_bias_valid = v;
    }

    pub fn get_laser_bias_valid(&self) -> i32 {
        self.laser_bias_valid
    }

    fn get_laser_bias_valid_for_reflect(&self) -> &i32 {
        &self.laser_bias_valid
    }

    fn mut_laser_bias_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.laser_bias_valid
    }

    // sint32 transmit_power_valid = 4;

    pub fn clear_transmit_power_valid(&mut self) {
        self.transmit_power_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_transmit_power_valid(&mut self, v: i32) {
        self.transmit_power_valid = v;
    }

    pub fn get_transmit_power_valid(&self) -> i32 {
        self.transmit_power_valid
    }

    fn get_transmit_power_valid_for_reflect(&self) -> &i32 {
        &self.transmit_power_valid
    }

    fn mut_transmit_power_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.transmit_power_valid
    }

    // sint32 receive_power_valid = 5;

    pub fn clear_receive_power_valid(&mut self) {
        self.receive_power_valid = 0;
    }

    // Param is passed by value, moved
    pub fn set_receive_power_valid(&mut self, v: i32) {
        self.receive_power_valid = v;
    }

    pub fn get_receive_power_valid(&self) -> i32 {
        self.receive_power_valid
    }

    fn get_receive_power_valid_for_reflect(&self) -> &i32 {
        &self.receive_power_valid
    }

    fn mut_receive_power_valid_for_reflect(&mut self) -> &mut i32 {
        &mut self.receive_power_valid
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_dom_threshold_validity {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.temperature_valid = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.voltage_valid = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.laser_bias_valid = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transmit_power_valid = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.receive_power_valid = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.temperature_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.temperature_valid);
        }
        if self.voltage_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.voltage_valid);
        }
        if self.laser_bias_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, self.laser_bias_valid);
        }
        if self.transmit_power_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, self.transmit_power_valid);
        }
        if self.receive_power_valid != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, self.receive_power_valid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.temperature_valid != 0 {
            os.write_sint32(1, self.temperature_valid)?;
        }
        if self.voltage_valid != 0 {
            os.write_sint32(2, self.voltage_valid)?;
        }
        if self.laser_bias_valid != 0 {
            os.write_sint32(3, self.laser_bias_valid)?;
        }
        if self.transmit_power_valid != 0 {
            os.write_sint32(4, self.transmit_power_valid)?;
        }
        if self.receive_power_valid != 0 {
            os.write_sint32(5, self.receive_power_valid)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_dom_threshold_validity {
    fn new() -> eth_ctrlr_phy_dom_threshold_validity {
        eth_ctrlr_phy_dom_threshold_validity::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_dom_threshold_validity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "temperature_valid",
                    eth_ctrlr_phy_dom_threshold_validity::get_temperature_valid_for_reflect,
                    eth_ctrlr_phy_dom_threshold_validity::mut_temperature_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "voltage_valid",
                    eth_ctrlr_phy_dom_threshold_validity::get_voltage_valid_for_reflect,
                    eth_ctrlr_phy_dom_threshold_validity::mut_voltage_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "laser_bias_valid",
                    eth_ctrlr_phy_dom_threshold_validity::get_laser_bias_valid_for_reflect,
                    eth_ctrlr_phy_dom_threshold_validity::mut_laser_bias_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transmit_power_valid",
                    eth_ctrlr_phy_dom_threshold_validity::get_transmit_power_valid_for_reflect,
                    eth_ctrlr_phy_dom_threshold_validity::mut_transmit_power_valid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "receive_power_valid",
                    eth_ctrlr_phy_dom_threshold_validity::get_receive_power_valid_for_reflect,
                    eth_ctrlr_phy_dom_threshold_validity::mut_receive_power_valid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_dom_threshold_validity>(
                    "eth_ctrlr_phy_dom_threshold_validity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_dom_threshold_validity {
    fn clear(&mut self) {
        self.clear_temperature_valid();
        self.clear_voltage_valid();
        self.clear_laser_bias_valid();
        self.clear_transmit_power_valid();
        self.clear_receive_power_valid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_dom_threshold_validity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_dom_threshold_validity {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_dom_thresholds {
    // message fields
    pub field_validity: ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_threshold_validity>,
    pub transceiver_temperature_alarm_high: i32,
    pub transceiver_temperature_warning_high: i32,
    pub transceiver_temperature_warning_low: i32,
    pub transceiver_temperature_alarm_low: i32,
    pub transceiver_voltage_alarm_high: u32,
    pub transceiver_voltage_warning_high: u32,
    pub transceiver_voltage_warning_low: u32,
    pub transceiver_voltage_alarm_low: u32,
    pub laser_bias_alarm_high: u32,
    pub laser_bias_warning_high: u32,
    pub laser_bias_warning_low: u32,
    pub laser_bias_alarm_low: u32,
    pub optical_transmit_power_alarm_high: u32,
    pub optical_transmit_power_warning_high: u32,
    pub optical_transmit_power_warning_low: u32,
    pub optical_transmit_power_alarm_low: u32,
    pub optical_receive_power_alarm_high: u32,
    pub optical_receive_power_warning_high: u32,
    pub optical_receive_power_warning_low: u32,
    pub optical_receive_power_alarm_low: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_dom_thresholds {}

impl eth_ctrlr_phy_dom_thresholds {
    pub fn new() -> eth_ctrlr_phy_dom_thresholds {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_dom_thresholds {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_dom_thresholds> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_dom_thresholds,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_dom_thresholds::new)
        }
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_dom_threshold_validity field_validity = 1;

    pub fn clear_field_validity(&mut self) {
        self.field_validity.clear();
    }

    pub fn has_field_validity(&self) -> bool {
        self.field_validity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_validity(&mut self, v: eth_ctrlr_phy_dom_threshold_validity) {
        self.field_validity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_validity(&mut self) -> &mut eth_ctrlr_phy_dom_threshold_validity {
        if self.field_validity.is_none() {
            self.field_validity.set_default();
        }
        self.field_validity.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_validity(&mut self) -> eth_ctrlr_phy_dom_threshold_validity {
        self.field_validity.take().unwrap_or_else(|| eth_ctrlr_phy_dom_threshold_validity::new())
    }

    pub fn get_field_validity(&self) -> &eth_ctrlr_phy_dom_threshold_validity {
        self.field_validity.as_ref().unwrap_or_else(|| eth_ctrlr_phy_dom_threshold_validity::default_instance())
    }

    fn get_field_validity_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_phy_dom_threshold_validity> {
        &self.field_validity
    }

    fn mut_field_validity_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_threshold_validity> {
        &mut self.field_validity
    }

    // sint32 transceiver_temperature_alarm_high = 2;

    pub fn clear_transceiver_temperature_alarm_high(&mut self) {
        self.transceiver_temperature_alarm_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_temperature_alarm_high(&mut self, v: i32) {
        self.transceiver_temperature_alarm_high = v;
    }

    pub fn get_transceiver_temperature_alarm_high(&self) -> i32 {
        self.transceiver_temperature_alarm_high
    }

    fn get_transceiver_temperature_alarm_high_for_reflect(&self) -> &i32 {
        &self.transceiver_temperature_alarm_high
    }

    fn mut_transceiver_temperature_alarm_high_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_temperature_alarm_high
    }

    // sint32 transceiver_temperature_warning_high = 3;

    pub fn clear_transceiver_temperature_warning_high(&mut self) {
        self.transceiver_temperature_warning_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_temperature_warning_high(&mut self, v: i32) {
        self.transceiver_temperature_warning_high = v;
    }

    pub fn get_transceiver_temperature_warning_high(&self) -> i32 {
        self.transceiver_temperature_warning_high
    }

    fn get_transceiver_temperature_warning_high_for_reflect(&self) -> &i32 {
        &self.transceiver_temperature_warning_high
    }

    fn mut_transceiver_temperature_warning_high_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_temperature_warning_high
    }

    // sint32 transceiver_temperature_warning_low = 4;

    pub fn clear_transceiver_temperature_warning_low(&mut self) {
        self.transceiver_temperature_warning_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_temperature_warning_low(&mut self, v: i32) {
        self.transceiver_temperature_warning_low = v;
    }

    pub fn get_transceiver_temperature_warning_low(&self) -> i32 {
        self.transceiver_temperature_warning_low
    }

    fn get_transceiver_temperature_warning_low_for_reflect(&self) -> &i32 {
        &self.transceiver_temperature_warning_low
    }

    fn mut_transceiver_temperature_warning_low_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_temperature_warning_low
    }

    // sint32 transceiver_temperature_alarm_low = 5;

    pub fn clear_transceiver_temperature_alarm_low(&mut self) {
        self.transceiver_temperature_alarm_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_temperature_alarm_low(&mut self, v: i32) {
        self.transceiver_temperature_alarm_low = v;
    }

    pub fn get_transceiver_temperature_alarm_low(&self) -> i32 {
        self.transceiver_temperature_alarm_low
    }

    fn get_transceiver_temperature_alarm_low_for_reflect(&self) -> &i32 {
        &self.transceiver_temperature_alarm_low
    }

    fn mut_transceiver_temperature_alarm_low_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_temperature_alarm_low
    }

    // uint32 transceiver_voltage_alarm_high = 6;

    pub fn clear_transceiver_voltage_alarm_high(&mut self) {
        self.transceiver_voltage_alarm_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_voltage_alarm_high(&mut self, v: u32) {
        self.transceiver_voltage_alarm_high = v;
    }

    pub fn get_transceiver_voltage_alarm_high(&self) -> u32 {
        self.transceiver_voltage_alarm_high
    }

    fn get_transceiver_voltage_alarm_high_for_reflect(&self) -> &u32 {
        &self.transceiver_voltage_alarm_high
    }

    fn mut_transceiver_voltage_alarm_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.transceiver_voltage_alarm_high
    }

    // uint32 transceiver_voltage_warning_high = 7;

    pub fn clear_transceiver_voltage_warning_high(&mut self) {
        self.transceiver_voltage_warning_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_voltage_warning_high(&mut self, v: u32) {
        self.transceiver_voltage_warning_high = v;
    }

    pub fn get_transceiver_voltage_warning_high(&self) -> u32 {
        self.transceiver_voltage_warning_high
    }

    fn get_transceiver_voltage_warning_high_for_reflect(&self) -> &u32 {
        &self.transceiver_voltage_warning_high
    }

    fn mut_transceiver_voltage_warning_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.transceiver_voltage_warning_high
    }

    // uint32 transceiver_voltage_warning_low = 8;

    pub fn clear_transceiver_voltage_warning_low(&mut self) {
        self.transceiver_voltage_warning_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_voltage_warning_low(&mut self, v: u32) {
        self.transceiver_voltage_warning_low = v;
    }

    pub fn get_transceiver_voltage_warning_low(&self) -> u32 {
        self.transceiver_voltage_warning_low
    }

    fn get_transceiver_voltage_warning_low_for_reflect(&self) -> &u32 {
        &self.transceiver_voltage_warning_low
    }

    fn mut_transceiver_voltage_warning_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.transceiver_voltage_warning_low
    }

    // uint32 transceiver_voltage_alarm_low = 9;

    pub fn clear_transceiver_voltage_alarm_low(&mut self) {
        self.transceiver_voltage_alarm_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_voltage_alarm_low(&mut self, v: u32) {
        self.transceiver_voltage_alarm_low = v;
    }

    pub fn get_transceiver_voltage_alarm_low(&self) -> u32 {
        self.transceiver_voltage_alarm_low
    }

    fn get_transceiver_voltage_alarm_low_for_reflect(&self) -> &u32 {
        &self.transceiver_voltage_alarm_low
    }

    fn mut_transceiver_voltage_alarm_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.transceiver_voltage_alarm_low
    }

    // uint32 laser_bias_alarm_high = 10;

    pub fn clear_laser_bias_alarm_high(&mut self) {
        self.laser_bias_alarm_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_alarm_high(&mut self, v: u32) {
        self.laser_bias_alarm_high = v;
    }

    pub fn get_laser_bias_alarm_high(&self) -> u32 {
        self.laser_bias_alarm_high
    }

    fn get_laser_bias_alarm_high_for_reflect(&self) -> &u32 {
        &self.laser_bias_alarm_high
    }

    fn mut_laser_bias_alarm_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.laser_bias_alarm_high
    }

    // uint32 laser_bias_warning_high = 11;

    pub fn clear_laser_bias_warning_high(&mut self) {
        self.laser_bias_warning_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_warning_high(&mut self, v: u32) {
        self.laser_bias_warning_high = v;
    }

    pub fn get_laser_bias_warning_high(&self) -> u32 {
        self.laser_bias_warning_high
    }

    fn get_laser_bias_warning_high_for_reflect(&self) -> &u32 {
        &self.laser_bias_warning_high
    }

    fn mut_laser_bias_warning_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.laser_bias_warning_high
    }

    // uint32 laser_bias_warning_low = 12;

    pub fn clear_laser_bias_warning_low(&mut self) {
        self.laser_bias_warning_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_warning_low(&mut self, v: u32) {
        self.laser_bias_warning_low = v;
    }

    pub fn get_laser_bias_warning_low(&self) -> u32 {
        self.laser_bias_warning_low
    }

    fn get_laser_bias_warning_low_for_reflect(&self) -> &u32 {
        &self.laser_bias_warning_low
    }

    fn mut_laser_bias_warning_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.laser_bias_warning_low
    }

    // uint32 laser_bias_alarm_low = 13;

    pub fn clear_laser_bias_alarm_low(&mut self) {
        self.laser_bias_alarm_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_bias_alarm_low(&mut self, v: u32) {
        self.laser_bias_alarm_low = v;
    }

    pub fn get_laser_bias_alarm_low(&self) -> u32 {
        self.laser_bias_alarm_low
    }

    fn get_laser_bias_alarm_low_for_reflect(&self) -> &u32 {
        &self.laser_bias_alarm_low
    }

    fn mut_laser_bias_alarm_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.laser_bias_alarm_low
    }

    // uint32 optical_transmit_power_alarm_high = 14;

    pub fn clear_optical_transmit_power_alarm_high(&mut self) {
        self.optical_transmit_power_alarm_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_transmit_power_alarm_high(&mut self, v: u32) {
        self.optical_transmit_power_alarm_high = v;
    }

    pub fn get_optical_transmit_power_alarm_high(&self) -> u32 {
        self.optical_transmit_power_alarm_high
    }

    fn get_optical_transmit_power_alarm_high_for_reflect(&self) -> &u32 {
        &self.optical_transmit_power_alarm_high
    }

    fn mut_optical_transmit_power_alarm_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_transmit_power_alarm_high
    }

    // uint32 optical_transmit_power_warning_high = 15;

    pub fn clear_optical_transmit_power_warning_high(&mut self) {
        self.optical_transmit_power_warning_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_transmit_power_warning_high(&mut self, v: u32) {
        self.optical_transmit_power_warning_high = v;
    }

    pub fn get_optical_transmit_power_warning_high(&self) -> u32 {
        self.optical_transmit_power_warning_high
    }

    fn get_optical_transmit_power_warning_high_for_reflect(&self) -> &u32 {
        &self.optical_transmit_power_warning_high
    }

    fn mut_optical_transmit_power_warning_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_transmit_power_warning_high
    }

    // uint32 optical_transmit_power_warning_low = 16;

    pub fn clear_optical_transmit_power_warning_low(&mut self) {
        self.optical_transmit_power_warning_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_transmit_power_warning_low(&mut self, v: u32) {
        self.optical_transmit_power_warning_low = v;
    }

    pub fn get_optical_transmit_power_warning_low(&self) -> u32 {
        self.optical_transmit_power_warning_low
    }

    fn get_optical_transmit_power_warning_low_for_reflect(&self) -> &u32 {
        &self.optical_transmit_power_warning_low
    }

    fn mut_optical_transmit_power_warning_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_transmit_power_warning_low
    }

    // uint32 optical_transmit_power_alarm_low = 17;

    pub fn clear_optical_transmit_power_alarm_low(&mut self) {
        self.optical_transmit_power_alarm_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_transmit_power_alarm_low(&mut self, v: u32) {
        self.optical_transmit_power_alarm_low = v;
    }

    pub fn get_optical_transmit_power_alarm_low(&self) -> u32 {
        self.optical_transmit_power_alarm_low
    }

    fn get_optical_transmit_power_alarm_low_for_reflect(&self) -> &u32 {
        &self.optical_transmit_power_alarm_low
    }

    fn mut_optical_transmit_power_alarm_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_transmit_power_alarm_low
    }

    // uint32 optical_receive_power_alarm_high = 18;

    pub fn clear_optical_receive_power_alarm_high(&mut self) {
        self.optical_receive_power_alarm_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_receive_power_alarm_high(&mut self, v: u32) {
        self.optical_receive_power_alarm_high = v;
    }

    pub fn get_optical_receive_power_alarm_high(&self) -> u32 {
        self.optical_receive_power_alarm_high
    }

    fn get_optical_receive_power_alarm_high_for_reflect(&self) -> &u32 {
        &self.optical_receive_power_alarm_high
    }

    fn mut_optical_receive_power_alarm_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_receive_power_alarm_high
    }

    // uint32 optical_receive_power_warning_high = 19;

    pub fn clear_optical_receive_power_warning_high(&mut self) {
        self.optical_receive_power_warning_high = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_receive_power_warning_high(&mut self, v: u32) {
        self.optical_receive_power_warning_high = v;
    }

    pub fn get_optical_receive_power_warning_high(&self) -> u32 {
        self.optical_receive_power_warning_high
    }

    fn get_optical_receive_power_warning_high_for_reflect(&self) -> &u32 {
        &self.optical_receive_power_warning_high
    }

    fn mut_optical_receive_power_warning_high_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_receive_power_warning_high
    }

    // uint32 optical_receive_power_warning_low = 20;

    pub fn clear_optical_receive_power_warning_low(&mut self) {
        self.optical_receive_power_warning_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_receive_power_warning_low(&mut self, v: u32) {
        self.optical_receive_power_warning_low = v;
    }

    pub fn get_optical_receive_power_warning_low(&self) -> u32 {
        self.optical_receive_power_warning_low
    }

    fn get_optical_receive_power_warning_low_for_reflect(&self) -> &u32 {
        &self.optical_receive_power_warning_low
    }

    fn mut_optical_receive_power_warning_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_receive_power_warning_low
    }

    // uint32 optical_receive_power_alarm_low = 21;

    pub fn clear_optical_receive_power_alarm_low(&mut self) {
        self.optical_receive_power_alarm_low = 0;
    }

    // Param is passed by value, moved
    pub fn set_optical_receive_power_alarm_low(&mut self, v: u32) {
        self.optical_receive_power_alarm_low = v;
    }

    pub fn get_optical_receive_power_alarm_low(&self) -> u32 {
        self.optical_receive_power_alarm_low
    }

    fn get_optical_receive_power_alarm_low_for_reflect(&self) -> &u32 {
        &self.optical_receive_power_alarm_low
    }

    fn mut_optical_receive_power_alarm_low_for_reflect(&mut self) -> &mut u32 {
        &mut self.optical_receive_power_alarm_low
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_dom_thresholds {
    fn is_initialized(&self) -> bool {
        for v in &self.field_validity {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.field_validity)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_temperature_alarm_high = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_temperature_warning_high = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_temperature_warning_low = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_temperature_alarm_low = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.transceiver_voltage_alarm_high = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.transceiver_voltage_warning_high = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.transceiver_voltage_warning_low = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.transceiver_voltage_alarm_low = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.laser_bias_alarm_high = tmp;
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.laser_bias_warning_high = tmp;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.laser_bias_warning_low = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.laser_bias_alarm_low = tmp;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_transmit_power_alarm_high = tmp;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_transmit_power_warning_high = tmp;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_transmit_power_warning_low = tmp;
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_transmit_power_alarm_low = tmp;
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_receive_power_alarm_high = tmp;
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_receive_power_warning_high = tmp;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_receive_power_warning_low = tmp;
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optical_receive_power_alarm_low = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.field_validity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.transceiver_temperature_alarm_high != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.transceiver_temperature_alarm_high);
        }
        if self.transceiver_temperature_warning_high != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, self.transceiver_temperature_warning_high);
        }
        if self.transceiver_temperature_warning_low != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, self.transceiver_temperature_warning_low);
        }
        if self.transceiver_temperature_alarm_low != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, self.transceiver_temperature_alarm_low);
        }
        if self.transceiver_voltage_alarm_high != 0 {
            my_size += ::protobuf::rt::value_size(6, self.transceiver_voltage_alarm_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.transceiver_voltage_warning_high != 0 {
            my_size += ::protobuf::rt::value_size(7, self.transceiver_voltage_warning_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.transceiver_voltage_warning_low != 0 {
            my_size += ::protobuf::rt::value_size(8, self.transceiver_voltage_warning_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.transceiver_voltage_alarm_low != 0 {
            my_size += ::protobuf::rt::value_size(9, self.transceiver_voltage_alarm_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.laser_bias_alarm_high != 0 {
            my_size += ::protobuf::rt::value_size(10, self.laser_bias_alarm_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.laser_bias_warning_high != 0 {
            my_size += ::protobuf::rt::value_size(11, self.laser_bias_warning_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.laser_bias_warning_low != 0 {
            my_size += ::protobuf::rt::value_size(12, self.laser_bias_warning_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.laser_bias_alarm_low != 0 {
            my_size += ::protobuf::rt::value_size(13, self.laser_bias_alarm_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_transmit_power_alarm_high != 0 {
            my_size += ::protobuf::rt::value_size(14, self.optical_transmit_power_alarm_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_transmit_power_warning_high != 0 {
            my_size += ::protobuf::rt::value_size(15, self.optical_transmit_power_warning_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_transmit_power_warning_low != 0 {
            my_size += ::protobuf::rt::value_size(16, self.optical_transmit_power_warning_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_transmit_power_alarm_low != 0 {
            my_size += ::protobuf::rt::value_size(17, self.optical_transmit_power_alarm_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_receive_power_alarm_high != 0 {
            my_size += ::protobuf::rt::value_size(18, self.optical_receive_power_alarm_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_receive_power_warning_high != 0 {
            my_size += ::protobuf::rt::value_size(19, self.optical_receive_power_warning_high, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_receive_power_warning_low != 0 {
            my_size += ::protobuf::rt::value_size(20, self.optical_receive_power_warning_low, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.optical_receive_power_alarm_low != 0 {
            my_size += ::protobuf::rt::value_size(21, self.optical_receive_power_alarm_low, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_validity.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.transceiver_temperature_alarm_high != 0 {
            os.write_sint32(2, self.transceiver_temperature_alarm_high)?;
        }
        if self.transceiver_temperature_warning_high != 0 {
            os.write_sint32(3, self.transceiver_temperature_warning_high)?;
        }
        if self.transceiver_temperature_warning_low != 0 {
            os.write_sint32(4, self.transceiver_temperature_warning_low)?;
        }
        if self.transceiver_temperature_alarm_low != 0 {
            os.write_sint32(5, self.transceiver_temperature_alarm_low)?;
        }
        if self.transceiver_voltage_alarm_high != 0 {
            os.write_uint32(6, self.transceiver_voltage_alarm_high)?;
        }
        if self.transceiver_voltage_warning_high != 0 {
            os.write_uint32(7, self.transceiver_voltage_warning_high)?;
        }
        if self.transceiver_voltage_warning_low != 0 {
            os.write_uint32(8, self.transceiver_voltage_warning_low)?;
        }
        if self.transceiver_voltage_alarm_low != 0 {
            os.write_uint32(9, self.transceiver_voltage_alarm_low)?;
        }
        if self.laser_bias_alarm_high != 0 {
            os.write_uint32(10, self.laser_bias_alarm_high)?;
        }
        if self.laser_bias_warning_high != 0 {
            os.write_uint32(11, self.laser_bias_warning_high)?;
        }
        if self.laser_bias_warning_low != 0 {
            os.write_uint32(12, self.laser_bias_warning_low)?;
        }
        if self.laser_bias_alarm_low != 0 {
            os.write_uint32(13, self.laser_bias_alarm_low)?;
        }
        if self.optical_transmit_power_alarm_high != 0 {
            os.write_uint32(14, self.optical_transmit_power_alarm_high)?;
        }
        if self.optical_transmit_power_warning_high != 0 {
            os.write_uint32(15, self.optical_transmit_power_warning_high)?;
        }
        if self.optical_transmit_power_warning_low != 0 {
            os.write_uint32(16, self.optical_transmit_power_warning_low)?;
        }
        if self.optical_transmit_power_alarm_low != 0 {
            os.write_uint32(17, self.optical_transmit_power_alarm_low)?;
        }
        if self.optical_receive_power_alarm_high != 0 {
            os.write_uint32(18, self.optical_receive_power_alarm_high)?;
        }
        if self.optical_receive_power_warning_high != 0 {
            os.write_uint32(19, self.optical_receive_power_warning_high)?;
        }
        if self.optical_receive_power_warning_low != 0 {
            os.write_uint32(20, self.optical_receive_power_warning_low)?;
        }
        if self.optical_receive_power_alarm_low != 0 {
            os.write_uint32(21, self.optical_receive_power_alarm_low)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_dom_thresholds {
    fn new() -> eth_ctrlr_phy_dom_thresholds {
        eth_ctrlr_phy_dom_thresholds::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_dom_thresholds>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_dom_threshold_validity>>(
                    "field_validity",
                    eth_ctrlr_phy_dom_thresholds::get_field_validity_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_field_validity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_temperature_alarm_high",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_temperature_alarm_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_temperature_alarm_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_temperature_warning_high",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_temperature_warning_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_temperature_warning_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_temperature_warning_low",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_temperature_warning_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_temperature_warning_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_temperature_alarm_low",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_temperature_alarm_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_temperature_alarm_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "transceiver_voltage_alarm_high",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_voltage_alarm_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_voltage_alarm_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "transceiver_voltage_warning_high",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_voltage_warning_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_voltage_warning_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "transceiver_voltage_warning_low",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_voltage_warning_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_voltage_warning_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "transceiver_voltage_alarm_low",
                    eth_ctrlr_phy_dom_thresholds::get_transceiver_voltage_alarm_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_transceiver_voltage_alarm_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "laser_bias_alarm_high",
                    eth_ctrlr_phy_dom_thresholds::get_laser_bias_alarm_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_laser_bias_alarm_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "laser_bias_warning_high",
                    eth_ctrlr_phy_dom_thresholds::get_laser_bias_warning_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_laser_bias_warning_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "laser_bias_warning_low",
                    eth_ctrlr_phy_dom_thresholds::get_laser_bias_warning_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_laser_bias_warning_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "laser_bias_alarm_low",
                    eth_ctrlr_phy_dom_thresholds::get_laser_bias_alarm_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_laser_bias_alarm_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_transmit_power_alarm_high",
                    eth_ctrlr_phy_dom_thresholds::get_optical_transmit_power_alarm_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_transmit_power_alarm_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_transmit_power_warning_high",
                    eth_ctrlr_phy_dom_thresholds::get_optical_transmit_power_warning_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_transmit_power_warning_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_transmit_power_warning_low",
                    eth_ctrlr_phy_dom_thresholds::get_optical_transmit_power_warning_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_transmit_power_warning_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_transmit_power_alarm_low",
                    eth_ctrlr_phy_dom_thresholds::get_optical_transmit_power_alarm_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_transmit_power_alarm_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_receive_power_alarm_high",
                    eth_ctrlr_phy_dom_thresholds::get_optical_receive_power_alarm_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_receive_power_alarm_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_receive_power_warning_high",
                    eth_ctrlr_phy_dom_thresholds::get_optical_receive_power_warning_high_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_receive_power_warning_high_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_receive_power_warning_low",
                    eth_ctrlr_phy_dom_thresholds::get_optical_receive_power_warning_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_receive_power_warning_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optical_receive_power_alarm_low",
                    eth_ctrlr_phy_dom_thresholds::get_optical_receive_power_alarm_low_for_reflect,
                    eth_ctrlr_phy_dom_thresholds::mut_optical_receive_power_alarm_low_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_dom_thresholds>(
                    "eth_ctrlr_phy_dom_thresholds",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_dom_thresholds {
    fn clear(&mut self) {
        self.clear_field_validity();
        self.clear_transceiver_temperature_alarm_high();
        self.clear_transceiver_temperature_warning_high();
        self.clear_transceiver_temperature_warning_low();
        self.clear_transceiver_temperature_alarm_low();
        self.clear_transceiver_voltage_alarm_high();
        self.clear_transceiver_voltage_warning_high();
        self.clear_transceiver_voltage_warning_low();
        self.clear_transceiver_voltage_alarm_low();
        self.clear_laser_bias_alarm_high();
        self.clear_laser_bias_warning_high();
        self.clear_laser_bias_warning_low();
        self.clear_laser_bias_alarm_low();
        self.clear_optical_transmit_power_alarm_high();
        self.clear_optical_transmit_power_warning_high();
        self.clear_optical_transmit_power_warning_low();
        self.clear_optical_transmit_power_alarm_low();
        self.clear_optical_receive_power_alarm_high();
        self.clear_optical_receive_power_warning_high();
        self.clear_optical_receive_power_warning_low();
        self.clear_optical_receive_power_alarm_low();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_dom_thresholds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_dom_thresholds {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ether_phy_details {
    // message fields
    pub vendor: ::std::string::String,
    pub vendor_part_number: ::std::string::String,
    pub vendor_serial_number: ::std::string::String,
    pub transceiver_temperature: i32,
    pub transceiver_voltage: i32,
    pub transceiver_tx_power: i32,
    pub transceiver_rx_power: i32,
    pub transceiver_tx_bias: i32,
    pub lane: ::protobuf::RepeatedField<eth_ctrlr_phy_lane_opt_mon>,
    pub lane_field_validity: ::protobuf::SingularPtrField<eth_ctrlr_phy_lane_opt_mon_validity>,
    pub dig_opt_mon_alarm_thresholds: ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_thresholds>,
    pub dig_opt_mon_alarms: ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_alarms>,
    pub optics_wavelength: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ether_phy_details {}

impl ether_phy_details {
    pub fn new() -> ether_phy_details {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ether_phy_details {
        static mut instance: ::protobuf::lazy::Lazy<ether_phy_details> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ether_phy_details,
        };
        unsafe {
            instance.get(ether_phy_details::new)
        }
    }

    // string vendor = 1;

    pub fn clear_vendor(&mut self) {
        self.vendor.clear();
    }

    // Param is passed by value, moved
    pub fn set_vendor(&mut self, v: ::std::string::String) {
        self.vendor = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vendor(&mut self) -> &mut ::std::string::String {
        &mut self.vendor
    }

    // Take field
    pub fn take_vendor(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.vendor, ::std::string::String::new())
    }

    pub fn get_vendor(&self) -> &str {
        &self.vendor
    }

    fn get_vendor_for_reflect(&self) -> &::std::string::String {
        &self.vendor
    }

    fn mut_vendor_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.vendor
    }

    // string vendor_part_number = 2;

    pub fn clear_vendor_part_number(&mut self) {
        self.vendor_part_number.clear();
    }

    // Param is passed by value, moved
    pub fn set_vendor_part_number(&mut self, v: ::std::string::String) {
        self.vendor_part_number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vendor_part_number(&mut self) -> &mut ::std::string::String {
        &mut self.vendor_part_number
    }

    // Take field
    pub fn take_vendor_part_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.vendor_part_number, ::std::string::String::new())
    }

    pub fn get_vendor_part_number(&self) -> &str {
        &self.vendor_part_number
    }

    fn get_vendor_part_number_for_reflect(&self) -> &::std::string::String {
        &self.vendor_part_number
    }

    fn mut_vendor_part_number_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.vendor_part_number
    }

    // string vendor_serial_number = 3;

    pub fn clear_vendor_serial_number(&mut self) {
        self.vendor_serial_number.clear();
    }

    // Param is passed by value, moved
    pub fn set_vendor_serial_number(&mut self, v: ::std::string::String) {
        self.vendor_serial_number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_vendor_serial_number(&mut self) -> &mut ::std::string::String {
        &mut self.vendor_serial_number
    }

    // Take field
    pub fn take_vendor_serial_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.vendor_serial_number, ::std::string::String::new())
    }

    pub fn get_vendor_serial_number(&self) -> &str {
        &self.vendor_serial_number
    }

    fn get_vendor_serial_number_for_reflect(&self) -> &::std::string::String {
        &self.vendor_serial_number
    }

    fn mut_vendor_serial_number_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.vendor_serial_number
    }

    // sint32 transceiver_temperature = 4;

    pub fn clear_transceiver_temperature(&mut self) {
        self.transceiver_temperature = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_temperature(&mut self, v: i32) {
        self.transceiver_temperature = v;
    }

    pub fn get_transceiver_temperature(&self) -> i32 {
        self.transceiver_temperature
    }

    fn get_transceiver_temperature_for_reflect(&self) -> &i32 {
        &self.transceiver_temperature
    }

    fn mut_transceiver_temperature_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_temperature
    }

    // sint32 transceiver_voltage = 5;

    pub fn clear_transceiver_voltage(&mut self) {
        self.transceiver_voltage = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_voltage(&mut self, v: i32) {
        self.transceiver_voltage = v;
    }

    pub fn get_transceiver_voltage(&self) -> i32 {
        self.transceiver_voltage
    }

    fn get_transceiver_voltage_for_reflect(&self) -> &i32 {
        &self.transceiver_voltage
    }

    fn mut_transceiver_voltage_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_voltage
    }

    // sint32 transceiver_tx_power = 6;

    pub fn clear_transceiver_tx_power(&mut self) {
        self.transceiver_tx_power = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_tx_power(&mut self, v: i32) {
        self.transceiver_tx_power = v;
    }

    pub fn get_transceiver_tx_power(&self) -> i32 {
        self.transceiver_tx_power
    }

    fn get_transceiver_tx_power_for_reflect(&self) -> &i32 {
        &self.transceiver_tx_power
    }

    fn mut_transceiver_tx_power_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_tx_power
    }

    // sint32 transceiver_rx_power = 7;

    pub fn clear_transceiver_rx_power(&mut self) {
        self.transceiver_rx_power = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_rx_power(&mut self, v: i32) {
        self.transceiver_rx_power = v;
    }

    pub fn get_transceiver_rx_power(&self) -> i32 {
        self.transceiver_rx_power
    }

    fn get_transceiver_rx_power_for_reflect(&self) -> &i32 {
        &self.transceiver_rx_power
    }

    fn mut_transceiver_rx_power_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_rx_power
    }

    // sint32 transceiver_tx_bias = 8;

    pub fn clear_transceiver_tx_bias(&mut self) {
        self.transceiver_tx_bias = 0;
    }

    // Param is passed by value, moved
    pub fn set_transceiver_tx_bias(&mut self, v: i32) {
        self.transceiver_tx_bias = v;
    }

    pub fn get_transceiver_tx_bias(&self) -> i32 {
        self.transceiver_tx_bias
    }

    fn get_transceiver_tx_bias_for_reflect(&self) -> &i32 {
        &self.transceiver_tx_bias
    }

    fn mut_transceiver_tx_bias_for_reflect(&mut self) -> &mut i32 {
        &mut self.transceiver_tx_bias
    }

    // repeated .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_lane_opt_mon lane = 9;

    pub fn clear_lane(&mut self) {
        self.lane.clear();
    }

    // Param is passed by value, moved
    pub fn set_lane(&mut self, v: ::protobuf::RepeatedField<eth_ctrlr_phy_lane_opt_mon>) {
        self.lane = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lane(&mut self) -> &mut ::protobuf::RepeatedField<eth_ctrlr_phy_lane_opt_mon> {
        &mut self.lane
    }

    // Take field
    pub fn take_lane(&mut self) -> ::protobuf::RepeatedField<eth_ctrlr_phy_lane_opt_mon> {
        ::std::mem::replace(&mut self.lane, ::protobuf::RepeatedField::new())
    }

    pub fn get_lane(&self) -> &[eth_ctrlr_phy_lane_opt_mon] {
        &self.lane
    }

    fn get_lane_for_reflect(&self) -> &::protobuf::RepeatedField<eth_ctrlr_phy_lane_opt_mon> {
        &self.lane
    }

    fn mut_lane_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<eth_ctrlr_phy_lane_opt_mon> {
        &mut self.lane
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_lane_opt_mon_validity lane_field_validity = 10;

    pub fn clear_lane_field_validity(&mut self) {
        self.lane_field_validity.clear();
    }

    pub fn has_lane_field_validity(&self) -> bool {
        self.lane_field_validity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lane_field_validity(&mut self, v: eth_ctrlr_phy_lane_opt_mon_validity) {
        self.lane_field_validity = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lane_field_validity(&mut self) -> &mut eth_ctrlr_phy_lane_opt_mon_validity {
        if self.lane_field_validity.is_none() {
            self.lane_field_validity.set_default();
        }
        self.lane_field_validity.as_mut().unwrap()
    }

    // Take field
    pub fn take_lane_field_validity(&mut self) -> eth_ctrlr_phy_lane_opt_mon_validity {
        self.lane_field_validity.take().unwrap_or_else(|| eth_ctrlr_phy_lane_opt_mon_validity::new())
    }

    pub fn get_lane_field_validity(&self) -> &eth_ctrlr_phy_lane_opt_mon_validity {
        self.lane_field_validity.as_ref().unwrap_or_else(|| eth_ctrlr_phy_lane_opt_mon_validity::default_instance())
    }

    fn get_lane_field_validity_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_phy_lane_opt_mon_validity> {
        &self.lane_field_validity
    }

    fn mut_lane_field_validity_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_phy_lane_opt_mon_validity> {
        &mut self.lane_field_validity
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_dom_thresholds dig_opt_mon_alarm_thresholds = 11;

    pub fn clear_dig_opt_mon_alarm_thresholds(&mut self) {
        self.dig_opt_mon_alarm_thresholds.clear();
    }

    pub fn has_dig_opt_mon_alarm_thresholds(&self) -> bool {
        self.dig_opt_mon_alarm_thresholds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dig_opt_mon_alarm_thresholds(&mut self, v: eth_ctrlr_phy_dom_thresholds) {
        self.dig_opt_mon_alarm_thresholds = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dig_opt_mon_alarm_thresholds(&mut self) -> &mut eth_ctrlr_phy_dom_thresholds {
        if self.dig_opt_mon_alarm_thresholds.is_none() {
            self.dig_opt_mon_alarm_thresholds.set_default();
        }
        self.dig_opt_mon_alarm_thresholds.as_mut().unwrap()
    }

    // Take field
    pub fn take_dig_opt_mon_alarm_thresholds(&mut self) -> eth_ctrlr_phy_dom_thresholds {
        self.dig_opt_mon_alarm_thresholds.take().unwrap_or_else(|| eth_ctrlr_phy_dom_thresholds::new())
    }

    pub fn get_dig_opt_mon_alarm_thresholds(&self) -> &eth_ctrlr_phy_dom_thresholds {
        self.dig_opt_mon_alarm_thresholds.as_ref().unwrap_or_else(|| eth_ctrlr_phy_dom_thresholds::default_instance())
    }

    fn get_dig_opt_mon_alarm_thresholds_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_phy_dom_thresholds> {
        &self.dig_opt_mon_alarm_thresholds
    }

    fn mut_dig_opt_mon_alarm_thresholds_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_thresholds> {
        &mut self.dig_opt_mon_alarm_thresholds
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_dom_alarms dig_opt_mon_alarms = 12;

    pub fn clear_dig_opt_mon_alarms(&mut self) {
        self.dig_opt_mon_alarms.clear();
    }

    pub fn has_dig_opt_mon_alarms(&self) -> bool {
        self.dig_opt_mon_alarms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dig_opt_mon_alarms(&mut self, v: eth_ctrlr_phy_dom_alarms) {
        self.dig_opt_mon_alarms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dig_opt_mon_alarms(&mut self) -> &mut eth_ctrlr_phy_dom_alarms {
        if self.dig_opt_mon_alarms.is_none() {
            self.dig_opt_mon_alarms.set_default();
        }
        self.dig_opt_mon_alarms.as_mut().unwrap()
    }

    // Take field
    pub fn take_dig_opt_mon_alarms(&mut self) -> eth_ctrlr_phy_dom_alarms {
        self.dig_opt_mon_alarms.take().unwrap_or_else(|| eth_ctrlr_phy_dom_alarms::new())
    }

    pub fn get_dig_opt_mon_alarms(&self) -> &eth_ctrlr_phy_dom_alarms {
        self.dig_opt_mon_alarms.as_ref().unwrap_or_else(|| eth_ctrlr_phy_dom_alarms::default_instance())
    }

    fn get_dig_opt_mon_alarms_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_phy_dom_alarms> {
        &self.dig_opt_mon_alarms
    }

    fn mut_dig_opt_mon_alarms_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_phy_dom_alarms> {
        &mut self.dig_opt_mon_alarms
    }

    // uint32 optics_wavelength = 13;

    pub fn clear_optics_wavelength(&mut self) {
        self.optics_wavelength = 0;
    }

    // Param is passed by value, moved
    pub fn set_optics_wavelength(&mut self, v: u32) {
        self.optics_wavelength = v;
    }

    pub fn get_optics_wavelength(&self) -> u32 {
        self.optics_wavelength
    }

    fn get_optics_wavelength_for_reflect(&self) -> &u32 {
        &self.optics_wavelength
    }

    fn mut_optics_wavelength_for_reflect(&mut self) -> &mut u32 {
        &mut self.optics_wavelength
    }
}

impl ::protobuf::Message for ether_phy_details {
    fn is_initialized(&self) -> bool {
        for v in &self.lane {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lane_field_validity {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dig_opt_mon_alarm_thresholds {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.dig_opt_mon_alarms {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.vendor)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.vendor_part_number)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.vendor_serial_number)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_temperature = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_voltage = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_tx_power = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_rx_power = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.transceiver_tx_bias = tmp;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lane)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lane_field_validity)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dig_opt_mon_alarm_thresholds)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.dig_opt_mon_alarms)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.optics_wavelength = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.vendor.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.vendor);
        }
        if !self.vendor_part_number.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.vendor_part_number);
        }
        if !self.vendor_serial_number.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.vendor_serial_number);
        }
        if self.transceiver_temperature != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, self.transceiver_temperature);
        }
        if self.transceiver_voltage != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, self.transceiver_voltage);
        }
        if self.transceiver_tx_power != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(6, self.transceiver_tx_power);
        }
        if self.transceiver_rx_power != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, self.transceiver_rx_power);
        }
        if self.transceiver_tx_bias != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(8, self.transceiver_tx_bias);
        }
        for value in &self.lane {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.lane_field_validity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.dig_opt_mon_alarm_thresholds.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.dig_opt_mon_alarms.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.optics_wavelength != 0 {
            my_size += ::protobuf::rt::value_size(13, self.optics_wavelength, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.vendor.is_empty() {
            os.write_string(1, &self.vendor)?;
        }
        if !self.vendor_part_number.is_empty() {
            os.write_string(2, &self.vendor_part_number)?;
        }
        if !self.vendor_serial_number.is_empty() {
            os.write_string(3, &self.vendor_serial_number)?;
        }
        if self.transceiver_temperature != 0 {
            os.write_sint32(4, self.transceiver_temperature)?;
        }
        if self.transceiver_voltage != 0 {
            os.write_sint32(5, self.transceiver_voltage)?;
        }
        if self.transceiver_tx_power != 0 {
            os.write_sint32(6, self.transceiver_tx_power)?;
        }
        if self.transceiver_rx_power != 0 {
            os.write_sint32(7, self.transceiver_rx_power)?;
        }
        if self.transceiver_tx_bias != 0 {
            os.write_sint32(8, self.transceiver_tx_bias)?;
        }
        for v in &self.lane {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.lane_field_validity.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.dig_opt_mon_alarm_thresholds.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.dig_opt_mon_alarms.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.optics_wavelength != 0 {
            os.write_uint32(13, self.optics_wavelength)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ether_phy_details {
    fn new() -> ether_phy_details {
        ether_phy_details::new()
    }

    fn descriptor_static(_: ::std::option::Option<ether_phy_details>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "vendor",
                    ether_phy_details::get_vendor_for_reflect,
                    ether_phy_details::mut_vendor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "vendor_part_number",
                    ether_phy_details::get_vendor_part_number_for_reflect,
                    ether_phy_details::mut_vendor_part_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "vendor_serial_number",
                    ether_phy_details::get_vendor_serial_number_for_reflect,
                    ether_phy_details::mut_vendor_serial_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_temperature",
                    ether_phy_details::get_transceiver_temperature_for_reflect,
                    ether_phy_details::mut_transceiver_temperature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_voltage",
                    ether_phy_details::get_transceiver_voltage_for_reflect,
                    ether_phy_details::mut_transceiver_voltage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_tx_power",
                    ether_phy_details::get_transceiver_tx_power_for_reflect,
                    ether_phy_details::mut_transceiver_tx_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_rx_power",
                    ether_phy_details::get_transceiver_rx_power_for_reflect,
                    ether_phy_details::mut_transceiver_rx_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "transceiver_tx_bias",
                    ether_phy_details::get_transceiver_tx_bias_for_reflect,
                    ether_phy_details::mut_transceiver_tx_bias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_lane_opt_mon>>(
                    "lane",
                    ether_phy_details::get_lane_for_reflect,
                    ether_phy_details::mut_lane_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_lane_opt_mon_validity>>(
                    "lane_field_validity",
                    ether_phy_details::get_lane_field_validity_for_reflect,
                    ether_phy_details::mut_lane_field_validity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_dom_thresholds>>(
                    "dig_opt_mon_alarm_thresholds",
                    ether_phy_details::get_dig_opt_mon_alarm_thresholds_for_reflect,
                    ether_phy_details::mut_dig_opt_mon_alarm_thresholds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_phy_dom_alarms>>(
                    "dig_opt_mon_alarms",
                    ether_phy_details::get_dig_opt_mon_alarms_for_reflect,
                    ether_phy_details::mut_dig_opt_mon_alarms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "optics_wavelength",
                    ether_phy_details::get_optics_wavelength_for_reflect,
                    ether_phy_details::mut_optics_wavelength_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ether_phy_details>(
                    "ether_phy_details",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ether_phy_details {
    fn clear(&mut self) {
        self.clear_vendor();
        self.clear_vendor_part_number();
        self.clear_vendor_serial_number();
        self.clear_transceiver_temperature();
        self.clear_transceiver_voltage();
        self.clear_transceiver_tx_power();
        self.clear_transceiver_rx_power();
        self.clear_transceiver_tx_bias();
        self.clear_lane();
        self.clear_lane_field_validity();
        self.clear_dig_opt_mon_alarm_thresholds();
        self.clear_dig_opt_mon_alarms();
        self.clear_optics_wavelength();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ether_phy_details {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ether_phy_details {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_alarms {
    // message fields
    pub received_loss_of_signal_alarm: ::std::string::String,
    pub pcs_loss_of_block_lock_alarm: ::std::string::String,
    pub local_fault_alarm: ::std::string::String,
    pub remote_fault_alarm: ::std::string::String,
    pub sd_ber_alarm: ::std::string::String,
    pub sf_ber_alarm: ::std::string::String,
    pub loss_of_synchronization_data_alarm: ::std::string::String,
    pub hi_ber_alarm: ::std::string::String,
    pub squelch_alarm: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_alarms {}

impl eth_ctrlr_alarms {
    pub fn new() -> eth_ctrlr_alarms {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_alarms {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_alarms> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_alarms,
        };
        unsafe {
            instance.get(eth_ctrlr_alarms::new)
        }
    }

    // string received_loss_of_signal_alarm = 1;

    pub fn clear_received_loss_of_signal_alarm(&mut self) {
        self.received_loss_of_signal_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_received_loss_of_signal_alarm(&mut self, v: ::std::string::String) {
        self.received_loss_of_signal_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_received_loss_of_signal_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.received_loss_of_signal_alarm
    }

    // Take field
    pub fn take_received_loss_of_signal_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.received_loss_of_signal_alarm, ::std::string::String::new())
    }

    pub fn get_received_loss_of_signal_alarm(&self) -> &str {
        &self.received_loss_of_signal_alarm
    }

    fn get_received_loss_of_signal_alarm_for_reflect(&self) -> &::std::string::String {
        &self.received_loss_of_signal_alarm
    }

    fn mut_received_loss_of_signal_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.received_loss_of_signal_alarm
    }

    // string pcs_loss_of_block_lock_alarm = 2;

    pub fn clear_pcs_loss_of_block_lock_alarm(&mut self) {
        self.pcs_loss_of_block_lock_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_pcs_loss_of_block_lock_alarm(&mut self, v: ::std::string::String) {
        self.pcs_loss_of_block_lock_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pcs_loss_of_block_lock_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.pcs_loss_of_block_lock_alarm
    }

    // Take field
    pub fn take_pcs_loss_of_block_lock_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pcs_loss_of_block_lock_alarm, ::std::string::String::new())
    }

    pub fn get_pcs_loss_of_block_lock_alarm(&self) -> &str {
        &self.pcs_loss_of_block_lock_alarm
    }

    fn get_pcs_loss_of_block_lock_alarm_for_reflect(&self) -> &::std::string::String {
        &self.pcs_loss_of_block_lock_alarm
    }

    fn mut_pcs_loss_of_block_lock_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.pcs_loss_of_block_lock_alarm
    }

    // string local_fault_alarm = 3;

    pub fn clear_local_fault_alarm(&mut self) {
        self.local_fault_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_local_fault_alarm(&mut self, v: ::std::string::String) {
        self.local_fault_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_local_fault_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.local_fault_alarm
    }

    // Take field
    pub fn take_local_fault_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.local_fault_alarm, ::std::string::String::new())
    }

    pub fn get_local_fault_alarm(&self) -> &str {
        &self.local_fault_alarm
    }

    fn get_local_fault_alarm_for_reflect(&self) -> &::std::string::String {
        &self.local_fault_alarm
    }

    fn mut_local_fault_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.local_fault_alarm
    }

    // string remote_fault_alarm = 4;

    pub fn clear_remote_fault_alarm(&mut self) {
        self.remote_fault_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_remote_fault_alarm(&mut self, v: ::std::string::String) {
        self.remote_fault_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_remote_fault_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.remote_fault_alarm
    }

    // Take field
    pub fn take_remote_fault_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.remote_fault_alarm, ::std::string::String::new())
    }

    pub fn get_remote_fault_alarm(&self) -> &str {
        &self.remote_fault_alarm
    }

    fn get_remote_fault_alarm_for_reflect(&self) -> &::std::string::String {
        &self.remote_fault_alarm
    }

    fn mut_remote_fault_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.remote_fault_alarm
    }

    // string sd_ber_alarm = 5;

    pub fn clear_sd_ber_alarm(&mut self) {
        self.sd_ber_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_sd_ber_alarm(&mut self, v: ::std::string::String) {
        self.sd_ber_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sd_ber_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.sd_ber_alarm
    }

    // Take field
    pub fn take_sd_ber_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sd_ber_alarm, ::std::string::String::new())
    }

    pub fn get_sd_ber_alarm(&self) -> &str {
        &self.sd_ber_alarm
    }

    fn get_sd_ber_alarm_for_reflect(&self) -> &::std::string::String {
        &self.sd_ber_alarm
    }

    fn mut_sd_ber_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sd_ber_alarm
    }

    // string sf_ber_alarm = 6;

    pub fn clear_sf_ber_alarm(&mut self) {
        self.sf_ber_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_sf_ber_alarm(&mut self, v: ::std::string::String) {
        self.sf_ber_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sf_ber_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.sf_ber_alarm
    }

    // Take field
    pub fn take_sf_ber_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sf_ber_alarm, ::std::string::String::new())
    }

    pub fn get_sf_ber_alarm(&self) -> &str {
        &self.sf_ber_alarm
    }

    fn get_sf_ber_alarm_for_reflect(&self) -> &::std::string::String {
        &self.sf_ber_alarm
    }

    fn mut_sf_ber_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sf_ber_alarm
    }

    // string loss_of_synchronization_data_alarm = 7;

    pub fn clear_loss_of_synchronization_data_alarm(&mut self) {
        self.loss_of_synchronization_data_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_loss_of_synchronization_data_alarm(&mut self, v: ::std::string::String) {
        self.loss_of_synchronization_data_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_loss_of_synchronization_data_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.loss_of_synchronization_data_alarm
    }

    // Take field
    pub fn take_loss_of_synchronization_data_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.loss_of_synchronization_data_alarm, ::std::string::String::new())
    }

    pub fn get_loss_of_synchronization_data_alarm(&self) -> &str {
        &self.loss_of_synchronization_data_alarm
    }

    fn get_loss_of_synchronization_data_alarm_for_reflect(&self) -> &::std::string::String {
        &self.loss_of_synchronization_data_alarm
    }

    fn mut_loss_of_synchronization_data_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.loss_of_synchronization_data_alarm
    }

    // string hi_ber_alarm = 8;

    pub fn clear_hi_ber_alarm(&mut self) {
        self.hi_ber_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_hi_ber_alarm(&mut self, v: ::std::string::String) {
        self.hi_ber_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hi_ber_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.hi_ber_alarm
    }

    // Take field
    pub fn take_hi_ber_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.hi_ber_alarm, ::std::string::String::new())
    }

    pub fn get_hi_ber_alarm(&self) -> &str {
        &self.hi_ber_alarm
    }

    fn get_hi_ber_alarm_for_reflect(&self) -> &::std::string::String {
        &self.hi_ber_alarm
    }

    fn mut_hi_ber_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.hi_ber_alarm
    }

    // string squelch_alarm = 9;

    pub fn clear_squelch_alarm(&mut self) {
        self.squelch_alarm.clear();
    }

    // Param is passed by value, moved
    pub fn set_squelch_alarm(&mut self, v: ::std::string::String) {
        self.squelch_alarm = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_squelch_alarm(&mut self) -> &mut ::std::string::String {
        &mut self.squelch_alarm
    }

    // Take field
    pub fn take_squelch_alarm(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.squelch_alarm, ::std::string::String::new())
    }

    pub fn get_squelch_alarm(&self) -> &str {
        &self.squelch_alarm
    }

    fn get_squelch_alarm_for_reflect(&self) -> &::std::string::String {
        &self.squelch_alarm
    }

    fn mut_squelch_alarm_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.squelch_alarm
    }
}

impl ::protobuf::Message for eth_ctrlr_alarms {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.received_loss_of_signal_alarm)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pcs_loss_of_block_lock_alarm)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.local_fault_alarm)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.remote_fault_alarm)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sd_ber_alarm)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sf_ber_alarm)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.loss_of_synchronization_data_alarm)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.hi_ber_alarm)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.squelch_alarm)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.received_loss_of_signal_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.received_loss_of_signal_alarm);
        }
        if !self.pcs_loss_of_block_lock_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.pcs_loss_of_block_lock_alarm);
        }
        if !self.local_fault_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.local_fault_alarm);
        }
        if !self.remote_fault_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.remote_fault_alarm);
        }
        if !self.sd_ber_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.sd_ber_alarm);
        }
        if !self.sf_ber_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.sf_ber_alarm);
        }
        if !self.loss_of_synchronization_data_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.loss_of_synchronization_data_alarm);
        }
        if !self.hi_ber_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.hi_ber_alarm);
        }
        if !self.squelch_alarm.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.squelch_alarm);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.received_loss_of_signal_alarm.is_empty() {
            os.write_string(1, &self.received_loss_of_signal_alarm)?;
        }
        if !self.pcs_loss_of_block_lock_alarm.is_empty() {
            os.write_string(2, &self.pcs_loss_of_block_lock_alarm)?;
        }
        if !self.local_fault_alarm.is_empty() {
            os.write_string(3, &self.local_fault_alarm)?;
        }
        if !self.remote_fault_alarm.is_empty() {
            os.write_string(4, &self.remote_fault_alarm)?;
        }
        if !self.sd_ber_alarm.is_empty() {
            os.write_string(5, &self.sd_ber_alarm)?;
        }
        if !self.sf_ber_alarm.is_empty() {
            os.write_string(6, &self.sf_ber_alarm)?;
        }
        if !self.loss_of_synchronization_data_alarm.is_empty() {
            os.write_string(7, &self.loss_of_synchronization_data_alarm)?;
        }
        if !self.hi_ber_alarm.is_empty() {
            os.write_string(8, &self.hi_ber_alarm)?;
        }
        if !self.squelch_alarm.is_empty() {
            os.write_string(9, &self.squelch_alarm)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_alarms {
    fn new() -> eth_ctrlr_alarms {
        eth_ctrlr_alarms::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_alarms>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "received_loss_of_signal_alarm",
                    eth_ctrlr_alarms::get_received_loss_of_signal_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_received_loss_of_signal_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "pcs_loss_of_block_lock_alarm",
                    eth_ctrlr_alarms::get_pcs_loss_of_block_lock_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_pcs_loss_of_block_lock_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "local_fault_alarm",
                    eth_ctrlr_alarms::get_local_fault_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_local_fault_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "remote_fault_alarm",
                    eth_ctrlr_alarms::get_remote_fault_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_remote_fault_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sd_ber_alarm",
                    eth_ctrlr_alarms::get_sd_ber_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_sd_ber_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sf_ber_alarm",
                    eth_ctrlr_alarms::get_sf_ber_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_sf_ber_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "loss_of_synchronization_data_alarm",
                    eth_ctrlr_alarms::get_loss_of_synchronization_data_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_loss_of_synchronization_data_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hi_ber_alarm",
                    eth_ctrlr_alarms::get_hi_ber_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_hi_ber_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "squelch_alarm",
                    eth_ctrlr_alarms::get_squelch_alarm_for_reflect,
                    eth_ctrlr_alarms::mut_squelch_alarm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_alarms>(
                    "eth_ctrlr_alarms",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_alarms {
    fn clear(&mut self) {
        self.clear_received_loss_of_signal_alarm();
        self.clear_pcs_loss_of_block_lock_alarm();
        self.clear_local_fault_alarm();
        self.clear_remote_fault_alarm();
        self.clear_sd_ber_alarm();
        self.clear_sf_ber_alarm();
        self.clear_loss_of_synchronization_data_alarm();
        self.clear_hi_ber_alarm();
        self.clear_squelch_alarm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_alarms {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_alarms {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_error_counters {
    // message fields
    pub sync_header_errors: u64,
    pub pcsbip_errors: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_error_counters {}

impl eth_ctrlr_error_counters {
    pub fn new() -> eth_ctrlr_error_counters {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_error_counters {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_error_counters> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_error_counters,
        };
        unsafe {
            instance.get(eth_ctrlr_error_counters::new)
        }
    }

    // uint64 sync_header_errors = 1;

    pub fn clear_sync_header_errors(&mut self) {
        self.sync_header_errors = 0;
    }

    // Param is passed by value, moved
    pub fn set_sync_header_errors(&mut self, v: u64) {
        self.sync_header_errors = v;
    }

    pub fn get_sync_header_errors(&self) -> u64 {
        self.sync_header_errors
    }

    fn get_sync_header_errors_for_reflect(&self) -> &u64 {
        &self.sync_header_errors
    }

    fn mut_sync_header_errors_for_reflect(&mut self) -> &mut u64 {
        &mut self.sync_header_errors
    }

    // uint64 pcsbip_errors = 2;

    pub fn clear_pcsbip_errors(&mut self) {
        self.pcsbip_errors = 0;
    }

    // Param is passed by value, moved
    pub fn set_pcsbip_errors(&mut self, v: u64) {
        self.pcsbip_errors = v;
    }

    pub fn get_pcsbip_errors(&self) -> u64 {
        self.pcsbip_errors
    }

    fn get_pcsbip_errors_for_reflect(&self) -> &u64 {
        &self.pcsbip_errors
    }

    fn mut_pcsbip_errors_for_reflect(&mut self) -> &mut u64 {
        &mut self.pcsbip_errors
    }
}

impl ::protobuf::Message for eth_ctrlr_error_counters {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.sync_header_errors = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.pcsbip_errors = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.sync_header_errors != 0 {
            my_size += ::protobuf::rt::value_size(1, self.sync_header_errors, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.pcsbip_errors != 0 {
            my_size += ::protobuf::rt::value_size(2, self.pcsbip_errors, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.sync_header_errors != 0 {
            os.write_uint64(1, self.sync_header_errors)?;
        }
        if self.pcsbip_errors != 0 {
            os.write_uint64(2, self.pcsbip_errors)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_error_counters {
    fn new() -> eth_ctrlr_error_counters {
        eth_ctrlr_error_counters::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_error_counters>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "sync_header_errors",
                    eth_ctrlr_error_counters::get_sync_header_errors_for_reflect,
                    eth_ctrlr_error_counters::mut_sync_header_errors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "pcsbip_errors",
                    eth_ctrlr_error_counters::get_pcsbip_errors_for_reflect,
                    eth_ctrlr_error_counters::mut_pcsbip_errors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_error_counters>(
                    "eth_ctrlr_error_counters",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_error_counters {
    fn clear(&mut self) {
        self.clear_sync_header_errors();
        self.clear_pcsbip_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_error_counters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_error_counters {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ether_ber_settings {
    // message fields
    pub signal_degrade_threshold: u32,
    pub signal_degrade_alarm: i32,
    pub signal_fail_threshold: u32,
    pub signal_fail_alarm: i32,
    pub signal_remote_fault: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ether_ber_settings {}

impl ether_ber_settings {
    pub fn new() -> ether_ber_settings {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ether_ber_settings {
        static mut instance: ::protobuf::lazy::Lazy<ether_ber_settings> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ether_ber_settings,
        };
        unsafe {
            instance.get(ether_ber_settings::new)
        }
    }

    // uint32 signal_degrade_threshold = 1;

    pub fn clear_signal_degrade_threshold(&mut self) {
        self.signal_degrade_threshold = 0;
    }

    // Param is passed by value, moved
    pub fn set_signal_degrade_threshold(&mut self, v: u32) {
        self.signal_degrade_threshold = v;
    }

    pub fn get_signal_degrade_threshold(&self) -> u32 {
        self.signal_degrade_threshold
    }

    fn get_signal_degrade_threshold_for_reflect(&self) -> &u32 {
        &self.signal_degrade_threshold
    }

    fn mut_signal_degrade_threshold_for_reflect(&mut self) -> &mut u32 {
        &mut self.signal_degrade_threshold
    }

    // sint32 signal_degrade_alarm = 2;

    pub fn clear_signal_degrade_alarm(&mut self) {
        self.signal_degrade_alarm = 0;
    }

    // Param is passed by value, moved
    pub fn set_signal_degrade_alarm(&mut self, v: i32) {
        self.signal_degrade_alarm = v;
    }

    pub fn get_signal_degrade_alarm(&self) -> i32 {
        self.signal_degrade_alarm
    }

    fn get_signal_degrade_alarm_for_reflect(&self) -> &i32 {
        &self.signal_degrade_alarm
    }

    fn mut_signal_degrade_alarm_for_reflect(&mut self) -> &mut i32 {
        &mut self.signal_degrade_alarm
    }

    // uint32 signal_fail_threshold = 3;

    pub fn clear_signal_fail_threshold(&mut self) {
        self.signal_fail_threshold = 0;
    }

    // Param is passed by value, moved
    pub fn set_signal_fail_threshold(&mut self, v: u32) {
        self.signal_fail_threshold = v;
    }

    pub fn get_signal_fail_threshold(&self) -> u32 {
        self.signal_fail_threshold
    }

    fn get_signal_fail_threshold_for_reflect(&self) -> &u32 {
        &self.signal_fail_threshold
    }

    fn mut_signal_fail_threshold_for_reflect(&mut self) -> &mut u32 {
        &mut self.signal_fail_threshold
    }

    // sint32 signal_fail_alarm = 4;

    pub fn clear_signal_fail_alarm(&mut self) {
        self.signal_fail_alarm = 0;
    }

    // Param is passed by value, moved
    pub fn set_signal_fail_alarm(&mut self, v: i32) {
        self.signal_fail_alarm = v;
    }

    pub fn get_signal_fail_alarm(&self) -> i32 {
        self.signal_fail_alarm
    }

    fn get_signal_fail_alarm_for_reflect(&self) -> &i32 {
        &self.signal_fail_alarm
    }

    fn mut_signal_fail_alarm_for_reflect(&mut self) -> &mut i32 {
        &mut self.signal_fail_alarm
    }

    // sint32 signal_remote_fault = 5;

    pub fn clear_signal_remote_fault(&mut self) {
        self.signal_remote_fault = 0;
    }

    // Param is passed by value, moved
    pub fn set_signal_remote_fault(&mut self, v: i32) {
        self.signal_remote_fault = v;
    }

    pub fn get_signal_remote_fault(&self) -> i32 {
        self.signal_remote_fault
    }

    fn get_signal_remote_fault_for_reflect(&self) -> &i32 {
        &self.signal_remote_fault
    }

    fn mut_signal_remote_fault_for_reflect(&mut self) -> &mut i32 {
        &mut self.signal_remote_fault
    }
}

impl ::protobuf::Message for ether_ber_settings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.signal_degrade_threshold = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.signal_degrade_alarm = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.signal_fail_threshold = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.signal_fail_alarm = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.signal_remote_fault = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.signal_degrade_threshold != 0 {
            my_size += ::protobuf::rt::value_size(1, self.signal_degrade_threshold, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.signal_degrade_alarm != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, self.signal_degrade_alarm);
        }
        if self.signal_fail_threshold != 0 {
            my_size += ::protobuf::rt::value_size(3, self.signal_fail_threshold, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.signal_fail_alarm != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(4, self.signal_fail_alarm);
        }
        if self.signal_remote_fault != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, self.signal_remote_fault);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.signal_degrade_threshold != 0 {
            os.write_uint32(1, self.signal_degrade_threshold)?;
        }
        if self.signal_degrade_alarm != 0 {
            os.write_sint32(2, self.signal_degrade_alarm)?;
        }
        if self.signal_fail_threshold != 0 {
            os.write_uint32(3, self.signal_fail_threshold)?;
        }
        if self.signal_fail_alarm != 0 {
            os.write_sint32(4, self.signal_fail_alarm)?;
        }
        if self.signal_remote_fault != 0 {
            os.write_sint32(5, self.signal_remote_fault)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ether_ber_settings {
    fn new() -> ether_ber_settings {
        ether_ber_settings::new()
    }

    fn descriptor_static(_: ::std::option::Option<ether_ber_settings>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signal_degrade_threshold",
                    ether_ber_settings::get_signal_degrade_threshold_for_reflect,
                    ether_ber_settings::mut_signal_degrade_threshold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "signal_degrade_alarm",
                    ether_ber_settings::get_signal_degrade_alarm_for_reflect,
                    ether_ber_settings::mut_signal_degrade_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signal_fail_threshold",
                    ether_ber_settings::get_signal_fail_threshold_for_reflect,
                    ether_ber_settings::mut_signal_fail_threshold_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "signal_fail_alarm",
                    ether_ber_settings::get_signal_fail_alarm_for_reflect,
                    ether_ber_settings::mut_signal_fail_alarm_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "signal_remote_fault",
                    ether_ber_settings::get_signal_remote_fault_for_reflect,
                    ether_ber_settings::mut_signal_remote_fault_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ether_ber_settings>(
                    "ether_ber_settings",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ether_ber_settings {
    fn clear(&mut self) {
        self.clear_signal_degrade_threshold();
        self.clear_signal_degrade_alarm();
        self.clear_signal_fail_threshold();
        self.clear_signal_fail_alarm();
        self.clear_signal_remote_fault();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ether_ber_settings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ether_ber_settings {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ber_monitoring {
    // message fields
    pub supported: i32,
    pub settings: ::protobuf::SingularPtrField<ether_ber_settings>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ber_monitoring {}

impl eth_ber_monitoring {
    pub fn new() -> eth_ber_monitoring {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ber_monitoring {
        static mut instance: ::protobuf::lazy::Lazy<eth_ber_monitoring> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ber_monitoring,
        };
        unsafe {
            instance.get(eth_ber_monitoring::new)
        }
    }

    // sint32 supported = 1;

    pub fn clear_supported(&mut self) {
        self.supported = 0;
    }

    // Param is passed by value, moved
    pub fn set_supported(&mut self, v: i32) {
        self.supported = v;
    }

    pub fn get_supported(&self) -> i32 {
        self.supported
    }

    fn get_supported_for_reflect(&self) -> &i32 {
        &self.supported
    }

    fn mut_supported_for_reflect(&mut self) -> &mut i32 {
        &mut self.supported
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_ber_settings settings = 2;

    pub fn clear_settings(&mut self) {
        self.settings.clear();
    }

    pub fn has_settings(&self) -> bool {
        self.settings.is_some()
    }

    // Param is passed by value, moved
    pub fn set_settings(&mut self, v: ether_ber_settings) {
        self.settings = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_settings(&mut self) -> &mut ether_ber_settings {
        if self.settings.is_none() {
            self.settings.set_default();
        }
        self.settings.as_mut().unwrap()
    }

    // Take field
    pub fn take_settings(&mut self) -> ether_ber_settings {
        self.settings.take().unwrap_or_else(|| ether_ber_settings::new())
    }

    pub fn get_settings(&self) -> &ether_ber_settings {
        self.settings.as_ref().unwrap_or_else(|| ether_ber_settings::default_instance())
    }

    fn get_settings_for_reflect(&self) -> &::protobuf::SingularPtrField<ether_ber_settings> {
        &self.settings
    }

    fn mut_settings_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ether_ber_settings> {
        &mut self.settings
    }
}

impl ::protobuf::Message for eth_ber_monitoring {
    fn is_initialized(&self) -> bool {
        for v in &self.settings {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.supported = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.settings)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.supported != 0 {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, self.supported);
        }
        if let Some(ref v) = self.settings.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.supported != 0 {
            os.write_sint32(1, self.supported)?;
        }
        if let Some(ref v) = self.settings.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ber_monitoring {
    fn new() -> eth_ber_monitoring {
        eth_ber_monitoring::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ber_monitoring>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "supported",
                    eth_ber_monitoring::get_supported_for_reflect,
                    eth_ber_monitoring::mut_supported_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ether_ber_settings>>(
                    "settings",
                    eth_ber_monitoring::get_settings_for_reflect,
                    eth_ber_monitoring::mut_settings_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ber_monitoring>(
                    "eth_ber_monitoring",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ber_monitoring {
    fn clear(&mut self) {
        self.clear_supported();
        self.clear_settings();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ber_monitoring {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ber_monitoring {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_ucast_mac_filter {
    // message fields
    pub unicast_mac_addresses: ::protobuf::RepeatedField<mac_addr_type>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_ucast_mac_filter {}

impl eth_ctrlr_ucast_mac_filter {
    pub fn new() -> eth_ctrlr_ucast_mac_filter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_ucast_mac_filter {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_ucast_mac_filter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_ucast_mac_filter,
        };
        unsafe {
            instance.get(eth_ctrlr_ucast_mac_filter::new)
        }
    }

    // repeated .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.mac_addr_type unicast_mac_addresses = 1;

    pub fn clear_unicast_mac_addresses(&mut self) {
        self.unicast_mac_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_unicast_mac_addresses(&mut self, v: ::protobuf::RepeatedField<mac_addr_type>) {
        self.unicast_mac_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unicast_mac_addresses(&mut self) -> &mut ::protobuf::RepeatedField<mac_addr_type> {
        &mut self.unicast_mac_addresses
    }

    // Take field
    pub fn take_unicast_mac_addresses(&mut self) -> ::protobuf::RepeatedField<mac_addr_type> {
        ::std::mem::replace(&mut self.unicast_mac_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_unicast_mac_addresses(&self) -> &[mac_addr_type] {
        &self.unicast_mac_addresses
    }

    fn get_unicast_mac_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<mac_addr_type> {
        &self.unicast_mac_addresses
    }

    fn mut_unicast_mac_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<mac_addr_type> {
        &mut self.unicast_mac_addresses
    }
}

impl ::protobuf::Message for eth_ctrlr_ucast_mac_filter {
    fn is_initialized(&self) -> bool {
        for v in &self.unicast_mac_addresses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.unicast_mac_addresses)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.unicast_mac_addresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.unicast_mac_addresses {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_ucast_mac_filter {
    fn new() -> eth_ctrlr_ucast_mac_filter {
        eth_ctrlr_ucast_mac_filter::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_ucast_mac_filter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<mac_addr_type>>(
                    "unicast_mac_addresses",
                    eth_ctrlr_ucast_mac_filter::get_unicast_mac_addresses_for_reflect,
                    eth_ctrlr_ucast_mac_filter::mut_unicast_mac_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_ucast_mac_filter>(
                    "eth_ctrlr_ucast_mac_filter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_ucast_mac_filter {
    fn clear(&mut self) {
        self.clear_unicast_mac_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_ucast_mac_filter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_ucast_mac_filter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_mcast_mac_filter {
    // message fields
    pub multicast_promiscuous: u32,
    pub multicast_mac_addresses: ::protobuf::RepeatedField<ether_mcast_mac_type_>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_mcast_mac_filter {}

impl eth_ctrlr_mcast_mac_filter {
    pub fn new() -> eth_ctrlr_mcast_mac_filter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_mcast_mac_filter {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_mcast_mac_filter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_mcast_mac_filter,
        };
        unsafe {
            instance.get(eth_ctrlr_mcast_mac_filter::new)
        }
    }

    // uint32 multicast_promiscuous = 1;

    pub fn clear_multicast_promiscuous(&mut self) {
        self.multicast_promiscuous = 0;
    }

    // Param is passed by value, moved
    pub fn set_multicast_promiscuous(&mut self, v: u32) {
        self.multicast_promiscuous = v;
    }

    pub fn get_multicast_promiscuous(&self) -> u32 {
        self.multicast_promiscuous
    }

    fn get_multicast_promiscuous_for_reflect(&self) -> &u32 {
        &self.multicast_promiscuous
    }

    fn mut_multicast_promiscuous_for_reflect(&mut self) -> &mut u32 {
        &mut self.multicast_promiscuous
    }

    // repeated .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_mcast_mac_type_ multicast_mac_addresses = 2;

    pub fn clear_multicast_mac_addresses(&mut self) {
        self.multicast_mac_addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_multicast_mac_addresses(&mut self, v: ::protobuf::RepeatedField<ether_mcast_mac_type_>) {
        self.multicast_mac_addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_multicast_mac_addresses(&mut self) -> &mut ::protobuf::RepeatedField<ether_mcast_mac_type_> {
        &mut self.multicast_mac_addresses
    }

    // Take field
    pub fn take_multicast_mac_addresses(&mut self) -> ::protobuf::RepeatedField<ether_mcast_mac_type_> {
        ::std::mem::replace(&mut self.multicast_mac_addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_multicast_mac_addresses(&self) -> &[ether_mcast_mac_type_] {
        &self.multicast_mac_addresses
    }

    fn get_multicast_mac_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<ether_mcast_mac_type_> {
        &self.multicast_mac_addresses
    }

    fn mut_multicast_mac_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ether_mcast_mac_type_> {
        &mut self.multicast_mac_addresses
    }
}

impl ::protobuf::Message for eth_ctrlr_mcast_mac_filter {
    fn is_initialized(&self) -> bool {
        for v in &self.multicast_mac_addresses {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.multicast_promiscuous = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.multicast_mac_addresses)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.multicast_promiscuous != 0 {
            my_size += ::protobuf::rt::value_size(1, self.multicast_promiscuous, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.multicast_mac_addresses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.multicast_promiscuous != 0 {
            os.write_uint32(1, self.multicast_promiscuous)?;
        }
        for v in &self.multicast_mac_addresses {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_mcast_mac_filter {
    fn new() -> eth_ctrlr_mcast_mac_filter {
        eth_ctrlr_mcast_mac_filter::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_mcast_mac_filter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "multicast_promiscuous",
                    eth_ctrlr_mcast_mac_filter::get_multicast_promiscuous_for_reflect,
                    eth_ctrlr_mcast_mac_filter::mut_multicast_promiscuous_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ether_mcast_mac_type_>>(
                    "multicast_mac_addresses",
                    eth_ctrlr_mcast_mac_filter::get_multicast_mac_addresses_for_reflect,
                    eth_ctrlr_mcast_mac_filter::mut_multicast_mac_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_mcast_mac_filter>(
                    "eth_ctrlr_mcast_mac_filter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_mcast_mac_filter {
    fn clear(&mut self) {
        self.clear_multicast_promiscuous();
        self.clear_multicast_mac_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_mcast_mac_filter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_mcast_mac_filter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ether_fec_details {
    // message fields
    pub fec: ::std::string::String,
    pub corrected_codeword_count: u64,
    pub uncorrected_codeword_count: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ether_fec_details {}

impl ether_fec_details {
    pub fn new() -> ether_fec_details {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ether_fec_details {
        static mut instance: ::protobuf::lazy::Lazy<ether_fec_details> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ether_fec_details,
        };
        unsafe {
            instance.get(ether_fec_details::new)
        }
    }

    // string fec = 1;

    pub fn clear_fec(&mut self) {
        self.fec.clear();
    }

    // Param is passed by value, moved
    pub fn set_fec(&mut self, v: ::std::string::String) {
        self.fec = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fec(&mut self) -> &mut ::std::string::String {
        &mut self.fec
    }

    // Take field
    pub fn take_fec(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.fec, ::std::string::String::new())
    }

    pub fn get_fec(&self) -> &str {
        &self.fec
    }

    fn get_fec_for_reflect(&self) -> &::std::string::String {
        &self.fec
    }

    fn mut_fec_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.fec
    }

    // uint64 corrected_codeword_count = 2;

    pub fn clear_corrected_codeword_count(&mut self) {
        self.corrected_codeword_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_corrected_codeword_count(&mut self, v: u64) {
        self.corrected_codeword_count = v;
    }

    pub fn get_corrected_codeword_count(&self) -> u64 {
        self.corrected_codeword_count
    }

    fn get_corrected_codeword_count_for_reflect(&self) -> &u64 {
        &self.corrected_codeword_count
    }

    fn mut_corrected_codeword_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.corrected_codeword_count
    }

    // uint64 uncorrected_codeword_count = 3;

    pub fn clear_uncorrected_codeword_count(&mut self) {
        self.uncorrected_codeword_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_uncorrected_codeword_count(&mut self, v: u64) {
        self.uncorrected_codeword_count = v;
    }

    pub fn get_uncorrected_codeword_count(&self) -> u64 {
        self.uncorrected_codeword_count
    }

    fn get_uncorrected_codeword_count_for_reflect(&self) -> &u64 {
        &self.uncorrected_codeword_count
    }

    fn mut_uncorrected_codeword_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.uncorrected_codeword_count
    }
}

impl ::protobuf::Message for ether_fec_details {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.fec)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.corrected_codeword_count = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.uncorrected_codeword_count = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.fec.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.fec);
        }
        if self.corrected_codeword_count != 0 {
            my_size += ::protobuf::rt::value_size(2, self.corrected_codeword_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.uncorrected_codeword_count != 0 {
            my_size += ::protobuf::rt::value_size(3, self.uncorrected_codeword_count, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.fec.is_empty() {
            os.write_string(1, &self.fec)?;
        }
        if self.corrected_codeword_count != 0 {
            os.write_uint64(2, self.corrected_codeword_count)?;
        }
        if self.uncorrected_codeword_count != 0 {
            os.write_uint64(3, self.uncorrected_codeword_count)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ether_fec_details {
    fn new() -> ether_fec_details {
        ether_fec_details::new()
    }

    fn descriptor_static(_: ::std::option::Option<ether_fec_details>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "fec",
                    ether_fec_details::get_fec_for_reflect,
                    ether_fec_details::mut_fec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "corrected_codeword_count",
                    ether_fec_details::get_corrected_codeword_count_for_reflect,
                    ether_fec_details::mut_corrected_codeword_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "uncorrected_codeword_count",
                    ether_fec_details::get_uncorrected_codeword_count_for_reflect,
                    ether_fec_details::mut_uncorrected_codeword_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ether_fec_details>(
                    "ether_fec_details",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ether_fec_details {
    fn clear(&mut self) {
        self.clear_fec();
        self.clear_corrected_codeword_count();
        self.clear_uncorrected_codeword_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ether_fec_details {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ether_fec_details {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ether_pfc_details_type {
    // message fields
    pub priority_flowcontrol: ::std::string::String,
    pub priority_enabled_bitmap: u32,
    pub rx_frames: ::std::vec::Vec<u64>,
    pub tx_frames: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ether_pfc_details_type {}

impl ether_pfc_details_type {
    pub fn new() -> ether_pfc_details_type {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ether_pfc_details_type {
        static mut instance: ::protobuf::lazy::Lazy<ether_pfc_details_type> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ether_pfc_details_type,
        };
        unsafe {
            instance.get(ether_pfc_details_type::new)
        }
    }

    // string priority_flowcontrol = 1;

    pub fn clear_priority_flowcontrol(&mut self) {
        self.priority_flowcontrol.clear();
    }

    // Param is passed by value, moved
    pub fn set_priority_flowcontrol(&mut self, v: ::std::string::String) {
        self.priority_flowcontrol = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_priority_flowcontrol(&mut self) -> &mut ::std::string::String {
        &mut self.priority_flowcontrol
    }

    // Take field
    pub fn take_priority_flowcontrol(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.priority_flowcontrol, ::std::string::String::new())
    }

    pub fn get_priority_flowcontrol(&self) -> &str {
        &self.priority_flowcontrol
    }

    fn get_priority_flowcontrol_for_reflect(&self) -> &::std::string::String {
        &self.priority_flowcontrol
    }

    fn mut_priority_flowcontrol_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.priority_flowcontrol
    }

    // uint32 priority_enabled_bitmap = 2;

    pub fn clear_priority_enabled_bitmap(&mut self) {
        self.priority_enabled_bitmap = 0;
    }

    // Param is passed by value, moved
    pub fn set_priority_enabled_bitmap(&mut self, v: u32) {
        self.priority_enabled_bitmap = v;
    }

    pub fn get_priority_enabled_bitmap(&self) -> u32 {
        self.priority_enabled_bitmap
    }

    fn get_priority_enabled_bitmap_for_reflect(&self) -> &u32 {
        &self.priority_enabled_bitmap
    }

    fn mut_priority_enabled_bitmap_for_reflect(&mut self) -> &mut u32 {
        &mut self.priority_enabled_bitmap
    }

    // repeated uint64 rx_frames = 3;

    pub fn clear_rx_frames(&mut self) {
        self.rx_frames.clear();
    }

    // Param is passed by value, moved
    pub fn set_rx_frames(&mut self, v: ::std::vec::Vec<u64>) {
        self.rx_frames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rx_frames(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.rx_frames
    }

    // Take field
    pub fn take_rx_frames(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.rx_frames, ::std::vec::Vec::new())
    }

    pub fn get_rx_frames(&self) -> &[u64] {
        &self.rx_frames
    }

    fn get_rx_frames_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.rx_frames
    }

    fn mut_rx_frames_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.rx_frames
    }

    // repeated uint64 tx_frames = 4;

    pub fn clear_tx_frames(&mut self) {
        self.tx_frames.clear();
    }

    // Param is passed by value, moved
    pub fn set_tx_frames(&mut self, v: ::std::vec::Vec<u64>) {
        self.tx_frames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tx_frames(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.tx_frames
    }

    // Take field
    pub fn take_tx_frames(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.tx_frames, ::std::vec::Vec::new())
    }

    pub fn get_tx_frames(&self) -> &[u64] {
        &self.tx_frames
    }

    fn get_tx_frames_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.tx_frames
    }

    fn mut_tx_frames_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.tx_frames
    }
}

impl ::protobuf::Message for ether_pfc_details_type {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.priority_flowcontrol)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.priority_enabled_bitmap = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.rx_frames)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.tx_frames)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.priority_flowcontrol.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.priority_flowcontrol);
        }
        if self.priority_enabled_bitmap != 0 {
            my_size += ::protobuf::rt::value_size(2, self.priority_enabled_bitmap, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.rx_frames {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.tx_frames {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.priority_flowcontrol.is_empty() {
            os.write_string(1, &self.priority_flowcontrol)?;
        }
        if self.priority_enabled_bitmap != 0 {
            os.write_uint32(2, self.priority_enabled_bitmap)?;
        }
        for v in &self.rx_frames {
            os.write_uint64(3, *v)?;
        };
        for v in &self.tx_frames {
            os.write_uint64(4, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ether_pfc_details_type {
    fn new() -> ether_pfc_details_type {
        ether_pfc_details_type::new()
    }

    fn descriptor_static(_: ::std::option::Option<ether_pfc_details_type>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "priority_flowcontrol",
                    ether_pfc_details_type::get_priority_flowcontrol_for_reflect,
                    ether_pfc_details_type::mut_priority_flowcontrol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "priority_enabled_bitmap",
                    ether_pfc_details_type::get_priority_enabled_bitmap_for_reflect,
                    ether_pfc_details_type::mut_priority_enabled_bitmap_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "rx_frames",
                    ether_pfc_details_type::get_rx_frames_for_reflect,
                    ether_pfc_details_type::mut_rx_frames_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tx_frames",
                    ether_pfc_details_type::get_tx_frames_for_reflect,
                    ether_pfc_details_type::mut_tx_frames_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ether_pfc_details_type>(
                    "ether_pfc_details_type",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ether_pfc_details_type {
    fn clear(&mut self) {
        self.clear_priority_flowcontrol();
        self.clear_priority_enabled_bitmap();
        self.clear_rx_frames();
        self.clear_tx_frames();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ether_pfc_details_type {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ether_pfc_details_type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_phy_info {
    // message fields
    pub media_type: ::std::string::String,
    pub phy_present: ::std::string::String,
    pub phy_details: ::protobuf::SingularPtrField<ether_phy_details>,
    pub loopback: ::std::string::String,
    pub fec_details: ::protobuf::SingularPtrField<ether_fec_details>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_phy_info {}

impl eth_ctrlr_phy_info {
    pub fn new() -> eth_ctrlr_phy_info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_phy_info {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_phy_info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_phy_info,
        };
        unsafe {
            instance.get(eth_ctrlr_phy_info::new)
        }
    }

    // string media_type = 1;

    pub fn clear_media_type(&mut self) {
        self.media_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_media_type(&mut self, v: ::std::string::String) {
        self.media_type = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_media_type(&mut self) -> &mut ::std::string::String {
        &mut self.media_type
    }

    // Take field
    pub fn take_media_type(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.media_type, ::std::string::String::new())
    }

    pub fn get_media_type(&self) -> &str {
        &self.media_type
    }

    fn get_media_type_for_reflect(&self) -> &::std::string::String {
        &self.media_type
    }

    fn mut_media_type_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.media_type
    }

    // string phy_present = 2;

    pub fn clear_phy_present(&mut self) {
        self.phy_present.clear();
    }

    // Param is passed by value, moved
    pub fn set_phy_present(&mut self, v: ::std::string::String) {
        self.phy_present = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_phy_present(&mut self) -> &mut ::std::string::String {
        &mut self.phy_present
    }

    // Take field
    pub fn take_phy_present(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.phy_present, ::std::string::String::new())
    }

    pub fn get_phy_present(&self) -> &str {
        &self.phy_present
    }

    fn get_phy_present_for_reflect(&self) -> &::std::string::String {
        &self.phy_present
    }

    fn mut_phy_present_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.phy_present
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_phy_details phy_details = 3;

    pub fn clear_phy_details(&mut self) {
        self.phy_details.clear();
    }

    pub fn has_phy_details(&self) -> bool {
        self.phy_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phy_details(&mut self, v: ether_phy_details) {
        self.phy_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_phy_details(&mut self) -> &mut ether_phy_details {
        if self.phy_details.is_none() {
            self.phy_details.set_default();
        }
        self.phy_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_phy_details(&mut self) -> ether_phy_details {
        self.phy_details.take().unwrap_or_else(|| ether_phy_details::new())
    }

    pub fn get_phy_details(&self) -> &ether_phy_details {
        self.phy_details.as_ref().unwrap_or_else(|| ether_phy_details::default_instance())
    }

    fn get_phy_details_for_reflect(&self) -> &::protobuf::SingularPtrField<ether_phy_details> {
        &self.phy_details
    }

    fn mut_phy_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ether_phy_details> {
        &mut self.phy_details
    }

    // string loopback = 4;

    pub fn clear_loopback(&mut self) {
        self.loopback.clear();
    }

    // Param is passed by value, moved
    pub fn set_loopback(&mut self, v: ::std::string::String) {
        self.loopback = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_loopback(&mut self) -> &mut ::std::string::String {
        &mut self.loopback
    }

    // Take field
    pub fn take_loopback(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.loopback, ::std::string::String::new())
    }

    pub fn get_loopback(&self) -> &str {
        &self.loopback
    }

    fn get_loopback_for_reflect(&self) -> &::std::string::String {
        &self.loopback
    }

    fn mut_loopback_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.loopback
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_fec_details fec_details = 5;

    pub fn clear_fec_details(&mut self) {
        self.fec_details.clear();
    }

    pub fn has_fec_details(&self) -> bool {
        self.fec_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fec_details(&mut self, v: ether_fec_details) {
        self.fec_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fec_details(&mut self) -> &mut ether_fec_details {
        if self.fec_details.is_none() {
            self.fec_details.set_default();
        }
        self.fec_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_fec_details(&mut self) -> ether_fec_details {
        self.fec_details.take().unwrap_or_else(|| ether_fec_details::new())
    }

    pub fn get_fec_details(&self) -> &ether_fec_details {
        self.fec_details.as_ref().unwrap_or_else(|| ether_fec_details::default_instance())
    }

    fn get_fec_details_for_reflect(&self) -> &::protobuf::SingularPtrField<ether_fec_details> {
        &self.fec_details
    }

    fn mut_fec_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ether_fec_details> {
        &mut self.fec_details
    }
}

impl ::protobuf::Message for eth_ctrlr_phy_info {
    fn is_initialized(&self) -> bool {
        for v in &self.phy_details {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fec_details {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.media_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.phy_present)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.phy_details)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.loopback)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fec_details)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.media_type.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.media_type);
        }
        if !self.phy_present.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.phy_present);
        }
        if let Some(ref v) = self.phy_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.loopback.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.loopback);
        }
        if let Some(ref v) = self.fec_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.media_type.is_empty() {
            os.write_string(1, &self.media_type)?;
        }
        if !self.phy_present.is_empty() {
            os.write_string(2, &self.phy_present)?;
        }
        if let Some(ref v) = self.phy_details.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.loopback.is_empty() {
            os.write_string(4, &self.loopback)?;
        }
        if let Some(ref v) = self.fec_details.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_phy_info {
    fn new() -> eth_ctrlr_phy_info {
        eth_ctrlr_phy_info::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_phy_info>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "media_type",
                    eth_ctrlr_phy_info::get_media_type_for_reflect,
                    eth_ctrlr_phy_info::mut_media_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "phy_present",
                    eth_ctrlr_phy_info::get_phy_present_for_reflect,
                    eth_ctrlr_phy_info::mut_phy_present_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ether_phy_details>>(
                    "phy_details",
                    eth_ctrlr_phy_info::get_phy_details_for_reflect,
                    eth_ctrlr_phy_info::mut_phy_details_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "loopback",
                    eth_ctrlr_phy_info::get_loopback_for_reflect,
                    eth_ctrlr_phy_info::mut_loopback_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ether_fec_details>>(
                    "fec_details",
                    eth_ctrlr_phy_info::get_fec_details_for_reflect,
                    eth_ctrlr_phy_info::mut_fec_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_phy_info>(
                    "eth_ctrlr_phy_info",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_phy_info {
    fn clear(&mut self) {
        self.clear_media_type();
        self.clear_phy_present();
        self.clear_phy_details();
        self.clear_loopback();
        self.clear_fec_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_phy_info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_phy_info {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_l1_info {
    // message fields
    pub link_state: ::std::string::String,
    pub led_state: ::std::string::String,
    pub autoneg: ::protobuf::SingularPtrField<ether_autoneg_>,
    pub speed: ::std::string::String,
    pub duplex: ::std::string::String,
    pub flowcontrol: ::std::string::String,
    pub ipg: ::std::string::String,
    pub current_alarms: ::protobuf::SingularPtrField<eth_ctrlr_alarms>,
    pub previous_alarms: ::protobuf::SingularPtrField<eth_ctrlr_alarms>,
    pub error_counts: ::protobuf::SingularPtrField<eth_ctrlr_error_counters>,
    pub ber_monitoring: ::protobuf::SingularPtrField<eth_ber_monitoring>,
    pub laser_squelch_enabled: u32,
    pub bandwidth_utilization: u32,
    pub pfc_info: ::protobuf::SingularPtrField<ether_pfc_details_type>,
    pub bandwidth: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_l1_info {}

impl eth_ctrlr_l1_info {
    pub fn new() -> eth_ctrlr_l1_info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_l1_info {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_l1_info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_l1_info,
        };
        unsafe {
            instance.get(eth_ctrlr_l1_info::new)
        }
    }

    // string link_state = 1;

    pub fn clear_link_state(&mut self) {
        self.link_state.clear();
    }

    // Param is passed by value, moved
    pub fn set_link_state(&mut self, v: ::std::string::String) {
        self.link_state = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link_state(&mut self) -> &mut ::std::string::String {
        &mut self.link_state
    }

    // Take field
    pub fn take_link_state(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.link_state, ::std::string::String::new())
    }

    pub fn get_link_state(&self) -> &str {
        &self.link_state
    }

    fn get_link_state_for_reflect(&self) -> &::std::string::String {
        &self.link_state
    }

    fn mut_link_state_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.link_state
    }

    // string led_state = 2;

    pub fn clear_led_state(&mut self) {
        self.led_state.clear();
    }

    // Param is passed by value, moved
    pub fn set_led_state(&mut self, v: ::std::string::String) {
        self.led_state = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_led_state(&mut self) -> &mut ::std::string::String {
        &mut self.led_state
    }

    // Take field
    pub fn take_led_state(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.led_state, ::std::string::String::new())
    }

    pub fn get_led_state(&self) -> &str {
        &self.led_state
    }

    fn get_led_state_for_reflect(&self) -> &::std::string::String {
        &self.led_state
    }

    fn mut_led_state_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.led_state
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_autoneg_ autoneg = 3;

    pub fn clear_autoneg(&mut self) {
        self.autoneg.clear();
    }

    pub fn has_autoneg(&self) -> bool {
        self.autoneg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_autoneg(&mut self, v: ether_autoneg_) {
        self.autoneg = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_autoneg(&mut self) -> &mut ether_autoneg_ {
        if self.autoneg.is_none() {
            self.autoneg.set_default();
        }
        self.autoneg.as_mut().unwrap()
    }

    // Take field
    pub fn take_autoneg(&mut self) -> ether_autoneg_ {
        self.autoneg.take().unwrap_or_else(|| ether_autoneg_::new())
    }

    pub fn get_autoneg(&self) -> &ether_autoneg_ {
        self.autoneg.as_ref().unwrap_or_else(|| ether_autoneg_::default_instance())
    }

    fn get_autoneg_for_reflect(&self) -> &::protobuf::SingularPtrField<ether_autoneg_> {
        &self.autoneg
    }

    fn mut_autoneg_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ether_autoneg_> {
        &mut self.autoneg
    }

    // string speed = 4;

    pub fn clear_speed(&mut self) {
        self.speed.clear();
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: ::std::string::String) {
        self.speed = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_speed(&mut self) -> &mut ::std::string::String {
        &mut self.speed
    }

    // Take field
    pub fn take_speed(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.speed, ::std::string::String::new())
    }

    pub fn get_speed(&self) -> &str {
        &self.speed
    }

    fn get_speed_for_reflect(&self) -> &::std::string::String {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.speed
    }

    // string duplex = 5;

    pub fn clear_duplex(&mut self) {
        self.duplex.clear();
    }

    // Param is passed by value, moved
    pub fn set_duplex(&mut self, v: ::std::string::String) {
        self.duplex = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_duplex(&mut self) -> &mut ::std::string::String {
        &mut self.duplex
    }

    // Take field
    pub fn take_duplex(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.duplex, ::std::string::String::new())
    }

    pub fn get_duplex(&self) -> &str {
        &self.duplex
    }

    fn get_duplex_for_reflect(&self) -> &::std::string::String {
        &self.duplex
    }

    fn mut_duplex_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.duplex
    }

    // string flowcontrol = 6;

    pub fn clear_flowcontrol(&mut self) {
        self.flowcontrol.clear();
    }

    // Param is passed by value, moved
    pub fn set_flowcontrol(&mut self, v: ::std::string::String) {
        self.flowcontrol = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_flowcontrol(&mut self) -> &mut ::std::string::String {
        &mut self.flowcontrol
    }

    // Take field
    pub fn take_flowcontrol(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.flowcontrol, ::std::string::String::new())
    }

    pub fn get_flowcontrol(&self) -> &str {
        &self.flowcontrol
    }

    fn get_flowcontrol_for_reflect(&self) -> &::std::string::String {
        &self.flowcontrol
    }

    fn mut_flowcontrol_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.flowcontrol
    }

    // string ipg = 7;

    pub fn clear_ipg(&mut self) {
        self.ipg.clear();
    }

    // Param is passed by value, moved
    pub fn set_ipg(&mut self, v: ::std::string::String) {
        self.ipg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ipg(&mut self) -> &mut ::std::string::String {
        &mut self.ipg
    }

    // Take field
    pub fn take_ipg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ipg, ::std::string::String::new())
    }

    pub fn get_ipg(&self) -> &str {
        &self.ipg
    }

    fn get_ipg_for_reflect(&self) -> &::std::string::String {
        &self.ipg
    }

    fn mut_ipg_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ipg
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_alarms current_alarms = 8;

    pub fn clear_current_alarms(&mut self) {
        self.current_alarms.clear();
    }

    pub fn has_current_alarms(&self) -> bool {
        self.current_alarms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_alarms(&mut self, v: eth_ctrlr_alarms) {
        self.current_alarms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_current_alarms(&mut self) -> &mut eth_ctrlr_alarms {
        if self.current_alarms.is_none() {
            self.current_alarms.set_default();
        }
        self.current_alarms.as_mut().unwrap()
    }

    // Take field
    pub fn take_current_alarms(&mut self) -> eth_ctrlr_alarms {
        self.current_alarms.take().unwrap_or_else(|| eth_ctrlr_alarms::new())
    }

    pub fn get_current_alarms(&self) -> &eth_ctrlr_alarms {
        self.current_alarms.as_ref().unwrap_or_else(|| eth_ctrlr_alarms::default_instance())
    }

    fn get_current_alarms_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_alarms> {
        &self.current_alarms
    }

    fn mut_current_alarms_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_alarms> {
        &mut self.current_alarms
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_alarms previous_alarms = 9;

    pub fn clear_previous_alarms(&mut self) {
        self.previous_alarms.clear();
    }

    pub fn has_previous_alarms(&self) -> bool {
        self.previous_alarms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previous_alarms(&mut self, v: eth_ctrlr_alarms) {
        self.previous_alarms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_alarms(&mut self) -> &mut eth_ctrlr_alarms {
        if self.previous_alarms.is_none() {
            self.previous_alarms.set_default();
        }
        self.previous_alarms.as_mut().unwrap()
    }

    // Take field
    pub fn take_previous_alarms(&mut self) -> eth_ctrlr_alarms {
        self.previous_alarms.take().unwrap_or_else(|| eth_ctrlr_alarms::new())
    }

    pub fn get_previous_alarms(&self) -> &eth_ctrlr_alarms {
        self.previous_alarms.as_ref().unwrap_or_else(|| eth_ctrlr_alarms::default_instance())
    }

    fn get_previous_alarms_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_alarms> {
        &self.previous_alarms
    }

    fn mut_previous_alarms_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_alarms> {
        &mut self.previous_alarms
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_error_counters error_counts = 10;

    pub fn clear_error_counts(&mut self) {
        self.error_counts.clear();
    }

    pub fn has_error_counts(&self) -> bool {
        self.error_counts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_counts(&mut self, v: eth_ctrlr_error_counters) {
        self.error_counts = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_counts(&mut self) -> &mut eth_ctrlr_error_counters {
        if self.error_counts.is_none() {
            self.error_counts.set_default();
        }
        self.error_counts.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_counts(&mut self) -> eth_ctrlr_error_counters {
        self.error_counts.take().unwrap_or_else(|| eth_ctrlr_error_counters::new())
    }

    pub fn get_error_counts(&self) -> &eth_ctrlr_error_counters {
        self.error_counts.as_ref().unwrap_or_else(|| eth_ctrlr_error_counters::default_instance())
    }

    fn get_error_counts_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_error_counters> {
        &self.error_counts
    }

    fn mut_error_counts_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_error_counters> {
        &mut self.error_counts
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ber_monitoring ber_monitoring = 11;

    pub fn clear_ber_monitoring(&mut self) {
        self.ber_monitoring.clear();
    }

    pub fn has_ber_monitoring(&self) -> bool {
        self.ber_monitoring.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ber_monitoring(&mut self, v: eth_ber_monitoring) {
        self.ber_monitoring = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ber_monitoring(&mut self) -> &mut eth_ber_monitoring {
        if self.ber_monitoring.is_none() {
            self.ber_monitoring.set_default();
        }
        self.ber_monitoring.as_mut().unwrap()
    }

    // Take field
    pub fn take_ber_monitoring(&mut self) -> eth_ber_monitoring {
        self.ber_monitoring.take().unwrap_or_else(|| eth_ber_monitoring::new())
    }

    pub fn get_ber_monitoring(&self) -> &eth_ber_monitoring {
        self.ber_monitoring.as_ref().unwrap_or_else(|| eth_ber_monitoring::default_instance())
    }

    fn get_ber_monitoring_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ber_monitoring> {
        &self.ber_monitoring
    }

    fn mut_ber_monitoring_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ber_monitoring> {
        &mut self.ber_monitoring
    }

    // uint32 laser_squelch_enabled = 12;

    pub fn clear_laser_squelch_enabled(&mut self) {
        self.laser_squelch_enabled = 0;
    }

    // Param is passed by value, moved
    pub fn set_laser_squelch_enabled(&mut self, v: u32) {
        self.laser_squelch_enabled = v;
    }

    pub fn get_laser_squelch_enabled(&self) -> u32 {
        self.laser_squelch_enabled
    }

    fn get_laser_squelch_enabled_for_reflect(&self) -> &u32 {
        &self.laser_squelch_enabled
    }

    fn mut_laser_squelch_enabled_for_reflect(&mut self) -> &mut u32 {
        &mut self.laser_squelch_enabled
    }

    // uint32 bandwidth_utilization = 13;

    pub fn clear_bandwidth_utilization(&mut self) {
        self.bandwidth_utilization = 0;
    }

    // Param is passed by value, moved
    pub fn set_bandwidth_utilization(&mut self, v: u32) {
        self.bandwidth_utilization = v;
    }

    pub fn get_bandwidth_utilization(&self) -> u32 {
        self.bandwidth_utilization
    }

    fn get_bandwidth_utilization_for_reflect(&self) -> &u32 {
        &self.bandwidth_utilization
    }

    fn mut_bandwidth_utilization_for_reflect(&mut self) -> &mut u32 {
        &mut self.bandwidth_utilization
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_pfc_details_type pfc_info = 14;

    pub fn clear_pfc_info(&mut self) {
        self.pfc_info.clear();
    }

    pub fn has_pfc_info(&self) -> bool {
        self.pfc_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pfc_info(&mut self, v: ether_pfc_details_type) {
        self.pfc_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pfc_info(&mut self) -> &mut ether_pfc_details_type {
        if self.pfc_info.is_none() {
            self.pfc_info.set_default();
        }
        self.pfc_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_pfc_info(&mut self) -> ether_pfc_details_type {
        self.pfc_info.take().unwrap_or_else(|| ether_pfc_details_type::new())
    }

    pub fn get_pfc_info(&self) -> &ether_pfc_details_type {
        self.pfc_info.as_ref().unwrap_or_else(|| ether_pfc_details_type::default_instance())
    }

    fn get_pfc_info_for_reflect(&self) -> &::protobuf::SingularPtrField<ether_pfc_details_type> {
        &self.pfc_info
    }

    fn mut_pfc_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ether_pfc_details_type> {
        &mut self.pfc_info
    }

    // uint64 bandwidth = 15;

    pub fn clear_bandwidth(&mut self) {
        self.bandwidth = 0;
    }

    // Param is passed by value, moved
    pub fn set_bandwidth(&mut self, v: u64) {
        self.bandwidth = v;
    }

    pub fn get_bandwidth(&self) -> u64 {
        self.bandwidth
    }

    fn get_bandwidth_for_reflect(&self) -> &u64 {
        &self.bandwidth
    }

    fn mut_bandwidth_for_reflect(&mut self) -> &mut u64 {
        &mut self.bandwidth
    }
}

impl ::protobuf::Message for eth_ctrlr_l1_info {
    fn is_initialized(&self) -> bool {
        for v in &self.autoneg {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.current_alarms {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.previous_alarms {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.error_counts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ber_monitoring {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pfc_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.link_state)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.led_state)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.autoneg)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.speed)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.duplex)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.flowcontrol)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ipg)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.current_alarms)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.previous_alarms)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error_counts)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ber_monitoring)?;
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.laser_squelch_enabled = tmp;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bandwidth_utilization = tmp;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pfc_info)?;
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bandwidth = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.link_state.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.link_state);
        }
        if !self.led_state.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.led_state);
        }
        if let Some(ref v) = self.autoneg.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.speed.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.speed);
        }
        if !self.duplex.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.duplex);
        }
        if !self.flowcontrol.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.flowcontrol);
        }
        if !self.ipg.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.ipg);
        }
        if let Some(ref v) = self.current_alarms.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.previous_alarms.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error_counts.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ber_monitoring.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.laser_squelch_enabled != 0 {
            my_size += ::protobuf::rt::value_size(12, self.laser_squelch_enabled, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bandwidth_utilization != 0 {
            my_size += ::protobuf::rt::value_size(13, self.bandwidth_utilization, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.pfc_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.bandwidth != 0 {
            my_size += ::protobuf::rt::value_size(15, self.bandwidth, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.link_state.is_empty() {
            os.write_string(1, &self.link_state)?;
        }
        if !self.led_state.is_empty() {
            os.write_string(2, &self.led_state)?;
        }
        if let Some(ref v) = self.autoneg.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.speed.is_empty() {
            os.write_string(4, &self.speed)?;
        }
        if !self.duplex.is_empty() {
            os.write_string(5, &self.duplex)?;
        }
        if !self.flowcontrol.is_empty() {
            os.write_string(6, &self.flowcontrol)?;
        }
        if !self.ipg.is_empty() {
            os.write_string(7, &self.ipg)?;
        }
        if let Some(ref v) = self.current_alarms.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.previous_alarms.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error_counts.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ber_monitoring.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.laser_squelch_enabled != 0 {
            os.write_uint32(12, self.laser_squelch_enabled)?;
        }
        if self.bandwidth_utilization != 0 {
            os.write_uint32(13, self.bandwidth_utilization)?;
        }
        if let Some(ref v) = self.pfc_info.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.bandwidth != 0 {
            os.write_uint64(15, self.bandwidth)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_l1_info {
    fn new() -> eth_ctrlr_l1_info {
        eth_ctrlr_l1_info::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_l1_info>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "link_state",
                    eth_ctrlr_l1_info::get_link_state_for_reflect,
                    eth_ctrlr_l1_info::mut_link_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "led_state",
                    eth_ctrlr_l1_info::get_led_state_for_reflect,
                    eth_ctrlr_l1_info::mut_led_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ether_autoneg_>>(
                    "autoneg",
                    eth_ctrlr_l1_info::get_autoneg_for_reflect,
                    eth_ctrlr_l1_info::mut_autoneg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "speed",
                    eth_ctrlr_l1_info::get_speed_for_reflect,
                    eth_ctrlr_l1_info::mut_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "duplex",
                    eth_ctrlr_l1_info::get_duplex_for_reflect,
                    eth_ctrlr_l1_info::mut_duplex_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "flowcontrol",
                    eth_ctrlr_l1_info::get_flowcontrol_for_reflect,
                    eth_ctrlr_l1_info::mut_flowcontrol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ipg",
                    eth_ctrlr_l1_info::get_ipg_for_reflect,
                    eth_ctrlr_l1_info::mut_ipg_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_alarms>>(
                    "current_alarms",
                    eth_ctrlr_l1_info::get_current_alarms_for_reflect,
                    eth_ctrlr_l1_info::mut_current_alarms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_alarms>>(
                    "previous_alarms",
                    eth_ctrlr_l1_info::get_previous_alarms_for_reflect,
                    eth_ctrlr_l1_info::mut_previous_alarms_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_error_counters>>(
                    "error_counts",
                    eth_ctrlr_l1_info::get_error_counts_for_reflect,
                    eth_ctrlr_l1_info::mut_error_counts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ber_monitoring>>(
                    "ber_monitoring",
                    eth_ctrlr_l1_info::get_ber_monitoring_for_reflect,
                    eth_ctrlr_l1_info::mut_ber_monitoring_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "laser_squelch_enabled",
                    eth_ctrlr_l1_info::get_laser_squelch_enabled_for_reflect,
                    eth_ctrlr_l1_info::mut_laser_squelch_enabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bandwidth_utilization",
                    eth_ctrlr_l1_info::get_bandwidth_utilization_for_reflect,
                    eth_ctrlr_l1_info::mut_bandwidth_utilization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ether_pfc_details_type>>(
                    "pfc_info",
                    eth_ctrlr_l1_info::get_pfc_info_for_reflect,
                    eth_ctrlr_l1_info::mut_pfc_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bandwidth",
                    eth_ctrlr_l1_info::get_bandwidth_for_reflect,
                    eth_ctrlr_l1_info::mut_bandwidth_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_l1_info>(
                    "eth_ctrlr_l1_info",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_l1_info {
    fn clear(&mut self) {
        self.clear_link_state();
        self.clear_led_state();
        self.clear_autoneg();
        self.clear_speed();
        self.clear_duplex();
        self.clear_flowcontrol();
        self.clear_ipg();
        self.clear_current_alarms();
        self.clear_previous_alarms();
        self.clear_error_counts();
        self.clear_ber_monitoring();
        self.clear_laser_squelch_enabled();
        self.clear_bandwidth_utilization();
        self.clear_pfc_info();
        self.clear_bandwidth();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_l1_info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_l1_info {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctrlr_mac_info {
    // message fields
    pub mtu: u32,
    pub mru: u32,
    pub burned_in_mac_address: ::std::string::String,
    pub operational_mac_address: ::protobuf::SingularPtrField<mac_addr_type>,
    pub unicast_mac_filters: ::protobuf::SingularPtrField<eth_ctrlr_ucast_mac_filter>,
    pub multicast_mac_filters: ::protobuf::SingularPtrField<eth_ctrlr_mcast_mac_filter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctrlr_mac_info {}

impl eth_ctrlr_mac_info {
    pub fn new() -> eth_ctrlr_mac_info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctrlr_mac_info {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctrlr_mac_info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctrlr_mac_info,
        };
        unsafe {
            instance.get(eth_ctrlr_mac_info::new)
        }
    }

    // uint32 mtu = 1;

    pub fn clear_mtu(&mut self) {
        self.mtu = 0;
    }

    // Param is passed by value, moved
    pub fn set_mtu(&mut self, v: u32) {
        self.mtu = v;
    }

    pub fn get_mtu(&self) -> u32 {
        self.mtu
    }

    fn get_mtu_for_reflect(&self) -> &u32 {
        &self.mtu
    }

    fn mut_mtu_for_reflect(&mut self) -> &mut u32 {
        &mut self.mtu
    }

    // uint32 mru = 2;

    pub fn clear_mru(&mut self) {
        self.mru = 0;
    }

    // Param is passed by value, moved
    pub fn set_mru(&mut self, v: u32) {
        self.mru = v;
    }

    pub fn get_mru(&self) -> u32 {
        self.mru
    }

    fn get_mru_for_reflect(&self) -> &u32 {
        &self.mru
    }

    fn mut_mru_for_reflect(&mut self) -> &mut u32 {
        &mut self.mru
    }

    // string burned_in_mac_address = 3;

    pub fn clear_burned_in_mac_address(&mut self) {
        self.burned_in_mac_address.clear();
    }

    // Param is passed by value, moved
    pub fn set_burned_in_mac_address(&mut self, v: ::std::string::String) {
        self.burned_in_mac_address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_burned_in_mac_address(&mut self) -> &mut ::std::string::String {
        &mut self.burned_in_mac_address
    }

    // Take field
    pub fn take_burned_in_mac_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.burned_in_mac_address, ::std::string::String::new())
    }

    pub fn get_burned_in_mac_address(&self) -> &str {
        &self.burned_in_mac_address
    }

    fn get_burned_in_mac_address_for_reflect(&self) -> &::std::string::String {
        &self.burned_in_mac_address
    }

    fn mut_burned_in_mac_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.burned_in_mac_address
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.mac_addr_type operational_mac_address = 4;

    pub fn clear_operational_mac_address(&mut self) {
        self.operational_mac_address.clear();
    }

    pub fn has_operational_mac_address(&self) -> bool {
        self.operational_mac_address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operational_mac_address(&mut self, v: mac_addr_type) {
        self.operational_mac_address = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_operational_mac_address(&mut self) -> &mut mac_addr_type {
        if self.operational_mac_address.is_none() {
            self.operational_mac_address.set_default();
        }
        self.operational_mac_address.as_mut().unwrap()
    }

    // Take field
    pub fn take_operational_mac_address(&mut self) -> mac_addr_type {
        self.operational_mac_address.take().unwrap_or_else(|| mac_addr_type::new())
    }

    pub fn get_operational_mac_address(&self) -> &mac_addr_type {
        self.operational_mac_address.as_ref().unwrap_or_else(|| mac_addr_type::default_instance())
    }

    fn get_operational_mac_address_for_reflect(&self) -> &::protobuf::SingularPtrField<mac_addr_type> {
        &self.operational_mac_address
    }

    fn mut_operational_mac_address_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<mac_addr_type> {
        &mut self.operational_mac_address
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_ucast_mac_filter unicast_mac_filters = 5;

    pub fn clear_unicast_mac_filters(&mut self) {
        self.unicast_mac_filters.clear();
    }

    pub fn has_unicast_mac_filters(&self) -> bool {
        self.unicast_mac_filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unicast_mac_filters(&mut self, v: eth_ctrlr_ucast_mac_filter) {
        self.unicast_mac_filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unicast_mac_filters(&mut self) -> &mut eth_ctrlr_ucast_mac_filter {
        if self.unicast_mac_filters.is_none() {
            self.unicast_mac_filters.set_default();
        }
        self.unicast_mac_filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_unicast_mac_filters(&mut self) -> eth_ctrlr_ucast_mac_filter {
        self.unicast_mac_filters.take().unwrap_or_else(|| eth_ctrlr_ucast_mac_filter::new())
    }

    pub fn get_unicast_mac_filters(&self) -> &eth_ctrlr_ucast_mac_filter {
        self.unicast_mac_filters.as_ref().unwrap_or_else(|| eth_ctrlr_ucast_mac_filter::default_instance())
    }

    fn get_unicast_mac_filters_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_ucast_mac_filter> {
        &self.unicast_mac_filters
    }

    fn mut_unicast_mac_filters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_ucast_mac_filter> {
        &mut self.unicast_mac_filters
    }

    // .cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_mcast_mac_filter multicast_mac_filters = 6;

    pub fn clear_multicast_mac_filters(&mut self) {
        self.multicast_mac_filters.clear();
    }

    pub fn has_multicast_mac_filters(&self) -> bool {
        self.multicast_mac_filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_multicast_mac_filters(&mut self, v: eth_ctrlr_mcast_mac_filter) {
        self.multicast_mac_filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_multicast_mac_filters(&mut self) -> &mut eth_ctrlr_mcast_mac_filter {
        if self.multicast_mac_filters.is_none() {
            self.multicast_mac_filters.set_default();
        }
        self.multicast_mac_filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_multicast_mac_filters(&mut self) -> eth_ctrlr_mcast_mac_filter {
        self.multicast_mac_filters.take().unwrap_or_else(|| eth_ctrlr_mcast_mac_filter::new())
    }

    pub fn get_multicast_mac_filters(&self) -> &eth_ctrlr_mcast_mac_filter {
        self.multicast_mac_filters.as_ref().unwrap_or_else(|| eth_ctrlr_mcast_mac_filter::default_instance())
    }

    fn get_multicast_mac_filters_for_reflect(&self) -> &::protobuf::SingularPtrField<eth_ctrlr_mcast_mac_filter> {
        &self.multicast_mac_filters
    }

    fn mut_multicast_mac_filters_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<eth_ctrlr_mcast_mac_filter> {
        &mut self.multicast_mac_filters
    }
}

impl ::protobuf::Message for eth_ctrlr_mac_info {
    fn is_initialized(&self) -> bool {
        for v in &self.operational_mac_address {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unicast_mac_filters {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.multicast_mac_filters {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mtu = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mru = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.burned_in_mac_address)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.operational_mac_address)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unicast_mac_filters)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.multicast_mac_filters)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.mtu != 0 {
            my_size += ::protobuf::rt::value_size(1, self.mtu, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.mru != 0 {
            my_size += ::protobuf::rt::value_size(2, self.mru, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.burned_in_mac_address.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.burned_in_mac_address);
        }
        if let Some(ref v) = self.operational_mac_address.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unicast_mac_filters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.multicast_mac_filters.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.mtu != 0 {
            os.write_uint32(1, self.mtu)?;
        }
        if self.mru != 0 {
            os.write_uint32(2, self.mru)?;
        }
        if !self.burned_in_mac_address.is_empty() {
            os.write_string(3, &self.burned_in_mac_address)?;
        }
        if let Some(ref v) = self.operational_mac_address.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unicast_mac_filters.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.multicast_mac_filters.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctrlr_mac_info {
    fn new() -> eth_ctrlr_mac_info {
        eth_ctrlr_mac_info::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctrlr_mac_info>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mtu",
                    eth_ctrlr_mac_info::get_mtu_for_reflect,
                    eth_ctrlr_mac_info::mut_mtu_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mru",
                    eth_ctrlr_mac_info::get_mru_for_reflect,
                    eth_ctrlr_mac_info::mut_mru_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "burned_in_mac_address",
                    eth_ctrlr_mac_info::get_burned_in_mac_address_for_reflect,
                    eth_ctrlr_mac_info::mut_burned_in_mac_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<mac_addr_type>>(
                    "operational_mac_address",
                    eth_ctrlr_mac_info::get_operational_mac_address_for_reflect,
                    eth_ctrlr_mac_info::mut_operational_mac_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_ucast_mac_filter>>(
                    "unicast_mac_filters",
                    eth_ctrlr_mac_info::get_unicast_mac_filters_for_reflect,
                    eth_ctrlr_mac_info::mut_unicast_mac_filters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<eth_ctrlr_mcast_mac_filter>>(
                    "multicast_mac_filters",
                    eth_ctrlr_mac_info::get_multicast_mac_filters_for_reflect,
                    eth_ctrlr_mac_info::mut_multicast_mac_filters_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctrlr_mac_info>(
                    "eth_ctrlr_mac_info",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctrlr_mac_info {
    fn clear(&mut self) {
        self.clear_mtu();
        self.clear_mru();
        self.clear_burned_in_mac_address();
        self.clear_operational_mac_address();
        self.clear_unicast_mac_filters();
        self.clear_multicast_mac_filters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctrlr_mac_info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctrlr_mac_info {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct eth_ctlr_transport_info {
    // message fields
    pub maintenance_mode_enabled: u32,
    pub ains_status: ::std::string::String,
    pub total_duration: u32,
    pub remaining_duration: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for eth_ctlr_transport_info {}

impl eth_ctlr_transport_info {
    pub fn new() -> eth_ctlr_transport_info {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static eth_ctlr_transport_info {
        static mut instance: ::protobuf::lazy::Lazy<eth_ctlr_transport_info> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const eth_ctlr_transport_info,
        };
        unsafe {
            instance.get(eth_ctlr_transport_info::new)
        }
    }

    // uint32 maintenance_mode_enabled = 1;

    pub fn clear_maintenance_mode_enabled(&mut self) {
        self.maintenance_mode_enabled = 0;
    }

    // Param is passed by value, moved
    pub fn set_maintenance_mode_enabled(&mut self, v: u32) {
        self.maintenance_mode_enabled = v;
    }

    pub fn get_maintenance_mode_enabled(&self) -> u32 {
        self.maintenance_mode_enabled
    }

    fn get_maintenance_mode_enabled_for_reflect(&self) -> &u32 {
        &self.maintenance_mode_enabled
    }

    fn mut_maintenance_mode_enabled_for_reflect(&mut self) -> &mut u32 {
        &mut self.maintenance_mode_enabled
    }

    // string ains_status = 2;

    pub fn clear_ains_status(&mut self) {
        self.ains_status.clear();
    }

    // Param is passed by value, moved
    pub fn set_ains_status(&mut self, v: ::std::string::String) {
        self.ains_status = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ains_status(&mut self) -> &mut ::std::string::String {
        &mut self.ains_status
    }

    // Take field
    pub fn take_ains_status(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ains_status, ::std::string::String::new())
    }

    pub fn get_ains_status(&self) -> &str {
        &self.ains_status
    }

    fn get_ains_status_for_reflect(&self) -> &::std::string::String {
        &self.ains_status
    }

    fn mut_ains_status_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ains_status
    }

    // uint32 total_duration = 3;

    pub fn clear_total_duration(&mut self) {
        self.total_duration = 0;
    }

    // Param is passed by value, moved
    pub fn set_total_duration(&mut self, v: u32) {
        self.total_duration = v;
    }

    pub fn get_total_duration(&self) -> u32 {
        self.total_duration
    }

    fn get_total_duration_for_reflect(&self) -> &u32 {
        &self.total_duration
    }

    fn mut_total_duration_for_reflect(&mut self) -> &mut u32 {
        &mut self.total_duration
    }

    // uint32 remaining_duration = 4;

    pub fn clear_remaining_duration(&mut self) {
        self.remaining_duration = 0;
    }

    // Param is passed by value, moved
    pub fn set_remaining_duration(&mut self, v: u32) {
        self.remaining_duration = v;
    }

    pub fn get_remaining_duration(&self) -> u32 {
        self.remaining_duration
    }

    fn get_remaining_duration_for_reflect(&self) -> &u32 {
        &self.remaining_duration
    }

    fn mut_remaining_duration_for_reflect(&mut self) -> &mut u32 {
        &mut self.remaining_duration
    }
}

impl ::protobuf::Message for eth_ctlr_transport_info {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.maintenance_mode_enabled = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ains_status)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.total_duration = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.remaining_duration = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.maintenance_mode_enabled != 0 {
            my_size += ::protobuf::rt::value_size(1, self.maintenance_mode_enabled, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.ains_status.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ains_status);
        }
        if self.total_duration != 0 {
            my_size += ::protobuf::rt::value_size(3, self.total_duration, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.remaining_duration != 0 {
            my_size += ::protobuf::rt::value_size(4, self.remaining_duration, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.maintenance_mode_enabled != 0 {
            os.write_uint32(1, self.maintenance_mode_enabled)?;
        }
        if !self.ains_status.is_empty() {
            os.write_string(2, &self.ains_status)?;
        }
        if self.total_duration != 0 {
            os.write_uint32(3, self.total_duration)?;
        }
        if self.remaining_duration != 0 {
            os.write_uint32(4, self.remaining_duration)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for eth_ctlr_transport_info {
    fn new() -> eth_ctlr_transport_info {
        eth_ctlr_transport_info::new()
    }

    fn descriptor_static(_: ::std::option::Option<eth_ctlr_transport_info>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "maintenance_mode_enabled",
                    eth_ctlr_transport_info::get_maintenance_mode_enabled_for_reflect,
                    eth_ctlr_transport_info::mut_maintenance_mode_enabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ains_status",
                    eth_ctlr_transport_info::get_ains_status_for_reflect,
                    eth_ctlr_transport_info::mut_ains_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "total_duration",
                    eth_ctlr_transport_info::get_total_duration_for_reflect,
                    eth_ctlr_transport_info::mut_total_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "remaining_duration",
                    eth_ctlr_transport_info::get_remaining_duration_for_reflect,
                    eth_ctlr_transport_info::mut_remaining_duration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<eth_ctlr_transport_info>(
                    "eth_ctlr_transport_info",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for eth_ctlr_transport_info {
    fn clear(&mut self) {
        self.clear_maintenance_mode_enabled();
        self.clear_ains_status();
        self.clear_total_duration();
        self.clear_remaining_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for eth_ctlr_transport_info {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for eth_ctlr_transport_info {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bproto.proto\x12Kcisco_ios_xr_drivers_media_eth_oper.ethernet_inter\
    face.interfaces.interface\"T\n+ethernet_controller_driver_bag_v2_type_KE\
    YS\x12%\n\x0einterface_name\x18\x01\x20\x01(\tR\rinterfaceName\"\xf4\x04\
    \n&ethernet_controller_driver_bag_v2_type\x12\x1f\n\x0badmin_state\x182\
    \x20\x01(\tR\nadminState\x12\"\n\roper_state_up\x183\x20\x01(\rR\x0boper\
    StateUp\x12z\n\x08phy_info\x184\x20\x01(\x0b2_.cisco_ios_xr_drivers_medi\
    a_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_infoR\
    \x07phyInfo\x12\x7f\n\x0blayer1_info\x185\x20\x01(\x0b2^.cisco_ios_xr_dr\
    ivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_l\
    1_infoR\nlayer1Info\x12z\n\x08mac_info\x186\x20\x01(\x0b2_.cisco_ios_xr_\
    drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr\
    _mac_infoR\x07macInfo\x12\x8b\x01\n\x0etransport_info\x187\x20\x01(\x0b2\
    d.cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.inte\
    rface.eth_ctlr_transport_infoR\rtransportInfo\"\xd8\x01\n\x0eether_auton\
    eg_\x12'\n\x0fautoneg_enabled\x18\x01\x20\x01(\x11R\x0eautonegEnabled\
    \x12\x12\n\x04mask\x18\x02\x20\x01(\rR\x04mask\x12\x14\n\x05speed\x18\
    \x03\x20\x01(\tR\x05speed\x12\x16\n\x06duplex\x18\x04\x20\x01(\tR\x06dup\
    lex\x12\x20\n\x0bflowcontrol\x18\x05\x20\x01(\tR\x0bflowcontrol\x12'\n\
    \x0fconfig_override\x18\x06\x20\x01(\x11R\x0econfigOverride\x12\x10\n\
    \x03fec\x18\x07\x20\x01(\tR\x03fec\"%\n\rmac_addr_type\x12\x14\n\x05valu\
    e\x18\x01\x20\x01(\tR\x05value\"L\n\x15ether_mcast_mac_type_\x12\x1f\n\
    \x0bmac_address\x18\x01\x20\x01(\tR\nmacAddress\x12\x12\n\x04mask\x18\
    \x02\x20\x01(\tR\x04mask\"\xb1\x01\n\x1deth_ctrlr_phy_dom_lane_alarms\
    \x120\n\x14transmit_laser_power\x18\x01\x20\x01(\tR\x12transmitLaserPowe\
    r\x120\n\x14received_laser_power\x18\x02\x20\x01(\tR\x12receivedLaserPow\
    er\x12,\n\x12laser_bias_current\x18\x03\x20\x01(\tR\x10laserBiasCurrent\
    \"\xdc\x01\n#eth_ctrlr_phy_lane_opt_mon_validity\x12)\n\x10wavelength_va\
    lid\x18\x01\x20\x01(\x11R\x0fwavelengthValid\x120\n\x14transmit_power_va\
    lid\x18\x02\x20\x01(\x11R\x12transmitPowerValid\x12.\n\x13receive_power_\
    valid\x18\x03\x20\x01(\x11R\x11receivePowerValid\x12(\n\x10laser_bias_va\
    lid\x18\x04\x20\x01(\x11R\x0elaserBiasValid\"\xf3\x02\n\x1aeth_ctrlr_phy\
    _lane_opt_mon\x12+\n\x11center_wavelength\x18\x01\x20\x01(\rR\x10centerW\
    avelength\x120\n\x14transmit_laser_power\x18\x02\x20\x01(\x11R\x12transm\
    itLaserPower\x120\n\x14received_laser_power\x18\x03\x20\x01(\x11R\x12rec\
    eivedLaserPower\x12,\n\x12laser_bias_current\x18\x04\x20\x01(\rR\x10lase\
    rBiasCurrent\x12\x95\x01\n\x11dig_opt_mon_alarm\x18\x05\x20\x01(\x0b2j.c\
    isco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interfa\
    ce.eth_ctrlr_phy_dom_lane_alarmsR\x0edigOptMonAlarm\"\x96\x02\n\x18eth_c\
    trlr_phy_dom_alarms\x127\n\x17transceiver_temperature\x18\x01\x20\x01(\t\
    R\x16transceiverTemperature\x12/\n\x13transceiver_voltage\x18\x02\x20\
    \x01(\tR\x12transceiverVoltage\x120\n\x14transmit_laser_power\x18\x03\
    \x20\x01(\tR\x12transmitLaserPower\x120\n\x14received_laser_power\x18\
    \x04\x20\x01(\tR\x12receivedLaserPower\x12,\n\x12laser_bias_current\x18\
    \x05\x20\x01(\tR\x10laserBiasCurrent\"\x84\x02\n$eth_ctrlr_phy_dom_thres\
    hold_validity\x12+\n\x11temperature_valid\x18\x01\x20\x01(\x11R\x10tempe\
    ratureValid\x12#\n\rvoltage_valid\x18\x02\x20\x01(\x11R\x0cvoltageValid\
    \x12(\n\x10laser_bias_valid\x18\x03\x20\x01(\x11R\x0elaserBiasValid\x120\
    \n\x14transmit_power_valid\x18\x04\x20\x01(\x11R\x12transmitPowerValid\
    \x12.\n\x13receive_power_valid\x18\x05\x20\x01(\x11R\x11receivePowerVali\
    d\"\xa9\x0c\n\x1ceth_ctrlr_phy_dom_thresholds\x12\x98\x01\n\x0efield_val\
    idity\x18\x01\x20\x01(\x0b2q.cisco_ios_xr_drivers_media_eth_oper.etherne\
    t_interface.interfaces.interface.eth_ctrlr_phy_dom_threshold_validityR\r\
    fieldValidity\x12K\n\"transceiver_temperature_alarm_high\x18\x02\x20\x01\
    (\x11R\x1ftransceiverTemperatureAlarmHigh\x12O\n$transceiver_temperature\
    _warning_high\x18\x03\x20\x01(\x11R!transceiverTemperatureWarningHigh\
    \x12M\n#transceiver_temperature_warning_low\x18\x04\x20\x01(\x11R\x20tra\
    nsceiverTemperatureWarningLow\x12I\n!transceiver_temperature_alarm_low\
    \x18\x05\x20\x01(\x11R\x1etransceiverTemperatureAlarmLow\x12C\n\x1etrans\
    ceiver_voltage_alarm_high\x18\x06\x20\x01(\rR\x1btransceiverVoltageAlarm\
    High\x12G\n\x20transceiver_voltage_warning_high\x18\x07\x20\x01(\rR\x1dt\
    ransceiverVoltageWarningHigh\x12E\n\x1ftransceiver_voltage_warning_low\
    \x18\x08\x20\x01(\rR\x1ctransceiverVoltageWarningLow\x12A\n\x1dtransceiv\
    er_voltage_alarm_low\x18\t\x20\x01(\rR\x1atransceiverVoltageAlarmLow\x12\
    1\n\x15laser_bias_alarm_high\x18\n\x20\x01(\rR\x12laserBiasAlarmHigh\x12\
    5\n\x17laser_bias_warning_high\x18\x0b\x20\x01(\rR\x14laserBiasWarningHi\
    gh\x123\n\x16laser_bias_warning_low\x18\x0c\x20\x01(\rR\x13laserBiasWarn\
    ingLow\x12/\n\x14laser_bias_alarm_low\x18\r\x20\x01(\rR\x11laserBiasAlar\
    mLow\x12H\n!optical_transmit_power_alarm_high\x18\x0e\x20\x01(\rR\x1dopt\
    icalTransmitPowerAlarmHigh\x12L\n#optical_transmit_power_warning_high\
    \x18\x0f\x20\x01(\rR\x1fopticalTransmitPowerWarningHigh\x12J\n\"optical_\
    transmit_power_warning_low\x18\x10\x20\x01(\rR\x1eopticalTransmitPowerWa\
    rningLow\x12F\n\x20optical_transmit_power_alarm_low\x18\x11\x20\x01(\rR\
    \x1copticalTransmitPowerAlarmLow\x12F\n\x20optical_receive_power_alarm_h\
    igh\x18\x12\x20\x01(\rR\x1copticalReceivePowerAlarmHigh\x12J\n\"optical_\
    receive_power_warning_high\x18\x13\x20\x01(\rR\x1eopticalReceivePowerWar\
    ningHigh\x12H\n!optical_receive_power_warning_low\x18\x14\x20\x01(\rR\
    \x1dopticalReceivePowerWarningLow\x12D\n\x1foptical_receive_power_alarm_\
    low\x18\x15\x20\x01(\rR\x1bopticalReceivePowerAlarmLow\"\x97\x08\n\x11et\
    her_phy_details\x12\x16\n\x06vendor\x18\x01\x20\x01(\tR\x06vendor\x12,\n\
    \x12vendor_part_number\x18\x02\x20\x01(\tR\x10vendorPartNumber\x120\n\
    \x14vendor_serial_number\x18\x03\x20\x01(\tR\x12vendorSerialNumber\x127\
    \n\x17transceiver_temperature\x18\x04\x20\x01(\x11R\x16transceiverTemper\
    ature\x12/\n\x13transceiver_voltage\x18\x05\x20\x01(\x11R\x12transceiver\
    Voltage\x120\n\x14transceiver_tx_power\x18\x06\x20\x01(\x11R\x12transcei\
    verTxPower\x120\n\x14transceiver_rx_power\x18\x07\x20\x01(\x11R\x12trans\
    ceiverRxPower\x12.\n\x13transceiver_tx_bias\x18\x08\x20\x01(\x11R\x11tra\
    nsceiverTxBias\x12{\n\x04lane\x18\t\x20\x03(\x0b2g.cisco_ios_xr_drivers_\
    media_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_lan\
    e_opt_monR\x04lane\x12\xa0\x01\n\x13lane_field_validity\x18\n\x20\x01(\
    \x0b2p.cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces\
    .interface.eth_ctrlr_phy_lane_opt_mon_validityR\x11laneFieldValidity\x12\
    \xa9\x01\n\x1cdig_opt_mon_alarm_thresholds\x18\x0b\x20\x01(\x0b2i.cisco_\
    ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.et\
    h_ctrlr_phy_dom_thresholdsR\x18digOptMonAlarmThresholds\x12\x92\x01\n\
    \x12dig_opt_mon_alarms\x18\x0c\x20\x01(\x0b2e.cisco_ios_xr_drivers_media\
    _eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_phy_dom_alar\
    msR\x0fdigOptMonAlarms\x12+\n\x11optics_wavelength\x18\r\x20\x01(\rR\x10\
    opticsWavelength\"\xc4\x03\n\x10eth_ctrlr_alarms\x12@\n\x1dreceived_loss\
    _of_signal_alarm\x18\x01\x20\x01(\tR\x19receivedLossOfSignalAlarm\x12=\n\
    \x1cpcs_loss_of_block_lock_alarm\x18\x02\x20\x01(\tR\x17pcsLossOfBlockLo\
    ckAlarm\x12*\n\x11local_fault_alarm\x18\x03\x20\x01(\tR\x0flocalFaultAla\
    rm\x12,\n\x12remote_fault_alarm\x18\x04\x20\x01(\tR\x10remoteFaultAlarm\
    \x12\x20\n\x0csd_ber_alarm\x18\x05\x20\x01(\tR\nsdBerAlarm\x12\x20\n\x0c\
    sf_ber_alarm\x18\x06\x20\x01(\tR\nsfBerAlarm\x12J\n\"loss_of_synchroniza\
    tion_data_alarm\x18\x07\x20\x01(\tR\x1elossOfSynchronizationDataAlarm\
    \x12\x20\n\x0chi_ber_alarm\x18\x08\x20\x01(\tR\nhiBerAlarm\x12#\n\rsquel\
    ch_alarm\x18\t\x20\x01(\tR\x0csquelchAlarm\"m\n\x18eth_ctrlr_error_count\
    ers\x12,\n\x12sync_header_errors\x18\x01\x20\x01(\x04R\x10syncHeaderErro\
    rs\x12#\n\rpcsbip_errors\x18\x02\x20\x01(\x04R\x0cpcsbipErrors\"\x90\x02\
    \n\x12ether_ber_settings\x128\n\x18signal_degrade_threshold\x18\x01\x20\
    \x01(\rR\x16signalDegradeThreshold\x120\n\x14signal_degrade_alarm\x18\
    \x02\x20\x01(\x11R\x12signalDegradeAlarm\x122\n\x15signal_fail_threshold\
    \x18\x03\x20\x01(\rR\x13signalFailThreshold\x12*\n\x11signal_fail_alarm\
    \x18\x04\x20\x01(\x11R\x0fsignalFailAlarm\x12.\n\x13signal_remote_fault\
    \x18\x05\x20\x01(\x11R\x11signalRemoteFault\"\xaf\x01\n\x12eth_ber_monit\
    oring\x12\x1c\n\tsupported\x18\x01\x20\x01(\x11R\tsupported\x12{\n\x08se\
    ttings\x18\x02\x20\x01(\x0b2_.cisco_ios_xr_drivers_media_eth_oper.ethern\
    et_interface.interfaces.interface.ether_ber_settingsR\x08settings\"\xad\
    \x01\n\x1aeth_ctrlr_ucast_mac_filter\x12\x8e\x01\n\x15unicast_mac_addres\
    ses\x18\x01\x20\x03(\x0b2Z.cisco_ios_xr_drivers_media_eth_oper.ethernet_\
    interface.interfaces.interface.mac_addr_typeR\x13unicastMacAddresses\"\
    \xee\x01\n\x1aeth_ctrlr_mcast_mac_filter\x123\n\x15multicast_promiscuous\
    \x18\x01\x20\x01(\rR\x14multicastPromiscuous\x12\x9a\x01\n\x17multicast_\
    mac_addresses\x18\x02\x20\x03(\x0b2b.cisco_ios_xr_drivers_media_eth_oper\
    .ethernet_interface.interfaces.interface.ether_mcast_mac_type_R\x15multi\
    castMacAddresses\"\x9d\x01\n\x11ether_fec_details\x12\x10\n\x03fec\x18\
    \x01\x20\x01(\tR\x03fec\x128\n\x18corrected_codeword_count\x18\x02\x20\
    \x01(\x04R\x16correctedCodewordCount\x12<\n\x1auncorrected_codeword_coun\
    t\x18\x03\x20\x01(\x04R\x18uncorrectedCodewordCount\"\xbd\x01\n\x16ether\
    _pfc_details_type\x121\n\x14priority_flowcontrol\x18\x01\x20\x01(\tR\x13\
    priorityFlowcontrol\x126\n\x17priority_enabled_bitmap\x18\x02\x20\x01(\r\
    R\x15priorityEnabledBitmap\x12\x1b\n\trx_frames\x18\x03\x20\x03(\x04R\
    \x08rxFrames\x12\x1b\n\ttx_frames\x18\x04\x20\x03(\x04R\x08txFrames\"\
    \xf2\x02\n\x12eth_ctrlr_phy_info\x12\x1d\n\nmedia_type\x18\x01\x20\x01(\
    \tR\tmediaType\x12\x1f\n\x0bphy_present\x18\x02\x20\x01(\tR\nphyPresent\
    \x12\x7f\n\x0bphy_details\x18\x03\x20\x01(\x0b2^.cisco_ios_xr_drivers_me\
    dia_eth_oper.ethernet_interface.interfaces.interface.ether_phy_detailsR\
    \nphyDetails\x12\x1a\n\x08loopback\x18\x04\x20\x01(\tR\x08loopback\x12\
    \x7f\n\x0bfec_details\x18\x05\x20\x01(\x0b2^.cisco_ios_xr_drivers_media_\
    eth_oper.ethernet_interface.interfaces.interface.ether_fec_detailsR\nfec\
    Details\"\xd3\x08\n\x11eth_ctrlr_l1_info\x12\x1d\n\nlink_state\x18\x01\
    \x20\x01(\tR\tlinkState\x12\x1b\n\tled_state\x18\x02\x20\x01(\tR\x08ledS\
    tate\x12u\n\x07autoneg\x18\x03\x20\x01(\x0b2[.cisco_ios_xr_drivers_media\
    _eth_oper.ethernet_interface.interfaces.interface.ether_autoneg_R\x07aut\
    oneg\x12\x14\n\x05speed\x18\x04\x20\x01(\tR\x05speed\x12\x16\n\x06duplex\
    \x18\x05\x20\x01(\tR\x06duplex\x12\x20\n\x0bflowcontrol\x18\x06\x20\x01(\
    \tR\x0bflowcontrol\x12\x10\n\x03ipg\x18\x07\x20\x01(\tR\x03ipg\x12\x84\
    \x01\n\x0ecurrent_alarms\x18\x08\x20\x01(\x0b2].cisco_ios_xr_drivers_med\
    ia_eth_oper.ethernet_interface.interfaces.interface.eth_ctrlr_alarmsR\rc\
    urrentAlarms\x12\x86\x01\n\x0fprevious_alarms\x18\t\x20\x01(\x0b2].cisco\
    _ios_xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.e\
    th_ctrlr_alarmsR\x0epreviousAlarms\x12\x88\x01\n\x0cerror_counts\x18\n\
    \x20\x01(\x0b2e.cisco_ios_xr_drivers_media_eth_oper.ethernet_interface.i\
    nterfaces.interface.eth_ctrlr_error_countersR\x0berrorCounts\x12\x86\x01\
    \n\x0eber_monitoring\x18\x0b\x20\x01(\x0b2_.cisco_ios_xr_drivers_media_e\
    th_oper.ethernet_interface.interfaces.interface.eth_ber_monitoringR\rber\
    Monitoring\x122\n\x15laser_squelch_enabled\x18\x0c\x20\x01(\rR\x13laserS\
    quelchEnabled\x123\n\x15bandwidth_utilization\x18\r\x20\x01(\rR\x14bandw\
    idthUtilization\x12~\n\x08pfc_info\x18\x0e\x20\x01(\x0b2c.cisco_ios_xr_d\
    rivers_media_eth_oper.ethernet_interface.interfaces.interface.ether_pfc_\
    details_typeR\x07pfcInfo\x12\x1c\n\tbandwidth\x18\x0f\x20\x01(\x04R\tban\
    dwidth\"\xb8\x04\n\x12eth_ctrlr_mac_info\x12\x10\n\x03mtu\x18\x01\x20\
    \x01(\rR\x03mtu\x12\x10\n\x03mru\x18\x02\x20\x01(\rR\x03mru\x121\n\x15bu\
    rned_in_mac_address\x18\x03\x20\x01(\tR\x12burnedInMacAddress\x12\x92\
    \x01\n\x17operational_mac_address\x18\x04\x20\x01(\x0b2Z.cisco_ios_xr_dr\
    ivers_media_eth_oper.ethernet_interface.interfaces.interface.mac_addr_ty\
    peR\x15operationalMacAddress\x12\x97\x01\n\x13unicast_mac_filters\x18\
    \x05\x20\x01(\x0b2g.cisco_ios_xr_drivers_media_eth_oper.ethernet_interfa\
    ce.interfaces.interface.eth_ctrlr_ucast_mac_filterR\x11unicastMacFilters\
    \x12\x9b\x01\n\x15multicast_mac_filters\x18\x06\x20\x01(\x0b2g.cisco_ios\
    _xr_drivers_media_eth_oper.ethernet_interface.interfaces.interface.eth_c\
    trlr_mcast_mac_filterR\x13multicastMacFilters\"\xca\x01\n\x17eth_ctlr_tr\
    ansport_info\x128\n\x18maintenance_mode_enabled\x18\x01\x20\x01(\rR\x16m\
    aintenanceModeEnabled\x12\x1f\n\x0bains_status\x18\x02\x20\x01(\tR\nains\
    Status\x12%\n\x0etotal_duration\x18\x03\x20\x01(\rR\rtotalDuration\x12-\
    \n\x12remaining_duration\x18\x04\x20\x01(\rR\x11remainingDurationb\x06pr\
    oto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
