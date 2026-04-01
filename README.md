<!-- AUTOMATICALLY GENERATED, DO NOT EDIT -->
<!-- edit README.md.template instead -->

# Rust serialization benchmark

The goal of these benchmarks is to provide thorough and complete benchmarks for various rust
serialization frameworks.

## Maintainers

These benchmarks are maintained by a small group of volunteers. Special thanks to:

- [djkoloski](https://github.com/djkoloski)
- [mumbleskates](https://github.com/mumbleskates)
- [finnbear](https://github.com/finnbear)

## These benchmarks are a work in progress

These benchmarks are still being developed and pull requests to improve benchmarks are welcome.

## [Interactive site](https://djkoloski.github.io/rust_serialization_benchmark/)

Calculate the number of messages per second that can be sent/received with various rust serialization frameworks and compression libraries.
[Documentation](pages/README.md)

## Format

All tests benchmark the following properties (time or size):

* **Serialize**: serialize data into a buffer
* **Deserialize**: deserializes a buffer into a normal rust object
* **Borrow**: deserializes a buffer into a rust object that borrows string data from the input, with lifetime
* **Size**: the size of the buffer when serialized
* **Zlib**: the size of the buffer after zlib compression
* **Zstd**: the size of the buffer after zstd compression
* **Zstd Time**: the time taken to compress the serialized buffer with zstd

Zero-copy deserialization libraries have an additional set of benchmarks:

* **Access**: accesses a buffer as structured data
* **Read**: runs through a buffer and reads fields out of it
* **Update**: updates a buffer as structured data

Some benchmark results may be italicized and followed by an asterisk. Mouse over these for more details on what situation was benchmarked. Other footnotes are located at the bottom.

## Last updated: 2026-03-28 03:18:11

<details><summary>Runtime info</summary>

### `rustc` version

```
rustc 1.96.0-nightly (fda6d37bb 2026-03-27)
binary: rustc
commit-hash: fda6d37bb88ee12fd50fa54d15859f1f91b74f55
commit-date: 2026-03-27
host: x86_64-unknown-linux-gnu
release: 1.96.0-nightly
LLVM version: 22.1.2
```

### CPU info

```
Architecture:                            x86_64
CPU op-mode(s):                          32-bit, 64-bit
Address sizes:                           48 bits physical, 48 bits virtual
Byte Order:                              Little Endian
CPU(s):                                  4
On-line CPU(s) list:                     0-3
Vendor ID:                               AuthenticAMD
Model name:                              AMD EPYC 7763 64-Core Processor
CPU family:                              25
Model:                                   1
Thread(s) per core:                      2
Core(s) per socket:                      2
Socket(s):                               1
Stepping:                                1
BogoMIPS:                                4890.86
Flags:                                   fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl tsc_reliable nonstop_tsc cpuid extd_apicid aperfmperf tsc_known_freq pni pclmulqdq ssse3 fma cx16 pcid sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand hypervisor lahf_lm cmp_legacy svm cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw topoext vmmcall fsgsbase bmi1 avx2 smep bmi2 erms invpcid rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves user_shstk clzero xsaveerptr rdpru arat npt nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold v_vmsave_vmload umip vaes vpclmulqdq rdpid fsrm
Virtualization:                          AMD-V
Hypervisor vendor:                       Microsoft
Virtualization type:                     full
L1d cache:                               64 KiB (2 instances)
L1i cache:                               64 KiB (2 instances)
L2 cache:                                1 MiB (2 instances)
L3 cache:                                32 MiB (1 instance)
NUMA node(s):                            1
NUMA node0 CPU(s):                       0-3
Vulnerability Gather data sampling:      Not affected
Vulnerability Ghostwrite:                Not affected
Vulnerability Indirect target selection: Not affected
Vulnerability Itlb multihit:             Not affected
Vulnerability L1tf:                      Not affected
Vulnerability Mds:                       Not affected
Vulnerability Meltdown:                  Not affected
Vulnerability Mmio stale data:           Not affected
Vulnerability Old microcode:             Not affected
Vulnerability Reg file data sampling:    Not affected
Vulnerability Retbleed:                  Not affected
Vulnerability Spec rstack overflow:      Vulnerable: Safe RET, no microcode
Vulnerability Spec store bypass:         Vulnerable
Vulnerability Spectre v1:                Mitigation; usercopy/swapgs barriers and __user pointer sanitization
Vulnerability Spectre v2:                Mitigation; Retpolines; STIBP disabled; RSB filling; PBRSB-eIBRS Not affected; BHI Not affected
Vulnerability Srbds:                     Not affected
Vulnerability Tsa:                       Vulnerable: No microcode
Vulnerability Tsx async abort:           Not affected
Vulnerability Vmscape:                   Not affected
```

</details>

## `log`

This data set is composed of HTTP request logs that are small and contain many strings.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*454.08 µs\**</span> <span title="prepend">*420.07 µs\**</span> | 2.5561 ms | 855.17 µs | 804955 | 328941 | 284849 | 4.1450 ms |
| [bin-proto 0.12.3][bin-proto] | 4.5199 ms | 4.7250 ms | † | 1045784 | 373127 | 311553 | 4.4993 ms |
| [bincode 2.0.1][bincode] | 331.58 µs | 2.1495 ms | 675.72 µs | 741295 | 303944 | 256422 | 3.6112 ms |
| [bincode 1.3.3][bincode1] | 552.90 µs | 2.1002 ms | 615.10 µs | 1045784 | 373127 | 311553 | 4.4870 ms |
| [bitcode 0.6.6][bitcode] | 145.91 µs | 1.5017 ms | 60.061 µs | 703710 | 288826 | 227322 | 2.4893 ms |
| [borsh 1.5.7][borsh] | 553.88 µs | 2.1586 ms | † | 885780 | 362204 | 286248 | 4.1394 ms |
| [capnp 0.23.2][capnp] | 459.74 µs | † | † | 1443216 | 513986 | 426532 | 6.6657 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 613.31 µs | 5.1824 ms | 3.3615 ms | 1407835 | 403440 | 323561 | 4.7000 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 4.0979 ms | 10.922 ms | † | 1407835 | 403440 | 323561 | 4.7299 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8868 ms | 4.8063 ms | 3.1153 ms | 1407835 | 403440 | 323561 | 5.0286 ms |
| [columnar 0.11.1][columnar] | 252.95 µs | 2.1580 ms <span title="copy_from">*821.75 µs\**</span> | † | 1045928 | 370212 | 293907 | 4.2310 ms |
| [databuf 0.5.0][databuf] | 272.19 µs | 2.0731 ms | 648.14 µs | 765778 | 311715 | 263914 | 3.5308 ms |
| [dlhn 0.1.7][dlhn] | 620.05 µs | 2.6265 ms | † | 724953 | 301446 | 253056 | 3.2036 ms |
| [flatbuffers 25.12.19][flatbuffers] | 1.0376 ms | † | † | 1276368 | 468539 | 388381 | 4.7492 ms |
| [flexbuffers 25.2.10][flexbuffers] | 6.8188 ms | 7.6208 ms | 5.9286 ms | 1829756 | 714318 | 691541 | 8.6290 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7129 ms | 3.8245 ms | † | 1827461 | 470560 | 360727 | 5.4823 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.8695 ms | 6.2277 ms | † | 1827461 | 470560 | 360727 | 5.9855 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.1326 ms | 4.7335 ms | † | 1827461 | 470560 | 360727 | 5.6024 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 907.09 µs | 2.5784 ms | † | 764996 | 315291 | 264212 | 3.6047 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.3597 ms | 3.2021 ms | 1.3528 ms | 784997 | 325384 | 277608 | 3.7697 ms |
| [minicbor 1.0.0][minicbor] | 480.88 µs | 3.0494 ms | 1.3656 ms | 817830 | 332671 | 284034 | 3.9637 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.3784 ms | 4.0698 ms | 2.5694 ms | 818669 | 332556 | 284797 | 4.1405 ms |
| [nanoserde 0.2.1][nanoserde] | 255.07 µs | 2.1320 ms | † | 1045784 | 373127 | 311553 | 4.2314 ms |
| [nibblecode 0.1.0][nibblecode] | 221.40 µs | † | † | 1011487 | 483275 | 413250 | 5.5017 ms |
| [postcard 1.1.1][postcard] | 419.39 µs | 2.2774 ms | 794.12 µs | 724953 | 302399 | 252968 | 3.1729 ms |
| [pot 3.0.1][pot] | 2.2846 ms | 6.6729 ms | 4.9795 ms | 971922 | 372513 | 303636 | 4.3722 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*936.68 µs\**</span> <span title="populate + encode">*2.4820 ms\**</span> | 3.5498 ms | † | 884628 | 363130 | 314959 | 4.3469 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.3157 ms\**</span> <span title="populate + encode">*3.1474 ms\**</span> | 3.8714 ms | † | 884628 | 363130 | 314959 | 4.3711 ms |
| [rkyv 0.8.10][rkyv] | 246.63 µs | <span title="unvalidated">*1.5489 ms\**</span> <span title="validated upfront with error">*1.9040 ms\**</span> | † | 1011488 | 393526 | 325965 | 4.5596 ms |
| [ron 0.10.1][ron] | 12.313 ms | 23.944 ms | 22.345 ms | 1607459 | 449158 | 349324 | 5.5621 ms |
| [savefile 0.18.6][savefile] | 196.82 µs | 2.1007 ms | † | 1045800 | 373139 | 311562 | 4.2084 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 656.05 µs | 2.3853 ms | † | 765778 | 311743 | 263822 | 3.6158 ms |
| [serde-brief 0.1.1][serde-brief] | 1.3682 ms | 4.6577 ms | 3.0138 ms | 1584946 | 413733 | 339964 | 5.1789 ms |
| [serde_bare 0.5.0][serde_bare] | 694.20 µs | 2.1107 ms | † | 765778 | 311715 | 263914 | 3.5156 ms |
| [speedy 0.8.7][speedy] | 201.86 µs | 1.7672 ms | 365.40 µs | 885780 | 362204 | 286248 | 3.8090 ms |
| [wincode 0.2.4][wincode] | 169.22 µs | 1.9357 ms | 470.62 µs | 1045784 | 373127 | 311553 | 4.1968 ms |
| [wiring 0.2.4][wiring] | 194.96 µs | 2.0543 ms | † | 1045784 | 337930 | 275808 | 3.6764 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*84.815 ns\**</span> | <span title="validated on-demand with error">*129.49 µs\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 23.019 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4895 ns\**</span> <span title="validated upfront with error">*2.1784 ms\**</span> | <span title="unvalidated">*51.754 µs\**</span> <span title="validated upfront with error">*2.2467 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2446 ns\**</span> <span title="validated upfront with error">*234.20 µs\**</span> | <span title="unvalidated">*10.721 µs\**</span> <span title="validated upfront with error">*245.52 µs\**</span> | <span title="unvalidated">*7.6631 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*350.75 µs\**</span> | <span title="unvalidated">*10.506 µs\**</span> <span title="validated upfront with error">*361.41 µs\**</span> | <span title="unvalidated">*7.4382 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*32.13%\**</span> <span title="prepend">*34.73%\**</span> | 32.15% | 7.02% | 87.42% | 87.80% | 79.80% | 60.06% |
| [bin-proto 0.12.3][bin-proto] | 3.23% | 17.39% | † | 67.29% | 77.41% | 72.96% | 55.33% |
| [bincode 2.0.1][bincode] | 44.00% | 38.23% | 8.89% | 94.93% | 95.03% | 88.65% | 68.93% |
| [bincode 1.3.3][bincode1] | 26.39% | 39.13% | 9.76% | 67.29% | 77.41% | 72.96% | 55.48% |
| [bitcode 0.6.6][bitcode] | 100.00% | 54.72% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 26.34% | 38.07% | † | 79.45% | 79.74% | 79.41% | 60.14% |
| [capnp 0.23.2][capnp] | 31.74% | † | † | 48.76% | 56.19% | 53.30% | 37.34% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 23.79% | 15.86% | 1.79% | 49.99% | 71.59% | 70.26% | 52.96% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.56% | 7.52% | † | 49.99% | 71.59% | 70.26% | 52.63% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 7.73% | 17.10% | 1.93% | 49.99% | 71.59% | 70.26% | 49.50% |
| [columnar 0.11.1][columnar] | 57.68% | 38.08% <span title="copy_from">*100.00%\**</span> | † | 67.28% | 78.02% | 77.34% | 58.83% |
| [databuf 0.5.0][databuf] | 53.61% | 39.64% | 9.27% | 91.89% | 92.66% | 86.13% | 70.50% |
| [dlhn 0.1.7][dlhn] | 23.53% | 31.29% | † | 97.07% | 95.81% | 89.83% | 77.70% |
| [flatbuffers 25.12.19][flatbuffers] | 14.06% | † | † | 55.13% | 61.64% | 58.53% | 52.42% |
| [flexbuffers 25.2.10][flexbuffers] | 2.14% | 10.78% | 1.01% | 38.46% | 40.43% | 32.87% | 28.85% |
| json:<br> [flexon 0.4.5][flexon] | 5.38% | 21.49% | † | 38.51% | 61.38% | 63.02% | 45.41% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.77% | 13.20% | † | 38.51% | 61.38% | 63.02% | 41.59% |
| json:<br> [simd-json 0.15.1][simd-json] | 6.84% | 17.36% | † | 38.51% | 61.38% | 63.02% | 44.43% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 16.09% | 31.87% | † | 91.99% | 91.61% | 86.04% | 69.06% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 10.73% | 25.66% | 4.44% | 89.64% | 88.76% | 81.89% | 66.03% |
| [minicbor 1.0.0][minicbor] | 30.34% | 26.95% | 4.40% | 86.05% | 86.82% | 80.03% | 62.80% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.71% | 20.19% | 2.34% | 85.96% | 86.85% | 79.82% | 60.12% |
| [nanoserde 0.2.1][nanoserde] | 57.20% | 38.54% | † | 67.29% | 77.41% | 72.96% | 58.83% |
| [nibblecode 0.1.0][nibblecode] | 65.90% | † | † | 69.57% | 59.76% | 55.01% | 45.25% |
| [postcard 1.1.1][postcard] | 34.79% | 36.08% | 7.56% | 97.07% | 95.51% | 89.86% | 78.46% |
| [pot 3.0.1][pot] | 6.39% | 12.31% | 1.21% | 72.40% | 77.53% | 74.87% | 56.93% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*15.58%\**</span> <span title="populate + encode">*5.88%\**</span> | 23.15% | † | 79.55% | 79.54% | 72.18% | 57.27% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*11.09%\**</span> <span title="populate + encode">*4.64%\**</span> | 21.23% | † | 79.55% | 79.54% | 72.18% | 56.95% |
| [rkyv 0.8.10][rkyv] | 59.16% | <span title="unvalidated">*53.05%\**</span> <span title="validated upfront with error">*43.16%\**</span> | † | 69.57% | 73.39% | 69.74% | 54.59% |
| [ron 0.10.1][ron] | 1.19% | 3.43% | 0.27% | 43.78% | 64.30% | 65.07% | 44.75% |
| [savefile 0.18.6][savefile] | 74.13% | 39.12% | † | 67.29% | 77.40% | 72.96% | 59.15% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 22.24% | 34.45% | † | 91.89% | 92.65% | 86.16% | 68.85% |
| [serde-brief 0.1.1][serde-brief] | 10.66% | 17.64% | 1.99% | 44.40% | 69.81% | 66.87% | 48.07% |
| [serde_bare 0.5.0][serde_bare] | 21.02% | 38.93% | † | 91.89% | 92.66% | 86.13% | 70.81% |
| [speedy 0.8.7][speedy] | 72.28% | 46.50% | 16.44% | 79.45% | 79.74% | 79.41% | 65.35% |
| [wincode 0.2.4][wincode] | 86.23% | 42.45% | 12.76% | 67.29% | 77.41% | 72.96% | 59.31% |
| [wiring 0.2.4][wiring] | 74.84% | 40.00% | † | 67.29% | 85.47% | 82.42% | 67.71% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.47%\**</span> | <span title="validated on-demand with error">*8.11%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.41% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*20.30%\**</span> <span title="validated upfront with error">*0.47%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*97.99%\**</span> <span title="validated upfront with error">*4.28%\**</span> | <span title="unvalidated">*97.07%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*2.91%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mesh`

This data set is a single mesh. The mesh contains an array of triangles, each of which has three vertices and a normal vector.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*7.3138 ms\**</span> <span title="prepend">*8.7504 ms\**</span> | 7.7349 ms | 8625005 | 6443961 | 6231572 | 77.579 ms |
| [bin-proto 0.12.3][bin-proto] | 9.0894 ms | 9.4905 ms | 6000008 | 5378500 | 5346908 | 8.5419 ms |
| [bincode 2.0.1][bincode] | 2.8897 ms | 1.0478 ms | 6000005 | 5378497 | 5346882 | 8.6619 ms |
| [bincode 1.3.3][bincode1] | 5.8548 ms | 4.7262 ms | 6000008 | 5378500 | 5346908 | 8.4628 ms |
| [bitcode 0.6.6][bitcode] | 1.3191 ms | 811.43 µs | 6000006 | 5182295 | 4921841 | 13.297 ms |
| [borsh 1.5.7][borsh] | 6.3861 ms | 4.1448 ms | 6000004 | 5378496 | 5346866 | 8.4805 ms |
| [capnp 0.23.2][capnp] | 6.1103 ms | † | 14000088 | 7130367 | 6046182 | 80.457 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 8.9917 ms | 50.537 ms | 13125016 | 7524114 | 6757437 | 93.943 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 63.349 ms | 111.59 ms | 13122324 | 7524660 | 6759128 | 94.092 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 32.305 ms | 41.736 ms | 13122324 | 7524660 | 6759128 | 90.060 ms |
| [columnar 0.11.1][columnar] | 1.7344 ms | 1.4471 ms <span title="copy_from">*711.98 µs\**</span> | 6000120 | 5378435 | 5347039 | 8.4549 ms |
| [databuf 0.5.0][databuf] | 2.4147 ms | 5.4112 ms | 6000003 | 5378495 | 5346897 | 8.5963 ms |
| [dlhn 0.1.7][dlhn] | 4.7391 ms | 6.9295 ms | 6000003 | 5378495 | 5346897 | 8.7367 ms |
| [flatbuffers 25.12.19][flatbuffers] | 444.62 µs | † | 6000024 | 5378434 | 5346878 | 8.7584 ms |
| [flexbuffers 25.2.10][flexbuffers] | 104.55 ms | 93.209 ms | 26609424 | 11901040 | 12486322 | 148.77 ms |
| json:<br> [flexon 0.4.5][flexon] | 76.345 ms | 56.336 ms | 26192883 | 9566084 | 8584671 | 155.35 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 87.912 ms | 100.86 ms | 26192883 | 9566084 | 8584671 | 157.55 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 53.706 ms | 66.491 ms | 26192883 | 9566084 | 8584671 | 157.52 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 2.0937 ms | 5.1096 ms | 7500005 | 6058442 | 6014500 | 10.277 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 18.886 ms | 16.013 ms | 8125006 | 6494876 | 6391037 | 69.451 ms |
| [minicbor 1.0.0][minicbor] | 5.1969 ms | 11.992 ms | 8125006 | 6494907 | 6390894 | 69.461 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 121.62 ms | 28.102 ms | 8125037 | 6493484 | 6386940 | 74.468 ms |
| [nanoserde 0.2.1][nanoserde] | 1.6652 ms | 827.92 µs | 6000008 | 5378500 | 5346908 | 8.4682 ms |
| [nibblecode 0.1.0][nibblecode] | 148.48 µs | † | 6000008 | 5378500 | 5346908 | 8.4519 ms |
| [postcard 1.1.1][postcard] | 482.34 µs | 1.0053 ms | 6000003 | 5378495 | 5346897 | 8.5618 ms |
| [pot 3.0.1][pot] | 38.649 ms | 70.820 ms | 10122342 | 6814618 | 6852252 | 81.782 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*7.8291 ms\**</span> <span title="populate + encode">*8.4528 ms\**</span> | 14.031 ms | 8750000 | 6665735 | 6421877 | 72.240 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*14.326 ms\**</span> <span title="populate + encode">*30.725 ms\**</span> | 29.289 ms | 8750000 | 6665735 | 6421877 | 79.615 ms |
| [rkyv 0.8.10][rkyv] | 200.06 µs | <span title="unvalidated">*148.85 µs\**</span> <span title="validated upfront with error">*149.32 µs\**</span> | 6000008 | 5378500 | 5346872 | 8.5432 ms |
| [ron 0.10.1][ron] | 167.66 ms | 544.16 ms | 22192885 | 8970395 | 8137334 | 149.94 ms |
| [savefile 0.18.6][savefile] | 148.58 µs | 148.77 µs | 6000024 | 5378519 | 5346896 | 8.5484 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 4.8035 ms | 4.1894 ms | 6000004 | 5378496 | 5346866 | 8.5612 ms |
| [serde-brief 0.1.1][serde-brief] | 18.277 ms | 35.407 ms | 15750015 | 8024540 | 6813667 | 94.267 ms |
| [serde_bare 0.5.0][serde_bare] | 5.7834 ms | 4.8164 ms | 6000003 | 5378495 | 5346897 | 8.5413 ms |
| [speedy 0.8.7][speedy] | 199.27 µs | 199.40 µs | 6000004 | 5378496 | 5346866 | 8.5545 ms |
| [wincode 0.2.4][wincode] | 199.55 µs | 148.82 µs | 6000008 | 5378500 | 5346908 | 8.5544 ms |
| [wiring 0.2.4][wiring] | 149.90 µs | 342.26 µs | 6000008 | 5378952 | 5346905 | 8.5266 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*110.96 ns\**</span> | <span title="validated on-demand with error">*1.9185 ms\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 21.784 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4889 ns\**</span> <span title="validated upfront with error">*44.234 ns\**</span> | <span title="unvalidated">*77.823 µs\**</span> <span title="validated upfront with error">*77.914 µs\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*1.5567 ns\**</span> | <span title="unvalidated">*38.898 µs\**</span> <span title="validated upfront with error">*38.898 µs\**</span> | <span title="unvalidated">*79.176 µs\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2447 ns\**</span> <span title="validated upfront with error">*4.9904 ns\**</span> | <span title="unvalidated">*38.890 µs\**</span> <span title="validated upfront with error">*38.904 µs\**</span> | <span title="unvalidated">*100.06 µs\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*2.03%\**</span> <span title="prepend">*1.70%\**</span> | 1.92% | 69.57% | 80.42% | 78.98% | 10.89% |
| [bin-proto 0.12.3][bin-proto] | 1.63% | 1.57% | 100.00% | 96.35% | 92.05% | 98.95% |
| [bincode 2.0.1][bincode] | 5.14% | 14.20% | 100.00% | 96.35% | 92.05% | 97.58% |
| [bincode 1.3.3][bincode1] | 2.54% | 3.15% | 100.00% | 96.35% | 92.05% | 99.87% |
| [bitcode 0.6.6][bitcode] | 11.26% | 18.33% | 100.00% | 100.00% | 100.00% | 63.56% |
| [borsh 1.5.7][borsh] | 2.33% | 3.59% | 100.00% | 96.35% | 92.05% | 99.66% |
| [capnp 0.23.2][capnp] | 2.43% | † | 42.86% | 72.68% | 81.40% | 10.50% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 1.65% | 0.29% | 45.71% | 68.88% | 72.84% | 9.00% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 0.23% | 0.13% | 45.72% | 68.87% | 72.82% | 8.98% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 0.46% | 0.36% | 45.72% | 68.87% | 72.82% | 9.38% |
| [columnar 0.11.1][columnar] | 8.56% | 10.28% <span title="copy_from">*20.90%\**</span> | 100.00% | 96.35% | 92.05% | 99.96% |
| [databuf 0.5.0][databuf] | 6.15% | 2.75% | 100.00% | 96.35% | 92.05% | 98.32% |
| [dlhn 0.1.7][dlhn] | 3.13% | 2.15% | 100.00% | 96.35% | 92.05% | 96.74% |
| [flatbuffers 25.12.19][flatbuffers] | 33.39% | † | 100.00% | 96.35% | 92.05% | 96.50% |
| [flexbuffers 25.2.10][flexbuffers] | 0.14% | 0.16% | 22.55% | 43.54% | 39.42% | 5.68% |
| json:<br> [flexon 0.4.5][flexon] | 0.19% | 0.26% | 22.91% | 54.17% | 57.33% | 5.44% |
| json:<br> [serde_json 1.0.140][serde_json] | 0.17% | 0.15% | 22.91% | 54.17% | 57.33% | 5.36% |
| json:<br> [simd-json 0.15.1][simd-json] | 0.28% | 0.22% | 22.91% | 54.17% | 57.33% | 5.37% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 7.09% | 2.91% | 80.00% | 85.54% | 81.83% | 82.24% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 0.79% | 0.93% | 73.85% | 79.79% | 77.01% | 12.17% |
| [minicbor 1.0.0][minicbor] | 2.86% | 1.24% | 73.85% | 79.79% | 77.01% | 12.17% |
| [nachricht-serde 0.4.0][nachricht-serde] | 0.12% | 0.53% | 73.85% | 79.81% | 77.06% | 11.35% |
| [nanoserde 0.2.1][nanoserde] | 8.92% | 17.97% | 100.00% | 96.35% | 92.05% | 99.81% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 100.00% | 96.35% | 92.05% | 100.00% |
| [postcard 1.1.1][postcard] | 30.78% | 14.80% | 100.00% | 96.35% | 92.05% | 98.72% |
| [pot 3.0.1][pot] | 0.38% | 0.21% | 59.27% | 76.05% | 71.83% | 10.33% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.90%\**</span> <span title="populate + encode">*1.76%\**</span> | 1.06% | 68.57% | 77.75% | 76.64% | 11.70% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.04%\**</span> <span title="populate + encode">*0.48%\**</span> | 0.51% | 68.57% | 77.75% | 76.64% | 10.62% |
| [rkyv 0.8.10][rkyv] | 74.22% | <span title="unvalidated">*99.95%\**</span> <span title="validated upfront with error">*99.63%\**</span> | 100.00% | 96.35% | 92.05% | 98.93% |
| [ron 0.10.1][ron] | 0.09% | 0.03% | 27.04% | 57.77% | 60.48% | 5.64% |
| [savefile 0.18.6][savefile] | 99.93% | 100.00% | 100.00% | 96.35% | 92.05% | 98.87% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.09% | 3.55% | 100.00% | 96.35% | 92.05% | 98.72% |
| [serde-brief 0.1.1][serde-brief] | 0.81% | 0.42% | 38.10% | 64.58% | 72.23% | 8.97% |
| [serde_bare 0.5.0][serde_bare] | 2.57% | 3.09% | 100.00% | 96.35% | 92.05% | 98.95% |
| [speedy 0.8.7][speedy] | 74.51% | 74.61% | 100.00% | 96.35% | 92.05% | 98.80% |
| [wincode 0.2.4][wincode] | 74.41% | 99.97% | 100.00% | 96.35% | 92.05% | 98.80% |
| [wiring 0.2.4][wiring] | 99.05% | 43.47% | 100.00% | 96.34% | 92.05% | 99.12% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.12%\**</span> | <span title="validated on-demand with error">*2.03%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 5.71% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*50.01%\**</span> <span title="validated upfront with error">*2.81%\**</span> | <span title="unvalidated">*49.97%\**</span> <span title="validated upfront with error">*49.91%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*79.96%\**</span> | <span title="unvalidated">*99.98%\**</span> <span title="validated upfront with error">*99.98%\**</span> | <span title="unvalidated">*100.00%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*24.94%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*99.96%\**</span> | <span title="unvalidated">*79.13%\**</span> |

## `minecraft_savedata`

This data set is composed of Minecraft player saves that contain highly structured data.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*884.53 µs\**</span> <span title="prepend">*796.41 µs\**</span> | 3.1217 ms | 1.6699 ms | 489348 | 281173 | 249360 | 2.6026 ms |
| [bin-proto 0.12.3][bin-proto] | 1.9572 ms | 2.7621 ms | † | 566975 | 239350 | 231475 | 2.4427 ms |
| [bincode 2.0.1][bincode] | 318.19 µs | 1.8228 ms | 773.29 µs | 367413 | 221291 | 206242 | 2.0512 ms |
| [bincode 1.3.3][bincode1] | 595.21 µs | 1.8657 ms | 862.46 µs | 569975 | 240525 | 231884 | 2.4327 ms |
| [bitcode 0.6.6][bitcode] | 127.91 µs | 1.2682 ms | 169.51 µs | 327688 | 200947 | 182040 | 741.99 µs |
| [borsh 1.5.7][borsh] | 548.73 µs | 1.7976 ms | † | 446595 | 234236 | 209834 | 2.1687 ms |
| [capnp 0.23.2][capnp] | 453.09 µs | † | † | 803896 | 335606 | 280744 | 3.5863 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 766.64 µs | 4.5966 ms | 3.3959 ms | 1109831 | 344745 | 274333 | 3.4529 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.6819 ms | 9.9262 ms | † | 1109821 | 344751 | 274345 | 3.4568 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 1.8643 ms | 4.5372 ms | 3.2698 ms | 1109821 | 344751 | 274345 | 3.5584 ms |
| [columnar 0.11.1][columnar] | 280.98 µs | 1.9386 ms <span title="copy_from">*787.44 µs\**</span> | † | 563728 | 249696 | 217582 | 1.5990 ms |
| [databuf 0.5.0][databuf] | 295.17 µs | 1.7565 ms | 777.67 µs | 356311 | 213062 | 198403 | 1.9252 ms |
| [dlhn 0.1.7][dlhn] | 682.16 µs | 2.6240 ms | † | 366496 | 220600 | 205586 | 2.0090 ms |
| [flatbuffers 25.12.19][flatbuffers] | 3.2428 ms | † | † | 849472 | 347816 | 294871 | 3.5392 ms |
| [flexbuffers 25.2.10][flexbuffers] | 7.9632 ms | 7.0675 ms | 5.9202 ms | 1187688 | 557642 | 553730 | 6.2447 ms |
| json:<br> [flexon 0.4.5][flexon] | 2.7798 ms | 4.6042 ms | † | 1623191 | 466527 | 359157 | 5.6883 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 3.6806 ms | 7.1423 ms | † | 1623191 | 466527 | 359157 | 5.6614 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 2.2118 ms | 4.6154 ms | † | 1623191 | 466527 | 359157 | 5.6891 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 714.98 µs | 2.8818 ms | † | 391251 | 236877 | 220395 | 2.2938 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 1.4274 ms | 3.0124 ms | 1.6631 ms | 424533 | 245214 | 226077 | 2.2779 ms |
| [minicbor 1.0.0][minicbor] | 536.64 µs | 3.4052 ms | 1.8810 ms | 428773 | 249857 | 228630 | 2.3402 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 5.0679 ms | 3.9253 ms | 2.7571 ms | 449745 | 252432 | 230965 | 2.2880 ms |
| [nanoserde 0.2.1][nanoserde] | 264.54 µs | 1.8915 ms | † | 567975 | 239930 | 231872 | 2.4594 ms |
| [nibblecode 0.1.0][nibblecode] | 178.79 µs | † | † | 603928 | 429716 | 405134 | 3.6265 ms |
| [postcard 1.1.1][postcard] | 444.26 µs | 2.0703 ms | 810.94 µs | 367489 | 221913 | 207244 | 2.1283 ms |
| [pot 3.0.1][pot] | 2.4181 ms | 6.2366 ms | 5.1548 ms | 599125 | 299158 | 247675 | 3.0810 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*1.2513 ms\**</span> <span title="populate + encode">*2.9762 ms\**</span> | 3.5725 ms | † | 596811 | 305319 | 268737 | 3.0493 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*1.0462 ms\**</span> <span title="populate + encode">*3.0167 ms\**</span> | 3.9119 ms | † | 596811 | 305319 | 268737 | 3.0133 ms |
| [rkyv 0.8.10][rkyv] | 329.27 µs | <span title="unvalidated">*1.5266 ms\**</span> <span title="validated upfront with error">*1.8674 ms\**</span> | † | 603776 | 254776 | 219421 | 2.3235 ms |
| [ron 0.10.1][ron] | 8.0567 ms | 24.718 ms | 23.817 ms | 1465223 | 434935 | 342907 | 5.5589 ms |
| [savefile 0.18.6][savefile] | 217.17 µs | 1.8005 ms | † | 566991 | 239362 | 231478 | 2.4666 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 614.22 µs | 2.1141 ms | † | 356311 | 212976 | 198423 | 1.9562 ms |
| [serde-brief 0.1.1][serde-brief] | 1.2643 ms | 5.3205 ms | 3.5200 ms | 1276014 | 373898 | 293384 | 3.6773 ms |
| [serde_bare 0.5.0][serde_bare] | 758.62 µs | 2.3649 ms | † | 356311 | 213062 | 198403 | 1.9646 ms |
| [speedy 0.8.7][speedy] | 269.96 µs | 1.7021 ms | 529.31 µs | 449595 | 234970 | 210192 | 2.0594 ms |
| [wincode 0.2.4][wincode] | 204.23 µs | 1.6760 ms | 629.21 µs | 566975 | 239350 | 231475 | 2.4410 ms |
| [wiring 0.2.4][wiring] | 210.47 µs | 1.9037 ms | † | 566975 | 247810 | 225086 | 2.4985 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*79.096 ns\**</span> | <span title="validated on-demand with error">*572.87 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 893.88 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4916 ns\**</span> <span title="validated upfront with error">*2.1778 ms\**</span> | <span title="unvalidated">*1.4288 µs\**</span> <span title="validated upfront with error">*2.1786 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2442 ns\**</span> <span title="validated upfront with error">*259.37 µs\**</span> | <span title="unvalidated">*156.27 ns\**</span> <span title="validated upfront with error">*254.20 µs\**</span> | <span title="unvalidated">*784.84 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2446 ns\**</span> <span title="validated upfront with error">*336.55 µs\**</span> | <span title="unvalidated">*156.15 ns\**</span> <span title="validated upfront with error">*343.15 µs\**</span> | <span title="unvalidated">*754.30 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Borrow | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*14.46%\**</span> <span title="prepend">*16.06%\**</span> | 25.22% | 10.15% | 66.96% | 71.47% | 73.00% | 28.51% |
| [bin-proto 0.12.3][bin-proto] | 6.54% | 28.51% | † | 57.80% | 83.96% | 78.64% | 30.38% |
| [bincode 2.0.1][bincode] | 40.20% | 43.20% | 21.92% | 89.19% | 90.81% | 88.27% | 36.17% |
| [bincode 1.3.3][bincode1] | 21.49% | 42.21% | 19.65% | 57.49% | 83.55% | 78.50% | 30.50% |
| [bitcode 0.6.6][bitcode] | 100.00% | 62.09% | 100.00% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 23.31% | 43.81% | † | 73.37% | 85.79% | 86.75% | 34.21% |
| [capnp 0.23.2][capnp] | 28.23% | † | † | 40.76% | 59.88% | 64.84% | 20.69% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 16.68% | 17.13% | 4.99% | 29.53% | 58.29% | 66.36% | 21.49% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 3.47% | 7.93% | † | 29.53% | 58.29% | 66.35% | 21.46% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 6.86% | 17.36% | 5.18% | 29.53% | 58.29% | 66.35% | 20.85% |
| [columnar 0.11.1][columnar] | 45.52% | 40.62% <span title="copy_from">*100.00%\**</span> | † | 58.13% | 80.48% | 83.67% | 46.40% |
| [databuf 0.5.0][databuf] | 43.33% | 44.83% | 21.80% | 91.97% | 94.31% | 91.75% | 38.54% |
| [dlhn 0.1.7][dlhn] | 18.75% | 30.01% | † | 89.41% | 91.09% | 88.55% | 36.93% |
| [flatbuffers 25.12.19][flatbuffers] | 3.94% | † | † | 38.58% | 57.77% | 61.74% | 20.96% |
| [flexbuffers 25.2.10][flexbuffers] | 1.61% | 11.14% | 2.86% | 27.59% | 36.04% | 32.88% | 11.88% |
| json:<br> [flexon 0.4.5][flexon] | 4.60% | 17.10% | † | 20.19% | 43.07% | 50.69% | 13.04% |
| json:<br> [serde_json 1.0.140][serde_json] | 3.48% | 11.03% | † | 20.19% | 43.07% | 50.69% | 13.11% |
| json:<br> [simd-json 0.15.1][simd-json] | 5.78% | 17.06% | † | 20.19% | 43.07% | 50.69% | 13.04% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 17.89% | 27.32% | † | 83.75% | 84.83% | 82.60% | 32.35% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 8.96% | 26.14% | 10.19% | 77.19% | 81.95% | 80.52% | 32.57% |
| [minicbor 1.0.0][minicbor] | 23.84% | 23.12% | 9.01% | 76.42% | 80.42% | 79.62% | 31.71% |
| [nachricht-serde 0.4.0][nachricht-serde] | 2.52% | 20.06% | 6.15% | 72.86% | 79.60% | 78.82% | 32.43% |
| [nanoserde 0.2.1][nanoserde] | 48.35% | 41.63% | † | 57.69% | 83.75% | 78.51% | 30.17% |
| [nibblecode 0.1.0][nibblecode] | 71.54% | † | † | 54.26% | 46.76% | 44.93% | 20.46% |
| [postcard 1.1.1][postcard] | 28.79% | 38.04% | 20.90% | 89.17% | 90.55% | 87.84% | 34.86% |
| [pot 3.0.1][pot] | 5.29% | 12.63% | 3.29% | 54.69% | 67.17% | 73.50% | 24.08% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*10.22%\**</span> <span title="populate + encode">*4.30%\**</span> | 22.04% | † | 54.91% | 65.82% | 67.74% | 24.33% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*12.23%\**</span> <span title="populate + encode">*4.24%\**</span> | 20.13% | † | 54.91% | 65.82% | 67.74% | 24.62% |
| [rkyv 0.8.10][rkyv] | 38.85% | <span title="unvalidated">*51.58%\**</span> <span title="validated upfront with error">*42.17%\**</span> | † | 54.27% | 78.87% | 82.96% | 31.93% |
| [ron 0.10.1][ron] | 1.59% | 3.19% | 0.71% | 22.36% | 46.20% | 53.09% | 13.35% |
| [savefile 0.18.6][savefile] | 58.90% | 43.73% | † | 57.79% | 83.95% | 78.64% | 30.08% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 20.82% | 37.25% | † | 91.97% | 94.35% | 91.74% | 37.93% |
| [serde-brief 0.1.1][serde-brief] | 10.12% | 14.80% | 4.82% | 25.68% | 53.74% | 62.05% | 20.18% |
| [serde_bare 0.5.0][serde_bare] | 16.86% | 33.30% | † | 91.97% | 94.31% | 91.75% | 37.77% |
| [speedy 0.8.7][speedy] | 47.38% | 46.26% | 32.02% | 72.89% | 85.52% | 86.61% | 36.03% |
| [wincode 0.2.4][wincode] | 62.63% | 46.98% | 26.94% | 57.80% | 83.96% | 78.64% | 30.40% |
| [wiring 0.2.4][wiring] | 60.77% | 41.36% | † | 57.80% | 81.09% | 80.88% | 29.70% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.57%\**</span> | <span title="validated on-demand with error">*27.26%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 0.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.94%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*10.93%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*99.92%\**</span> <span title="validated upfront with error">*0.06%\**</span> | <span title="unvalidated">*96.11%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.97%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.05%\**</span> | <span title="unvalidated">*100.00%\**</span> |

## `mk48`

This data set is composed of mk48.io game updates that contain data with many exploitable patterns and invariants.

### Raw data

For operations, time per iteration; for size, bytes. Lower is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*4.5329 ms\**</span> <span title="prepend">*2.5466 ms\**</span> | 8.5191 ms | 1704643 | 1294259 | 1245668 | 11.774 ms |
| [bin-proto 0.12.3][bin-proto] | 5.6974 ms | 6.2688 ms | 1791489 | 1127998 | 1051146 | 10.239 ms |
| [bincode 2.0.1][bincode] | 1.4648 ms | 3.6506 ms | 1406257 | 1117802 | 1062438 | 9.5996 ms |
| [bincode 1.3.3][bincode1] | 3.8415 ms | 4.3172 ms | 1854234 | 1141994 | 1048745 | 10.339 ms |
| [bitcode 0.6.6][bitcode] | 713.13 µs | 2.3808 ms | 971318 | 878034 | 850340 | 2.9043 ms |
| [borsh 1.5.7][borsh] | 2.7777 ms | 2.9217 ms | 1521989 | 1108471 | 1038528 | 9.9448 ms |
| [capnp 0.23.2][capnp] | 2.1973 ms | † | 2724288 | 1546992 | 1239111 | 14.466 ms |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 3.2175 ms | 18.074 ms | 6012539 | 1695215 | 1464951 | 21.237 ms |
| cbor:<br> [ciborium 0.2.2][ciborium] | 23.449 ms | 52.126 ms | 6012373 | 1695146 | 1465025 | 21.703 ms |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 9.7769 ms | 20.777 ms | 6012373 | 1695146 | 1465025 | 21.263 ms |
| [columnar 0.11.1][columnar] | 920.54 µs | 3.7392 ms <span title="copy_from">*1.2101 ms\**</span> | 1544752 | 996728 | 897073 | 4.7150 ms |
| [databuf 0.5.0][databuf] | 1.3118 ms | 3.7341 ms | 1319999 | 1062631 | 1008334 | 8.8978 ms |
| [dlhn 0.1.7][dlhn] | 4.2567 ms | 7.4056 ms | 1311281 | 1077520 | 1046095 | 8.7042 ms |
| [flatbuffers 25.12.19][flatbuffers] | 4.9992 ms | † | 2325620 | 1439185 | 1268060 | 13.473 ms |
| [flexbuffers 25.2.10][flexbuffers] | 41.512 ms | 36.989 ms | 5352680 | 2658295 | 2777967 | 35.377 ms |
| json:<br> [flexon 0.4.5][flexon] | 15.153 ms | 25.253 ms | 9390461 | 2391679 | 1842767 | 35.167 ms |
| json:<br> [serde_json 1.0.140][serde_json] | 20.124 ms | 31.707 ms | 9390461 | 2391679 | 1842767 | 34.856 ms |
| json:<br> [simd-json 0.15.1][simd-json] | 12.170 ms | 25.548 ms | 9390461 | 2391679 | 1842767 | 34.931 ms |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 1.7591 ms | 6.1488 ms | 1458773 | 1156055 | 1137788 | 9.7765 ms |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 9.9050 ms | 10.944 ms | 1745322 | 1261627 | 1228923 | 11.705 ms |
| [minicbor 1.0.0][minicbor] | 2.0955 ms | 11.343 ms | 1777386 | 1276218 | 1252558 | 12.626 ms |
| [nachricht-serde 0.4.0][nachricht-serde] | 29.816 ms | 16.698 ms | 1770060 | 1277755 | 1263362 | 12.754 ms |
| [nanoserde 0.2.1][nanoserde] | 1.2956 ms | 2.7850 ms | 1812404 | 1134820 | 1053109 | 10.652 ms |
| [nibblecode 0.1.0][nibblecode] | 509.39 µs | † | 2075936 | 1543244 | 1439095 | 13.993 ms |
| [postcard 1.1.1][postcard] | 1.8067 ms | 4.2414 ms | 1311281 | 1083900 | 1041434 | 8.8085 ms |
| [pot 3.0.1][pot] | 14.584 ms | 30.428 ms | 2604812 | 1482233 | 1298928 | 16.632 ms |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*5.4229 ms\**</span> <span title="populate + encode">*9.3460 ms\**</span> | 9.1804 ms | 1859886 | 1338076 | 1295351 | 12.890 ms |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*5.9830 ms\**</span> <span title="populate + encode">*13.361 ms\**</span> | 12.219 ms | 1859886 | 1338076 | 1295351 | 12.412 ms |
| [rkyv 0.8.10][rkyv] | 979.94 µs | <span title="unvalidated">*2.1939 ms\**</span> <span title="validated upfront with error">*2.6287 ms\**</span> | 2075936 | 1383779 | 1210377 | 13.181 ms |
| [ron 0.10.1][ron] | 41.199 ms | 153.18 ms | 8677703 | 2233642 | 1826180 | 34.586 ms |
| [savefile 0.18.6][savefile] | 869.11 µs | 2.5820 ms | 1791505 | 1128012 | 1051153 | 10.390 ms |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 3.1437 ms | 3.5316 ms | 1319999 | 1064380 | 1010708 | 9.2230 ms |
| [serde-brief 0.1.1][serde-brief] | 6.1458 ms | 22.349 ms | 6951772 | 1796265 | 1567819 | 23.531 ms |
| [serde_bare 0.5.0][serde_bare] | 4.9039 ms | 5.0737 ms | 1319999 | 1062645 | 1008349 | 9.0142 ms |
| [speedy 0.8.7][speedy] | 744.49 µs | 2.4751 ms | 1584734 | 1119837 | 1037992 | 10.532 ms |
| [wincode 0.2.4][wincode] | 589.17 µs | 2.3813 ms | 1791489 | 1127998 | 1051146 | 10.433 ms |
| [wiring 0.2.4][wiring] | 649.27 µs | 2.7616 ms | 1791489 | 1156963 | 1082815 | 10.582 ms |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*88.364 ns\**</span> | <span title="validated on-demand with error">*717.74 ns\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 58.202 ns | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*2.4887 ns\**</span> <span title="validated upfront with error">*5.6853 ms\**</span> | <span title="unvalidated">*2.7411 µs\**</span> <span title="validated upfront with error">*5.6869 ms\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*1.2440 ns\**</span> <span title="validated upfront with error">*367.65 µs\**</span> | <span title="unvalidated">*378.66 ns\**</span> <span title="validated upfront with error">*367.30 µs\**</span> | <span title="unvalidated">*238.00 ns\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*1.2445 ns\**</span> <span title="validated upfront with error">*429.62 µs\**</span> | <span title="unvalidated">*385.22 ns\**</span> <span title="validated upfront with error">*426.38 µs\**</span> | <span title="unvalidated">*236.62 ns\**</span> |

### Comparison

Relative to best. Higher is better.

#### Serialize / deserialize speed and size

| Crate | Serialize | Deserialize | Size | Zlib | Zstd | Zstd Time |
|---|--:|--:|--:|--:|--:|--:|
| [bilrost 0.1013.0][bilrost] | <span title="encode">*11.24%\**</span> <span title="prepend">*20.00%\**</span> | 14.20% | 56.98% | 67.84% | 68.26% | 24.67% |
| [bin-proto 0.12.3][bin-proto] | 8.94% | 19.30% | 54.22% | 77.84% | 80.90% | 28.36% |
| [bincode 2.0.1][bincode] | 34.78% | 33.15% | 69.07% | 78.55% | 80.04% | 30.25% |
| [bincode 1.3.3][bincode1] | 13.26% | 28.03% | 52.38% | 76.89% | 81.08% | 28.09% |
| [bitcode 0.6.6][bitcode] | 71.43% | 50.83% | 100.00% | 100.00% | 100.00% | 100.00% |
| [borsh 1.5.7][borsh] | 18.34% | 41.42% | 63.82% | 79.21% | 81.88% | 29.20% |
| [capnp 0.23.2][capnp] | 23.18% | † | 35.65% | 56.76% | 68.63% | 20.08% |
| cbor:<br> [cbor4ii 1.0.0][cbor4ii] | 15.83% | 6.70% | 16.15% | 51.79% | 58.05% | 13.68% |
| cbor:<br> [ciborium 0.2.2][ciborium] | 2.17% | 2.32% | 16.16% | 51.80% | 58.04% | 13.38% |
| cbor:<br> [serde_cbor 0.11.2][serde_cbor] | 5.21% | 5.82% | 16.16% | 51.80% | 58.04% | 13.66% |
| [columnar 0.11.1][columnar] | 55.34% | 32.36% <span title="copy_from">*100.00%\**</span> | 62.88% | 88.09% | 94.79% | 61.60% |
| [databuf 0.5.0][databuf] | 38.83% | 32.41% | 73.58% | 82.63% | 84.33% | 32.64% |
| [dlhn 0.1.7][dlhn] | 11.97% | 16.34% | 74.07% | 81.49% | 81.29% | 33.37% |
| [flatbuffers 25.12.19][flatbuffers] | 10.19% | † | 41.77% | 61.01% | 67.06% | 21.56% |
| [flexbuffers 25.2.10][flexbuffers] | 1.23% | 3.27% | 18.15% | 33.03% | 30.61% | 8.21% |
| json:<br> [flexon 0.4.5][flexon] | 3.36% | 4.79% | 10.34% | 36.71% | 46.14% | 8.26% |
| json:<br> [serde_json 1.0.140][serde_json] | 2.53% | 3.82% | 10.34% | 36.71% | 46.14% | 8.33% |
| json:<br> [simd-json 0.15.1][simd-json] | 4.19% | 4.74% | 10.34% | 36.71% | 46.14% | 8.31% |
| messagepack:<br> [msgpacker 0.4.8][msgpacker] | 28.96% | 19.68% | 66.58% | 75.95% | 74.74% | 29.71% |
| messagepack:<br> [rmp-serde 1.3.0][rmp-serde] | 5.14% | 11.06% | 55.65% | 69.60% | 69.19% | 24.81% |
| [minicbor 1.0.0][minicbor] | 24.31% | 10.67% | 54.65% | 68.80% | 67.89% | 23.00% |
| [nachricht-serde 0.4.0][nachricht-serde] | 1.71% | 7.25% | 54.87% | 68.72% | 67.31% | 22.77% |
| [nanoserde 0.2.1][nanoserde] | 39.32% | 43.45% | 53.59% | 77.37% | 80.75% | 27.27% |
| [nibblecode 0.1.0][nibblecode] | 100.00% | † | 46.79% | 56.90% | 59.09% | 20.75% |
| [postcard 1.1.1][postcard] | 28.19% | 28.53% | 74.07% | 81.01% | 81.65% | 32.97% |
| [pot 3.0.1][pot] | 3.49% | 3.98% | 37.29% | 59.24% | 65.46% | 17.46% |
| protobuf:<br> [prost 0.14.1][prost] | <span title="encode">*9.39%\**</span> <span title="populate + encode">*5.45%\**</span> | 13.18% | 52.22% | 65.62% | 65.65% | 22.53% |
| protobuf:<br> [protobuf 3.7.2][protobuf] | <span title="encode">*8.51%\**</span> <span title="populate + encode">*3.81%\**</span> | 9.90% | 52.22% | 65.62% | 65.65% | 23.40% |
| [rkyv 0.8.10][rkyv] | 51.98% | <span title="unvalidated">*55.16%\**</span> <span title="validated upfront with error">*46.03%\**</span> | 46.79% | 63.45% | 70.25% | 22.03% |
| [ron 0.10.1][ron] | 1.24% | 0.79% | 11.19% | 39.31% | 46.56% | 8.40% |
| [savefile 0.18.6][savefile] | 58.61% | 46.87% | 54.22% | 77.84% | 80.90% | 27.95% |
| scale:<br> [parity-scale-codec 3.7.5][parity-scale-codec] | 16.20% | 34.26% | 73.58% | 82.49% | 84.13% | 31.49% |
| [serde-brief 0.1.1][serde-brief] | 8.29% | 5.41% | 13.97% | 48.88% | 54.24% | 12.34% |
| [serde_bare 0.5.0][serde_bare] | 10.39% | 23.85% | 73.58% | 82.63% | 84.33% | 32.22% |
| [speedy 0.8.7][speedy] | 68.42% | 48.89% | 61.29% | 78.41% | 81.92% | 27.58% |
| [wincode 0.2.4][wincode] | 86.46% | 50.82% | 54.22% | 77.84% | 80.90% | 27.84% |
| [wiring 0.2.4][wiring] | 78.46% | 43.82% | 54.22% | 75.89% | 78.53% | 27.44% |

#### Zero-copy deserialization speed

| Crate | Access | Read | Update |
|---|--:|--:|--:|
| [capnp 0.23.2][capnp] | <span title="validated on-demand with error">*1.41%\**</span> | <span title="validated on-demand with error">*52.76%\**</span> | ‡ |
| [columnar 0.11.1][columnar] | 2.14% | ‡ | ‡ |
| [flatbuffers 25.12.19][flatbuffers] | <span title="unvalidated">*49.99%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*13.81%\**</span> <span title="validated upfront with error">*0.01%\**</span> | ‡ |
| [nibblecode 0.1.0][nibblecode] | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*100.00%\**</span> <span title="validated upfront with error">*0.10%\**</span> | <span title="unvalidated">*99.42%\**</span> |
| [rkyv 0.8.10][rkyv] | <span title="unvalidated">*99.96%\**</span> <span title="validated upfront with error">*0.00%\**</span> | <span title="unvalidated">*98.30%\**</span> <span title="validated upfront with error">*0.09%\**</span> | <span title="unvalidated">*100.00%\**</span> |

[bilrost]: https://crates.io/crates/bilrost/0.1013.0
[bin-proto]: https://crates.io/crates/bin-proto/0.12.3
[bincode]: https://crates.io/crates/bincode/2.0.1
[bincode1]: https://crates.io/crates/bincode/1.3.3
[bitcode]: https://crates.io/crates/bitcode/0.6.6
[borsh]: https://crates.io/crates/borsh/1.5.7
[capnp]: https://crates.io/crates/capnp/0.23.2
[cbor4ii]: https://crates.io/crates/cbor4ii/1.0.0
[ciborium]: https://crates.io/crates/ciborium/0.2.2
[columnar]: https://crates.io/crates/columnar/0.11.1
[databuf]: https://crates.io/crates/databuf/0.5.0
[dlhn]: https://crates.io/crates/dlhn/0.1.7
[flatbuffers]: https://crates.io/crates/flatbuffers/25.12.19
[flexbuffers]: https://crates.io/crates/flexbuffers/25.2.10
[flexon]: https://crates.io/crates/flexon/0.4.5
[minicbor]: https://crates.io/crates/minicbor/1.0.0
[msgpacker]: https://crates.io/crates/msgpacker/0.4.8
[nachricht-serde]: https://crates.io/crates/nachricht-serde/0.4.0
[nanoserde]: https://crates.io/crates/nanoserde/0.2.1
[nibblecode]: https://crates.io/crates/nibblecode/0.1.0
[parity-scale-codec]: https://crates.io/crates/parity-scale-codec/3.7.5
[postcard]: https://crates.io/crates/postcard/1.1.1
[pot]: https://crates.io/crates/pot/3.0.1
[prost]: https://crates.io/crates/prost/0.14.1
[protobuf]: https://crates.io/crates/protobuf/3.7.2
[rkyv]: https://crates.io/crates/rkyv/0.8.10
[rmp-serde]: https://crates.io/crates/rmp-serde/1.3.0
[ron]: https://crates.io/crates/ron/0.10.1
[savefile]: https://crates.io/crates/savefile/0.18.6
[serde-brief]: https://crates.io/crates/serde-brief/0.1.1
[serde_bare]: https://crates.io/crates/serde_bare/0.5.0
[serde_cbor]: https://crates.io/crates/serde_cbor/0.11.2
[serde_json]: https://crates.io/crates/serde_json/1.0.140
[simd-json]: https://crates.io/crates/simd-json/0.15.1
[speedy]: https://crates.io/crates/speedy/0.8.7
[wincode]: https://crates.io/crates/wincode/0.2.4
[wiring]: https://crates.io/crates/wiring/0.2.4


## Footnotes:

\* *mouse over for situational details*

† *this deserialization capability is not supported*

‡ *buffer mutation is not supported (`capnp` and `flatbuffers` may but not for rust)*
