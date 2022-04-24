mod corpus;
mod decode;
mod encode;

use decode::avp_decode;
use encode::avp_encode;

use criterion::criterion_group;

criterion_group!(benches, avp_encode, avp_decode);
