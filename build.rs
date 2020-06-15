use std::process::{Command, Stdio};
use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(d) => d,
        None => panic!("Unable to read manifest dir"),
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // path to ipmimonitoring source code
    let src = PathBuf::from(&manifest_dir).join("freeipmi");
    // where we will put our built library for static linking
    let dst = PathBuf::from(&out_path).join("build");
    let _ = fs::create_dir(&dst);
    // path to our final libipmimonitoring file
    let dst_file = dst.join("libipmimonitoring.a");

    let file = Command::new("which").arg("file").output().expect("Failed to find 'file'").stdout;
    let file = std::str::from_utf8(&file).unwrap().trim();
    let sed = format!("s@/usr/bin/file@{}@", &file);
    println!("{}: {}", file, sed);

    // building libipmimonitoring as a static lib
    if !dst_file.exists() {
        run(Command::new("./autogen.sh").current_dir(&src));
        // configure has hard-coded the path for 'file'
        run(Command::new("sed")
            .arg("-i")
            .arg("-e")
            .arg(format!("s@/usr/bin/file@{}@", &file))
            .arg("./configure")
            .current_dir(&src));
        run(Command::new("./configure").arg("--enable-static").current_dir(&src));
        run(Command::new("make")
            .arg(format!("-j{}", num_cpus::get())).current_dir(&src));
        let _ = fs::copy(&src.join("libipmimonitoring/.libs/libipmimonitoring.a"), &dst_file);
    }

    // Link to ipmimonitoring static library
    println!("cargo:rustc-link-lib=static=ipmimonitoring");
    println!("cargo:rustc-link-search={}", dst.display());
}

fn run(cmd: &mut Command) {
    assert!(cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap()
        .success());
}

