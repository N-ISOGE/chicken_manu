# 기술 채택 판단

2026-10-08 기준 사전 조사 결과다. 2026-10-09 다른 AI의 조사에서 알게 된 것은 4절, 5절, 10절에 따로 표시했다. 구현 계획이 아니며, 채택 전에 버전과 라이선스를 다시 확인해야 한다.
근거 링크는 [참고 자료](crate::docs::references)에 있다.

## 1. 판정 요약

| 결정                       | 판정                    | 근거                                                                           |
| -------------------------- | ----------------------- | ------------------------------------------------------------------------------ |
| sha3 크레이트              | 채택(교체)              | `0.11.0-pre.4`는 구버전이고 안정판 0.11.0, 0.12.0이 있다. SHA3-256 출력은 같다 |
| XML 쓰기                   | 채택                    | quick-xml 0.42(MIT)                                                            |
| 메타데이터 스키마          | 시험 후 채택            | METS 2 + 자체 profile. METS 2는 `CHECKSUMTYPE`이 자유 문자열이다               |
| BMS 파서                   | 시험 후 채택            | bms-rs 1.0.0을 얇은 어댑터 뒤에 두고, 기존 정규식 파서를 폴백으로 유지         |
| 순수 Rust CID 계산         | 시험 후 채택            | rust-unixfs 0.6.0. kubo와 같은 CID인지 검증된 적이 없다                        |
| IPFS 노드                  | 시험 후 채택            | kubo를 사이드카로. 단 유지보수 종료 위험(아래)                                 |
| Rust 네이티브 Bitswap      | 보류                    | kubo와 통신한 실증이 없다                                                      |
| iroh                       | 시험 후 채택(선택 트랙) | 코어는 안정. iroh-blobs는 0.x이고 CID와 호환되지 않는다                        |
| XSD 검증(Rust)             | 보류                    | 신뢰할 만한 순수 Rust 검증기가 없다. CI에서 xmllint 사용                       |
| RAR 해제(`unrar` 크레이트) | 회피                    | UnRAR 라이선스가 GPL과 충돌할 수 있다                                          |

## 2. 먼저 알아야 할 위험

1. **kubo/Boxo 유지보수 종료 위험.** Boxo v0.43.0 공지에 따르면 Shipyard는 2026-09-30 이후 Boxo를 유지하지 않는다. kubo master의 마지막 커밋은 2026-09-27이고 조사일(2026-10-08)까지 v0.44는 없었다. 후속 유지자나 포크는 확인하지 못했다. CID 기준값, RPC, 게이트웨이 경로가 모두 kubo에 기댄다.
2. **식별자가 5종이다.** MD5, SHA-256, SHA3-256, IPFS CID, BLAKE3. 단일 content ID가 없다.
3. **CID 기본값이 구현마다 다르다.** kubo 기본은 CIDv0, 256KiB, 링크 174, raw leaves 없음이다. `unixfs-v1-2025` 프로파일은 CIDv1, 1MiB, 링크 1024다. rust-unixfs 기본은 CIDv1 + raw leaves인데 256KiB/174라서 이름 있는 프로파일이 아닌 조합이다.
4. **네이티브 IPFS와 iroh는 공유하는 것이 없는 별개 트랙이다.** libp2p 버전과 해시 체계가 모두 다르다.
5. **저작권·재배포 조건은 조건의 내용을 조사하지 못했다.** 곡·그림·영상의 배포 조건, 행사별 약관, 삭제 요청 처리가 해당한다. 2026-10-09에 다른 AI의 조사가 BMS 행사 규약(BOF21, 무명전), osu! 정책 3건, 한국·일본·미국 법령의 **원문 위치**를 모았지만 조항 내용은 요약하지 않았고, StepMania·ITG 배포처는 열지 못했다([참고 자료](crate::docs::references::preservation) 2절).

## 3. IPFS와 kubo

### kubo

