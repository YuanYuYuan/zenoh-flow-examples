//
// Copyright (c) 2022 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use std::collections::HashMap;
use zenoh_flow::async_std::sync::Arc;
use zenoh_flow::zenoh_flow_derive::ZFState;
use zenoh_flow::Configuration;
use zenoh_flow::LocalDeadlineMiss;
use zenoh_flow::PortId;
use zenoh_flow::{
    default_input_rule, default_output_rule, Data, Node, NodeOutput, Operator, State, ZFError,
    ZFResult,
};
use zenoh_flow_example_types::ZFUsize;

#[derive(Debug)]
struct SumAndSend;

#[derive(Debug, Clone, ZFState)]
struct SumAndSendState {
    pub x: ZFUsize,
}

static INPUT: &str = "Number";
static OUTPUT: &str = "Sum";

impl Operator for SumAndSend {
    fn input_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        tokens: &mut HashMap<PortId, zenoh_flow::InputToken>,
    ) -> zenoh_flow::ZFResult<bool> {
        default_input_rule(state, tokens)
    }

    fn run(
        &self,
        _context: &mut zenoh_flow::Context,
        dyn_state: &mut State,
        inputs: &mut HashMap<PortId, zenoh_flow::runtime::message::DataMessage>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, Data>> {
        let mut results: HashMap<PortId, Data> = HashMap::new();

        // Downcasting state to right type
        let mut state = dyn_state.try_get::<SumAndSendState>()?;

        let mut input_value = inputs
            .remove(INPUT)
            .ok_or_else(|| ZFError::InvalidData("No data".to_string()))?;
        let data = input_value.get_inner_data().try_get::<ZFUsize>()?;

        let res = ZFUsize(state.x.0 + data.0);
        state.x = res.clone();

        results.insert(OUTPUT.into(), Data::from::<ZFUsize>(res));
        Ok(results)
    }

    fn output_rule(
        &self,
        _context: &mut zenoh_flow::Context,
        state: &mut State,
        outputs: HashMap<PortId, Data>,
        _deadlinemiss: Option<LocalDeadlineMiss>,
    ) -> zenoh_flow::ZFResult<HashMap<zenoh_flow::PortId, NodeOutput>> {
        default_output_rule(state, outputs)
    }
}

impl Node for SumAndSend {
    fn initialize(&self, _configuration: &Option<Configuration>) -> ZFResult<State> {
        Ok(State::from(SumAndSendState { x: ZFUsize(0) }))
    }

    fn finalize(&self, _state: &mut State) -> ZFResult<()> {
        Ok(())
    }
}

// Also generated by macro
zenoh_flow::export_operator!(register);

fn register() -> ZFResult<Arc<dyn Operator>> {
    Ok(Arc::new(SumAndSend) as Arc<dyn Operator>)
}
