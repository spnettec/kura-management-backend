fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let mut config = tonic_prost_build::Config::new();
    config.protoc_executable(protoc);
    // Connect is the protocol RPC name; omit Tonic's same-named endpoint constructor.
    tonic_prost_build::configure()
        .build_transport(false)
        .type_attribute("WiredConfig", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("StaticRoute", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("NetworkChange", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("NetworkAction", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("NetworkTakeover", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields, default)]")
        .type_attribute("KuraConfigurationTarget", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("KuraConfigurationUpdate", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("OrdinaryBootAction", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("OrdinaryHandoffAction", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("SystemControlAction", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("FirewallChange", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("FirewallAction", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("NmAction", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("NmChange", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("KuraSnapshotSave", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("KuraSnapshotRollback", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("KuraConfigurationProperty", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("KuraConfigurationProperty.value", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\")]")
        .type_attribute("KuraTextArray", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .type_attribute("KuraBooleanArray", "#[derive(serde::Serialize, serde::Deserialize)] #[serde(rename_all=\"camelCase\", deny_unknown_fields)]")
        .compile_with_config(config, &["proto/management/v1/control.proto"], &["proto"])?;
    println!("cargo:rerun-if-changed=proto/management/v1/control.proto");
    Ok(())
}