| 항목                    | 내용                                                                                                                          | 상태                                  |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| 최신                    | v0.43.1(2026-09-15), MIT/Apache-2.0                                                                                           | 확인                                  |
| 기본값                  | CIDv0, `size-262144`, 링크 174. 0.43 changelog에 "Nothing changes by default"                                                 | 확인                                  |
| 재현 가능 프로파일      | `ipfs config profile apply unixfs-v1-2025`(kubo 0.40 이상). 같은 입력·프로파일이면 같은 CID                                   | 확인                                  |
| 프로파일 내용           | CIDv1, raw leaves, 1MiB 청크, 링크 1024, sha2-256, balanced, links-first, HAMT는 256KiB 초과 시(엄격한 초과)                  | 확인                                  |
| 명시 플래그 대안        | `ipfs add -r --cid-version 1 --hash sha2-256 --raw-leaves --chunker size-1048576`                                             | 부분: 실행해 프로파일과 비교하지 않음 |
| `--only-hash`           | CLI는 프로세스 안에서 동작한다. RPC `/api/v0/add`는 데몬이 있어야 한다. 초기화된 repo가 꼭 필요한지는 미확인                  | 부분                                  |
| Rust RPC 클라이언트     | 유지되는 크레이트가 없다. IPFS 문서가 나열한 4개가 모두 Inactive이고 `ipfs-api-backend-hyper`는 2022-12-31 이후 릴리스가 없다 | 확인                                  |
| trustless gateway + CAR | 명세상 Bitswap 없이 가져올 수 있다. 호스트 주소를 알아야 하고 UnixFS 검증은 수신 측 책임이다                                  | 부분                                  |

권장: 사설 데몬(loopback, 별도 `IPFS_PATH`)으로 쓰고 얇은 HTTP 래퍼 또는 기존 CLI 호출을 유지한다. 바이너리 SHA-512을 검증해 고정하고, 프로파일 이름을 각 CID 옆에 기록한다.

### 순수 Rust CID 계산

- rust-unixfs 0.6.0(MIT OR Apache-2.0, 2026-06-18)이 유일하게 믿을 만한 순수 Rust UnixFS 임포터다. 기본값은 256KiB 청크, 분기 174, CIDv1 raw leaves, `wrap_with_directory` false, `block_size_limit` 512KiB다.
- 골든 테스트는 **CIDv0뿐**이고 `bafy`/`bafk` 문자열을 검증하는 테스트는 없다. HAMT는 kubo로 검증된 적이 없고 추정식도 v1-2025와 다르다(links-bytes 방식).
- `multihash-codetable` 0.2.2가 `sha3 ^0.11`을 요구하므로 rust-unixfs를 쓰면 sha3 0.12와 충돌한다.
- 채택 조건: IPIP-0499의 fixture와 실제 BMS 묶음(Shift-JIS 파일명, 중첩 디렉터리, 1MiB 초과 파일, 링크 1024 초과)을 프로파일을 적용한 `ipfs add -n` 결과와 비교하는 **차분 테스트**를 통과해야 한다. 이 테스트는 아직 하지 않았다.

### Rust 네이티브 Bitswap

| 크레이트                                                                 | 상태                                                                                                                                             | 판정    |
| ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ | ------- |
| beetswap 0.5.0(Apache-2.0, 2025-09-19)                                   | 약 12개월간 커밋 없음. libp2p-swarm 0.47을 요구하는데 rust-libp2p 0.57.0은 swarm 0.48이라 **libp2p 0.56에 고정**해야 한다. kubo 언급·테스트 없음 | 시험 후 |
| rust-ipfs 0.16.0(MIT OR Apache-2.0, 2026-07-04)                          | alpha, 사실상 1인 유지, Bitswap 1.2.0만. kubo 호환 테스트 없음                                                                                   | 시험 후 |
| freedom-ipfs                                                             | 유일한 kubo 대상 테스트(`--ignored`)가 있으나 공개된 통과 기록이 없다                                                                            | 참고용  |
| libp2p-bitswap 0.25.1(2023), beetle-bitswap-next 0.5.1(2023), ipfs-embed | 방치                                                                                                                                             | 회피    |

