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

#[cfg(test)]
mod test {
    use crate::{
        architecture::{self, Architecture},
        build_profile::BuildProfile,
        dependency::{
            ArchConstraint, ArchConstraints, BuildProfileConstraint, BuildProfileConstraints,
            Dependency, Package, Relation, VersionConstraint, VersionOperator,
        },
        version::Version,
    };

    macro_rules! check_parse_fails {
        ($name:ident, $dep:expr) => {
            #[test]
            fn $name() {
                assert!($dep.parse::<Dependency>().is_err());
            }
        };
    }

    check_parse_fails!(double_arch_constraints, "foo [amd64] [arm64]");
    check_parse_fails!(double_version_constraint, "foo (= 1.0) (= 2.0)");
    check_parse_fails!(invalid_version_constraint, "foo (1.0)");
    check_parse_fails!(non_alpha_package, "💩");
    check_parse_fails!(spacy_package, "a space");
    check_parse_fails!(unknown_relation, "foo {bar}");
    check_parse_fails!(no_package_arch_constraints, "[amd64]");
    check_parse_fails!(no_package_arch, ":amd64");

    macro_rules! check_matches {
        ($name:ident, ( $( $dep:expr ),+ ), $check:expr) => {
            #[test]
            fn $name() {
                $(
                {
                    let dep: Dependency = $dep.parse().unwrap();
                    assert_eq!($check, dep, "expected {:?}, got {:?}", $check, dep);
                }
                )*
            }
        };

        ($name:ident, $dep:expr, $check:expr) => {
            check_matches!($name, ($dep), $check);
        };
    }

    macro_rules! check_round_trips {
        ($name:ident, ( $( $dep:expr ),+ ), $check:expr) => {
            #[test]
            fn $name() {
                $(
                {
                    let dep: Dependency = $dep.parse().unwrap();
                    assert_eq!($check, dep.to_string());
                }
                )*
            }
        };

        ($name:ident, $dep:expr, $check:expr) => {
            check_round_trips!($name, ($dep), $check);
        };
    }

    macro_rules! simple_package {
        ($package:expr) => {
            Dependency {
                relations: vec![Relation {
                    packages: vec![$package],
                }],
            }
        };
    }

