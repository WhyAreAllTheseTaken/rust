use crate::spec::{Cc, Env, LinkSelfContainedDefault, LinkerFlavor, Lld, Os, RelocModel, StackProbeType, TargetOptions, cvs};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Kernel,
        env: Env::Unspecified,
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        relocation_model: RelocModel::Static,
        stack_probes: StackProbeType::Inline,
        link_self_contained: LinkSelfContainedDefault::True,
        ..Default::default()
    }
}