결론: "Rust가 kubo에서 블록을 받았다"는 실증을 찾지 못했다. 필요하면 고정한 kubo 0.43.x를 대상으로 시간을 정해 스파이크하고, 아니면 사이드카 RPC나 trustless gateway로 간다.

## 4. 메타데이터 스키마

- METS 2(`v2/mets2.xsd`, Version 2.0, 2025년 3월, 헤더 CC0)는 `CHECKSUMTYPE`이 선택적 `xsd:string`이고 열거 제한이 없다. XSD 설명은 "enumerated string"이라 혼동되지만 제약은 없다(원문 확인).
- 제안 값 위키에는 METS 1의 11개 값뿐이다. SHA3·BLAKE3은 없고, "다른 값의 해석은 각 METS profile에 달렸다"고 적혀 있다.
- METS 1.12.1은 11개 값의 닫힌 열거형이라 SHA-3을 넣을 수 없다. PREMIS 해시 어휘(13개 항)에도 SHA3·BLAKE이 없다.
- METS-schema 저장소는 릴리스·태그가 없고 LICENSE 파일이 없으며 마지막 커밋이 2025-03-12다. XSD를 특정 커밋으로 고정해 저장소에 포함하는 것을 권장한다(헤더의 CC0 표기를 인용).
- 해시 이름 철자는 하나로 정해야 한다. METS 제안 값은 `SHA-256`, IANA는 소문자 `sha3-256`(blake3 없음), CycloneDX는 `SHA3-256`·`BLAKE3`, multicodec은 `sha3-256`(0x16)·`blake3`(0x1e)다.
- METS 2 profile 문서, 마이그레이션 노트, 예제, 도구는 읽지 못했다.
- 대안 비교: 처음에는 2차 정보만 있어 제외했다. 2026-10-09에 다른 AI의 조사가 BagIt(RFC 8493)과 OCFL 1.1의 핵심 규정을 직접 읽어 정리했고, RO-Crate, OAIS, MAME software list, No-Intro/Redump, SWHID, Data Package, 인터넷 아카이브는 인덱스·소개 수준(부분)이다([참고 자료](crate::docs::references::preservation) 1절, [스키마](crate::docs::schema) 4절). 비교와 판정은 조사가 하지 않았으므로 이 문서도 판정하지 않는다.
- BagIt은 SHA-256·SHA-512 지원을 도구의 필수로 하고 OCFL은 콘텐츠 주소에 `sha512`·`sha256`만 허용한다. 둘 다 SHA3-256이 필수·허용 집합에 없어서, 3절의 "필수 해시 집합" 논의에 SHA-512 보관 여부가 추가된다.

## 5. BMS 파서와 인코딩

- bms-rs 1.0.0은 Apache-2.0이라 GPL-3.0과 호환된다(GPLv2-only와는 아님). 파일 참조 헤더(`stage_file`, `banner`, `#WAVxx` 등)를 `PathBuf`로 노출하고 `#RANDOM`의 모든 분기를 보존한다. 반면 `&str`만 받아 인코딩 감지가 없고, BOM 제거와 경로 정규화(역슬래시, 대소문자)가 없으며, `#BASE 62` 지원은 확인하지 못했다. 릴리스마다 파싱 API가 바뀌고 유지보수자는 1~2명이다.
- 인코딩 판별 순서 권고: BOM → 엄격한 UTF-8 → 엄격한 Shift_JIS(`decode_without_bom_handling_and_without_replacement`) → chardetng → 손실 디코딩(메타데이터에 표시). beatoraja의 순서와 달라서 두 프로그램이 같은 파일을 다르게 읽을 수 있다.
- WHATWG Shift_JIS는 `0x5C`를 백슬래시로 유지한다. CP932와의 차이(웨이브 대시 `0x8160`, NEC 13행 `0x8790` 등)를 2026-10-09에 한 PC의 BMS 샘플 189개로 시험했고, MS932와 엄격 Shift_JIS의 결과가 같았다(차이 나는 파일 0, 두 바이트 모두 샘플에 없음). 샘플 밖에서는 갈릴 수 있고 BOM 파일과 비ASCII UTF-8 파일이 샘플에 없어 그 경로는 검증하지 못했다(`docs/bms-parsers.md` 2.1절).
- 해시는 항상 **디코딩 전 원시 바이트**에서 계산한다.