    check_matches!(check_empty, "", Dependency { relations: vec![] });
    check_matches!(
        check_simple,
        "foo",
        simple_package!(Package {
            name: "foo".to_owned(),
            ..Default::default()
        })
    );
    check_matches!(
        check_simple_arch,
        "foo:armhf",
        simple_package!(Package {
            name: "foo".to_owned(),
            arch: Some(architecture::ARMHF),
            ..Default::default()
        })
    );
    check_matches!(
        check_simple_packages,
        "foo, bar | baz",
        Dependency {
            relations: vec![
                Relation {
                    packages: vec![Package {
                        name: "foo".to_owned(),
                        ..Default::default()
                    },]
                },
                Relation {
                    packages: vec![
                        Package {
                            name: "bar".to_owned(),
                            ..Default::default()
                        },
                        Package {
                            name: "baz".to_owned(),
                            ..Default::default()
                        },
                    ]
                }
            ],
        }
    );
    check_matches!(
        check_versioned_gte,
        "foo (>= 1.0)",
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::GreaterThanOrEqual,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_versioned_lte,
        "foo (<= 1.0)",
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::LessThanOrEqual,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_versioned_eq,
        "foo (= 1.0)",
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::Equal,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_versioned_eq2,
        "foo (== 1.0)",
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::Equal,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_versioned_gt,
        "foo (>> 1.0)",
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::GreaterThan,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_versioned_lt,
        "foo (<< 1.0)",
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::LessThan,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_arch_qualified,
        "foo [armhf]",
        simple_package!(Package {
            name: "foo".to_owned(),
            arch_constraints: Some(ArchConstraints {
                arches: vec![ArchConstraint {
                    negated: false,
                    arch: architecture::ARMHF,
                }]
            }),
            ..Default::default()
        })
    );
    check_matches!(
        check_arch_qualified2,
        "foo [armhf amd64]",
        simple_package!(Package {
            name: "foo".to_owned(),
            arch_constraints: Some(ArchConstraints {
                arches: vec![
                    ArchConstraint {
                        negated: false,
                        arch: architecture::ARMHF,
                    },
                    ArchConstraint {
                        negated: false,
                        arch: architecture::AMD64,
                    },
                ]
            }),
            ..Default::default()
        })
    );
    check_matches!(
        check_arch_qualified_not,
        "foo [!armhf !amd64]",
        simple_package!(Package {
            name: "foo".to_owned(),
            arch_constraints: Some(ArchConstraints {
                arches: vec![
                    ArchConstraint {
                        negated: true,
                        arch: architecture::ARMHF,
                    },
                    ArchConstraint {
                        negated: true,
                        arch: architecture::AMD64,
                    },
                ]
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_build_profile,
        "foo <buildprofile1>",
        simple_package!(Package {
            name: "foo".to_owned(),
            build_profile_restriction_formula: Some(
                vec![BuildProfileConstraints {
                    build_profiles: vec![BuildProfileConstraint {
                        negated: false,
                        build_profile: BuildProfile::Unknown("buildprofile1".to_owned()),
                    }]
                }]
                .into()
            ),
            ..Default::default()
        })
    );

    check_matches!(
        check_build_profile_multiple,
        "foo <buildprofile1 cross>",
        simple_package!(Package {
            name: "foo".to_owned(),
            build_profile_restriction_formula: Some(
                vec![BuildProfileConstraints {
                    build_profiles: vec![
                        BuildProfileConstraint {
                            negated: false,
                            build_profile: BuildProfile::Unknown("buildprofile1".to_owned()),
                        },
                        BuildProfileConstraint {
                            negated: false,
                            build_profile: BuildProfile::Cross,
                        },
                    ]
                }]
                .into()
            ),
            ..Default::default()
        })
    );
    check_matches!(
        build_profile_multiple_not,
        "foo <buildprofile1 !cross>",
        simple_package!(Package {
            name: "foo".to_owned(),
            build_profile_restriction_formula: Some(
                vec![BuildProfileConstraints {
                    build_profiles: vec![
                        BuildProfileConstraint {
                            negated: false,
                            build_profile: BuildProfile::Unknown("buildprofile1".to_owned()),
                        },
                        BuildProfileConstraint {
                            negated: true,
                            build_profile: BuildProfile::Cross,
                        },
                    ]
                }]
                .into()
            ),
            ..Default::default()
        })
    );

    check_matches!(
        check_spaces,
        ("foo", "  foo", "foo   "),
        simple_package!(Package {
            name: "foo".to_owned(),
            ..Default::default()
        })
    );

    check_matches!(
        check_spaces_version,
        (
            "foo(=1.0)",
            "foo (= 1.0)",
            "foo    (=1.0)",
            "   foo    (  =   1.0   )"
        ),
        simple_package!(Package {
            name: "foo".to_owned(),
            version_constraint: Some(VersionConstraint {
                operator: VersionOperator::Equal,
                version: Version::from_parts(None, "1.0", None).unwrap(),
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_spaces_build_profile,
        (
            "foo<foo>",
            "foo <foo>",
            "foo    <foo>",
            "foo    < foo>",
            "foo    <foo >",
            "foo    <   foo   >"
        ),
        simple_package!(Package {
            name: "foo".to_owned(),
            build_profile_restriction_formula: Some(
                vec![BuildProfileConstraints {
                    build_profiles: vec![BuildProfileConstraint {
                        negated: false,
                        build_profile: BuildProfile::Unknown("foo".to_owned()),
                    }],
                }]
                .into()
            ),
            ..Default::default()
        })
    );

    check_matches!(
        check_spaces_build_profile_not,
        (
            "foo<!foo>",
            "foo <!foo>",
            "foo    <!foo>",
            "foo    < !foo>",
            "foo    <!foo >",
            "foo    < !foo >",
            "   foo    <!foo>",
            "   foo    < ! foo >",
            "   foo    <   !     foo >"
        ),
        simple_package!(Package {
            name: "foo".to_owned(),
            build_profile_restriction_formula: Some(
                vec![BuildProfileConstraints {
                    build_profiles: vec![BuildProfileConstraint {
                        negated: true,
                        build_profile: BuildProfile::Unknown("foo".to_owned()),
                    }],
                }]
                .into()
            ),
            ..Default::default()
        })
    );

    check_matches!(
        check_spaces_build_profiles,
        (
            "foo<!foo bar>",
            "foo <!foo bar>",
            "foo    <!foo bar>",
            "foo    < !foo bar>",
            "foo    <!foo bar >",
            "foo    <!foo     bar>",
            "foo    < !foo bar >",
            "   foo    <!foo bar>",
            "   foo    < ! foo    bar >",
            "   foo    <   !     foo  bar >"
        ),
        simple_package!(Package {
            name: "foo".to_owned(),
            build_profile_restriction_formula: Some(
                vec![BuildProfileConstraints {
                    build_profiles: vec![
                        BuildProfileConstraint {
                            negated: true,
                            build_profile: BuildProfile::Unknown("foo".to_owned()),
                        },
                        BuildProfileConstraint {
                            negated: false,
                            build_profile: BuildProfile::Unknown("bar".to_owned())
                        }
                    ],
                }]
                .into()
            ),
            ..Default::default()
        })
    );

    check_round_trips!(
        rt_build_profile_pkg,
        "foo <pkg.foo-bar.baz>",
        "foo <pkg.foo-bar.baz>"
    );
    check_round_trips!(rt_build_profile_multi, "foo <foo> <bar>", "foo <foo> <bar>");

    check_round_trips!(rt_simple, ("foo", " foo", " foo "), "foo");
    check_round_trips!(
        rt_relations,
        (
            "foo, bar",
            "foo,  bar",
            " foo,  bar ",
            " foo , bar ",
            "foo, bar ",
            " foo,  bar"
        ),
        "foo, bar"
    );
    check_round_trips!(
        rt_poss,
        (
            "foo,bar|baz",
            "foo, bar | baz",
            "foo , bar | baz",
            " foo, bar | baz",
            "foo, bar | baz ",
            " foo, bar | baz"
        ),
        "foo, bar | baz"
    );
    check_round_trips!(
        rt_constraints,
        (
            "foo (= 1.0) [arch1 !arch2] <buildprofile1 !buildprofile2>",
            "foo (== 1.0) [arch1 !arch2] <buildprofile1 !buildprofile2>",
            "foo [arch1 !arch2] (= 1.0) <buildprofile1 !buildprofile2>",
            "foo [arch1 !arch2] <buildprofile1 !buildprofile2> (= 1.0)",
            "foo <buildprofile1 !buildprofile2> (= 1.0) [arch1 !arch2]",
            "foo <buildprofile1 !buildprofile2> [arch1 !arch2] (= 1.0)"
        ),
        "foo (= 1.0) [arch1 !arch2] <buildprofile1 !buildprofile2>"
    );

    check_matches!(
        check_arch_tuple_match,
        "foo [linux-any]",
        simple_package!(Package {
            name: "foo".to_owned(),
            arch_constraints: Some(ArchConstraints {
                arches: vec![ArchConstraint {
                    negated: false,
                    arch: Architecture::from_parts("any", "any", "linux", "any").unwrap(),
                },]
            }),
            ..Default::default()
        })
    );

    check_matches!(
        check_newlines,
        "\
foo,
bar | baz
",
        Dependency {
            relations: vec![
                Relation {
                    packages: vec![Package {
                        name: "foo".to_owned(),
                        ..Default::default()
                    },]
                },
                Relation {
                    packages: vec![
                        Package {
                            name: "bar".to_owned(),
                            ..Default::default()
                        },
                        Package {
                            name: "baz".to_owned(),
                            ..Default::default()
                        },
                    ]
                }
            ],
        }
    );
}

// vim: foldmethod=marker
