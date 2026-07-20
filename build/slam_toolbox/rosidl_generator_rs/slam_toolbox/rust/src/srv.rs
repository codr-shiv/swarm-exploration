#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to slam_toolbox__srv__Pause_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Pause_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Pause_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Pause_Request {
  type RmwMsg = super::srv::rmw::Pause_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__Pause_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Pause_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for Pause_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Pause_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Pause_Response {
  type RmwMsg = super::srv::rmw::Pause_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to slam_toolbox__srv__ClearQueue_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearQueue_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ClearQueue_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearQueue_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ClearQueue_Request {
  type RmwMsg = super::srv::rmw::ClearQueue_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__ClearQueue_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClearQueue_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for ClearQueue_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ClearQueue_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ClearQueue_Response {
  type RmwMsg = super::srv::rmw::ClearQueue_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
    }
  }
}


// Corresponds to slam_toolbox__srv__ToggleInteractive_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToggleInteractive_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ToggleInteractive_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToggleInteractive_Request::default())
  }
}

impl rosidl_runtime_rs::Message for ToggleInteractive_Request {
  type RmwMsg = super::srv::rmw::ToggleInteractive_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__ToggleInteractive_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ToggleInteractive_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for ToggleInteractive_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::ToggleInteractive_Response::default())
  }
}

impl rosidl_runtime_rs::Message for ToggleInteractive_Response {
  type RmwMsg = super::srv::rmw::ToggleInteractive_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__Clear_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Clear_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Clear_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Clear_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Clear_Request {
  type RmwMsg = super::srv::rmw::Clear_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__Clear_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Clear_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for Clear_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Clear_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Clear_Response {
  type RmwMsg = super::srv::rmw::Clear_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__SaveMap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SaveMap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std_msgs::msg::String,

}



impl Default for SaveMap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveMap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Request {
  type RmwMsg = super::srv::rmw::SaveMap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: std_msgs::msg::String::into_rmw_message(std::borrow::Cow::Owned(msg.name)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: std_msgs::msg::String::into_rmw_message(std::borrow::Cow::Borrowed(&msg.name)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: std_msgs::msg::String::from_rmw_message(msg.name),
    }
  }
}


// Corresponds to slam_toolbox__srv__SaveMap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SaveMap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SaveMap_Response {
  type RmwMsg = super::srv::rmw::SaveMap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
  }
}


// Corresponds to slam_toolbox__srv__LoopClosure_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoopClosure_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for LoopClosure_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoopClosure_Request::default())
  }
}

impl rosidl_runtime_rs::Message for LoopClosure_Request {
  type RmwMsg = super::srv::rmw::LoopClosure_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__LoopClosure_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LoopClosure_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for LoopClosure_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::LoopClosure_Response::default())
  }
}

impl rosidl_runtime_rs::Message for LoopClosure_Response {
  type RmwMsg = super::srv::rmw::LoopClosure_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__MergeMaps_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MergeMaps_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MergeMaps_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MergeMaps_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MergeMaps_Request {
  type RmwMsg = super::srv::rmw::MergeMaps_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__MergeMaps_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MergeMaps_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MergeMaps_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MergeMaps_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MergeMaps_Response {
  type RmwMsg = super::srv::rmw::MergeMaps_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__AddSubmap_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddSubmap_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,

}



impl Default for AddSubmap_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddSubmap_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddSubmap_Request {
  type RmwMsg = super::srv::rmw::AddSubmap_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
    }
  }
}


// Corresponds to slam_toolbox__srv__AddSubmap_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddSubmap_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for AddSubmap_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddSubmap_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddSubmap_Response {
  type RmwMsg = super::srv::rmw::AddSubmap_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__DeserializePoseGraph_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeserializePoseGraph_Request {
    /// inital_pose should be Map -> base_frame (parameter, generally base_link)
    pub filename: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub match_type: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub initial_pose: geometry_msgs::msg::Pose2D,

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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeserializePoseGraph_Request::default())
  }
}

impl rosidl_runtime_rs::Message for DeserializePoseGraph_Request {
  type RmwMsg = super::srv::rmw::DeserializePoseGraph_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
        match_type: msg.match_type,
        initial_pose: geometry_msgs::msg::Pose2D::into_rmw_message(std::borrow::Cow::Owned(msg.initial_pose)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      match_type: msg.match_type,
        initial_pose: geometry_msgs::msg::Pose2D::into_rmw_message(std::borrow::Cow::Borrowed(&msg.initial_pose)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
      match_type: msg.match_type,
      initial_pose: geometry_msgs::msg::Pose2D::from_rmw_message(msg.initial_pose),
    }
  }
}


// Corresponds to slam_toolbox__srv__DeserializePoseGraph_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DeserializePoseGraph_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for DeserializePoseGraph_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::DeserializePoseGraph_Response::default())
  }
}

impl rosidl_runtime_rs::Message for DeserializePoseGraph_Response {
  type RmwMsg = super::srv::rmw::DeserializePoseGraph_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to slam_toolbox__srv__SerializePoseGraph_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SerializePoseGraph_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub filename: std::string::String,

}



impl Default for SerializePoseGraph_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SerializePoseGraph_Request::default())
  }
}

impl rosidl_runtime_rs::Message for SerializePoseGraph_Request {
  type RmwMsg = super::srv::rmw::SerializePoseGraph_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        filename: msg.filename.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      filename: msg.filename.to_string(),
    }
  }
}


// Corresponds to slam_toolbox__srv__SerializePoseGraph_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::SerializePoseGraph_Response::default())
  }
}

impl rosidl_runtime_rs::Message for SerializePoseGraph_Response {
  type RmwMsg = super::srv::rmw::SerializePoseGraph_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        result: msg.result,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      result: msg.result,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      result: msg.result,
    }
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


