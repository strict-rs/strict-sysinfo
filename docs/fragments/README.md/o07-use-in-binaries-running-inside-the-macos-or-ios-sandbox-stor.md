### Use in binaries running inside the macOS or iOS Sandbox/stores

Apple has restrictions as to which APIs can be linked into binaries that are distributed through the app store.
By default, `sysinfo` is not compatible with these restrictions. You can use the `apple-app-store`
feature flag to disable the Apple prohibited features. This also enables the `apple-sandbox` feature.
In the case of applications using the sandbox outside of the app store, the `apple-sandbox` feature
can be used alone to avoid causing policy violations at runtime.
