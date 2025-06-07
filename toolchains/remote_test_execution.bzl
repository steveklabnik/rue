load("@prelude//tests:remote_test_execution_toolchain.bzl", "RemoteTestExecutionToolchainInfo")

def _remote_test_execution_impl(ctx):
    return [
        DefaultInfo(),
        RemoteTestExecutionToolchainInfo(
            default_profile = None,
            profiles = {},
            default_run_as_bundle = False,
        )
    ]

noop_remote_test_execution_toolchain = rule(
    impl = _remote_test_execution_impl,
    attrs = {},
    is_toolchain_rule = True,
)