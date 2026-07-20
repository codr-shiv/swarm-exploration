#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Pause_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__Pause_Request__init(msg: *mut Pause_Request) -> bool;
    fn slam_toolbox__srv__Pause_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pause_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__Pause_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pause_Request>);
    fn slam_toolbox__srv__Pause_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pause_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Pause_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__Pause_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Pause_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__Pause_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__Pause_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pause_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Pause_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Pause_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Pause_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pause_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pause_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/Pause_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Pause_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Pause_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__Pause_Response__init(msg: *mut Pause_Response) -> bool;
    fn slam_toolbox__srv__Pause_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Pause_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__Pause_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Pause_Response>);
    fn slam_toolbox__srv__Pause_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Pause_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Pause_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__Pause_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for Pause_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__Pause_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__Pause_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Pause_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Pause_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Pause_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Pause_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Pause_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Pause_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/Pause_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Pause_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ClearQueue_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__ClearQueue_Request__init(msg: *mut ClearQueue_Request) -> bool;
    fn slam_toolbox__srv__ClearQueue_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearQueue_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__ClearQueue_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearQueue_Request>);
    fn slam_toolbox__srv__ClearQueue_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearQueue_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearQueue_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__ClearQueue_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearQueue_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ClearQueue_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__ClearQueue_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__ClearQueue_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearQueue_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ClearQueue_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ClearQueue_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ClearQueue_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearQueue_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearQueue_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/ClearQueue_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ClearQueue_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ClearQueue_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__ClearQueue_Response__init(msg: *mut ClearQueue_Response) -> bool;
    fn slam_toolbox__srv__ClearQueue_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ClearQueue_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__ClearQueue_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ClearQueue_Response>);
    fn slam_toolbox__srv__ClearQueue_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ClearQueue_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ClearQueue_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__ClearQueue_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearQueue_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for ClearQueue_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__ClearQueue_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__ClearQueue_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ClearQueue_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ClearQueue_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ClearQueue_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ClearQueue_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ClearQueue_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ClearQueue_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/ClearQueue_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ClearQueue_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ToggleInteractive_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__ToggleInteractive_Request__init(msg: *mut ToggleInteractive_Request) -> bool;
    fn slam_toolbox__srv__ToggleInteractive_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToggleInteractive_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__ToggleInteractive_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToggleInteractive_Request>);
    fn slam_toolbox__srv__ToggleInteractive_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToggleInteractive_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<ToggleInteractive_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__ToggleInteractive_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToggleInteractive_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ToggleInteractive_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__ToggleInteractive_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__ToggleInteractive_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToggleInteractive_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ToggleInteractive_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ToggleInteractive_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ToggleInteractive_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToggleInteractive_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToggleInteractive_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/ToggleInteractive_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ToggleInteractive_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ToggleInteractive_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__ToggleInteractive_Response__init(msg: *mut ToggleInteractive_Response) -> bool;
    fn slam_toolbox__srv__ToggleInteractive_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ToggleInteractive_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__ToggleInteractive_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ToggleInteractive_Response>);
    fn slam_toolbox__srv__ToggleInteractive_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ToggleInteractive_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<ToggleInteractive_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__ToggleInteractive_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToggleInteractive_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ToggleInteractive_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__ToggleInteractive_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__ToggleInteractive_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ToggleInteractive_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ToggleInteractive_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ToggleInteractive_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__ToggleInteractive_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ToggleInteractive_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ToggleInteractive_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/ToggleInteractive_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__ToggleInteractive_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Clear_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__Clear_Request__init(msg: *mut Clear_Request) -> bool;
    fn slam_toolbox__srv__Clear_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Clear_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__Clear_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Clear_Request>);
    fn slam_toolbox__srv__Clear_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Clear_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Clear_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__Clear_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Clear_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Clear_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__Clear_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__Clear_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Clear_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Clear_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Clear_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Clear_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Clear_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Clear_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/Clear_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Clear_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Clear_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__Clear_Response__init(msg: *mut Clear_Response) -> bool;
    fn slam_toolbox__srv__Clear_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Clear_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__Clear_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Clear_Response>);
    fn slam_toolbox__srv__Clear_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Clear_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Clear_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__Clear_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Clear_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Clear_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__Clear_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__Clear_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Clear_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Clear_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Clear_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__Clear_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Clear_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Clear_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/Clear_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__Clear_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SaveMap_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__SaveMap_Request__init(msg: *mut SaveMap_Request) -> bool;
    fn slam_toolbox__srv__SaveMap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__SaveMap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Request>);
    fn slam_toolbox__srv__SaveMap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveMap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__SaveMap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std_msgs::msg::rmw::String,

}



