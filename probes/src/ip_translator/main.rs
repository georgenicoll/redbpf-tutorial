#![no_std]
#![no_main]
use cty::*;

use probes::ip_translator::*;

use redbpf_probes::xdp::prelude::*;

// Use the types you're going to share with userspace, eg:
// use probes::ip_translator::SomeEvent;

program!(0xFFFFFFFE, "GPL");

#[xdp]
fn convert_10022_to_22(ctx: XdpContext) -> XdpResult {
    let transport = ctx.transport()?;
}
