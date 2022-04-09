use criterion::{criterion_group, BenchmarkId, Criterion, Throughput};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rl2tp::avp::*;
use rl2tp::common::{SliceReader, VecWriter};

const RNG_SEED: u64 = 0x1337133713371337;

fn make_test(size: usize, rng: &mut impl Rng) -> Vec<u8> {
    let mut writer = VecWriter::new();

    let test_set = vec![
        AVP::AssignedTunnelId(types::AssignedTunnelId { value: 0x1337 }),
        AVP::BearerCapabilities(types::BearerCapabilities::new(true, true)),
        AVP::BearerType(types::BearerType::new(true, true)),
        AVP::CallErrors(types::CallErrors {
            crc_errors: 10,
            framing_errors: 11,
            hardware_overruns: 12,
            buffer_overruns: 13,
            timeout_errors: 14,
            alignment_errors: 15,
        }),
        AVP::CallSerialNumber(types::CallSerialNumber { value: 0x1337 }),
        AVP::CalledNumber(types::CalledNumber {
            value: "TestingNumber".to_owned(),
        }),
        AVP::CallingNumber(types::CallingNumber {
            value: "TestingNumber".to_owned(),
        }),
        AVP::ChallengeResponse(types::ChallengeResponse {
            data: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f,
            ],
        }),
        AVP::Challenge(types::Challenge {
            value: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        }),
    ];

    for _ in 0..size {
        let index = rng.gen_range(0..test_set.len());
        unsafe { test_set[index].write(&mut writer) };
    }

    writer.data
}

fn avp_decode(c: &mut Criterion) {
    const TEST_SIZES: [usize; 4] = [64, 128, 256, 512];

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
                    let reader = Box::new(SliceReader::from(data));
                    AVP::try_read_greedy(reader);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, avp_decode);