impl Default for SaveMap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__SaveMap_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__SaveMap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveMap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SaveMap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SaveMap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SaveMap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveMap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/SaveMap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SaveMap_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SaveMap_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__SaveMap_Response__init(msg: *mut SaveMap_Response) -> bool;
    fn slam_toolbox__srv__SaveMap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__SaveMap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Response>);
    fn slam_toolbox__srv__SaveMap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SaveMap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SaveMap_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__SaveMap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: u8,

}

impl SaveMap_Response {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESULT_SUCCESS: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESULT_NO_MAP_RECEIEVD: u8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESULT_UNDEFINED_FAILURE: u8 = 255;

}


impl Default for SaveMap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__SaveMap_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__SaveMap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SaveMap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SaveMap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SaveMap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SaveMap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SaveMap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/SaveMap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SaveMap_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__LoopClosure_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__LoopClosure_Request__init(msg: *mut LoopClosure_Request) -> bool;
    fn slam_toolbox__srv__LoopClosure_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoopClosure_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__LoopClosure_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoopClosure_Request>);
    fn slam_toolbox__srv__LoopClosure_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoopClosure_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<LoopClosure_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__LoopClosure_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoopClosure_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for LoopClosure_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__LoopClosure_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__LoopClosure_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoopClosure_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__LoopClosure_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__LoopClosure_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__LoopClosure_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoopClosure_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoopClosure_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/LoopClosure_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__LoopClosure_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__LoopClosure_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__LoopClosure_Response__init(msg: *mut LoopClosure_Response) -> bool;
    fn slam_toolbox__srv__LoopClosure_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LoopClosure_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__LoopClosure_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LoopClosure_Response>);
    fn slam_toolbox__srv__LoopClosure_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LoopClosure_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<LoopClosure_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__LoopClosure_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoopClosure_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for LoopClosure_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__LoopClosure_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__LoopClosure_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LoopClosure_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__LoopClosure_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__LoopClosure_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__LoopClosure_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LoopClosure_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LoopClosure_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/LoopClosure_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__LoopClosure_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__MergeMaps_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__MergeMaps_Request__init(msg: *mut MergeMaps_Request) -> bool;
    fn slam_toolbox__srv__MergeMaps_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MergeMaps_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__MergeMaps_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MergeMaps_Request>);
    fn slam_toolbox__srv__MergeMaps_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MergeMaps_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MergeMaps_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__MergeMaps_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MergeMaps_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MergeMaps_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__MergeMaps_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__MergeMaps_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MergeMaps_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__MergeMaps_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__MergeMaps_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__MergeMaps_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MergeMaps_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MergeMaps_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/MergeMaps_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__MergeMaps_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__MergeMaps_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__MergeMaps_Response__init(msg: *mut MergeMaps_Response) -> bool;
    fn slam_toolbox__srv__MergeMaps_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MergeMaps_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__MergeMaps_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MergeMaps_Response>);
    fn slam_toolbox__srv__MergeMaps_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MergeMaps_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MergeMaps_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__MergeMaps_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MergeMaps_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MergeMaps_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__MergeMaps_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__MergeMaps_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MergeMaps_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__MergeMaps_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__MergeMaps_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__MergeMaps_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MergeMaps_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MergeMaps_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/MergeMaps_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__MergeMaps_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__AddSubmap_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__AddSubmap_Request__init(msg: *mut AddSubmap_Request) -> bool;
    fn slam_toolbox__srv__AddSubmap_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddSubmap_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__AddSubmap_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddSubmap_Request>);
    fn slam_toolbox__srv__AddSubmap_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddSubmap_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddSubmap_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__AddSubmap_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddSubmap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,

}



