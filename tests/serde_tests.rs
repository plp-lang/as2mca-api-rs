use as2mca_api::{
  requests::{CaseInsensitiveFilter, Filter, MethodValidate, UserFilter, ViewDataGetCancelable},
  responses::{ControlState, ControlsStates, MethodResult, Validate},
};

#[test]
fn test_se_de_method_result() {
  {
    let res = MethodResult {
      value: Some("4".to_owned()),
      controls_states: Some(ControlsStates {
        items: vec![ControlState {
          id: 4,
          value: "4".to_string(),
        }],
      }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(
      xml,
      "<Result Value=\"4\"><ControlsStates><ControlState ID=\"4\" Value=\"4\"/></ControlsStates></Result>"
    );
    let new_res: MethodResult = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = MethodResult {
      value: None,
      controls_states: Some(ControlsStates {
        items: vec![ControlState {
          id: 4,
          value: "4".to_string(),
        }],
      }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(
      xml,
      "<Result><ControlsStates><ControlState ID=\"4\" Value=\"4\"/></ControlsStates></Result>"
    );
    let new_res: MethodResult = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = MethodResult {
      value: None,
      controls_states: Some(ControlsStates { items: vec![] }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(xml, "<Result><ControlsStates/></Result>");
    let new_res: MethodResult = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = MethodResult {
      value: None,
      controls_states: None,
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(xml, "<Result/>");
    let new_res: MethodResult = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
}

#[test]
fn test_se_de_validate() {
  {
    let res = Validate {
      debug_text: Some("4".to_owned()),
      object_id: Some(4),
      controls_states: Some(ControlsStates {
        items: vec![ControlState {
          id: 4,
          value: "4".to_string(),
        }],
      }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(
      xml,
      "<Validate DebugText=\"4\" ObjectID=\"4\"><ControlsStates><ControlState ID=\"4\" Value=\"4\"/></ControlsStates></Validate>"
    );
    let new_res: Validate = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = Validate {
      debug_text: Some("4".to_owned()),
      object_id: Some(4),
      controls_states: Some(ControlsStates { items: vec![] }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(
      xml,
      "<Validate DebugText=\"4\" ObjectID=\"4\"><ControlsStates/></Validate>"
    );
    let new_res: Validate = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = Validate {
      debug_text: None,
      object_id: Some(4),
      controls_states: Some(ControlsStates { items: vec![] }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(xml, "<Validate ObjectID=\"4\"><ControlsStates/></Validate>");
    let new_res: Validate = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = Validate {
      debug_text: None,
      object_id: None,
      controls_states: Some(ControlsStates { items: vec![] }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(xml, "<Validate><ControlsStates/></Validate>");
    let new_res: Validate = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
  {
    let res = Validate {
      debug_text: None,
      object_id: None,
      controls_states: None,
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(xml, "<Validate/>");
    let new_res: Validate = quick_xml::de::from_str(&xml).unwrap();
    assert_eq!(res, new_res);
  }
}

#[test]
fn test_se_de_view_data_get_cancelable() {
  {
    let res = ViewDataGetCancelable {
      session_id: "4",
      view_short_name: "4",
      class_id: "4",
      hint: "4",
      allow_timestamp_milliseconds: false,
      rows_limit: None,
      additional_filter_bind: None,
      object_filter: None,
      user_filter: Some(UserFilter {
        extra_filter: None,
        filters: vec![Filter::And(vec![Filter::CaseInsensitive(CaseInsensitiveFilter {
          column_name: "4",
          operator: "=",
          value: Some("4"),
        })])],
      }),
    };
    let xml = quick_xml::se::to_string(&res).unwrap();
    assert_eq!(
      xml,
      "<ViewDataGetCancelable SessionID=\"4\" ViewShortName=\"4\" ClassID=\"4\" Hint=\"4\" AllowTimestampMilliseconds=\"false\"><UserFilter><AND><CaseInsensitiveFilter ColumnName=\"4\" Operator=\"=\" Value=\"4\"/></AND></UserFilter></ViewDataGetCancelable>"
    );
  }
}

#[test]
fn test_se_de_method_validate() {
  let res = MethodValidate {
    session_id: "test",
    method_id: 2_255_110_342,
    r#type: as2mca_api::requests::ValidateType::Validate,
    info: "%PLPCALL%",
    do_commit: true,
    get_debug_text: true,
    optimized_grid_updates: true,
    controls_states: vec![
      as2mca_api::requests::ControlState {
        id: 1_761_776_655,
        value: "",
      },
      as2mca_api::requests::ControlState {
        id: 1_761_776_748,
        value: "",
      },
    ],
    ..Default::default()
  };
  let xml = quick_xml::se::to_string(&res).unwrap();
  assert_eq!(
    xml,
    "<MethodValidate SessionID=\"test\" MethodID=\"2255110342\" Type=\"VALIDATE\" Info=\"%PLPCALL%\" DoCommit=\"true\" GetDebugText=\"true\" OptimizedGridUpdates=\"true\"><ControlsStates><ControlState ID=\"1761776655\" Value=\"\"/><ControlState ID=\"1761776748\" Value=\"\"/></ControlsStates><PLPCallParameters/></MethodValidate>"
  );
}
