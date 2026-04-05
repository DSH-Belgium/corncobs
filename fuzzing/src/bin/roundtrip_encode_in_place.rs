use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let mut out = data.to_vec();
            out.resize(corncobs::max_encoded_len(data.len()) + 20, 0);
            let n = corncobs::encode_in_place(&mut out, data.len());
            let m = corncobs::decode_in_place(&mut out[..n]).unwrap();
            assert_eq!(data, &out[..m]);
        });
    }
}
