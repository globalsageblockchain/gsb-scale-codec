#[derive(::gsb_scale_codec::Decode, ::gsb_scale_codec::Encode)]
#[codec(crate = ::gsb_scale_codec)]
enum T {
	A = 1,
	B,
}

#[derive(::gsb_scale_codec::Decode, ::gsb_scale_codec::Encode)]
#[codec(crate = ::gsb_scale_codec)]
enum T2 {
	#[codec(index = 1)]
	A,
	B,
}

fn main() {}