impl Default for AddSubmap_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__AddSubmap_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__AddSubmap_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddSubmap_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__AddSubmap_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__AddSubmap_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__AddSubmap_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddSubmap_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddSubmap_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/AddSubmap_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__AddSubmap_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__AddSubmap_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__AddSubmap_Response__init(msg: *mut AddSubmap_Response) -> bool;
    fn slam_toolbox__srv__AddSubmap_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddSubmap_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__AddSubmap_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddSubmap_Response>);
    fn slam_toolbox__srv__AddSubmap_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddSubmap_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddSubmap_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__AddSubmap_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddSubmap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for AddSubmap_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__AddSubmap_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__AddSubmap_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddSubmap_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__AddSubmap_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__AddSubmap_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__AddSubmap_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddSubmap_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddSubmap_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/AddSubmap_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__AddSubmap_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__DeserializePoseGraph_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__DeserializePoseGraph_Request__init(msg: *mut DeserializePoseGraph_Request) -> bool;
    fn slam_toolbox__srv__DeserializePoseGraph_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeserializePoseGraph_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__DeserializePoseGraph_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeserializePoseGraph_Request>);
    fn slam_toolbox__srv__DeserializePoseGraph_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeserializePoseGraph_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<DeserializePoseGraph_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__DeserializePoseGraph_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeserializePoseGraph_Request {
    /// inital_pose should be Map -> base_frame (parameter, generally base_link)
    pub filename: rosidl_runtime_rs::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub match_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub initial_pose: geometry_msgs::msg::rmw::Pose2D,

}

impl DeserializePoseGraph_Request {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const UNSET: i8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const START_AT_FIRST_NODE: i8 = 1;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const START_AT_GIVEN_POSE: i8 = 2;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const LOCALIZE_AT_POSE: i8 = 3;

}


impl Default for DeserializePoseGraph_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__DeserializePoseGraph_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__DeserializePoseGraph_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeserializePoseGraph_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__DeserializePoseGraph_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__DeserializePoseGraph_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__DeserializePoseGraph_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeserializePoseGraph_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeserializePoseGraph_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/DeserializePoseGraph_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__DeserializePoseGraph_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__DeserializePoseGraph_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__DeserializePoseGraph_Response__init(msg: *mut DeserializePoseGraph_Response) -> bool;
    fn slam_toolbox__srv__DeserializePoseGraph_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<DeserializePoseGraph_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__DeserializePoseGraph_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<DeserializePoseGraph_Response>);
    fn slam_toolbox__srv__DeserializePoseGraph_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<DeserializePoseGraph_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<DeserializePoseGraph_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__DeserializePoseGraph_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeserializePoseGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeserializePoseGraph_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__DeserializePoseGraph_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__DeserializePoseGraph_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for DeserializePoseGraph_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__DeserializePoseGraph_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__DeserializePoseGraph_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__DeserializePoseGraph_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for DeserializePoseGraph_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for DeserializePoseGraph_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/DeserializePoseGraph_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__DeserializePoseGraph_Response() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SerializePoseGraph_Request() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__SerializePoseGraph_Request__init(msg: *mut SerializePoseGraph_Request) -> bool;
    fn slam_toolbox__srv__SerializePoseGraph_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SerializePoseGraph_Request>, size: usize) -> bool;
    fn slam_toolbox__srv__SerializePoseGraph_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SerializePoseGraph_Request>);
    fn slam_toolbox__srv__SerializePoseGraph_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SerializePoseGraph_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<SerializePoseGraph_Request>) -> bool;
}

// Corresponds to slam_toolbox__srv__SerializePoseGraph_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SerializePoseGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: rosidl_runtime_rs::String,

}



impl Default for SerializePoseGraph_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__SerializePoseGraph_Request__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__SerializePoseGraph_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SerializePoseGraph_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SerializePoseGraph_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SerializePoseGraph_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SerializePoseGraph_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SerializePoseGraph_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SerializePoseGraph_Request where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/SerializePoseGraph_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SerializePoseGraph_Request() }
  }
}


#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SerializePoseGraph_Response() -> *const std::ffi::c_void;
}

#[link(name = "slam_toolbox__rosidl_generator_c")]
extern "C" {
    fn slam_toolbox__srv__SerializePoseGraph_Response__init(msg: *mut SerializePoseGraph_Response) -> bool;
    fn slam_toolbox__srv__SerializePoseGraph_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<SerializePoseGraph_Response>, size: usize) -> bool;
    fn slam_toolbox__srv__SerializePoseGraph_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<SerializePoseGraph_Response>);
    fn slam_toolbox__srv__SerializePoseGraph_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<SerializePoseGraph_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<SerializePoseGraph_Response>) -> bool;
}

