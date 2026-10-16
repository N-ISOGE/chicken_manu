# 참고 자료: BMS 명세와 도구

[← 길라잡이][project-guide]

## 처음 읽는다면

[1. BMS 명세](#1-bms-명세)에서 자료 목록을 살펴보실 수 있습니다. 확인 범위는 아래 안내와 각 자료의 메모를 함께 살펴봅니다.

## 목차

- [1. BMS 명세](#1-bms-명세)
- [2. BMS 도구·구동기·파서](#2-bms-도구구동기파서)

BMS의 명세 문서와 BMS를 읽는 구동기·파서·편집기의 링크를 모았습니다. 확인 상태는 [공통 안내의 문서 읽는 법][nav-1]의 표기를 따릅니다(확인 / 부분 / 미확인).
1절의 일부 행은 별도 조사(2026-10-09)에서 옮겼습니다. 상태는 조사가 스스로 적은 확인 수준이며, 이 저장소가 다시 확인한 값은 아닙니다.

표의 버전·배포일·확인 상태는 기록된 조사 시점의 정보입니다. 현재 판본에 적용할 때는 원문과 대상 판본을 다시 대조합니다.

## 1. BMS 명세

| 자료                                                                                                                          | 상태   | 메모                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ----------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [hitkey BMS command memo](https://hitkey.bms.ms/cmds.htm)                                                                     | 확인   | 사실상 표준으로 쓰이는 문서입니다. 전체 원문을 받아 확인했습니다. 최종 갱신일은 2014-07-11입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| [hitkey command memo, 원 호스트](https://hitkey.nekokan.dyndns.info/cmds.htm)                                                 | 부분   | 원본 기록에서 인용한 주소입니다. 별도의 두 조사가 열어 `latest update: 2014-07-11`을 확인했으나 본문이 중간(BME 설명 도중)에서 잘렸습니다. 작성자는 "only my memo"라고 밝혔습니다. 위 미러는 두 조사 모두 열지 못해 미러의 "확인"을 뒤집는 근거는 아닙니다.                                                                                                                                                                                                                                                                                                                                                             |
| [hitkey 2014 archive](https://web.archive.org/web/20240505175610/https://hitkey.nekokan.dyndns.info/cmds.htm)                 | 미확인 | 원본 기록이 "과거 기록, 현재 반영"으로 둔 사본입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [BM98 원전: BMS Format Specification](https://bm98.yaneu.com/bm98/bmsformat.html)                                             | 부분   | 1차 출처입니다. 두 조사가 HTTPS 주소로 본문을 읽었고 지정한 HTTP 주소는 열지 못했습니다. 문서 작성자는 Urao Yane이며, 형식 제작자는 Urao Yane와 NBK입니다. 게시·갱신일 표기와 웹 아카이브 사본은 찾지 못했습니다. 서두 "anyone can use this format freely", 말미 "This document and this format is free!" 외에 명명된 라이선스는 못 찾았습니다. 확인한 헤더는 `#PLAYER #GENRE #TITLE #ARTIST #BPM #MIDIFILE #PLAYLEVEL #RANK #VOLWAV #WAVxx #BMPxx #random #if #endif`이고 `#TOTAL`, `#ExtChr`는 이 목록에 없습니다(hitkey의 BM98 요약표에는 있습니다). 예제에 `//` 설명이 보이나 주석 문법으로 정의돼 있지는 않습니다. |
| [BMS Format Specification 요약](https://github.com/MikuroXina/bms-rs/blob/main/SPECIFICATION.md)                              | 확인   | 1998년 원본 사양의 요약입니다. `#WAVxx`·`#BMPxx`는 16진 01-FF이며, 인코딩 규정은 없습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [Be-Music Source (Wikipedia)](https://en.wikipedia.org/wiki/Be-Music_Source)                                                  | 확인   | 역사와 확장자의 개요를 설명합니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [bmson 명세](https://bmson-spec.readthedocs.io/en/master/doc/index.html)                                                      | 확인   | 판본은 1.0.0-beta(2015-12-26)이며 Web IDL로 기술합니다. 경로 규칙에서는 절대경로·상위 경로·널 문자를 거부합니다. 재확인 조사는 이 주소를 열지 못하고 [`.org` 호스트](http://bmson-spec.readthedocs.org/en/master/doc/)에서 `Version 1.0.0-beta (2015/12/26)`를 읽었습니다.                                                                                                                                                                                                                                                                                                                                              |
| [bmson 명세 포크](https://bmson-spec-fork.readthedocs.io/en/latest/doc/index.html)                                            | 부분   | `DJKero/bmson-spec-fork`의 문서입니다. 첫 제목 아래에 `Version 2.0.0 (2023/10/24)`가 표시돼 있습니다(재확인 조사가 본문을 확인했습니다). 1차 조사의 검색 결과에는 제목 `2.0.0-rc1`이 있었으나 본문에서 위치를 못 찾았습니다. 공식 `1.0.0-beta`를 대체한 판으로 보지 않습니다. 저장소 본문은 열지 못했습니다.                                                                                                                                                                                                                                                                                                            |
| [bmson-schema](https://github.com/JLChnToZ/bmson-schema)                                                                      | 미확인 | "A JSON Schema for validating Bmson format"이라는 설명만 확인했습니다. 파일 본문, 대응 명세 버전, 라이선스, 공식 채택 여부는 미확인입니다. 공식 `bemusic/bmson-spec`의 루트와 `doc`에서는 JSON Schema 파일을 찾지 못했습니다.                                                                                                                                                                                                                                                                                                                                                                                           |
| [bmspec](https://github.com/bemusic/bmspec)                                                                                   | 부분   | Gherkin으로 쓴 실행 가능 명세입니다. "공식 명세가 아니다"라고 명시합니다. 라이선스는 Unlicense입니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [Bemuse BMS 지원 문서](https://bemuse.ninja/project/docs/bms-support)                                                         | 확인   | `#RANK`, `#TOTAL`, 지뢰, BGA 등을 지원하지 않습니다. 두 조사는 끝에 `/`가 있는 주소로 열었고 "Last updated on Jul 27, 2026"을 확인했습니다.                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| [Bemuse's BMS Extensions](https://bemuse.ninja/project/docs/bms-extensions/)                                                  | 부분   | 별도의 공식 페이지입니다. 본문은 두 조사 모두 열지 못했고, 검색 발췌에 `#SCROLL01`, `#SCROLL02`, `#SPEED01`과 절 제목 "Base36 Channels"가 보입니다(검색 발췌 수준). 검색 도구의 날짜 2026-07-27은 본문 갱신일로 확정하지 못했습니다.                                                                                                                                                                                                                                                                                                                                                                                    |
| [beatoraja 楽曲製作者向け資料](https://github.com/exch-bms2/beatoraja/wiki/楽曲製作者向け資料)                                | 확인   | 2026-10-16에 본문을 확인했습니다. 본문에 표시된 최종 편집일은 2022-01-15입니다. 재생 형식(BMS·PMS·BMSON), BMS 확장(`#PREVIEW`, `#LNMODE`, `#RANK 4`, `#DEFEXRANK`, `#VOLWAV`, `#SCROLLxx`, `#nnnSC`)과 BMSON 확장을 설명합니다. 문서 열람 결과이며, 현재 구동기에서 모든 동작을 실행 검증한 것은 아닙니다.                                                                                                                                                                                                                                                                                                              |
| [jbmstable-parser 難易度表の仕様](https://github.com/exch-bms2/jbmstable-parser/wiki/難易度表の仕様)                          | 확인   | 2026-10-16에 본문을 확인했습니다. 페이지의 최종 편집 표시는 2018-04-13입니다. HTML·헤더 JSON·데이터 JSON의 구성을 설명합니다. 헤더 필수 항목은 `name`, `data_url`, `symbol`이며, 데이터에는 `level`과 `md5`·`sha256` 중 하나가 필요합니다. 선택 항목 `ipfs`는 디렉터리 경로, `ipfs_diff`는 디렉터리 또는 파일 경로로 정의합니다. 문서의 필드 정의를 확인한 것이며 파서 실행 결과는 아닙니다.                                                                                                                                                                                                                            |
| [Guide to understand BMS format (Wayback Machine)](https://web.archive.org/web/*/https://cosmic.mearie.org/2005/03/bmsguide/) | 부분   | hitkey가 참고 자료로 인용합니다. 원본에는 Qwilight가 이를 참고해 파싱한다고 기록돼 있습니다. 두 조사가 본문을 읽었으나 명령 표가 추출에서 빠졌고 끝이 잘렸습니다. 본문 버전은 `1.2.2 (2005.3.22)`이며, 2013-07-05 노후화 안내가 있습니다. 서두에는 GFDL이 표시돼 있습니다(1차 조사만 근거로 하며, 저자는 "토끼군"입니다). 작성자가 현행 BMS를 반영하지 못한다고 밝혀 현행 명세로 사용하지 않습니다.                                                                                                                                                                                                                     |

## 2. BMS 도구·구동기·파서

원본 기록의 관찰을 보존했습니다. 코드 위치와 헤더 지원 여부는 원본에서 직접 읽은 내용이며, 이 문서에서는 재확인하지 않았습니다.

### BMSE

- 링크: [사이트](http://ucn.tokonats.net/software/bmse/), [GitHub](https://github.com/Nekokan/BMSE)
- 지원하지 않는 헤더
  - 파일 관련: `#BANNER`, `#BACKBMP`, `#EXBMPzz`, `#VIDEOFILE`, `#CHARFILE`, `#MIDIFILE`, `#EXWAVzz`, `#PREVIEW`, `#MATERIALSWAV`, `#MATERIALSBMP`, `#PATH_WAV`, `#CDDA`
  - 메타데이터 관련: `#SUBTITLE`, `#SUBARTIST`, `#MAKER`

### iBMSC

- 링크: [GitHub](https://github.com/aqtq314/iBMSC). 2013-11-09의 3.0.5가 마지막 릴리스이고 유지보수가 중단됐습니다(조사 확인).
- BGA를 대신할 기능을 지원하려는 목적으로 `#BMP`를 제외했습니다.
- `source/iBMSC/iBMSC/Form1.vb`
  - `Private Sub OpenBMS(ByVal As String)`
  - `Private Function SaveBMS() As String`
    - 메타데이터 관련: `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`
    - 파일 정보 관련: `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#WAV`

### μBMSC

- 링크: [GitHub](https://github.com/zardoru/iBMSC). 원본 기록이 적은 주소이며, 정식 저장소 주소는 이번 조사에서 확인하지 못했습니다.
- iBMSC와 비슷하게 `#BMP`를 제외했습니다.
- `iBMSC/ChartIO.vb`
  - `Private Sub OpenBMS(ByVal As String)`
    - 메타데이터 관련: `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`
    - 파일 정보 관련: `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#WAV`

### Qwilight

- [Qwilight](https://taehui.ddns.net/ko)는 hitkey 메모가 참고한 Guide to understand BMS format을 참고해서 파싱합니다(원본 기록).
- 원본 기록에서는 구동기를 추가하는 대신 구현을 살펴 정리하는 방침을 적었습니다.

### Beatoraja

- [Beatoraja](https://github.com/exch-bms2/beatoraja)는 GPL-3.0입니다(확인).
- 파서는 [jbms-parser](https://github.com/exch-bms2/jbms-parser)입니다. 저장소에 라이선스가 표시돼 있지 않아 코드를 복사하면 안 되고 동작만 참고합니다.
- [BMSDecoder.java](https://raw.githubusercontent.com/exch-bms2/jbms-parser/master/src/bms/model/BMSDecoder.java)에서 확인한 동작
  - `#WAV`·`#BMP` 헤더 이름은 대소문자를 구분하지 않고 인식하며, 값의 경로에서 `\`를 `/`로 바꿉니다.
  - 인코딩은 BOM → EUC-KR → MS932 → UTF-8 → UTF-16/32 순으로 64KB 표본을 시험하고 기본값은 MS932입니다.
  - MD5와 SHA-256을 원시 바이트에서 계산합니다.
  - `#BASE`는 사전 스캔하며 62 또는 36을 받습니다.

### bemuse

- [bemuse](https://github.com/bemusic/bemuse)는 AGPL-3.0입니다(주 프로젝트).
- 파서는 [bms-js](https://github.com/bemusic/bemuse/tree/master/packages/bms)입니다. Reader(문자셋 감지), Compiler(`#RANDOM` 처리), 추출 모듈의 3단계 구조입니다.

### Rust 파서

| 자료                                                                                              | 상태 | 메모                                                                                                                                             |
| ------------------------------------------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| [bms-rs](https://github.com/MikuroXina/bms-rs) · [docs.rs](https://docs.rs/bms-rs/latest/bms_rs/) | 확인 | 라이선스는 Apache-2.0이며, 판본은 1.0.0(crates.io 2026-04-18)입니다. 인코딩 감지·BOM 제거·경로 정규화 기능은 없습니다. 유지보수자는 1~2명입니다. |
| [bms-table](https://docs.rs/bms-table)                                                            | 확인 | 난이도표(header.json, data.json) 파서입니다. 라이선스는 Apache-2.0입니다.                                                                        |
| [BMS Search](https://bmssearch.net/)                                                              | 확인 | 곡 색인 사이트입니다. API 문서는 찾지 못했습니다.                                                                                                |

[nav-1]: ../index.md#문서-읽는-법
[project-guide]: ../../index.md
