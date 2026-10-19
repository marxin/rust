use crate::spec::{CfgAbi, LinkerFlavor, LinkerSelection, TargetOptions, base};

pub(crate) fn opts() -> TargetOptions {
    let mut opts =
        TargetOptions { cfg_abi: CfgAbi::Uwp, vendor: "uwp".into(), ..base::windows_msvc::opts() };

    opts.add_pre_link_args(LinkerFlavor::Msvc(LinkerSelection::System), &["/APPCONTAINER", "mincore.lib"]);

    opts
}
