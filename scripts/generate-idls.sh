#!/bin/bash

anchor build
cp target/idl/flux_core.json sdk/src/idl/
cp target/idl/flux_incinerator.json sdk/src/idl/

