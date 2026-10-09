# 참고 자료: BMS 명세와 도구

BMS의 명세 문서와 BMS를 읽는 구동기·파서·편집기의 링크다. 확인 상태는 [첫 페이지](crate)의 표기를 따른다(확인 / 부분 / 미확인).
1절의 일부 행은 별도 조사(2026-10-09)에서 옮겼다. 상태는 조사가 스스로 적은 확인 수준이고 이 저장소가 다시 확인한 값이 아니다.

## 1. BMS 명세

| 자료                                                                                                          | 상태   | 메모                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [hitkey BMS command memo](https://hitkey.bms.ms/cmds.htm)                                                     | 확인   | 사실상 표준. 전체 원문을 받아 확인했다. 최종 갱신 2014-07-11                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| [hitkey command memo, 원 호스트](https://hitkey.nekokan.dyndns.info/cmds.htm)                                 | 부분   | 원본 기록이 인용한 주소. 별도의 두 조사가 열어 `latest update: 2014-07-11`을 확인했으나 본문이 중간(BME 설명 도중)에서 잘렸다. 작성자는 "only my memo"라고 밝혀 신빙성 B. 위 미러는 두 조사 모두 열지 못해 미러의 "확인"을 뒤집는 근거는 아니다                                                                                                                                                                                                                                                                                                                             |
| [hitkey 2014 archive](https://web.archive.org/web/20240505175610/https://hitkey.nekokan.dyndns.info/cmds.htm) | 미확인 | 원본 기록이 "과거 기록, 현재 반영"으로 둔 사본                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| [BM98 원전: BMS Format Specification](https://bm98.yaneu.com/bm98/bmsformat.html)                             | 부분   | 1차 출처(A). 두 조사가 HTTPS 주소로 본문을 읽었고 지정한 HTTP 주소는 열지 못했다. 문서 작성자 Urao Yane, 형식 제작자 Urao Yane와 NBK. 게시·갱신일 표기와 웹 아카이브 사본은 찾지 못했다. 서두 "anyone can use this format freely", 말미 "This document and this format is free!" 외에 명명된 라이선스는 못 찾았다. 읽힌 헤더는 `#PLAYER #GENRE #TITLE #ARTIST #BPM #MIDIFILE #PLAYLEVEL #RANK #VOLWAV #WAVxx #BMPxx #random #if #endif`이고 `#TOTAL`, `#ExtChr`는 이 목록에 없다(hitkey의 BM98 요약표에는 있다). 예제에 `//` 설명이 보이나 주석 문법으로 정의돼 있지는 않다 |
| [BMS Format Specification 요약](https://github.com/MikuroXina/bms-rs/blob/main/SPECIFICATION.md)              | 확인   | 1998 원본 사양의 요약. `#WAVxx`·`#BMPxx`는 16진 01-FF, 인코딩 규정 없음                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [Be-Music Source (Wikipedia)](https://en.wikipedia.org/wiki/Be-Music_Source)                                  | 확인   | 역사와 확장자 개요                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| [bmson 명세](https://bmson-spec.readthedocs.io/en/master/doc/index.html)                                      | 확인   | 1.0.0-beta(2015-12-26). Web IDL로 기술. 경로 규칙: 절대경로·상위 경로·널 문자는 거부. 재확인 조사는 이 주소를 열지 못하고 [`.org` 호스트](http://bmson-spec.readthedocs.org/en/master/doc/)에서 `Version 1.0.0-beta (2015/12/26)`를 읽었다                                                                                                                                                                                                                                                                                                                                  |
| [bmson 명세 포크](https://bmson-spec-fork.readthedocs.io/en/latest/doc/index.html)                            | 부분   | `DJKero/bmson-spec-fork`의 문서. 첫 제목 아래 `Version 2.0.0 (2023/10/24)`(재확인 조사가 본문 확인). 1차 조사의 검색 결과에는 제목 `2.0.0-rc1`이 있었으나 본문에서 위치를 못 찾았다. 공식 `1.0.0-beta`를 대체한 판으로 보지 않는다. 저장소 본문은 열지 못했다                                                                                                                                                                                                                                                                                                               |
| [bmson-schema](https://github.com/JLChnToZ/bmson-schema)                                                      | 미확인 | "A JSON Schema for validating Bmson format"이라는 설명만 확인. 파일 본문, 대응 명세 버전, 라이선스, 공식 채택 여부를 모른다. 공식 `bemusic/bmson-spec`의 루트와 `doc`에서는 JSON Schema 파일을 찾지 못했다                                                                                                                                                                                                                                                                                                                                                                  |
| [bmspec](https://github.com/bemusic/bmspec)                                                                   | 부분   | Gherkin으로 쓴 실행 가능 명세. "공식 명세가 아니다"라고 명시. Unlicense                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| [Bemuse BMS 지원 문서](https://bemuse.ninja/project/docs/bms-support)                                         | 확인   | 지원하지 않는 항목: `#RANK`, `#TOTAL`, 지뢰, BGA 등. 두 조사는 끝에 `/`가 있는 주소로 열었고 "Last updated on Jul 27, 2026"을 확인했다                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| [Bemuse's BMS Extensions](https://bemuse.ninja/project/docs/bms-extensions/)                                  | 부분   | 별도 공식 페이지. 본문은 두 조사 모두 열지 못했고, 검색 발췌에 `#SCROLL01`, `#SCROLL02`, `#SPEED01`과 절 제목 "Base36 Channels"가 보인다(검색 발췌 수준). 검색 도구의 날짜 2026-07-27은 본문 갱신일로 확정하지 못했다                                                                                                                                                                                                                                                                                                                                                       |
| [beatoraja 楽曲製作者向け資料](https://github.com/exch-bms2/beatoraja/wiki)                                   | 미확인 | 위치만 안다(hitkey의 BMSE 도움말 `beatoraja.html`이 링크). 두 조사 모두 본문을 열지 못해 내용 사실은 없다                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| [jbmstable-parser Wiki](https://github.com/exch-bms2/jbmstable-parser/wiki)                                   | 미확인 | 난이도표 명세(`md5`, `sha256`, `ipfs` 필드)가 있다고 알려진 곳. 본문을 열지 못했고, 검색 결과가 없다는 것을 필드가 없다는 증거로 쓰지 않는다. 저장소 [jbmstable-parser](https://github.com/exch-bms2/jbmstable-parser)는 존재 확인                                                                                                                                                                                                                                                                                                                                          |
| [Guide to understand BMS format](https://cosmic.mearie.org/2005/03/bmsguide/)                                 | 부분   | hitkey가 참고 자료로 인용. Qwilight가 이를 참고해 파싱한다고 원본에 기록. 두 조사가 본문을 읽었으나 명령 표가 추출에서 빠졌고 끝이 잘렸다. 본문 버전 `1.2.2 (2005.3.22)`, 2013-07-05 노후화 안내, 서두에 GFDL 표시(1차 조사만 근거, 저자 "토끼군"). 작성자가 현행 BMS를 반영하지 못한다고 밝혀 현행 명세로 쓰지 않는다. 신빙성은 조사 값 B                                                                                                                                                                                                                                  |

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

| 자료                                                                                              | 상태 | 메모                                                                                             |
| ------------------------------------------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------------------------ |
| [bms-rs](https://github.com/MikuroXina/bms-rs) · [docs.rs](https://docs.rs/bms-rs/latest/bms_rs/) | 확인 | Apache-2.0. 1.0.0(crates.io 2026-04-18). 인코딩 감지·BOM 제거·경로 정규화 없음. 유지보수자 1~2명 |
| [bms-table](https://docs.rs/bms-table)                                                            | 확인 | 난이도표(header.json, data.json) 파서. Apache-2.0                                                |
| [BMS Search](https://bmssearch.net/)                                                              | 확인 | 곡 색인 사이트. API 문서는 찾지 못함                                                             |
