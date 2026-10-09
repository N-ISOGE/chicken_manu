# 참고 자료: 표준과 기술

메타데이터 표준, IPFS와 P2P, Rust 크레이트와 표준, 인프라와 라이선스의 링크다. 2026-10-08 조사에서 확인한 자료이고, 확인 상태는 [첫 페이지](crate)의 표기를 따른다.
조사 중 정정된 사실은 [기술 채택 판단](crate::docs::adoption)에 있다.

## 1. METS와 메타데이터

| 자료                                                                                                                                                                      | 상태 | 메모                                                                                                                                       |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| [METS 홈](https://www.loc.gov/standards/mets/)                                                                                                                            | 확인 | 원본 기록은 mets-home.html 주소를 적었다. METS 2가 2025년에 나왔다                                                                         |
| [mets.xsd 1.12.1](https://www.loc.gov/standards/mets/mets.xsd)                                                                                                            | 확인 | `CHECKSUMTYPE`은 닫힌 열거형이고 SHA-3이 없다                                                                                              |
| [METS 2 스키마](https://raw.githubusercontent.com/mets/METS-schema/main/v2/mets2.xsd)                                                                                     | 확인 | `CHECKSUMTYPE`은 열거형이 아닌 자유 문자열이다(원문 다운로드로 확인)                                                                       |
| [METS 2 제안 값 위키](https://github.com/mets/METS-schema/wiki/METS2-Suggested-Attribute-Values)                                                                          | 확인 | 11개 값뿐이고 SHA-3·BLAKE3 없음. "다른 값은 각 profile이 해석한다"                                                                         |
| [METS 2 문서](https://mets.github.io)                                                                                                                                     | 부분 | 가이드, 마이그레이션 도구 안내. profile 문서는 읽지 못함                                                                                   |
| [METS Profiles](https://www.loc.gov/standards/mets/mets-profiles.html)                                                                                                    | 확인 | 자체 스키마를 profile로 정의하는 경로                                                                                                      |
| [PREMIS 3.0](https://www.loc.gov/standards/premis/)                                                                                                                       | 확인 | fixity 위치. [해시 함수 어휘](https://id.loc.gov/vocabulary/preservation/cryptographicHashFunctions.rdf.xml)는 13개 항이고 SHA3·BLAKE 없음 |
| [MODS](https://www.loc.gov/standards/mods/) · [DCMI Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) · [MIX](https://www.loc.gov/standards/mix/) | 확인 | 제목·아티스트, 이미지 기술 메타데이터 매핑 후보                                                                                            |
| [XML 1.0](https://www.w3.org/TR/xml/) · [XML Schema 1.1](https://www.w3.org/TR/xmlschema11-1/) · [XLink 1.1](https://www.w3.org/TR/xlink11/)                              | 확인 | UTF-8은 필수 지원, 그 외 인코딩은 선언이 필요                                                                                              |

## 2. IPFS와 P2P

| 자료                                                                                                                                                                                                   | 상태 | 메모                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---- | ------------------------------------------------------------- |
| [IPFS 문서](https://docs.ipfs.tech/)                                                                                                                                                                   | 확인 | 원본 기록이 "배포 및 공유 기반"으로 지정                      |
| [콘텐츠 주소 지정과 CID](https://docs.ipfs.tech/concepts/content-addressing/)                                                                                                                          | 확인 | 청크·레이아웃·CID 버전·해시가 다르면 같은 파일도 CID가 다르다 |
| [UnixFS 명세](https://specs.ipfs.tech/unixfs/) · [개념](https://docs.ipfs.tech/concepts/file-systems/)                                                                                                 | 확인 | 단일 블록은 raw(0x55), 그 외 dag-pb(0x70)                     |
| [CID 명세](https://specs.ipfs.tech/cid/)                                                                                                                                                               | 확인 | 원본 `multiformats/cid`는 archived                            |
| [IPIP-0499 (unixfs-v1-2025)](https://specs.ipfs.tech/ipips/ipip-0499/)                                                                                                                                 | 확인 | 재현 가능한 CID 프로파일. CC0                                 |
| [Kubo CLI](https://docs.ipfs.tech/reference/kubo/cli/) · [config.md](https://github.com/ipfs/kubo/blob/master/docs/config.md)                                                                          | 확인 | 기본값 CIDv0, `size-262144`, 링크 174                         |
| [Trustless Gateway 명세](https://specs.ipfs.tech/http-gateways/trustless-gateway/)                                                                                                                     | 확인 | Bitswap 없이 블록·CAR를 가져오는 경로                         |
| [Bitswap 명세](https://specs.ipfs.tech/bitswap-protocol/)                                                                                                                                              | 확인 | 프로토콜 ID 1.0.0, 1.1.0, 1.2.0                               |
| [multihash](https://github.com/multiformats/multihash) · [multicodec](https://github.com/multiformats/multicodec)                                                                                      | 부분 | sha2-256=0x12, sha3-256=0x16, blake3=0x1e(요약 도구 결과)     |
| Rust: [cid](https://docs.rs/cid/latest/cid/) · [rust-unixfs](https://docs.rs/rust-unixfs/latest/rust_unixfs/) · [multihash-codetable](https://docs.rs/multihash-codetable/latest/multihash_codetable/) | 확인 | kubo 없이 CID를 계산할 후보. kubo와의 동등성은 검증되지 않음  |
| [rust-ipfs](https://crates.io/crates/rust-ipfs) · [저장소](https://github.com/dariusc93/rust-ipfs)                                                                                                     | 확인 | 원본 기록이 지목한 구현. 0.16.0(2026-07-04), 자체 표기 alpha  |
| [rust-libp2p](https://github.com/libp2p/rust-libp2p)                                                                                                                                                   | 확인 | 0.57.0(2026-09-11). Bitswap 기능 없음                         |
| [beetswap](https://github.com/celestiaorg/beetswap)                                                                                                                                                    | 확인 | Rust Bitswap, 0.5.0(2025-09-19), kubo 호환 증거 없음          |

## 3. iroh

원본 기록이 지목한 [iroh](https://iroh.computer/docs)와 [iroh-rust](https://crates.io/crates/iroh)에 대한 조사 결과다.

| 자료                                                           | 상태 | 메모                                        |
| -------------------------------------------------------------- | ---- | ------------------------------------------- |
| [iroh 문서](https://docs.iroh.computer/)                       | 확인 | QUIC, 공개키(endpoint ID) 접속, 릴레이 폴백 |
| [iroh 1.0 글](https://iroh.computer/blog/the-road-to-iroh-1-0) | 확인 | 1.0 안정화, IPFS 호환 포기                  |
| [iroh 크레이트](https://docs.rs/iroh/latest/iroh/)             | 확인 | 1.3.0(2026-09-28). MSRV 1.91                |
| [Blobs 프로토콜](https://docs.iroh.computer/protocols/blobs)   | 확인 | 식별자는 BLAKE3. CID와 호환되지 않음        |
| [iroh-blobs](https://docs.rs/iroh-blobs/latest/iroh_blobs/)    | 확인 | 0.103.1(2026-10-06), "프로덕션 품질 아님"   |

## 4. Rust 크레이트와 표준

| 자료                                                                                                                                    | 상태 | 메모                                                        |
| --------------------------------------------------------------------------------------------------------------------------------------- | ---- | ----------------------------------------------------------- |
| [encoding_rs](https://docs.rs/encoding_rs/latest/encoding_rs/struct.Encoding.html)                                                      | 확인 | `decode`의 bool은 오류를 U+FFFD로 대체했는지 여부           |
| [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/)                                                                           | 부분 | Shift_JIS 라벨과 디코더. 앞 100k자만 읽음                   |
| [chardetng](https://docs.rs/chardetng/latest/chardetng/struct.EncodingDetector.html)                                                    | 확인 | `encoding_rs::Encoding`을 돌려줘 바로 연결됨                |
| [`regex`](https://docs.rs/regex/latest/regex/)                                                                                          | 확인 | 1.13.1. 한 번 컴파일해 재사용                               |
| [sha3](https://docs.rs/sha3/latest/sha3/)                                                                                               | 확인 | 0.12.0이 최신. 0.11.0-pre.4는 yank되진 않았지만 구버전      |
| [FIPS 202](https://csrc.nist.gov/pubs/fips/202/final)                                                                                   | 부분 | SHA-3 표준. 랜딩 페이지만 확인                              |
| [magic](https://docs.rs/magic/latest/magic/) · [`README`](https://github.com/robo9k/rust-magic/blob/main/README-crate.md)               | 확인 | 0.16.7. Windows는 vcpkg로 libmagic이 필요                   |
| [libmagic(3)](https://man7.org/linux/man-pages/man3/libmagic.3.html) · [file(1)](https://man7.org/linux/man-pages/man1/file.1.html)     | 확인 | DB 경로는 `MAGIC` 환경변수                                  |
| [RFC 6838](https://www.rfc-editor.org/rfc/rfc6838) · [IANA 미디어 타입](https://www.iana.org/assignments/media-types/media-types.xhtml) | 부분 | WAV 항목은 확인하지 못함                                    |
| [RFC 5234 (ABNF)](https://www.rfc-editor.org/rfc/rfc5234)                                                                               | 확인 | 문법 표기법. 따옴표 문자열은 기본적으로 대소문자 무시       |
| [Cargo Book: config](https://doc.rust-lang.org/cargo/reference/config.html)                                                             | 확인 | `[env]`는 `.cargo/config.toml` 기능이고 매니페스트에는 없다 |
| [rustdoc: 문서 쓰기](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)                                                 | 확인 | `///`, `//!`, `#[doc = include_str!]`                       |

## 5. 인프라와 라이선스

| 자료                                                                                                                                                | 상태   | 메모                                                |
| --------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------- |
| [super-linter v8.7.0](https://github.com/super-linter/super-linter/tree/v8.7.0)                                                                     | 확인   | 저장소 린트 워크플로가 고정한 버전(커밋 SHA로 고정) |
| [GitHub Actions 워크플로 문법](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions)                             | 확인   | 워크플로 파일은 `.github/workflows/`에 있어야 한다  |
| [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) | 확인   |                                                     |
| [Contributor Covenant 2.0](https://www.contributor-covenant.org/version/2/0/code_of_conduct/)                                                       | 확인   |                                                     |
| [CC BY-SA 4.0 법률 조항](https://creativecommons.org/licenses/by-sa/4.0/legalcode)                                                                  | 확인   | README가 문서 기본 라이선스로 지정                  |
| [GNU GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.html)                                                                                           | 미확인 | 서버가 429를 반환해 열지 못함                       |
| [Firebase Studio 전환 공지](https://firebase.google.com/docs/studio/idx-is-firebase-studio)                                                         | 확인   | `.idx/dev.nix`가 속한 IDX가 2027-03-22에 종료 예정  |
| [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) · [SemVer](https://semver.org/)                                                            | 확인   | 기록 문서 형식 참고                                 |