// Corresponds to slam_toolbox__srv__SerializePoseGraph_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SerializePoseGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub result: u8,

}

impl SerializePoseGraph_Response {

    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESULT_SUCCESS: u8 = 0;


    // This constant is not documented.
    #[allow(missing_docs)]
    pub const RESULT_FAILED_TO_WRITE_FILE: u8 = 255;

}


impl Default for SerializePoseGraph_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !slam_toolbox__srv__SerializePoseGraph_Response__init(&mut msg as *mut _) {
        panic!("Call to slam_toolbox__srv__SerializePoseGraph_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for SerializePoseGraph_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SerializePoseGraph_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SerializePoseGraph_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { slam_toolbox__srv__SerializePoseGraph_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for SerializePoseGraph_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for SerializePoseGraph_Response where Self: Sized {
  const TYPE_NAME: &'static str = "slam_toolbox/srv/SerializePoseGraph_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__slam_toolbox__srv__SerializePoseGraph_Response() }
  }
}






#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__Pause() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__Pause
#[allow(missing_docs, non_camel_case_types)]
pub struct Pause;

impl rosidl_runtime_rs::Service for Pause {
    type Request = Pause_Request;
    type Response = Pause_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__Pause() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__ClearQueue() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__ClearQueue
#[allow(missing_docs, non_camel_case_types)]
pub struct ClearQueue;

impl rosidl_runtime_rs::Service for ClearQueue {
    type Request = ClearQueue_Request;
    type Response = ClearQueue_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__ClearQueue() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__ToggleInteractive() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__ToggleInteractive
#[allow(missing_docs, non_camel_case_types)]
pub struct ToggleInteractive;

impl rosidl_runtime_rs::Service for ToggleInteractive {
    type Request = ToggleInteractive_Request;
    type Response = ToggleInteractive_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__ToggleInteractive() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__Clear() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__Clear
#[allow(missing_docs, non_camel_case_types)]
pub struct Clear;

impl rosidl_runtime_rs::Service for Clear {
    type Request = Clear_Request;
    type Response = Clear_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__Clear() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__SaveMap() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__SaveMap
#[allow(missing_docs, non_camel_case_types)]
pub struct SaveMap;

impl rosidl_runtime_rs::Service for SaveMap {
    type Request = SaveMap_Request;
    type Response = SaveMap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__SaveMap() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__LoopClosure() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__LoopClosure
#[allow(missing_docs, non_camel_case_types)]
pub struct LoopClosure;

impl rosidl_runtime_rs::Service for LoopClosure {
    type Request = LoopClosure_Request;
    type Response = LoopClosure_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__LoopClosure() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__MergeMaps() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__MergeMaps
#[allow(missing_docs, non_camel_case_types)]
pub struct MergeMaps;

impl rosidl_runtime_rs::Service for MergeMaps {
    type Request = MergeMaps_Request;
    type Response = MergeMaps_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__MergeMaps() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__AddSubmap() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__AddSubmap
#[allow(missing_docs, non_camel_case_types)]
pub struct AddSubmap;

impl rosidl_runtime_rs::Service for AddSubmap {
    type Request = AddSubmap_Request;
    type Response = AddSubmap_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__AddSubmap() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__DeserializePoseGraph() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__DeserializePoseGraph
#[allow(missing_docs, non_camel_case_types)]
pub struct DeserializePoseGraph;

impl rosidl_runtime_rs::Service for DeserializePoseGraph {
    type Request = DeserializePoseGraph_Request;
    type Response = DeserializePoseGraph_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__DeserializePoseGraph() }
    }
}




#[link(name = "slam_toolbox__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__SerializePoseGraph() -> *const std::ffi::c_void;
}

// Corresponds to slam_toolbox__srv__SerializePoseGraph
#[allow(missing_docs, non_camel_case_types)]
pub struct SerializePoseGraph;

impl rosidl_runtime_rs::Service for SerializePoseGraph {
    type Request = SerializePoseGraph_Request;
    type Response = SerializePoseGraph_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__slam_toolbox__srv__SerializePoseGraph() }
    }
}