## 6. iroh

- iroh 1.3.0(2026-09-28), MIT OR Apache-2.0, MSRV 1.91, Windows 지원. 1.0.0은 crates.io 기준 2026-06-15다.
- iroh-blobs 0.103.1(2026-10-06)은 아직 0.x다. 0.94.0~0.103.0에는 아무 피어나 제공자의 디스크를 채울 수 있는 push 처리 버그가 있었고 0.103.1에서 고쳐졌다.
- 버전 고정: iroh 1.3.x + iroh-blobs `=0.103.1`. iroh-blobs README의 "프로덕션은 0.35" 권고는 0.35용 공용 릴레이 지원이 2026-12-31에 끝나므로 신규 프로젝트에 맞지 않는다.
- 패키지(디렉터리) 공유는 파일별로 가져온 뒤 이름-해시 쌍의 Collection을 만들고 BlobTicket으로 공유한다. 이름은 iroh 고유 메타 blob에 저장되고 형식은 "변경될 수 있다". 검증은 BLAKE3만 하므로 SHA3-256과 CID는 받은 뒤 따로 확인해야 한다. CID로는 가져올 수 없다.
- 공용 릴레이는 개발·취미용이고 SLA와 속도 제한 수치가 공개돼 있지 않다. 유료 공유 플랜은 월 $19(100GB 포함, 초과 $0.09/GB, 5MB/s). 직접 연결 트래픽은 비용이 없다.
- 위치 선정: kubo를 대체하는 것이 아니라 BLAKE3 티켓 기반의 비공개 공유 채널로 보는 것이 현실적이다.

## 7. 그 밖의 Rust 부품

| 영역   | 선택                                                                                    | 메모                                                                        |
| ------ | --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| 해시   | sha3 0.11.0(`multihash-codetable`을 쓸 때) 또는 0.12.0                                  | 최신 `digest`의 yank 여부는 미확인                                          |
| BLAKE3 | blake3 1.8.7(CC0 OR Apache-2.0), 선택                                                   |                                                                             |
| 인코딩 | encoding_rs 0.8.42(Apache-2.0 OR MIT, BSD-3 고지 포함)                                  |                                                                             |
| MIME   | infer 0.22.0(MIT)을 기본, 확장자 표를 자체 보유, magic은 선택 기능                      | BMS 텍스트는 내용 기반 감지가 안 된다. Windows에서 magic은 vcpkg가 필요하다 |
| XML    | quick-xml 0.42.0(MIT, MSRV 1.86). UTF-8 선언 명시                                       | xml-rs, serde-xml-rs, xmlwriter 회피                                        |
| 정규식 | `regex` 1.13.1                                                                          | 단일 검색은 O(m*n), 반복자는 최악 O(m*n²)                                   |
| 압축   | `zip` 8.6.0(MSRV 1.88). 항목 이름은 `name_raw`를 직접 디코딩하고 `enclosed_name()` 사용 | 7z(sevenz-rust2)는 MSRV 1.93이라 보류                                       |

툴체인 하한은 기본 1.85, zip과 libxml 1.88, iroh 1.91, sevenz-rust2 1.93이다.

## 8. 라이선스 메모

법률 자문이 아니다. 저장소의 코드는 GPL-3.0, 문서와 스키마는 CC BY-SA 4.0이 기본이다.

