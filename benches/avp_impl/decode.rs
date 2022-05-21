use criterion::{BenchmarkId, Criterion, Throughput};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rl2tp::avp::*;
use rl2tp::common::{SliceReader, VecWriter};

use super::corpus::AVP_CORPUS;

const RNG_SEED: u64 = 0x1337133713371337;
const TEST_SIZES: [usize; 1] = [20480];

fn make_test(size: usize, rng: &mut impl Rng) -> Vec<u8> {
    let mut writer = VecWriter::new();

    let corpus = AVP_CORPUS.lock().unwrap();

    for _ in 0..size {
        let index = rng.gen_range(0..corpus.len());
        unsafe { corpus[index].write(&mut writer) };
    }

    writer.data
}

pub(crate) fn avp_decode(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(RNG_SEED);
    let mut group = c.benchmark_group("avp_decode");

    for test_size in TEST_SIZES.iter() {
        let test_case = make_test(*test_size, &mut rng);
        group.throughput(Throughput::Bytes(test_case.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(test_size),
            &test_case,
            |b, data| {
                b.iter(|| {
                    let mut reader = SliceReader::from(data);
                    AVP::try_read_greedy(&mut reader);
                });
            },
        );
    }
    group.finish();
}
