use crate::spec::{Cc, LinkerFlavor, Lld, Os, RelocModel, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Kernel,
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        relocation_model: RelocModel::Static,
        families: ["unix".into()].into(),
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}
