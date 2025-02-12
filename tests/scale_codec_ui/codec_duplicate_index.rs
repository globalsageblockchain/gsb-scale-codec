#[derive(::gsb_scale_codec::Decode, ::gsb_scale_codec::Encode)]
#[codec(crate = ::gsb_scale_codec)]
enum T {
	A = 3,
	#[codec(index = 3)]
	B,
}

#[derive(::gsb_scale_codec::Decode, ::gsb_scale_codec::Encode)]
#[codec(crate = ::gsb_scale_codec)]
enum T1 {
	A,
	#[codec(index = 0)]
	B,
}

fn main() {}
