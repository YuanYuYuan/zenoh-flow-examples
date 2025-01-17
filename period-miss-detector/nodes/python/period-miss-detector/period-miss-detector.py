#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#

from zenoh_flow.interfaces import Operator
from zenoh_flow import Input, Output
from zenoh_flow.types import Context
from typing import Dict, Any
import datetime
import asyncio
import struct


class PeriodMissDetector(Operator):
    def __init__(
        self,
        context: Context,
        configuration: Dict[str, Any],
        inputs: Dict[str, Input],
        outputs: Dict[str, Output],
    ):
        print(f"Context: {context}")
        self.output = outputs.get("in", None)
        self.in_stream = inputs.get("out", None)

        if self.in_stream is None:
            raise ValueError("No input 'in' found")
        if self.output is None:
            raise ValueError("No output 'out' found")

        self.period = 5
        self.next_period = datetime.datetime.now() + datetime.timedelta(seconds=self.period)

    def finalize(self) -> None:
        return None

    async def iteration(self) -> None:

        now = datetime.datetime.now()
        sleep_duration = self.next_period.timestamp() - now.timestamp()
        if sleep_duration < 0:
            self.next_period = datetime.datetime.now() + datetime.timedelta(seconds=self.period)
            sleep_duration = self.period

        (done, pending) = await asyncio.wait(
            self.create_task_list(),
            return_when=asyncio.FIRST_COMPLETED,
        )

        for task in list(pending):
            task.cancel()

        return None

    async def default(self, sleep_duration):
        await asyncio.sleep(sleep_duration)
        self.output.send("(default) 0\n".encode("utf-8"))
        self.next_period = \
             self.next_period + datetime.timedelta(seconds=self.period)
        return "tick"

    async def wait_input(self):
        data_msg = await self.in_stream.recv()
        number = struct.unpack("!d", data_msg.data)
        self.output.send(f"Received: {number}\n".encode("utf-8"))

        now = datetime.datetime.now()
        interval = self.next_period.timestamp() - now.timestamp()
        if interval > 0 and interval < self.period:
            self.next_period = datetime.datetime.now() + datetime.timedelta(seconds=interval)
        else:
            self.next_period = datetime.datetime.now() + datetime.timedelta(seconds=self.period)

        return "in"

    def create_task_list(self, sleep_duration):
        task_list = []

        if not any(t.get_name() == "in" for t in task_list):
            task_list.append(
                asyncio.create_task(self.wait_input(), name="in")
            )

        if not any(t.get_name() == "tick" for t in task_list):
            task_list.append(
                asyncio.create_task(self.default(sleep_duration), name="tick")
            )
        return task_list


def register():
    return PeriodMissDetector
