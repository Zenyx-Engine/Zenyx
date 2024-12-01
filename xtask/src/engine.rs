use std::process::Stdio;


pub fn build_engine() {
    
}

pub fn build_core() {
    let threads = format!("-j{}",std::thread::available_parallelism().unwrap().get());
    let mut run = std::process::Command::new("cargo")
        .arg("run")

        .arg(threads)
        .arg("--bin")
        .arg("zenyx")
        .stdin(Stdio::inherit())  
        .stdout(Stdio::inherit()) 
        .stderr(Stdio::inherit()) 
        .spawn()
        .unwrap();
    run.wait().unwrap();
}