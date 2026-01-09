use crate::spec::{Cc, LinkerFlavor, Lld, Os, RelocModel, StackProbeType, TargetOptions, cvs};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Kernel,
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        relocation_model: RelocModel::Static,
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}
