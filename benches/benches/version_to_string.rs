use criterion::{criterion_group, criterion_main, Criterion};
use deb::version::Version;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("version_to_string");

    group.bench_function("simple", |b| {
        let v: Version = "1.0".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("debian", |b| {
        let v: Version = "1.0-1".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("epoch", |b| {
        let v: Version = "1:1.0".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("full", |b| {
        let v: Version = "1:1.0-1".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("long1", |b| {
        // longest version in the archive; ty golang-go.crypto for this
        let v: Version = "1:0.0~git20170407.0.55a552f+REALLY.0.0~git20161012.0.5f31782-1+deb8u1"
            .parse()
            .unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("long2", |b| {
        // jsbundle-web-interfaces
        let v: Version = "1.1.0+~2.0.1~ds+~6.1.0+~0~20180821-1~bpo10+1"
            .parse()
            .unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("moderate1", |b| {
        // zipios++
        let v: Version = "0.1.5.9+cvs.2007.04.28-10+deb10u1".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("moderate2", |b| {
        // zipios++
        let v: Version = "1.0+git20230411.3b22df2-1~bpo11+1".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });

    group.bench_function("moderate3", |b| {
        // ovn
        let v: Version = "21.06.0+ds1-2~bpo11+1".parse().unwrap();
        b.iter(|| {
            v.to_string();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
