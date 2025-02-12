use gsb_scale_codec::{Encode, MaxEncodedLen};

#[derive(Encode, MaxEncodedLen)]
#[codec(crate = "gsb_scale_codec")]
struct Example;

fn main() {
	let _ = Example::max_encoded_len();
}
