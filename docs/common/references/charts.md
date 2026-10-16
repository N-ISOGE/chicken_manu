# 참고 자료: 채보 포맷 색인, 엔진·파서·변환기, 에디터

[← 길라잡이][project-guide]

## 처음 읽는다면

[1. 채보 포맷 색인](#1-채보-포맷-색인)에서 자료 목록을 살펴보실 수 있습니다. 확인 범위는 아래 안내와 각 자료의 메모를 함께 살펴봅니다.

## 목차

- [1. 채보 포맷 색인](#1-채보-포맷-색인)
- [2. 엔진·파서·변환기](#2-엔진파서변환기)
- [3. 채보 에디터·저작 도구](#3-채보-에디터저작-도구)

채보 포맷을 모아 둔 색인과 포맷을 읽고 쓰는 엔진·파서·변환기, 채보 에디터·저작 도구의 출처를 안내합니다.
2절과 3절은 별도 조사(2026-10-09)에서 옮긴 내용입니다. 상태 열은 해당 조사가 기록한 확인 수준이며, 이 저장소에서 다시 검증한 결과는 아닙니다.

표의 버전·배포일·확인 상태는 기록된 조사 시점의 정보입니다. 현재 판본에 적용할 때는 원문과 대상 판본을 다시 대조합니다.

## 1. 채보 포맷 색인

채보 형식을 찾아볼 수 있는 공개 색인입니다. 각 자료의 소개를 바탕으로 다루는 범위를 적었습니다. 이 가운데 vsrg-format-docs는 별도 조사에서 직접 열어 확인했습니다.

| 자료                                                                                  | 범위                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [vsrg-format-docs](https://github.com/kangalio/vsrg-format-docs)                      | 마니아형 VSRG를 다룹니다. README의 `Formats` 절에 이름 36개와 이름 없는 확장자 7종이 있습니다. 기본 브랜치는 `master`이며 라이선스 표시는 없습니다. 확인한 최종 커밋 날짜는 2020-12-09입니다. |
| [GuitarGame_ChartFormats](https://thenathannator.github.io/GuitarGame_ChartFormats/)  | `.chart`, `.mid`, `song.ini` 등의 형식을 다룹니다.                                                                                                                                            |
| [rhythm-game-formats (SaxxonPike)](https://github.com/SaxxonPike/rhythm-game-formats) | DDR·IIDX 등 상용 형식을 역공학한 자료입니다.                                                                                                                                                  |
| [Project OutFox Wiki](https://outfox.wiki/)                                           | SM·SSC·KSF·TJA 호환 형식을 안내합니다.                                                                                                                                                        |
| [BSMG Wiki map format](https://bsmg.wiki/mapping/map-format.html)                     | Beat Saber v2/v3/v4 형식을 안내합니다.                                                                                                                                                        |
| [Sonolus Wiki](https://wiki.sonolus.com/)                                             | 엔진·레벨·리플레이의 명세를 제공합니다.                                                                                                                                                       |
| [UltraStar format](https://github.com/UltraStar-Deluxe/format)                        | 노래 게임의 채보 형식을 다룹니다.                                                                                                                                                             |

## 2. 엔진·파서·변환기

별도 조사(2026-10-09)에서 9종의 출처를 확인한 기록입니다. 일부 활동·갱신 시점은 연도나 월 단위로만 기록돼 있습니다. 라이선스가 한 가지로 확인되지 않은 항목은 `미확인`으로 표시했습니다.
도구 안에서 사용하는 공통 모델은 디렉터리·클래스 이름 수준으로 파악한 것이며, 구체적인 필드 정의는 미확인입니다. 변환 기능의 소개와 실제 변환 결과의 검증을 구별해 읽습니다.

| 자료                                                                                | 상태 | 메모                                                                                                                                                                                                                                                                                                                          |
| ----------------------------------------------------------------------------------- | ---- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [YARG.Core](https://github.com/YARC-Official/YARG.Core)                             | 확인 | C#으로 작성했으며 라이선스는 LGPL-3.0-or-later입니다. 2026-10 기준 활동이 활발한 것으로 기록돼 있습니다. `.mid`(DryWetMidi)·`.chart`(Moonscraper)를 런타임 차트로 읽습니다. 모델은 `SongChart`, `InstrumentTrack`, `SyncTrack`, `ChartEvent`, `Note`입니다. 번들된 Moonscraper 소스에는 BSD-3-Clause 고지가 있습니다.         |
| [RhythmCodex](https://github.com/SaxxonPike/RhythmCodex)                            | 확인 | C#·MIT 프로젝트이며, 2025-05 기준 작업 중(WIP)으로 기록돼 있습니다. 코나미 아케이드·콘솔 역공학 포맷, BMS, MIDI를 읽어 MIDI, StepMania, WAV로 변환합니다. 모델은 `Chart`, `Event`, `Track`, `LinearData`입니다. 1절의 rhythm-game-formats(SaxxonPike)와 **다른 저장소**입니다.                                                |
| [MuConvert](https://github.com/MuNET-OSS/MuConvert)                                 | 확인 | C#·GPL-3.0 프로젝트이며, 2026년 기준 활동이 활발한 것으로 기록돼 있습니다. maimai(Simai와 MA2 상호 변환), CHUNITHM(UGC와 C2S 상호 변환), ONGEKI(OGKR)를 다룹니다. 게임별 IR(`MaiChart`, `OgkChart`, `ChuniChart`)이고 README가 "IR에서 정보를 잃지 않는다"를 설계 목표로 밝힙니다. 초기 `SaltNya/MuConvert`에서 이관했습니다. |
| [rconv](https://github.com/prefixaut/rconv)                                         | 확인 | Rust 프로젝트이며, 2022년 기준 안정·보존 상태로 기록돼 있습니다. 라이선스는 조사에 "MIT 또는 Apache-2.0"으로 적혀 **미확인**입니다. Malody `.mc`, osu!, BMS, StepMania, bmson 등을 상호 변환합니다. trait·구조체 기반의 통합 차트 추상화를 사용하며 변환 손실(타이밍 양자화)에 대한 주의가 있습니다.                          |
| [simfile](https://github.com/garcia/simfile)                                        | 확인 | Python·MIT 프로젝트이며, 2026년 기준 활동이 활발한 것으로 기록돼 있습니다. `.sm`·`.ssc`를 읽고 씁니다. 모델은 `Simfile`(`SMSimfile`, `SSCSimfile`), `Chart`, `TimingData`입니다. `ashastral/simfile`에서 계정명이 바뀌었습니다.                                                                                               |
| [osu-parsers](https://github.com/kionell/osu-parsers)                               | 확인 | TypeScript·MIT 프로젝트이며, 2026년 기준 활동이 활발한 것으로 기록돼 있습니다. `.osu`, `.osb`, `.osr`, `.db`를 읽고 `.osu`·`.osb`를 재인코딩합니다. osu!lazer의 객체 모델을 따릅니다.                                                                                                                                         |
| [jubeatools](https://github.com/Stepland/jubeatools)                                | 확인 | Python·MIT 프로젝트이며, 확인한 판본은 v3.1.0(2026-09-14, PyPI 확인)입니다. `.eve`, `.jbsq`, `.mc`, `memon` 등을 상호 변환합니다. `Song` 아래에 복수 `Chart`를 두며 4x4 패널 좌표를 사용합니다.                                                                                                                               |
| [MaiConverter](https://github.com/donmai-me/MaiConverter)                           | 확인 | Python·MIT 프로젝트이며, 2026년 기준 활동이 활발한 것으로 기록돼 있습니다. Simai, Ma2(평문), `S*T`(평문), `S*B`(AES 암복호)를 다룹니다. 1소절은 384틱입니다. `ma2tosimai`는 완전한 Simai 파일이 아니라 채보만 만든다고 README에서 밝힙니다.                                                                                   |
| [sonolus-pjsekai-engine](https://github.com/NonSpicyBurrito/sonolus-pjsekai-engine) | 확인 | TypeScript·MIT 프로젝트이며, 2025-12 기준 공개 아카이브(읽기 전용) 상태이고 포크가 활발한 것으로 기록돼 있습니다. SUS를 중간 모델 USC를 거쳐 Sonolus LevelData(GZip JSON)로 변환합니다.                                                                                                                                       |

### 공통 모델을 비교할 때

조사 기록에서는 중간 표현(IR)을 다음 세 유형으로 나눴습니다.

- 게임 특화 IR: MuConvert, MaiConverter, jubeatools, USC
- 단일 게임군의 런타임 IR: YARG.Core, simfile, osu-parsers
- 여러 포맷을 다루는 IR: RhythmCodex, rconv

곡 컨테이너와 채보를 나누는 모델(`Song`/`Chart`, `Simfile`/`Chart`)은 이 저장소의 `binder`/`pattern` 구분과 비교해 볼 수 있습니다. 다만 도구의 IR은 노트·타이밍을, 이 저장소 스키마는 파일 식별과 묶음 관리를 다루므로 표현하는 수준이 다릅니다. 이 비교는 해석이며 모델 사이의 필드 대응이 검증됐다는 뜻은 아닙니다.

## 3. 채보 에디터·저작 도구

별도 조사(2026-10-09)에서 확인한 에디터와 저작 도구입니다. 각 행에 도구의 직접 주소와 확인 범위를 적었습니다. 버전·날짜·해시는 조사 기록의 값이며, 이 저장소에서 모든 원문과 실행 결과를 다시 대조한 것은 아닙니다.

| 자료                                                                                                                                              | 상태   | 메모                                                                                                                                                                                                             |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Moonscraper Chart Editor](https://github.com/FireFox2000000/Moonscraper-Chart-Editor)                                                            | 부분   | BSD-3-Clause로 표시돼 있으며 판본은 v1.5.13(2026-02-15)입니다. README에서는 Moonscraper 2 개발로 이 저장소의 주요 업데이트를 중단했다고 밝힙니다. `.chart`로 저장하며 모델은 미확인입니다.                       |
| [Editor on Fire (EOF)](https://github.com/raynebc/editor-on-fire)                                                                                 | 확인   | `license.txt`는 수정 BSD 3조항 문안입니다(GitHub은 NOASSERTION). 자체 프로젝트 형식은 revision H(헤더 `EOFSONH\0`)이며 `src/song.h`에 `EOF_SONG` 등의 타입이 있습니다. 확인한 최신 커밋 날짜는 2026-10-08입니다. |
| [FeedBack (보존 저장소)](https://github.com/TurkeyMan/feedback-editor)                                                                            | 부분   | 원본을 보존한 저장소입니다. 라이선스·저장 포맷은 미확인입니다. 버전·날짜(v0.97b, 2008-06-26)는 2차 자료의 값입니다.                                                                                              |
| [C3 Authoring Tools 안내](https://rhythmgamingworld.com/c3-authoring-tools/)                                                                      | 부분   | REAPER·RBN 도구·Magma·Nautilus·Onyx 등을 모은 안내입니다. 기록된 날짜는 2024-05-01입니다. 단일 포맷·라이선스는 없습니다.                                                                                         |
| RBN 문서: [Magma](http://docs.c3universe.com/rbndocs/index.php?title=Magma) · [Reaper](http://docs.c3universe.com/rbndocs/index.php?title=Reaper) | 부분   | 원본 Magma의 입력은 MIDI+WAV, 출력은 `.rba`입니다. REAPER 4.22는 legacy로 표시돼 있습니다.                                                                                                                       |
| [Magma: Rok On Edition](https://github.com/NemosNautilus/magma-rok-on)                                                                            | 부분   | 판본은 v4.1.0(2026-03-13)입니다. **프로젝트 라이선스는 미확인**입니다(README의 보존·교육 목적 문구는 허가문이 아닙니다). `.rbproj`/`.mid`를 `.rba`로 변환합니다.                                                 |
| [Onyx](https://github.com/mtolly/onyx)                                                                                                            | 부분   | 소프트웨어 라이선스는 GPL v3입니다. 입출력 목록은 미확인입니다.                                                                                                                                                  |
| [Nautilus (Rock Band)](https://github.com/NemosNautilus/Nautilus)                                                                                 | 부분   | `LICENSE.txt`는 있으나 본문은 검증하지 않았습니다. README의 기준일은 2026-07-29입니다.                                                                                                                           |
| [OCTAVE](https://github.com/opria123/octave)                                                                                                      | 확인   | 라이선스는 MIT이며 판본은 v1.0.28(2026-08-23)입니다. `.mid`/`.chart`/`song.ini`를 읽고 씁니다. "round-trip 무손실" 주장은 실행으로 검증하지 않았습니다.                                                          |
| [ArrowVortex](https://github.com/uvcat7/ArrowVortex)                                                                                              | 확인   | GPL-3.0-or-later이지만 README가 원저자에게 받은 원본 코드에 정식 라이선스가 없었다고 밝힙니다. 판본은 v1.0.1(2025-07-12)이며 `.sm .ssc .dwi .osu`를 읽고 씁니다.                                                 |
| [GrooveAuthor](https://github.com/PerryAsleep/GrooveAuthor)                                                                                       | 확인   | 라이선스는 MIT이며 판본은 v1.1.4(2026-04-13), 실행 환경은 .NET 10입니다. `.sm .ssc`를 다룹니다. "v0.4.1"은 오래된 검색 색인의 값으로 기록돼 있습니다.                                                            |
| [SMEditor](https://github.com/tillvit/smeditor)                                                                                                   | 부분   | 라이선스는 MIT이며 판본은 v1.0.0(2024-08-14)입니다. 작업 중(WIP)이며 `.sm .ssc`를 다룹니다.                                                                                                                      |
| [K-Shoot MANIA v1 (K-Shoot Editor)](https://github.com/kshootmania/ksm-v1)                                                                        | 확인   | 원본 편집기는 `kshooteditor.hsp`(HSP)이며 `.ksh`를 열고 저장합니다. HSP 3.6 이상에서 정상 작동하지 않는다고 README에서 밝힙니다. 코드는 MIT이지만 일부 음원·바이너리는 예외입니다.                               |
| [Beafowl/ksm-editor](https://github.com/Beafowl/ksm-editor)                                                                                       | 확인   | 브라우저 도구입니다. `.ksh`를 읽고 쓰며 `.kson`·ZIP으로 내보냅니다. **라이선스 표시는 없습니다.**                                                                                                                |
| [ArcCreate](https://github.com/Arcthesia/ArcCreate)                                                                                               | 부분   | Arcaea 커뮤니티 에디터입니다. 라이선스는 GPL-3.0이며 `.aff`를 다룹니다. 판본은 1.2.21(2024-08-21)입니다.                                                                                                         |
| [Phichain](https://github.com/Ivan-1F/phichain)                                                                                                   | 부분   | Phigros 비공식 도구 체인(Rust·Bevy)입니다. 코드는 LGPL-3.0이며 `assets/respack`은 CC BY-NC 4.0입니다. 초기 단계이며 저장 포맷은 미확인입니다.                                                                    |
| [Re:PhiEdit](https://pgrfm.miraheze.org/wiki/Re:PhiEdit/en)                                                                                       | 부분   | 근거는 2차 위키이며 원 배포·라이선스는 미확인입니다.                                                                                                                                                             |
| [Ched](https://github.com/paralleltree/Ched)                                                                                                      | 부분   | 라이선스는 MIT이며 판본은 v3.2.0(2021-07-23)입니다. `.sus`(Sliding Universal Score)로 내보냅니다. 본가 소프트웨어에 출력을 직접 입력하거나 본가 자산을 유용하는 것을 금지한다고 저장소에서 밝힙니다.             |
| [MikuMikuWorld (원본 후보)](https://github.com/crash5band/MikuMikuWorld)                                                                          | 미확인 | 조회 시 HTTP 404였으나 삭제·비공개·이동 여부는 단정하지 않습니다. 파생 포크가 여럿 있습니다.                                                                                                                     |
| [Margrete](https://umgr.inonote.jp/margrete/intro/)                                                                                               | 부분   | UMIGURI용 도구이며 `*.ugc` 출력을 확인했습니다. 소스·라이선스는 미확인입니다.                                                                                                                                    |
| [ChroMapper](https://github.com/Caeden117/ChroMapper)                                                                                             | 부분   | Beat Saber 맵 에디터이며 라이선스는 GPL-2.0입니다. `pushed_at`은 2026-10-06이며 릴리스 날짜는 아닙니다. 포맷 세부 내용의 근거는 BSMG 위키입니다.                                                                 |
| [Beat Saber 공식 문서](https://beatsaber.com/documentation/preparing-the-song/index.html)                                                         | 부분   | 공식 편집기의 사용 문서입니다. 출력 스키마·소스 공개 여부는 미확인입니다. BSMG를 공식 명세로 표기하지 않습니다.                                                                                                  |
| [Heaven Studio](https://github.com/RHeavenStudio/HeavenStudio)                                                                                    | 부분   | 라이선스는 MIT이며 조직 목록에서 공개 아카이브(2024-06-14)로 확인했습니다. RIQ 계열이며 Jukebox 저장소는 RIQ chart format 파서입니다. Nintendo 지식재산권은 별도로 구별합니다.                                   |
| [UNBEATABLE Chart Editor](https://dcellgames.itch.io/unbeatable-chart-editor)                                                                     | 부분   | 판본은 v1.0.5(2026-05-12)이며 권리 표기는 All Rights Reserved입니다. 채보 확장자는 미확인입니다.                                                                                                                 |

### 라이선스 확인 범위

소스가 공개돼도 프로젝트 라이선스가 없는 도구(Magma: Rok On Edition, Beafowl/ksm-editor), GPL 표기와 원본 코드 권리 유보가 함께 있는 도구(ArrowVortex),
일부 자산만 비상업인 도구(Phichain)가 있습니다. FeedBack, Re:PhiEdit 원본, MikuMikuWorld 원본, Margrete, Beat Saber 편집기의 라이선스는 미확인입니다.

[project-guide]: ../../index.md
