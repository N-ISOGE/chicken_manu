# 참고 자료

이전 `change_log.md`의 "참고할 것들"과 BMS 관련 도구 메모에, 2026-10-08 조사에서 확인한 자료를 합쳤다.
확인 상태는 [첫 페이지](crate)의 표기를 따른다(확인 / 부분 / 미확인).
조사 중 정정된 사실은 [기술 채택 판단](crate::docs::adoption)에 있다.

이 문서는 링크를 모은 **개요**다. 구동기·파서·규격을 하나씩 세부 문서로 조사하는 방법과 대상 목록은
[기초 조사 지침](crate::docs::briefs::format_survey)에 있다.

## 1. BMS 명세

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [hitkey BMS command memo](https://hitkey.bms.ms/cmds.htm) | 확인 | 사실상 표준. 전체 원문을 받아 확인했다. 최종 갱신 2014-07-11 |
| [hitkey command memo, 원 호스트](https://hitkey.nekokan.dyndns.info/cmds.htm) | 미확인 | 원본 기록이 인용한 주소. 이번에는 위 호스트만 열었다 |
| [hitkey 2014 archive](https://web.archive.org/web/20240505175610/https://hitkey.nekokan.dyndns.info/cmds.htm) | 미확인 | 원본 기록이 "과거 기록, 현재 반영"으로 둔 사본 |
| [BMS Format Specification 요약](https://github.com/MikuroXina/bms-rs/blob/main/SPECIFICATION.md) | 확인 | 1998 원본 사양의 요약. `#WAVxx`·`#BMPxx`는 16진 01-FF, 인코딩 규정 없음 |
| [Be-Music Source (Wikipedia)](https://en.wikipedia.org/wiki/Be-Music_Source) | 확인 | 역사와 확장자 개요 |
| [bmson 명세](https://bmson-spec.readthedocs.io/en/master/doc/index.html) | 확인 | 1.0.0-beta(2015-12-26). Web IDL로 기술. 경로 규칙: 절대경로·상위 경로·널 문자는 거부 |
| [bmspec](https://github.com/bemusic/bmspec) | 부분 | Gherkin으로 쓴 실행 가능 명세. "공식 명세가 아니다"라고 명시. Unlicense |
| [Bemuse BMS 지원 문서](https://bemuse.ninja/project/docs/bms-support) | 확인 | 지원하지 않는 항목: `#RANK`, `#TOTAL`, 지뢰, BGA 등 |
| [Guide to understand BMS format](https://cosmic.mearie.org/2005/03/bmsguide/) | 미확인 | hitkey가 참고 자료로 인용. Qwilight가 이를 참고해 파싱한다고 원본에 기록 |

## 2. BMS 도구·구동기·파서

원본 기록의 관찰을 보존했다. 코드 위치와 헤더 지원 여부는 원본이 직접 읽은 내용이며 이번에 재확인하지 않았다.

### BMSE

- 링크: [사이트](http://ucn.tokonats.net/software/bmse/), [GitHub](https://github.com/Nekokan/BMSE)
- 대응하지 않는 header
  - 파일 관련: `#BANNER`, `#BACKBMP`, `#EXBMPzz`, `#VIDEOFILE`, `#CHARFILE`, `#MIDIFILE`, `#EXWAVzz`, `#PREVIEW`, `#MATERIALSWAV`, `#MATERIALSBMP`, `#PATH_WAV`, `#CDDA`
  - 메타데이터 관련: `#SUBTITLE`, `#SUBARTIST`, `#MAKER`

### iBMSC

- 링크: [GitHub](https://github.com/aqtq314/iBMSC). 2013-11-09의 3.0.5가 마지막 릴리스이고 유지보수가 중단됐다(조사 확인).
- BGA 대신할 것이 있는 쪽을 지원하는 목적이라 `#BMP`를 뺌.
- `source/iBMSC/iBMSC/Form1.vb`
  - `Private Sub OpenBMS(ByVal As String)`
  - `Private Function SaveBMS() As String`
    - 메타데이터 관련: `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`
    - 파일 정보 관련: `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#WAV`

### μBMSC

- 링크: [GitHub](https://github.com/zardoru/iBMSC). 원본 기록이 적은 주소이며, 정식 저장소 주소는 이번 조사에서 확인하지 못했다.
- iBMSC와 비슷하게 `#BMP`를 뺌.
- `iBMSC/ChartIO.vb`
  - `Private Sub OpenBMS(ByVal As String)`
    - 메타데이터 관련: `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`
    - 파일 정보 관련: `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#WAV`

### Qwilight

- [Qwilight](https://taehui.ddns.net/ko)는 hitkey 메모가 참고한 Guide to understand BMS format을 참고해서 파싱한다(원본 기록).
- 원본 기록의 방침: 구동기를 추가하는 대신 구현을 보고 정리한다.

### Beatoraja

- [Beatoraja](https://github.com/exch-bms2/beatoraja)는 GPL-3.0이다(확인).
- 파서는 [jbms-parser](https://github.com/exch-bms2/jbms-parser)다. 저장소에 라이선스가 표시돼 있지 않아 코드를 복사하면 안 되고 동작만 참고한다.
- [BMSDecoder.java](https://raw.githubusercontent.com/exch-bms2/jbms-parser/master/src/bms/model/BMSDecoder.java)에서 확인한 동작
  - `#WAV`·`#BMP` 헤더 이름은 대소문자를 구분하지 않고 인식하며, 값의 경로에서 `\`를 `/`로 바꾼다.
  - 인코딩은 BOM → EUC-KR → MS932 → UTF-8 → UTF-16/32 순으로 64KB 표본을 시험하고 기본값은 MS932다.
  - MD5와 SHA-256을 원시 바이트에서 계산한다.
  - `#BASE`는 사전 스캔하며 62 또는 36을 받는다.

### bemuse

- [bemuse](https://github.com/bemusic/bemuse)는 AGPL-3.0이다(주 프로젝트).
- 파서는 [bms-js](https://github.com/bemusic/bemuse/tree/master/packages/bms)다. Reader(문자셋 감지), Compiler(`#RANDOM` 처리), 추출 모듈의 3단계 구조다.

### Rust 파서

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [bms-rs](https://github.com/MikuroXina/bms-rs) · [docs.rs](https://docs.rs/bms-rs/latest/bms_rs/) | 확인 | Apache-2.0. 1.0.0(crates.io 2026-04-18). 인코딩 감지·BOM 제거·경로 정규화 없음. 유지보수자 1~2명 |
| [bms-table](https://docs.rs/bms-table) | 확인 | 난이도표(header.json, data.json) 파서. Apache-2.0 |
| [BMS Search](https://bmssearch.net/) | 확인 | 곡 색인 사이트. API 문서는 찾지 못함 |

## 3. METS와 메타데이터

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [METS 홈](https://www.loc.gov/standards/mets/) | 확인 | 원본 기록은 mets-home.html 주소를 적었다. METS 2가 2025년에 나왔다 |
| [mets.xsd 1.12.1](https://www.loc.gov/standards/mets/mets.xsd) | 확인 | `CHECKSUMTYPE`은 닫힌 열거형이고 SHA-3이 없다 |
| [METS 2 스키마](https://raw.githubusercontent.com/mets/METS-schema/main/v2/mets2.xsd) | 확인 | `CHECKSUMTYPE`은 열거형이 아닌 자유 문자열이다(원문 다운로드로 확인) |
| [METS 2 제안 값 위키](https://github.com/mets/METS-schema/wiki/METS2-Suggested-Attribute-Values) | 확인 | 11개 값뿐이고 SHA-3·BLAKE3 없음. "다른 값은 각 profile이 해석한다" |
| [METS 2 문서](https://mets.github.io) | 부분 | 가이드, 마이그레이션 도구 안내. profile 문서는 읽지 못함 |
| [METS Profiles](https://www.loc.gov/standards/mets/mets-profiles.html) | 확인 | 자체 스키마를 profile로 정의하는 경로 |
| [PREMIS 3.0](https://www.loc.gov/standards/premis/) | 확인 | fixity 위치. [해시 함수 어휘](https://id.loc.gov/vocabulary/preservation/cryptographicHashFunctions.rdf.xml)는 13개 항이고 SHA3·BLAKE 없음 |
| [MODS](https://www.loc.gov/standards/mods/) · [DCMI Terms](https://www.dublincore.org/specifications/dublin-core/dcmi-terms/) · [MIX](https://www.loc.gov/standards/mix/) | 확인 | 제목·아티스트, 이미지 기술 메타데이터 매핑 후보 |
| [XML 1.0](https://www.w3.org/TR/xml/) · [XML Schema 1.1](https://www.w3.org/TR/xmlschema11-1/) · [XLink 1.1](https://www.w3.org/TR/xlink11/) | 확인 | UTF-8은 필수 지원, 그 외 인코딩은 선언이 필요 |

## 4. IPFS와 P2P

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [IPFS 문서](https://docs.ipfs.tech/) | 확인 | 원본 기록이 "배포 및 공유 기반"으로 지정 |
| [콘텐츠 주소 지정과 CID](https://docs.ipfs.tech/concepts/content-addressing/) | 확인 | 청크·레이아웃·CID 버전·해시가 다르면 같은 파일도 CID가 다르다 |
| [UnixFS 명세](https://specs.ipfs.tech/unixfs/) · [개념](https://docs.ipfs.tech/concepts/file-systems/) | 확인 | 단일 블록은 raw(0x55), 그 외 dag-pb(0x70) |
| [CID 명세](https://specs.ipfs.tech/cid/) | 확인 | 원본 `multiformats/cid`는 archived |
| [IPIP-0499 (unixfs-v1-2025)](https://specs.ipfs.tech/ipips/ipip-0499/) | 확인 | 재현 가능한 CID 프로파일. CC0 |
| [Kubo CLI](https://docs.ipfs.tech/reference/kubo/cli/) · [config.md](https://github.com/ipfs/kubo/blob/master/docs/config.md) | 확인 | 기본값 CIDv0, `size-262144`, 링크 174 |
| [Trustless Gateway 명세](https://specs.ipfs.tech/http-gateways/trustless-gateway/) | 확인 | Bitswap 없이 블록·CAR를 가져오는 경로 |
| [Bitswap 명세](https://specs.ipfs.tech/bitswap-protocol/) | 확인 | 프로토콜 ID 1.0.0, 1.1.0, 1.2.0 |
| [multihash](https://github.com/multiformats/multihash) · [multicodec](https://github.com/multiformats/multicodec) | 부분 | sha2-256=0x12, sha3-256=0x16, blake3=0x1e(요약 도구 결과) |
| Rust: [cid](https://docs.rs/cid/latest/cid/) · [rust-unixfs](https://docs.rs/rust-unixfs/latest/rust_unixfs/) · [multihash-codetable](https://docs.rs/multihash-codetable/latest/multihash_codetable/) | 확인 | kubo 없이 CID를 계산할 후보. kubo와의 동등성은 검증되지 않음 |
| [rust-ipfs](https://crates.io/crates/rust-ipfs) · [저장소](https://github.com/dariusc93/rust-ipfs) | 확인 | 원본 기록이 지목한 구현. 0.16.0(2026-07-04), 자체 표기 alpha |
| [rust-libp2p](https://github.com/libp2p/rust-libp2p) | 확인 | 0.57.0(2026-09-11). Bitswap 기능 없음 |
| [beetswap](https://github.com/celestiaorg/beetswap) | 확인 | Rust Bitswap, 0.5.0(2025-09-19), kubo 호환 증거 없음 |

## 5. iroh

원본 기록이 지목한 [iroh](https://iroh.computer/docs)와 [iroh-rust](https://crates.io/crates/iroh)에 대한 조사 결과다.

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [iroh 문서](https://docs.iroh.computer/) | 확인 | QUIC, 공개키(endpoint ID) 접속, 릴레이 폴백 |
| [iroh 1.0 글](https://iroh.computer/blog/the-road-to-iroh-1-0) | 확인 | 1.0 안정화, IPFS 호환 포기 |
| [iroh 크레이트](https://docs.rs/iroh/latest/iroh/) | 확인 | 1.3.0(2026-09-28). MSRV 1.91 |
| [Blobs 프로토콜](https://docs.iroh.computer/protocols/blobs) | 확인 | 식별자는 BLAKE3. CID와 호환되지 않음 |
| [iroh-blobs](https://docs.rs/iroh-blobs/latest/iroh_blobs/) | 확인 | 0.103.1(2026-10-06), "프로덕션 품질 아님" |

## 6. Rust 크레이트와 표준

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [encoding_rs](https://docs.rs/encoding_rs/latest/encoding_rs/struct.Encoding.html) | 확인 | `decode`의 bool은 오류를 U+FFFD로 대체했는지 여부 |
| [WHATWG Encoding Standard](https://encoding.spec.whatwg.org/) | 부분 | Shift_JIS 라벨과 디코더. 앞 100k자만 읽음 |
| [chardetng](https://docs.rs/chardetng/latest/chardetng/struct.EncodingDetector.html) | 확인 | `encoding_rs::Encoding`을 돌려줘 바로 연결됨 |
| [`regex`](https://docs.rs/regex/latest/regex/) | 확인 | 1.13.1. 한 번 컴파일해 재사용 |
| [sha3](https://docs.rs/sha3/latest/sha3/) | 확인 | 0.12.0이 최신. 0.11.0-pre.4는 yank되진 않았지만 구버전 |
| [FIPS 202](https://csrc.nist.gov/pubs/fips/202/final) | 부분 | SHA-3 표준. 랜딩 페이지만 확인 |
| [magic](https://docs.rs/magic/latest/magic/) · [`README`](https://github.com/robo9k/rust-magic/blob/main/README-crate.md) | 확인 | 0.16.7. Windows는 vcpkg로 libmagic이 필요 |
| [libmagic(3)](https://man7.org/linux/man-pages/man3/libmagic.3.html) · [file(1)](https://man7.org/linux/man-pages/man1/file.1.html) | 확인 | DB 경로는 `MAGIC` 환경변수 |
| [RFC 6838](https://www.rfc-editor.org/rfc/rfc6838) · [IANA 미디어 타입](https://www.iana.org/assignments/media-types/media-types.xhtml) | 부분 | WAV 항목은 확인하지 못함 |
| [RFC 5234 (ABNF)](https://www.rfc-editor.org/rfc/rfc5234) | 확인 | 문법 표기법. 따옴표 문자열은 기본적으로 대소문자 무시 |
| [Cargo Book: config](https://doc.rust-lang.org/cargo/reference/config.html) | 확인 | `[env]`는 `.cargo/config.toml` 기능이고 매니페스트에는 없다 |
| [rustdoc: 문서 쓰기](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html) | 확인 | `///`, `//!`, `#[doc = include_str!]` |

## 7. 인프라와 라이선스

| 자료 | 상태 | 메모 |
| --- | --- | --- |
| [super-linter v6.8.0](https://github.com/super-linter/super-linter/tree/v6.8.0) | 확인 | 저장소 린트 워크플로가 고정한 버전 |
| [GitHub Actions 워크플로 문법](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions) | 확인 | 워크플로 파일은 `.github/workflows/`에 있어야 한다 |
| [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) | 확인 | |
| [Contributor Covenant 2.0](https://www.contributor-covenant.org/version/2/0/code_of_conduct/) | 확인 | |
| [CC BY-SA 4.0 법률 조항](https://creativecommons.org/licenses/by-sa/4.0/legalcode) | 확인 | README가 문서 기본 라이선스로 지정 |
| [GNU GPL v3.0](https://www.gnu.org/licenses/gpl-3.0.html) | 미확인 | 서버가 429를 반환해 열지 못함 |
| [Firebase Studio 전환 공지](https://firebase.google.com/docs/studio/idx-is-firebase-studio) | 확인 | `.idx/dev.nix`가 속한 IDX가 2027-03-22에 종료 예정 |
| [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) · [SemVer](https://semver.org/) | 확인 | 기록 문서 형식 참고 |

## 8. 채보 포맷 색인

별도로 정리 중인 채보 포맷·명세 원장(Google Drive 시트, 119행)의 안내 문서가 "다른 조사에도 쓰는 입구"로 적은 색인이다. 이번에 열어보지 않았고 안내 문서의 기재를 옮겼다.

| 자료 | 범위 |
| --- | --- |
| [vsrg-format-docs](https://github.com/kangalio/vsrg-format-docs) | 마니아형 VSRG 30여 포맷 |
| [GuitarGame_ChartFormats](https://thenathannator.github.io/GuitarGame_ChartFormats/) | `.chart`, `.mid`, `song.ini` 등 |
| [rhythm-game-formats (SaxxonPike)](https://github.com/SaxxonPike/rhythm-game-formats) | DDR·IIDX 등 상용 원본 역공학 |
| [Project OutFox Wiki](https://outfox.wiki/) | SM·SSC·KSF·TJA 호환 |
| [BSMG Wiki map format](https://bsmg.wiki/mapping/map-format.html) | Beat Saber v2/v3/v4 |
| [Sonolus Wiki](https://wiki.sonolus.com/) | 엔진·레벨·리플레이 스펙 |
| [UltraStar format](https://github.com/UltraStar-Deluxe/format) | 노래 게임 포맷 |
