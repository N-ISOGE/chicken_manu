# 참고 자료: 표준과 기술

[← 길라잡이][project-guide]

## 처음 읽는다면

[1. METS와 메타데이터](#1-mets와-메타데이터)에서 자료 목록을 살펴보실 수 있습니다. 확인 범위는 아래 안내와 각 자료의 메모를 함께 살펴봅니다.

## 목차

- [1. METS와 메타데이터](#1-mets와-메타데이터)
- [2. IPFS와 P2P](#2-ipfs와-p2p)
- [3. iroh](#3-iroh)
- [4. Rust 크레이트와 표준](#4-rust-크레이트와-표준)
- [5. 인프라와 라이선스](#5-인프라와-라이선스)

메타데이터 표준, 공유 기술, Rust 크레이트, 개발 도구와 라이선스의 출처를 안내합니다. 2026-10-08 조사에서 확인한 자료이며, 확인 상태는 [공통 안내의 문서 읽는 법][nav-1]의 표기를 따릅니다.
기술의 비교와 적용 여부는 [기술 채택 판단][nav-2]에서 이어 읽을 수 있습니다.

표의 버전·배포일·확인 상태는 기록된 조사 시점의 정보입니다. 현재 판본에 적용할 때는 원문과 대상 판본을 다시 대조합니다.

## 1. METS와 메타데이터

| 자료                                                                                                                                                                      | 상태 | 메모                                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [METS 홈](https://www.loc.gov/standards/mets/)                                                                                                                            | 확인 | METS의 공식 안내입니다. METS 2는 2025년에 나왔습니다.                                                                                                                       |
| [mets.xsd 1.12.1](https://www.loc.gov/standards/mets/mets.xsd)                                                                                                            | 확인 | `CHECKSUMTYPE`은 닫힌 열거형이며 SHA-3은 포함하지 않습니다.                                                                                                                 |
| [METS 2 스키마](https://raw.githubusercontent.com/mets/METS-schema/main/v2/mets2.xsd)                                                                                     | 확인 | `CHECKSUMTYPE`은 열거형이 아닌 자유 문자열입니다(원문 다운로드로 확인했습니다).                                                                                             |
| [METS 2 제안 값 위키](https://github.com/mets/METS-schema/wiki/METS2-Suggested-Attribute-Values)                                                                          | 확인 | 기록된 제안 값은 11개이며 SHA-3·BLAKE3은 없습니다. "다른 값은 각 profile이 해석한다"고 설명합니다.                                                                          |
| [METS 2 문서](https://mets.github.io)                                                                                                                                     | 부분 | 가이드와 마이그레이션 도구를 안내합니다. profile 문서는 읽지 못했습니다.                                                                                                    |
| [METS Profiles](https://www.loc.gov/standards/mets/mets-profiles.html)                                                                                                    | 확인 | 자체 스키마를 profile로 정의하는 방법을 안내합니다.                                                                                                                         |
| [PREMIS 3.0](https://www.loc.gov/standards/premis/)                                                                                                                       | 확인 | fixity를 다루는 위치를 확인했습니다. [해시 함수 어휘](https://id.loc.gov/vocabulary/preservation/cryptographicHashFunctions.rdf.xml)는 13개 항목이며 SHA3·BLAKE는 없습니다. |
| [MODS](https://www.loc.gov/standards/mods/) · [DCMI Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) · [MIX](https://www.loc.gov/standards/mix/) | 확인 | 제목·아티스트와 이미지 기술 메타데이터를 연결할 후보입니다.                                                                                                                 |
| [XML 1.0](https://www.w3.org/TR/xml/) · [XML Schema 1.1](https://www.w3.org/TR/xmlschema11-1/) · [XLink 1.1](https://www.w3.org/TR/xlink11/)                              | 확인 | UTF-8은 필수 지원 대상이며, 그 외 인코딩은 선언이 필요합니다.                                                                                                               |

## 2. IPFS와 P2P

| 자료                                                                                                                                                                                                   | 상태 | 메모                                                                                               |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---- | -------------------------------------------------------------------------------------------------- |
| [IPFS 문서](https://docs.ipfs.tech/)                                                                                                                                                                   | 확인 | 배포와 공유 기반을 검토할 때 참고하는 문서입니다.                                                  |
| [콘텐츠 주소 지정과 CID](https://docs.ipfs.tech/concepts/content-addressing/)                                                                                                                          | 확인 | 청크·레이아웃·CID 버전·해시가 다르면 같은 파일의 CID도 달라집니다.                                 |
| [UnixFS 명세](https://specs.ipfs.tech/unixfs/) · [개념](https://docs.ipfs.tech/concepts/file-systems/)                                                                                                 | 확인 | 단일 블록은 raw(0x55), 그 외에는 dag-pb(0x70)를 사용합니다.                                        |
| [CID 명세](https://specs.ipfs.tech/cid/)                                                                                                                                                               | 확인 | 원본 `multiformats/cid` 저장소는 archived 상태입니다.                                              |
| [IPIP-0499 (unixfs-v1-2025)](https://specs.ipfs.tech/ipips/ipip-0499/)                                                                                                                                 | 확인 | 재현 가능한 CID 프로파일입니다. 라이선스는 CC0입니다.                                              |
| [Kubo CLI](https://docs.ipfs.tech/reference/kubo/cli/) · [config.md](https://github.com/ipfs/kubo/blob/master/docs/config.md)                                                                          | 확인 | 기록된 기본값은 CIDv0, `size-262144`, 링크 174개입니다.                                            |
| [Trustless Gateway 명세](https://specs.ipfs.tech/http-gateways/trustless-gateway/)                                                                                                                     | 확인 | Bitswap 없이 블록·CAR를 가져오는 방법을 설명합니다.                                                |
| [Bitswap 명세](https://specs.ipfs.tech/bitswap-protocol/)                                                                                                                                              | 확인 | 프로토콜 ID는 1.0.0, 1.1.0, 1.2.0입니다.                                                           |
| [multihash](https://github.com/multiformats/multihash) · [multicodec](https://github.com/multiformats/multicodec)                                                                                      | 부분 | 식별 값은 sha2-256=0x12, sha3-256=0x16, blake3=0x1e로 기록돼 있습니다(요약 도구 결과).             |
| Rust: [cid](https://docs.rs/cid/latest/cid/) · [rust-unixfs](https://docs.rs/rust-unixfs/latest/rust_unixfs/) · [multihash-codetable](https://docs.rs/multihash-codetable/latest/multihash_codetable/) | 확인 | kubo 없이 CID를 계산할 후보입니다. kubo와의 동등성은 검증되지 않았습니다.                          |
| [rust-ipfs](https://crates.io/crates/rust-ipfs) · [저장소](https://github.com/dariusc93/rust-ipfs)                                                                                                     | 확인 | 검토 후보 구현입니다. 기록된 판본은 0.16.0(2026-07-04)이며 자체 표기는 alpha입니다.                |
| [rust-libp2p](https://github.com/libp2p/rust-libp2p)                                                                                                                                                   | 확인 | 기록된 판본은 0.57.0(2026-09-11)이며 Bitswap 기능은 없습니다.                                      |
| [beetswap](https://github.com/celestiaorg/beetswap)                                                                                                                                                    | 확인 | Rust Bitswap 구현이며 기록된 판본은 0.5.0(2025-09-19)입니다. kubo 호환 근거는 확인하지 못했습니다. |

## 3. iroh

[iroh](https://iroh.computer/docs)와 [iroh-rust](https://crates.io/crates/iroh)의 문서와 구현 자료를 모았습니다. IPFS 계열 자료와 구별해 살펴봅니다.

| 자료                                                           | 상태 | 메모                                                                            |
| -------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------- |
| [iroh 문서](https://docs.iroh.computer/)                       | 확인 | QUIC, 공개키(endpoint ID) 기반 접속과 릴레이 폴백을 설명합니다.                 |
| [iroh 1.0 글](https://iroh.computer/blog/the-road-to-iroh-1-0) | 확인 | 1.0 안정화와 IPFS 호환 중단을 설명합니다.                                       |
| [iroh 크레이트](https://docs.rs/iroh/latest/iroh/)             | 확인 | 기록된 판본은 1.3.0(2026-09-28)이며 MSRV는 1.91입니다.                          |
| [Blobs 프로토콜](https://docs.iroh.computer/protocols/blobs)   | 확인 | 식별자는 BLAKE3이며 CID와 호환되지 않습니다.                                    |
| [iroh-blobs](https://docs.rs/iroh-blobs/latest/iroh_blobs/)    | 확인 | 기록된 판본은 0.103.1(2026-10-06)이며 "프로덕션 품질 아님"으로 표시돼 있습니다. |

## 4. Rust 크레이트와 표준

| 자료                                                                                                                                    | 상태 | 메모                                                                                 |
| --------------------------------------------------------------------------------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------------ |
| [encoding_rs](https://docs.rs/encoding_rs/latest/encoding_rs/struct.Encoding.html)                                                      | 확인 | `decode`의 bool은 오류를 U+FFFD로 대체했는지를 나타냅니다.                           |
| [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/)                                                                           | 부분 | Shift_JIS 라벨과 디코더를 다룹니다. 앞 100k자만 읽었습니다.                          |
| [chardetng](https://docs.rs/chardetng/latest/chardetng/struct.EncodingDetector.html)                                                    | 확인 | `encoding_rs::Encoding`을 반환하므로 바로 연결할 수 있습니다.                        |
| [`regex`](https://docs.rs/regex/latest/regex/)                                                                                          | 확인 | 기록된 판본은 1.13.1입니다. 한 번 컴파일한 정규식을 재사용합니다.                    |
| [sha3](https://docs.rs/sha3/latest/sha3/)                                                                                               | 확인 | 조사 시점의 최신 판본은 0.12.0입니다. 0.11.0-pre.4는 yank되지 않았지만 구버전입니다. |
| [FIPS 202](https://csrc.nist.gov/pubs/fips/202/final)                                                                                   | 부분 | SHA-3 표준입니다. 랜딩 페이지만 확인했습니다.                                        |
| [magic](https://docs.rs/magic/latest/magic/) · [`README`](https://github.com/robo9k/rust-magic/blob/main/README-crate.md)               | 확인 | 기록된 판본은 0.16.7입니다. Windows에서는 vcpkg로 제공하는 libmagic이 필요합니다.    |
| [libmagic(3)](https://man7.org/linux/man-pages/man3/libmagic.3.html) · [file(1)](https://man7.org/linux/man-pages/man1/file.1.html)     | 확인 | DB 경로는 `MAGIC` 환경 변수로 지정합니다.                                            |
| [RFC 6838](https://www.rfc-editor.org/rfc/rfc6838) · [IANA 미디어 타입](https://www.iana.org/assignments/media-types/media-types.xhtml) | 부분 | WAV 항목은 확인하지 못했습니다.                                                      |
| [RFC 5234 (ABNF)](https://www.rfc-editor.org/rfc/rfc5234)                                                                               | 확인 | 문법 표기법입니다. 따옴표 문자열은 기본적으로 대소문자를 구별하지 않습니다.          |
| [Cargo Book: config](https://doc.rust-lang.org/cargo/reference/config.html)                                                             | 확인 | `[env]`는 `.cargo/config.toml`의 기능이며 매니페스트에는 없습니다.                   |
| [rustdoc: 문서 쓰기](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)                                                 | 확인 | `///`, `//!`, `#[doc = include_str!]`를 사용하는 문서 작성 방법을 설명합니다.        |

## 5. 인프라와 라이선스

| 자료                                                                                                                                                | 상태   | 메모                                                                    |
| --------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------- |
| [super-linter v8.7.0](https://github.com/super-linter/super-linter/tree/v8.7.0)                                                                     | 확인   | 저장소 린트 워크플로에서 사용하는 버전이며 커밋 SHA로 고정합니다.       |
| [GitHub Actions 워크플로 문법](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions)                             | 확인   | 워크플로 파일은 `.github/workflows/`에 둡니다.                          |
| [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) | 확인   |                                                                         |
| [Contributor Covenant 2.0](https://www.contributor-covenant.org/version/2/0/code_of_conduct/)                                                       | 확인   |                                                                         |
| [CC BY-SA 4.0 법률 조항](https://creativecommons.org/licenses/by-sa/4.0/legalcode)                                                                  | 확인   | README에서 문서의 기본 라이선스로 지정합니다.                           |
| [GNU GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.html)                                                                                           | 미확인 | 서버가 429를 반환해 열지 못했습니다.                                    |
| [Firebase Studio 전환 공지](https://firebase.google.com/docs/studio/idx-is-firebase-studio)                                                         | 확인   | `.idx/dev.nix`가 속한 IDX의 종료 예정일은 2027-03-22로 기록돼 있습니다. |
| [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) · [SemVer](https://semver.org/)                                                            | 확인   | 변경 기록의 형식과 버전 표기를 참고하는 자료입니다.                     |

[nav-1]: ../index.md#문서-읽는-법
[nav-2]: ../../preservation/adoption.md
[project-guide]: ../../index.md