- 호환으로 판단: Apache-2.0 의존성(bms-rs, beetswap, bms-table, sevenz-rust2), MIT/Apache 이중 라이선스(rust-unixfs, iroh, rust-ipfs). kubo는 별도 프로세스로 실행하면 링크 문제가 없고, 바이너리를 재배포하면 고지를 유지해야 한다.
- 회피: `unrar`(UnRAR 라이선스), sup-xml(독점), gfeh-ipfs(AGPL-3.0), chardet 0.2.4(LGPL-3.0, 방치), charset-normalizer-rs(라이선스 불명확).
- 출처가 불명확: jbms-parser, uBMSC, iBMSC는 라이선스 표시가 없어 코드를 복사하면 안 된다. hitkey, Bemuse, beatoraja 위키 문서도 라이선스 표시가 없으므로 CC BY-SA 문서에 원문을 그대로 옮기지 않는다(짧은 인용은 무방).
- 전이 의존성은 감사하지 않았다(`cargo-deny` 미실행).

## 9. 이전 조사에서 정정한 내용

1. iroh 1.0.0의 릴리스일은 crates.io 기준 **2026-06-15**다. 07-09는 블로그 글의 날짜였다.
2. iroh-blobs의 "프로덕션은 0.35" 권고는 신규 프로젝트에 맞지 않는다(위 6절).
3. METS 2의 "2025-03-03 출시"는 확인되지 않는다. 저장소는 "March 2025"와 마지막 커밋 2025-03-12만 보여 준다. `CHECKSUMTYPE`이 자유 문자열이라는 점은 원문으로 확인됐다.
4. hitkey에는 `#PREVIEW`, `#BASE`, `#LNMODE`가 **없다**(원문 전체 확인). 그러나 실제 파일에는 `#PREVIEW`와 `#LNMODE`가 있다. 따라서 "hitkey가 BMS 명세 그 자체"가 아니라 "hitkey + 구동기 동작"이 실질 표준이다.
5. beetswap 저장소는 eigerco가 아니라 celestiaorg/beetswap이다.
6. bms-rs 최신 릴리스는 crates.io 기준 2026-04-18이다.

## 10. 미확인 항목 (우선순위 순)

1. 차분 CID 테스트(kubo 프로파일, 명시 플래그, rust-unixfs 비교).
2. kubo/Boxo의 2026-09-27 이후 커밋·릴리스, 후속 유지자, 공용 게이트웨이 가용성.
3. 해시·식별자 정책과 METS 2 profile 설계(필드명, 철자, 필수 해시 집합).
4. BMS 구동기별 경로·대소문자·확장자 대체 동작(LR2는 비공개 소스), `#BASE 62` 권위 문서.
5. Windows 11 점검: 사설 kubo 데몬, libmagic(vcpkg)과 infer 비교, 후보 의존성 조합의 실제 빌드, Shift-JIS 파일명 처리.
6. 인코딩 바이트 시험: CP932 차이는 샘플 189개에서 시험했다(차이 0). 남은 것: beatoraja 순서와 chardetng 순서 비교, BOM·비ASCII UTF-8 경로, 샘플 밖 파일, Java EUC-KR의 `0xA4D4` 처리.
7. iroh: Windows `FsStore`, 직접·릴레이 전송, push 거부 설정, 릴레이 약관.
8. 저작권·재배포 조건: 원문 위치 일부를 모았을 뿐 조항 내용은 요약하지 못했다. 남은 것: 범위를 넓힌 후속 조사의 결과, StepMania·ITG 배포처, 한국·일본 법령 조문, 37 CFR §201.40(b)(19) 본문, BOF·무명전 규약의 게시일.
9. 보존·교환 포맷 선행사례의 재확인: 본문을 열지 못한 6건(RO-Crate, SWHID 승인 명세, No-Intro·Redump DAT, MAME DTD, Data Package 경로 규칙, CCSDS 650.0-M-3)의 재확인. 결과가 아직 없다.
