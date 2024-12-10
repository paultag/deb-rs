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
mod tests {
    use crate::control::RawParagraph;

    macro_rules! check_paragraph_parse {
        ($( ($name:ident, $paragraph:expr) ),* ) => {
$(
            #[test]
            fn $name() {
                RawParagraph::parse($paragraph).unwrap();
            }
)*
        };
    }

    check_paragraph_parse!(
        // apt output and files here
        (
            apt_show_hdparm,
            "\
Package: hdparm
Version: 9.65+ds-1.1
Priority: optional
Section: admin
Maintainer: Alexandre Mestiashvili <mestia@debian.org>
Installed-Size: 252 kB
Depends: libc6 (>= 2.34)
Recommends: powermgmt-base
Homepage: http://sourceforge.net/projects/hdparm/
Tag: admin::benchmarking, admin::hardware, hardware::storage,
 implemented-in::c, interface::commandline, role::program
Download-Size: 107 kB
APT-Manual-Installed: no
APT-Sources: http://archive.adref/debian unstable/main amd64 Packages
Description: tune hard disk parameters for high performance
 Get/set device parameters for Linux SATA/IDE drives.
 Provides a command line interface to various kernel interfaces supported by
 the Linux SATA/PATA/SAS \"libata\" subsystem and the older IDE driver subsystem.
 Many newer (2008 and later) USB drive enclosures now also support \"SAT\"
 (SCSI-ATA Command Translation) and therefore may also work with hdparm.

"
        ),
        (
            apt_sources_ubuntu,
            "\
Enabled: yes
Types: deb deb-src
URIs: http://archive.ubuntu.com/ubuntu
Suites: disco disco-updates disco-security disco-backports
Components: main universe multiverse restricted
"
        ),

        (
            bash_dsc,
            "\
Format: 3.0 (quilt)
Source: bash
Binary: bash, bash-static, bash-builtins, bash-doc
Architecture: any all
Version: 5.2.32-1
Maintainer: Matthias Klose <doko@debian.org>
Homepage: http://tiswww.case.edu/php/chet/bash/bashtop.html
Standards-Version: 4.6.2
Vcs-Browser: https://code.launchpad.net/~doko/+junk/pkg-bash-debian
Vcs-Bzr: http://bazaar.launchpad.net/~doko/+junk/pkg-bash-debian
Build-Depends: autoconf, autotools-dev, bison, libncurses5-dev, texinfo, texi2html, debhelper (>= 11), gettext, sharutils, locales <!nocheck>, time <!nocheck>, xz-utils
Build-Depends-Indep: texlive-latex-base, ghostscript, texlive-fonts-recommended, man2html-base
Build-Conflicts: r-base-core
Package-List:
 bash deb shells required arch=any essential=yes
 bash-builtins deb utils optional arch=any
 bash-doc deb doc optional arch=all
 bash-static deb shells optional arch=any
Checksums-Sha1:
 86554e8ee7cccef1ba074521eea5cef6d4735dd4 5598292 bash_5.2.32.orig.tar.xz
 040c446af56e04863f68577a762a4bc4d6baea8e 87896 bash_5.2.32-1.debian.tar.xz
Checksums-Sha256:
 b683d2674e316b7e49091f2f80901c5ea7455b6eab2431c73936fce0b4846cd2 5598292 bash_5.2.32.orig.tar.xz
 1105321d23bc5b93ee9b57007c65ff789443cad5504e509f49e16db815b4fc62 87896 bash_5.2.32-1.debian.tar.xz
Files:
 671662ba5cf841c1dd51c7462fc09b92 5598292 bash_5.2.32.orig.tar.xz
 2fc436a82776f6f372d828fc8591b702 87896 bash_5.2.32-1.debian.tar.xz"
        ),

        // Hello package files
        (
            hello_dsc,
            "\
Format: 3.0 (quilt)
Source: hello
Binary: hello
Architecture: any
Version: 2.10-3
Maintainer: Santiago Vila <sanvila@debian.org>
Homepage: https://www.gnu.org/software/hello/
Standards-Version: 4.6.2
Vcs-Browser: https://salsa.debian.org/sanvila/hello
Vcs-Git: https://salsa.debian.org/sanvila/hello.git
Testsuite: autopkgtest
Build-Depends: debhelper-compat (= 13), help2man, texinfo
Package-List:
 hello deb devel optional arch=any
Checksums-Sha1:
 f7bebf6f9c62a2295e889f66e05ce9bfaed9ace3 725946 hello_2.10.orig.tar.gz
 9dc7a584db576910856ac7aa5cffbaeefe9cf427 819 hello_2.10.orig.tar.gz.asc
 82e477ec77f09bae910e53592d28319774754af6 12688 hello_2.10-3.debian.tar.xz
Checksums-Sha256:
 31e066137a962676e89f69d1b65382de95a7ef7d914b8cb956f41ea72e0f516b 725946 hello_2.10.orig.tar.gz
 4ea69de913428a4034d30dcdcb34ab84f5c4a76acf9040f3091f0d3fac411b60 819 hello_2.10.orig.tar.gz.asc
 f43ddcca8d7168c5d52b53e1f2a69b78f42f8387633ef8955edd0621c73cf65c 12688 hello_2.10-3.debian.tar.xz
Files:
 6cd0ffea3884a4e79330338dcc2987d6 725946 hello_2.10.orig.tar.gz
 e6074bb23a0f184e00fdfb5c546b3bc2 819 hello_2.10.orig.tar.gz.asc
 16678389ba7fddcdfa05e0707d61f043 12688 hello_2.10-3.debian.tar.xz
"
        ),
        (
            hello_changes,
            "\
Format: 1.8
Date: Mon, 26 Dec 2022 16:30:00 +0100
Source: hello
Binary: hello hello-dbgsym
Architecture: source amd64
Version: 2.10-3
Distribution: unstable
Urgency: medium
Maintainer: Santiago Vila <sanvila@debian.org>
Changed-By: Santiago Vila <sanvila@debian.org>
Description:
 hello      - example package based on GNU hello
Closes: 871622 893083
Changes:
 hello (2.10-3) unstable; urgency=medium
 .
   * Add some autopkgtests. Closes: #871622.
   * Add Vcs-Git and Vcs-Browser fields to debian/control. Closes: #893083.
   * Raise debhelper compat level from 9 to 13. This enables autoreconf,
     and as a result, some additional build-dependencies are required:
   - Add texinfo to Build-Depends, for a normal build.
   - Add help2man to Build-Depends, for a build using git.
   * Use secure URI in Homepage field.
   * Set upstream metadata fields Bug-Submit, Name and Repository-Browse.
   * Add upstream signing-key.
   * Use a common debian/watch file which is valid for most GNU packages.
   * Sort control fields using wrap-and-sort.
   * Update standards version to 4.6.2.
Checksums-Sha1:
 4755bb94240986213836726f9b594e853920f541 1183 hello_2.10-3.dsc
 82e477ec77f09bae910e53592d28319774754af6 12688 hello_2.10-3.debian.tar.xz
 45a6ecadd0d8672ab875451c17f84067137783c8 36084 hello-dbgsym_2.10-3_amd64.deb
 9a6e6d94a7bbf07e8d8f46071dbaa3fc9c0f1227 7657 hello_2.10-3_amd64.buildinfo
 8439082041b2b154fdb48f98530cbdf54557abac 53324 hello_2.10-3_amd64.deb
Checksums-Sha256:
 e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba 1183 hello_2.10-3.dsc
 f43ddcca8d7168c5d52b53e1f2a69b78f42f8387633ef8955edd0621c73cf65c 12688 hello_2.10-3.debian.tar.xz
 16990db381cd1816fc65436447dedaa3298fc29179ee7e4379e7793a7d75cacb 36084 hello-dbgsym_2.10-3_amd64.deb
 ae955f1835dd9948fa6b8aaeb6f26aff21ff6501a41913ae52306aa2d627f918 7657 hello_2.10-3_amd64.buildinfo
 052cb5fdfa86bb3485d6194d9ae2fd1cabbccbdd9c7da3258aed1674b288bbf9 53324 hello_2.10-3_amd64.deb
Files:
 e7bd195571b19d33bd83d1c379fe6432 1183 devel optional hello_2.10-3.dsc
 16678389ba7fddcdfa05e0707d61f043 12688 devel optional hello_2.10-3.debian.tar.xz
 5b2bcd51a3ad0d0e611aafd9276b938e 36084 debug optional hello-dbgsym_2.10-3_amd64.deb
 57144f2c9158564350da3371b5b9a542 7657 devel optional hello_2.10-3_amd64.buildinfo
 d36abefbc87d8dfb7704238f0aee0e90 53324 devel optional hello_2.10-3_amd64.deb
"
        ),
        (
            hello_buildinfo,
            "\
Format: 1.0
Source: hello
Binary: hello hello-dbgsym
Architecture: amd64 source
Version: 2.10-3
Checksums-Md5:
 e7bd195571b19d33bd83d1c379fe6432 1183 hello_2.10-3.dsc
 5b2bcd51a3ad0d0e611aafd9276b938e 36084 hello-dbgsym_2.10-3_amd64.deb
 d36abefbc87d8dfb7704238f0aee0e90 53324 hello_2.10-3_amd64.deb
Checksums-Sha1:
 4755bb94240986213836726f9b594e853920f541 1183 hello_2.10-3.dsc
 45a6ecadd0d8672ab875451c17f84067137783c8 36084 hello-dbgsym_2.10-3_amd64.deb
 8439082041b2b154fdb48f98530cbdf54557abac 53324 hello_2.10-3_amd64.deb
Checksums-Sha256:
 e8ba61cf5c8e2ef3107cc1c6e4fb7125064947dd5565c22cde1b9a407c6264ba 1183 hello_2.10-3.dsc
 16990db381cd1816fc65436447dedaa3298fc29179ee7e4379e7793a7d75cacb 36084 hello-dbgsym_2.10-3_amd64.deb
 052cb5fdfa86bb3485d6194d9ae2fd1cabbccbdd9c7da3258aed1674b288bbf9 53324 hello_2.10-3_amd64.deb
Build-Origin: Debian
Build-Architecture: amd64
Build-Date: Wed, 04 Dec 2024 15:18:38 -0500
Build-Tainted-By:
 merged-usr-via-aliased-dirs
 usr-local-has-includes
 usr-local-has-libraries
 usr-local-has-programs
Installed-Build-Depends:
 autoconf (= 2.72-3),
 automake (= 1:1.16.5-1.3),
 autopoint (= 0.22.5-2),
 autotools-dev (= 20220109.1),
 base-files (= 13.5),
 base-passwd (= 3.6.5),
 bash (= 5.2.32-1+b2),
 binutils (= 2.43.1-5),
 binutils-common (= 2.43.1-5),
 binutils-x86-64-linux-gnu (= 2.43.1-5),
 bsdextrautils (= 2.40.2-11),
 bsdmainutils (= 12.1.8),
 bsdutils (= 1:2.40.2-11),
 build-essential (= 12.12),
 bzip2 (= 1.0.8-6),
 clang-13 (= 1:13.0.1-13),
 clang-14 (= 1:14.0.6-20),
 clang-16 (= 1:16.0.6-27+b1),
 clang-17 (= 1:17.0.6-18),
 clang-18 (= 1:18.1.8-12),
 clang-19 (= 1:19.1.3-2),
 coreutils (= 9.5-1+b1),
 cpp (= 4:14.2.0-1),
 cpp-11 (= 11.5.0-1),
 cpp-12 (= 12.4.0-2),
 cpp-13 (= 13.3.0-8),
 cpp-13-x86-64-linux-gnu (= 13.3.0-8),
 cpp-14 (= 14.2.0-8),
 cpp-14-x86-64-linux-gnu (= 14.2.0-8),
 cpp-x86-64-linux-gnu (= 4:14.2.0-1),
 dash (= 0.5.12-9),
 debconf (= 1.5.87),
 debhelper (= 13.20),
 debianutils (= 5.21),
 dh-autoreconf (= 20),
 dh-strip-nondeterminism (= 1.14.0-1),
 diffutils (= 1:3.10-1),
 dpkg (= 1.22.11),
 dpkg-dev (= 1.22.11),
 dwz (= 0.15-1+b1),
 file (= 1:5.45-3+b1),
 findutils (= 4.10.0-3),
 g++ (= 4:14.2.0-1),
 g++-14 (= 14.2.0-8),
 g++-14-x86-64-linux-gnu (= 14.2.0-8),
 g++-x86-64-linux-gnu (= 4:14.2.0-1),
 gawk (= 1:5.2.1-2+b1),
 gcc (= 4:14.2.0-1),
 gcc-11 (= 11.5.0-1),
 gcc-11-base (= 11.5.0-1),
 gcc-12 (= 12.4.0-2),
 gcc-12-base (= 12.4.0-2),
 gcc-13 (= 13.3.0-8),
 gcc-13-base (= 13.3.0-8),
 gcc-13-x86-64-linux-gnu (= 13.3.0-8),
 gcc-14 (= 14.2.0-8),
 gcc-14-base (= 14.2.0-8),
 gcc-14-x86-64-linux-gnu (= 14.2.0-8),
 gcc-x86-64-linux-gnu (= 4:14.2.0-1),
 gettext (= 0.22.5-2),
 gettext-base (= 0.22.5-2),
 grep (= 3.11-4),
 groff-base (= 1.23.0-5),
 gzip (= 1.12-1.1),
 help2man (= 1.49.3),
 hostname (= 3.25),
 init-system-helpers (= 1.67),
 install-info (= 7.1.1-1+b1),
 intltool-debian (= 0.35.0+20060710.6),
 lib32gcc-s1 (= 14.2.0-8),
 lib32stdc++6 (= 14.2.0-8),
 libacl1 (= 2.3.2-2+b1),
 libarchive-zip-perl (= 1.68-1),
 libasan6 (= 11.5.0-1),
 libasan8 (= 14.2.0-8),
 libatomic1 (= 14.2.0-8),
 libattr1 (= 1:2.5.2-2),
 libaudit-common (= 1:4.0.2-2),
 libaudit1 (= 1:4.0.2-2),
 libbinutils (= 2.43.1-5),
 libblkid1 (= 2.40.2-11),
 libbsd0 (= 0.12.2-2),
 libbz2-1.0 (= 1.0.8-6),
 libc-bin (= 2.40-3),
 libc-dev-bin (= 2.40-3),
 libc6 (= 2.40-3),
 libc6-dev (= 2.40-3),
 libc6-i386 (= 2.40-3),
 libcap-ng0 (= 0.8.5-3+b1),
 libcap2 (= 1:2.66-5+b1),
 libcc1-0 (= 14.2.0-8),
 libclang-common-13-dev (= 1:13.0.1-13),
 libclang-common-14-dev (= 1:14.0.6-20),
 libclang-common-16-dev (= 1:16.0.6-27+b1),
 libclang-common-17-dev (= 1:17.0.6-18),
 libclang-common-18-dev (= 1:18.1.8-12),
 libclang-common-19-dev (= 1:19.1.3-2),
 libclang-cpp13 (= 1:13.0.1-13),
 libclang-cpp14t64 (= 1:14.0.6-20),
 libclang-cpp16t64 (= 1:16.0.6-27+b1),
 libclang-cpp17t64 (= 1:17.0.6-18),
 libclang-cpp18 (= 1:18.1.8-12),
 libclang-cpp19 (= 1:19.1.3-2),
 libclang1-13 (= 1:13.0.1-13),
 libclang1-14t64 (= 1:14.0.6-20),
 libclang1-16t64 (= 1:16.0.6-27+b1),
 libclang1-17t64 (= 1:17.0.6-18),
 libclang1-18 (= 1:18.1.8-12),
 libclang1-19 (= 1:19.1.3-2),
 libcrypt-dev (= 1:4.4.36-5),
 libcrypt1 (= 1:4.4.36-5),
 libctf-nobfd0 (= 2.43.1-5),
 libctf0 (= 2.43.1-5),
 libdb5.3t64 (= 5.3.28+dfsg2-9),
 libdebconfclient0 (= 0.273),
 libdebhelper-perl (= 13.20),
 libdpkg-perl (= 1.22.11),
 libedit2 (= 3.1-20240808-1),
 libelf1t64 (= 0.192-4),
 libffi8 (= 3.4.6-1),
 libfile-find-rule-perl (= 0.34-3),
 libfile-stripnondeterminism-perl (= 1.14.0-1),
 libgc1 (= 1:8.2.8-1),
 libgcc-11-dev (= 11.5.0-1),
 libgcc-12-dev (= 12.4.0-2),
 libgcc-13-dev (= 13.3.0-8),
 libgcc-14-dev (= 14.2.0-8),
 libgcc-s1 (= 14.2.0-8),
 libgdbm-compat4t64 (= 1.24-2),
 libgdbm6t64 (= 1.24-2),
 libgmp10 (= 2:6.3.0+dfsg-2+b2),
 libgomp1 (= 14.2.0-8),
 libgprofng0 (= 2.43.1-5),
 libhwasan0 (= 14.2.0-8),
 libicu72 (= 72.1-5+b1),
 libisl23 (= 0.27-1),
 libitm1 (= 14.2.0-8),
 libjansson4 (= 2.14-2+b3),
 libllvm13 (= 1:13.0.1-13),
 libllvm14t64 (= 1:14.0.6-20),
 libllvm16t64 (= 1:16.0.6-27+b1),
 libllvm17t64 (= 1:17.0.6-18),
 libllvm18 (= 1:18.1.8-12),
 libllvm19 (= 1:19.1.3-2),
 liblocale-gettext-perl (= 1.07-7+b1),
 liblsan0 (= 14.2.0-8),
 liblzma5 (= 5.6.3-1+b1),
 libmagic-mgc (= 1:5.45-3+b1),
 libmagic1t64 (= 1:5.45-3+b1),
 libmd0 (= 1.1.0-2+b1),
 libmount1 (= 2.40.2-11),
 libmpc3 (= 1.3.1-1+b3),
 libmpfr6 (= 4.2.1-1+b2),
 libnumber-compare-perl (= 0.03-3),
 libobjc-13-dev (= 13.3.0-8),
 libobjc-14-dev (= 14.2.0-8),
 libobjc4 (= 14.2.0-8),
 libpam-modules (= 1.5.3-7+b1),
 libpam-modules-bin (= 1.5.3-7+b1),
 libpam-runtime (= 1.5.3-7),
 libpam0g (= 1.5.3-7+b1),
 libpcre2-8-0 (= 10.44-4),
 libperl5.40 (= 5.40.0-7),
 libpipeline1 (= 1.5.8-1),
 libquadmath0 (= 14.2.0-8),
 libreadline8t64 (= 8.2-5),
 libseccomp2 (= 2.5.5-1+b3),
 libselinux1 (= 3.7-3+b1),
 libsframe1 (= 2.43.1-5),
 libsigsegv2 (= 2.14-1+b2),
 libsmartcols1 (= 2.40.2-11),
 libssl3t64 (= 3.3.2-2),
 libstdc++-13-dev (= 13.3.0-8),
 libstdc++-14-dev (= 14.2.0-8),
 libstdc++6 (= 14.2.0-8),
 libsystemd0 (= 257~rc2-3),
 libtext-glob-perl (= 0.11-3),
 libtext-unidecode-perl (= 1.30-3),
 libtinfo6 (= 6.5-2+b1),
 libtool (= 2.4.7-8),
 libtsan0 (= 11.5.0-1),
 libtsan2 (= 14.2.0-8),
 libubsan1 (= 14.2.0-8),
 libuchardet0 (= 0.0.8-1+b2),
 libudev1 (= 257~rc2-3),
 libunistring5 (= 1.2-1+b1),
 libuuid1 (= 2.40.2-11),
 libxml-libxml-perl (= 2.0207+dfsg+really+2.0134-5+b1),
 libxml-namespacesupport-perl (= 1.12-2),
 libxml-sax-base-perl (= 1.09-3),
 libxml-sax-perl (= 1.02+dfsg-3),
 libxml2 (= 2.12.7+dfsg+really2.9.14-0.2+b1),
 libz3-4 (= 4.13.3-1),
 libzstd1 (= 1.5.6+dfsg-1+b1),
 linux-libc-dev (= 6.11.9-1),
 llvm-13-linker-tools (= 1:13.0.1-13),
 llvm-14-linker-tools (= 1:14.0.6-20),
 llvm-16-linker-tools (= 1:16.0.6-27+b1),
 llvm-17-linker-tools (= 1:17.0.6-18),
 llvm-18-linker-tools (= 1:18.1.8-12),
 llvm-19-linker-tools (= 1:19.1.3-2),
 m4 (= 1.4.19-4),
 make (= 4.3-4.1),
 man-db (= 2.13.0-1),
 mawk (= 1.3.4.20240905-1),
 ncal (= 12.1.8),
 ncurses-base (= 6.5-2),
 ncurses-bin (= 6.5-2+b1),
 openssl-provider-legacy (= 3.3.2-2),
 patch (= 2.7.6-7),
 perl (= 5.40.0-7),
 perl-base (= 5.40.0-7),
 perl-modules-5.40 (= 5.40.0-7),
 po-debconf (= 1.0.21+nmu1),
 readline-common (= 8.2-5),
 rpcsvc-proto (= 1.4.3-1),
 sed (= 4.9-2),
 sensible-utils (= 0.0.24),
 sysvinit-utils (= 3.11-1),
 tar (= 1.35+dfsg-3),
 tex-common (= 6.18),
 texinfo (= 7.1.1-1),
 texinfo-lib (= 7.1.1-1+b1),
 ucf (= 3.0043+nmu1),
 usr-is-merged (= 39),
 usrmerge (= 39),
 util-linux (= 2.40.2-11),
 xz-utils (= 5.6.3-1+b1),
 zlib1g (= 1:1.3.dfsg+really1.3.1-1+b1)
Environment:
 DEB_BUILD_OPTIONS=\"parallel=8\"
 LANG=\"en_US.UTF-8\"
 SOURCE_DATE_EPOCH=\"1672068600\"
"
        ),
        (
            hello_deb_control,
            "\
Package: hello
Version: 2.10-3
Architecture: amd64
Maintainer: Santiago Vila <sanvila@debian.org>
Installed-Size: 281
Depends: libc6 (>= 2.38)
Conflicts: hello-traditional
Breaks: hello-debhelper (<< 2.9)
Replaces: hello-debhelper (<< 2.9), hello-traditional
Section: devel
Priority: optional
Homepage: https://www.gnu.org/software/hello/
Description: example package based on GNU hello
 The GNU hello program produces a familiar, friendly greeting.  It
 allows non-programmers to use a classic computer science tool which
 would otherwise be unavailable to them.
 .
 Seriously, though: this is an example of how to do a Debian package.
 It is the Debian version of the GNU Project's `hello world' program
 (which is itself an example for the GNU Project).
"
        ),
        (
            hello_source_control_source,
            "\
Source: hello
Section: devel
Priority: optional
Maintainer: Santiago Vila <sanvila@debian.org>
Standards-Version: 4.6.2
Build-Depends: debhelper-compat (= 13), help2man, texinfo
Homepage: https://www.gnu.org/software/hello/
Vcs-Git: https://salsa.debian.org/sanvila/hello.git
Vcs-Browser: https://salsa.debian.org/sanvila/hello
Rules-Requires-Root: no
"
        ),
        (
            hello_source_control_binary,
            "\
Package: hello
Architecture: any
Depends: ${misc:Depends}, ${shlibs:Depends}
Conflicts: hello-traditional
Replaces: hello-debhelper (<< 2.9), hello-traditional
Breaks: hello-debhelper (<< 2.9)
Description: example package based on GNU hello
 The GNU hello program produces a familiar, friendly greeting.  It
 allows non-programmers to use a classic computer science tool which
 would otherwise be unavailable to them.
 .
 Seriously, though: this is an example of how to do a Debian package.
 It is the Debian version of the GNU Project's `hello world' program
 (which is itself an example for the GNU Project).
"
        ),

        // Debian archive stuff
        (
            archive_main_source_release,
            "\
Archive: unstable
Origin: Debian
Label: Debian
Acquire-By-Hash: yes
Component: main
Architecture: source
"
        ),
        (
            archive_main_sources_rustc,
            "\
Package: rustc
Binary: rustc, libstd-rust-1.63, libstd-rust-dev, libstd-rust-dev-windows, libstd-rust-dev-wasm32, rust-gdb, rust-lldb, rust-doc, rust-src, rust-clippy, rustfmt, rust-all
Version: 1.63.0+dfsg1-2
Maintainer: Debian Rust Maintainers <pkg-rust-maintainers@alioth-lists.debian.net>
Uploaders: Ximin Luo <infinity0@debian.org>, Sylvestre Ledru <sylvestre@debian.org>
Build-Depends: debhelper (>= 9), debhelper-compat (= 13), dpkg-dev (>= 1.17.14), python3:native, cargo:native (>= 0.60.0) <!pkg.rustc.dlstage0>, rustc:native (>= 1.62.0+dfsg) <!pkg.rustc.dlstage0>, rustc:native (<= 1.63.0++) <!pkg.rustc.dlstage0>, llvm-14-dev:native, llvm-14-tools:native, gcc-mingw-w64-x86-64-posix:native [amd64] <!nowindows>, gcc-mingw-w64-i686-posix:native [i386] <!nowindows>, libllvm14 (>= 1:14.0.0), cmake (>= 3.0) | cmake3, pkg-config, zlib1g-dev:native, zlib1g-dev, liblzma-dev:native, binutils (>= 2.26) <!nocheck> | binutils-2.26 <!nocheck>, git <!nocheck>, procps <!nocheck>, gdb (>= 7.12) <!nocheck>, curl <pkg.rustc.dlstage0>, ca-certificates <pkg.rustc.dlstage0>
Build-Depends-Indep: wasi-libc (>= 0.0~git20220510.9886d3d~~) <!nowasm>, wasi-libc (<= 0.0~git20220510.9886d3d++) <!nowasm>, clang-14:native
Build-Conflicts: gdb-minimal <!nocheck>
Architecture: any all
Standards-Version: 4.2.1
Format: 3.0 (quilt)
Files:
 454c4460543d0b24286230745100ed7b 3535 rustc_1.63.0+dfsg1-2.dsc
 8a49f05851fbe10382d8f925b5a58412 32527136 rustc_1.63.0+dfsg1.orig.tar.xz
 01804b16b0393885113e8b7a73d1be2c 92248 rustc_1.63.0+dfsg1-2.debian.tar.xz
Vcs-Browser: https://salsa.debian.org/rust-team/rust
Vcs-Git: https://salsa.debian.org/rust-team/rust.git
Checksums-Sha256:
 bf8819414af73093f2227658da8a77b5ae6cf04697cdc01cae8b877392b917a4 3535 rustc_1.63.0+dfsg1-2.dsc
 ad79e62b57ab06516595da2513369b0285b6e2bde1331c00a4197f492f7fad3d 32527136 rustc_1.63.0+dfsg1.orig.tar.xz
 523aaadf48741ad0d47bfba0dc677d9f50d114f5913794dafd00ae01ea74acff 92248 rustc_1.63.0+dfsg1-2.debian.tar.xz
Homepage: http://www.rust-lang.org/
Package-List: 
 libstd-rust-1.63 deb libs optional arch=any
 libstd-rust-dev deb libdevel optional arch=any
 libstd-rust-dev-wasm32 deb libdevel optional arch=all profile=!nowasm
 libstd-rust-dev-windows deb libdevel optional arch=amd64,i386 profile=!nowindows
 rust-all deb devel optional arch=all
 rust-clippy deb devel optional arch=any
 rust-doc deb doc optional arch=all profile=!nodoc
 rust-gdb deb devel optional arch=all
 rust-lldb deb devel optional arch=all
 rust-src deb devel optional arch=all
 rustc deb devel optional arch=any
 rustfmt deb devel optional arch=any
Extra-Source-Only: yes
Directory: pool/main/r/rustc
Priority: optional
Section: rust
"
        ),
        (
            archive_main_riscv64_cargo,
            "\
Package: cargo
Source: rustc
Version: 1.82.0+dfsg1-2
Installed-Size: 18664
Maintainer: Debian Rust Maintainers <pkg-rust-maintainers@alioth-lists.debian.net>
Architecture: riscv64
Depends: libc6 (>= 2.39), libcurl4t64 (>= 7.28.0), libgcc-s1 (>= 4.2), libgit2-1.8 (>= 1.8.1), libsqlite3-0 (>= 3.5.9), libssh2-1t64 (>= 1.2.5), libssl3t64 (>= 3.0.0), zlib1g (>= 1:1.2.3.4), rustc (= 1.82.0+dfsg1-2), binutils, gcc | clang | c-compiler
Suggests: cargo-doc, python3
Description: Rust package manager
Multi-Arch: allowed
Homepage: http://www.rust-lang.org/
Description-md5: e3c7d8ff9b0415379acb83f0119168b0
Section: rust
Priority: optional
Filename: pool/main/r/rustc/cargo_1.82.0+dfsg1-2_riscv64.deb
Size: 5687316
MD5sum: 5d6db6c5c651ebdf63e467c2ffcd4f78
SHA256: d55cd074cf203b4a07a42c353e1fe5becdb063d2f30a5688cc5cf7f00e12c139
"
        ),
        (
            archive_main_release_truncated,
            "\
Origin: Debian
Label: Debian
Suite: unstable
Codename: sid
Changelogs: https://metadata.ftp-master.debian.org/changelogs/@CHANGEPATH@_changelog
Date: Wed, 04 Dec 2024 14:20:42 UTC
Valid-Until: Wed, 11 Dec 2024 14:20:42 UTC
Acquire-By-Hash: yes
No-Support-for-Architecture-all: Packages
Architectures: all amd64 arm64 armel armhf i386 mips64el ppc64el riscv64 s390x
Components: main contrib non-free-firmware non-free
Description: Debian x.y Unstable - Not Released
MD5Sum:
 f57278429373da6a6db8135b653af94c  5361511 contrib/Contents-all
 2e6bf7f816776d250b799df9c0cf5e9e    63339 contrib/Contents-all.diff/Index
 a8df9ce09880bf7add0924b2fac7fe17   451576 contrib/Contents-all.gz
 73686cbf329c1139db19f06dd4b565b9  3120793 contrib/Contents-amd64
 ed8f0d230281494467fab6b475ae7d77    63339 contrib/Contents-amd64.diff/Index
 4f36e97cf77b4ce553c5406d45fb5a35   194038 contrib/Contents-amd64.gz
 fafce03bbaba67cb0235d4267e479dc2  1606698 contrib/Contents-arm64
 56d1da64749aae6c6e9b62436c6b4551    63339 contrib/Contents-arm64.diff/Index
 76552011e8d349656d51391a6832e7ee   112280 contrib/Contents-arm64.gz
 69714e265011b4e1b8a02541f9718b91   474872 contrib/Contents-armel
SHA256:
 fcba5d8c97f4515a5972bfd80fe1b880676a5ed21dd986fea452418fc2ace3be  5361511 contrib/Contents-all
 d4cebdef1f2f819b1b9e0fafe4f608eda9337113e11ebfe69e9cdefa04284bf2    63339 contrib/Contents-all.diff/Index
 0dde53f51dfae58d3347874c25c22d7c32d2ca972f952e2e932f50c871756e96   451576 contrib/Contents-all.gz
 6ea3e636f7229a9575a8a95546134e91c4f667eda4c4ad460ee600602d314417  3120793 contrib/Contents-amd64
 abe4e8daf0fdede26ab713175ea4e6e2979764146cf1a92679d6e614fcc6da39    63339 contrib/Contents-amd64.diff/Index
 cbc07658bd14ce92db47f23cf583d22852edfa4cb7afe4f784f98fd71bcd0c0d   194038 contrib/Contents-amd64.gz
 3127cd2bc7977c09377aefe7e25a56b1446b63d7d2c9589f4500753880e110f1  1606698 contrib/Contents-arm64
 9b7a0b2ab71a19a3281c2e9e66238887b03fcba0fcc589d738b2274b3cec368f    63339 contrib/Contents-arm64.diff/Index
 086a7968419c4208e2627396949f0168f0b58b79c512e95db9207b04b6b3a9ac   112280 contrib/Contents-arm64.gz
 04d2faf8e16fa5fd170c90e89a7a2912add287908a41ffbde7d0ee0e84624fdb   474872 contrib/Contents-armel
 a41d19841d812abd8952fc16f12aa6eec21690f15587f0cb6d9509db642794be    63339 contrib/Contents-armel.diff/Index
 84afe43d968bd2bf2ba70c9defa390475c500ded31a0737d23db87e3a0592065    38045 contrib/Contents-armel.gz
 571c0c425b88c76eedf1598c695c3b6fd0bdccb5b2a5b221e38b5c99fefcc6e9   483012 contrib/Contents-armhf
 b7513802a68d60d017a9e2e1ae0351eba64c9f9c9894b844b43a947835e68875    63339 contrib/Contents-armhf.diff/Index
 a18ba087e9fd78eb28ef179cdcd735892691505b19e1f58740a0a874ca919f5b    39908 contrib/Contents-armhf.gz
 a30c4ab23676f45ba09f13d6d23a66c6c4d106c7428c09007d83b8a478f427c3   496872 contrib/Contents-i386
 1854d31ccc5e5c3b56e61d934f7b0ab4b0a544ea9a73046383301f8976d00a55    63339 contrib/Contents-i386.diff/Index
 9b417560933d7bb5ec6b36c1e23f9d19ac86d680e0d11afe0b246e2cd02f1324    41689 contrib/Contents-i386.gz
 70c5b989f6dd89590cacc4f4fb47ad6e8349f4a81a670f329ba614275abadc9a   470432 contrib/Contents-mips64el
 3ae6a189abf4ab4c6c15b64b5d46bb2302b261a7106ef1ac7d651b431c56f8cc    63339 contrib/Contents-mips64el.diff/Index
"
        )
    );
}

// vim: foldmethod=marker
