use criterion::{BenchmarkId, Criterion, Throughput};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rl2tp::avp::*;
use rl2tp::common::VecWriter;

use super::corpus::AVP_CORPUS;

const RNG_SEED: u64 = 0x1337133713371337;
const TEST_SIZES: [usize; 1] = [64];

fn make_test(size: usize, rng: &mut impl Rng) -> Vec<AVP> {
    let mut test_data = Vec::new();

    let corpus = AVP_CORPUS.lock().unwrap();

    for _ in 0..size {
        let index = rng.gen_range(0..corpus.len());
        test_data.push(corpus[index].clone());
    }

    test_data
}

pub(crate) fn avp_encode(c: &mut Criterion) {
    let mut rng = SmallRng::seed_from_u64(RNG_SEED);
    let mut group = c.benchmark_group("avp_encode");

    for test_size in TEST_SIZES.iter() {
        let test_case = make_test(*test_size, &mut rng);

        let mut tmp_writer = VecWriter::new();
        for avp in test_case.iter() {
            unsafe { avp.write(&mut tmp_writer) };
        }
        group.throughput(Throughput::Bytes(tmp_writer.data.len() as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(test_size),
            &test_case,
            |b, data| {
                b.iter(|| {
                    let mut writer = VecWriter::new();
                    for avp in data.iter() {
                        unsafe { avp.write(&mut writer) };
                    }
                    writer.data;
                });
            },
        );
    }
    group.finish();
}
