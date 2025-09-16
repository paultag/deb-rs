// {{{ Copyright (c) Paul R. Tagliamonte <paultag@debian.org>, 2024
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE. }}}

use super::{Dependency, Package, Relation};
use crate::{architecture::Architecture, build_profile::BuildProfile};

impl Dependency {
    /// Consider every [Package] in the [Dependency], and determine if that
    /// specific [Package] is one that we should consider or not. This can be
    /// used to weed out package relationships based on, for example,
    /// Arch constraints, Build profile constraints or something else silly.
    pub fn filter<FilterFn>(&self, filter_fn: FilterFn) -> Self
    where
        FilterFn: Fn(&Package) -> bool,
    {
        let mut relations = vec![];
        for relation in self.relations.iter() {
            let packages = relation
                .packages
                .iter()
                .filter(|v| filter_fn(v))
                .cloned()
                .collect::<Vec<_>>();
            if packages.is_empty() {
                continue;
            }
            relations.push(Relation { packages });
        }
        Self { relations }
    }

    /// Remove any [Package] which is not considered for the target
    /// [Architecture] `arch`.
    pub fn filter_for_arch(&self, arch: &Architecture) -> Self {
        self.filter(|package| {
            let Some(ref arch_constraints) = package.arch_constraints else {
                // if this isn't arch constrained, we are valid.
                return true;
            };
            arch_constraints.matches(arch)
        })
    }

    /// Remove any [Package] which is not considered for the desired
    /// [BuildProfile] `profile`.
    pub fn filter_for_build_profiles(&self, profiles: &[BuildProfile]) -> Self {
        self.filter(|package| {
            let Some(ref bprf) = package.build_profile_restriction_formula else {
                // if this isn't arch constrained, we are valid.
                return true;
            };
            bprf.matches(profiles)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{architecture, build_profile::BuildProfile};

    macro_rules! def_filter_test {
        ($name:ident, $dep:expr, $result:expr, |$var:ident| $block:tt) => {
            #[test]
            fn $name() {
                let $var: Dependency = $dep.parse().unwrap();
                let result = $block.to_string();
                assert_eq!($result, result);
            }
        };
    }

    def_filter_test!(filter_nothing, "foo, bar", "foo, bar", |dep| {
        dep.filter(|_| true)
    });

    def_filter_test!(filter_everything, "foo, bar", "", |dep| {
        dep.filter(|_| false)
    });

    def_filter_test!(filter_bar, "foo, bar | baz", "foo, baz", |dep| {
        dep.filter(|package| package.name != "bar")
    });

    // Arch checks

    def_filter_test!(
        filter_amd64_nomatch,
        "foo, bar | baz",
        "foo, bar | baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_amd64_simple,
        "foo, bar [armel] | baz",
        "foo, baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_amd64_not_simple,
        "foo, bar [!amd64] | baz",
        "foo, baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_amd64_multi,
        "foo, bar [sparc amd64] | baz",
        "foo, bar [sparc amd64] | baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_amd64_multi_not,
        "foo, bar [!sparc !amd64] | baz",
        "foo, baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_amd64_multi_not_not,
        "foo, bar [!sparc !armel !ppc64] | baz",
        "foo, bar [!sparc !armel !ppc64] | baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_amd64_multi_boring,
        "foo, bar [amd64 arm64 ppc64] | baz",
        "foo, bar [amd64 arm64 ppc64] | baz",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    def_filter_test!(
        filter_bogus_relation,
        "foo [amd64 !amd64]",
        "foo [amd64 !amd64]",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    // this isn't really evaulate-able due to the semantics of how the
    // ! operator behaves. See #816473 for the last time I ran into
    // validating this. This bug is still open at the time of writing.
    def_filter_test!(
        filter_bogus_relation_2,
        "foo [amd64 !sparc]",
        "foo [amd64 !sparc]",
        |dep| { dep.filter_for_arch(&architecture::AMD64) }
    );

    // build profile

    def_filter_test!(
        filter_nodoc_nothing,
        "foo, bar | baz",
        "foo, bar | baz",
        |dep| { dep.filter_for_build_profiles(&[BuildProfile::NoDoc]) }
    );

    def_filter_test!(
        filter_nodoc_nodoc,
        "foo, bar | baz <!nodoc>",
        "foo, bar",
        |dep| { dep.filter_for_build_profiles(&[BuildProfile::NoDoc]) }
    );

    def_filter_test!(
        filter_nodoc_nodoc2,
        "foo, bar | baz <!nodoc> <!cross>",
        "foo, bar",
        |dep| { dep.filter_for_build_profiles(&[BuildProfile::NoDoc]) }
    );

    def_filter_test!(
        filter_nodoc_positive,
        "foo, bar <nodoc> | baz",
        "foo, bar <nodoc> | baz",
        |dep| { dep.filter_for_build_profiles(&[BuildProfile::NoDoc]) }
    );
}

// vim: foldmethod=marker
