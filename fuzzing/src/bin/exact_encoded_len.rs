use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let computed_len = corncobs::exact_encoded_len(data);
            let mut out = vec![0; corncobs::max_encoded_len(data.len())];
            let encoded_len = corncobs::encode_buf(data, &mut out);
            assert_eq!(computed_len, encoded_len);
        });
    }
}
