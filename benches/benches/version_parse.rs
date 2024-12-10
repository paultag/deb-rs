use criterion::{criterion_group, criterion_main, Criterion};
use deb::version::Version;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("version_parse");

    group.bench_function("simple", |b| {
        b.iter(|| {
            let _: Version = "1.0".parse().unwrap();
        })
    });

    group.bench_function("debian", |b| {
        b.iter(|| {
            let _: Version = "1.0-1".parse().unwrap();
        })
    });

    group.bench_function("epoch", |b| {
        b.iter(|| {
            let _: Version = "1:1.0".parse().unwrap();
        })
    });

    group.bench_function("full", |b| {
        b.iter(|| {
            let _: Version = "1:1.0-1".parse().unwrap();
        })
    });

    group.bench_function("long1", |b| {
        b.iter(|| {
            // longest version in the archive; ty golang-go.crypto for this
            let _: Version =
                "1:0.0~git20170407.0.55a552f+REALLY.0.0~git20161012.0.5f31782-1+deb8u1"
                    .parse()
                    .unwrap();
        })
    });

    group.bench_function("long2", |b| {
        b.iter(|| {
            // jsbundle-web-interfaces
            let _: Version = "1.1.0+~2.0.1~ds+~6.1.0+~0~20180821-1~bpo10+1"
                .parse()
                .unwrap();
        })
    });

    group.bench_function("moderate1", |b| {
        b.iter(|| {
            // zipios++
            let _: Version = "0.1.5.9+cvs.2007.04.28-10+deb10u1".parse().unwrap();
        })
    });

    group.bench_function("moderate2", |b| {
        b.iter(|| {
            // zipios++
            let _: Version = "1.0+git20230411.3b22df2-1~bpo11+1".parse().unwrap();
        })
    });

    group.bench_function("moderate3", |b| {
        b.iter(|| {
            // ovn
            let _: Version = "21.06.0+ds1-2~bpo11+1".parse().unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
