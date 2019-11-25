pub mod rpc {
    use std::io::Write;
    use std::process::{Command, Stdio};

    pub fn call_rpc(engine: &str, method: &str, params: &str) {
        let mut rpc: String = "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"".into();
        rpc.push_str(method);
        rpc.push_str("\",\"params\":[");
        rpc.push_str(params);
        rpc.push_str("]}");

        let mut child = Command::new(engine)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(rpc.as_bytes())
            .expect("Failed to write to stdin");

        let output = child.wait_with_output().expect("Failed to read stdout");

        println!("{:?}", output);
    }
}
