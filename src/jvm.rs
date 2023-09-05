use {
    jni::{InitArgsBuilder, JNIVersion, JavaVM},
    lazy_static::lazy_static,
};

// Global singleton for the JVM object.
lazy_static! {
    pub static ref JVM: JavaVM = startup();
}

/// Java Virtual Machine initialisation function. The environment variable `SWT_JAR_PATH` is expected
/// to be set in a config.toml. See [here](https://doc.rust-lang.org/cargo/reference/config.html)
pub fn startup() -> JavaVM
{
    let jar_path = format!("-Djava.class.path={}", env!("SWT_JAR_PATH"));

    // Build the VM properties
    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option("-Xcheck:jni")
        .option(jar_path.as_str())
        .option("-Xms1m")
        .option("-Xss256k")
        .build()
        .unwrap();

    // Create a new VM
    let jvm = JavaVM::new(jvm_args).expect("Failed to initialise JVM.");

    jvm.attach_current_thread_permanently()
        .expect("Failed to attach JVM to current thread.");
    jvm
}
