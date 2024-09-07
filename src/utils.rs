pub fn test_input(size:usize) -> Vec<u8>{
    let mut output: Vec<u8> = Vec::new();
    for _ in 0..size {
        output.push(b"X"[0])
    }
    output
}