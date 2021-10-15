//
// Copyright (c) 2017, 2021 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//

use std::collections::HashMap;
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::runtime::message::DataMessage;
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::{
    default_input_rule, default_output_rule, export_operator, get_input_from, types::ZFResult,
    zf_data, Node, NodeOutput, Operator, State, Token,
};
use zenoh_flow::{downcast, Context, SerDeData};
use zenoh_flow_example_types::{ZFString, ZFUsize};

struct BuzzOperator;

#[derive(Debug, ZFState)]
struct BuzzState {
    buzzword: String,
}

static LINK_ID_INPUT_INT: &str = "Int";
static LINK_ID_INPUT_STR: &str = "Str";
static LINK_ID_OUTPUT_STR: &str = "Str";

impl Operator for BuzzOperator {
    fn input_rule(
        &self,
        _context: &mut Context,
        state: &mut Box<dyn State>,
        tokens: &mut HashMap<zenoh_flow::PortId, Token>,
    ) -> ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut Context,
        dyn_state: &mut Box<dyn State>,
        inputs: &mut HashMap<zenoh_flow::PortId, DataMessage>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, SerDeData>> {
        let mut results = HashMap::<zenoh_flow::PortId, SerDeData>::with_capacity(1);

        let state = downcast!(BuzzState, dyn_state).unwrap();
        let (_, fizz) = get_input_from!(ZFString, String::from(LINK_ID_INPUT_STR), inputs)?;
        let (_, value) = get_input_from!(ZFUsize, String::from(LINK_ID_INPUT_INT), inputs)?;

        let mut buzz = fizz;
        if value.0 % 3 == 0 {
            buzz.0.push_str(&state.buzzword);
        }

        results.insert(LINK_ID_OUTPUT_STR.into(), zf_data!(buzz));

        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut Context,
        state: &mut Box<dyn State>,
        outputs: HashMap<zenoh_flow::PortId, SerDeData>,
    ) -> ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for BuzzOperator {
    fn initialize(&self, configuration: &Option<HashMap<String, String>>) -> Box<dyn State> {
        let state = match configuration {
            Some(config) => match config.get("buzzword") {
                Some(buzzword) => BuzzState {
                    buzzword: buzzword.to_string(),
                },
                None => BuzzState {
                    buzzword: "Buzz".to_string(),
                },
            },
            None => BuzzState {
                buzzword: "Buzz".to_string(),
            },
        };
        Box::new(state)
    }

    fn clean(&self, _state: &mut Box<dyn State>) -> ZFResult<()> {
        Ok(())
    }
}

export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(BuzzOperator) as Arc<dyn Operator>)
}
