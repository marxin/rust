use crate::spec::{
    Arch, Cc, CfgAbi, LinkerFlavor, LinkerSelection, LlvmAbi, StackProbeType, Target, TargetMetadata,
    TargetOptions, add_link_args, base,
};

pub(crate) fn target() -> Target {
    let mut base = base::freebsd::opts();
    base.cpu = "ppc64le".into();
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, LinkerSelection::System), &["-m64"]);
    // long double is IEEE-128; f128 soft-float ops emit the PPC __*kf* helpers,
    // which compiler_builtins lacks. Resolve them from base libgcc.
    add_link_args(&mut base.late_link_args, LinkerFlavor::Gnu(Cc::Yes, LinkerSelection::System), &["-lgcc"]);
    base.max_atomic_width = Some(64);
    base.stack_probes = StackProbeType::Inline;
    base.cfg_abi = CfgAbi::ElfV2;
    base.llvm_abiname = LlvmAbi::ElfV2;

    Target {
        llvm_target: "powerpc64le-unknown-freebsd".into(),
        metadata: TargetMetadata {
            description: Some("PPC64LE FreeBSD".into()),
            tier: Some(3),
            host_tools: Some(true),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout: "e-m:e-Fn32-i64:64-i128:128-n32:64".into(),
        arch: Arch::PowerPC64,
        options: TargetOptions { mcount: "_mcount".into(), ..base },
    }
}
