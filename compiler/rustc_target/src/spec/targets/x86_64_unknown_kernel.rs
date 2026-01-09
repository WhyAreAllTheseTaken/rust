use crate::spec::{Arch, PanicStrategy, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::kernel::opts();
    base.cpu = "x86-64".into();
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "-mmx,-sse,+soft-float".into();

    Target {
        llvm_target: "x86_64-unknown-none".into(),
        arch: Arch::X86_64,
        pointer_width: 64,
        metadata: TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        options: base,
    }
}
