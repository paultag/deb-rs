use criterion::{criterion_group, criterion_main, Criterion};
use deb::dependency::Dependency;

macro_rules! benchmark_dependency {
    ($grp:ident, $name:ident, $dep:expr) => {
        $grp.bench_function(stringify!($name), |b| {
            b.iter(|| {
                let _: Dependency = $dep.parse().unwrap();
            })
        });
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    {
        let mut group = c.benchmark_group("dependency_parse");

        benchmark_dependency!(group, single, "foo");
        benchmark_dependency!(group, multiple_relations, "foo, bar");
        benchmark_dependency!(group, multiple_possibilities, "foo | bar");
        benchmark_dependency!(group, single_constraint_version, "foo (== 1.0)");
        benchmark_dependency!(group, single_constraint_arch, "foo [amd64]");
        benchmark_dependency!(group, single_constraint_arch_unknown, "foo [poopie]");
        benchmark_dependency!(group, single_constraint_stage, "foo <foo>");
        benchmark_dependency!(
            group,
            double_constraint_version_arch,
            "foo (>= 1.0) [armhf]"
        );
        benchmark_dependency!(
            group,
            double_constraint_version_stage,
            "foo (>= 1.0) <stage1>"
        );
        benchmark_dependency!(group, double_constraint_arch_stage, "foo [sparc] <stage1>");
    }

    {
        let mut group = c.benchmark_group("dependency_real");

        // from libn32gcc-11-dev-mips64-cross
        benchmark_dependency!(group, libn32gcc_11_dev_mips64_cross, "gcc-11-cross-base-mipsen (= 11.4.0-2cross2), lib32gcc-s1-mips64-cross (>= 11.4.0-2cross2), libn32gcc-s1-mips64-cross (>= 11.4.0-2cross2), lib32gomp1-mips64-cross (>= 11.4.0-2cross2), libn32gomp1-mips64-cross (>= 11.4.0-2cross2), lib32atomic1-mips64-cross (>= 11.4.0-2cross2), libn32atomic1-mips64-cross (>= 11.4.0-2cross2)");

        // from libactivemq-java,activemq
        benchmark_dependency!(group, libactivemq_java_activemq, "glassfish-javaee, junit4, libactivemq-activeio-java, libaopalliance-java, libaxis-java, libcommons-collections3-java, libcommons-daemon-java, libcommons-lang-java (>= 2.6), libcommons-pool-java, libgentlyweb-utils-java (>= 1.5), libhttpclient-java, libjasypt-java, libjetty8-java, libjosql-java (>= 1.5), liblog4j1.2-java (>= 1.2.17), libmaven2-core-java, libslf4j-java, libspring-beans-java, libspring-context-java, libspring-core-java, libspring-jms-java, libspring-test-java, libxbean-java, libxpp3-java, libxstream-java, velocity");

        // from thunderbird-l10n-all
        benchmark_dependency!(group, thunderbird_l10n_all, "thunderbird-l10n-ar (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ast (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-be (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-bg (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-br (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ca (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-cs (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-da (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-de (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-dsb (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-el (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-en-gb (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-es-ar (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-es-es (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-et (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-eu (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-fi (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-fr (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-fy-nl (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ga-ie (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-gd (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-gl (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-he (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-hr (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-hsb (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-hu (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-hy-am (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-id (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-is (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-it (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ja (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-kab (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-kk (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ko (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-lt (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ms (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-nb-no (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-nl (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-nn-no (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-pl (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-pt-br (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-pt-pt (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-rm (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ro (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-ru (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-si (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-sk (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-sl (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-sq (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-sr (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-sv-se (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-tr (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-uk (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-vi (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-zh-cn (>= 1:60.9.0-1~deb8u1), thunderbird-l10n-zh-tw (>= 1:60.9.0-1~deb8u1)");
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
