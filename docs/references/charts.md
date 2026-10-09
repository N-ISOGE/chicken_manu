# 참고 자료: 채보 포맷 색인, 엔진·파서·변환기, 에디터

채보 포맷을 모아 둔 색인과, 포맷을 읽고 쓰는 엔진·파서·변환기, 채보 에디터·저작 도구의 링크다.
2절과 3절의 행은 별도 조사(2026-10-09)에서 옮겼다. 상태 열은 조사가 스스로 적은 확인 수준이고, 메모의 A/B/C는 조사가 매긴 신빙성이다. 이 저장소가 다시 확인한 값이 아니다. 조사가 서로 다른 값을 낸 곳은 나중 조사(재확인)를 따랐다.

## 1. 채보 포맷 색인

별도로 정리 중인 채보 포맷·명세 원장(119행)에 "다른 조사에도 쓰는 입구"로 적힌 색인이다. 그 기재를 옮겼고,
vsrg-format-docs만 별도의 재확인 조사가 직접 열어 확인했다.

| 자료                                                                                  | 범위                                                                                                                                           |
| ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| [vsrg-format-docs](https://github.com/kangalio/vsrg-format-docs)                      | 마니아형 VSRG. readme `Formats` 절에 이름 36개와 이름 없는 확장자 7종. 기본 브랜치 `master`, 라이선스 표시 없음, 최종 커밋 2020-12-09(확인, A) |
| [GuitarGame_ChartFormats](https://thenathannator.github.io/GuitarGame_ChartFormats/)  | `.chart`, `.mid`, `song.ini` 등                                                                                                                |
| [rhythm-game-formats (SaxxonPike)](https://github.com/SaxxonPike/rhythm-game-formats) | DDR·IIDX 등 상용 원본 역공학                                                                                                                   |
| [Project OutFox Wiki](https://outfox.wiki/)                                           | SM·SSC·KSF·TJA 호환                                                                                                                            |
| [BSMG Wiki map format](https://bsmg.wiki/mapping/map-format.html)                     | Beat Saber v2/v3/v4                                                                                                                            |
| [Sonolus Wiki](https://wiki.sonolus.com/)                                             | 엔진·레벨·리플레이 스펙                                                                                                                        |
| [UltraStar format](https://github.com/UltraStar-Deluxe/format)                        | 노래 게임 포맷                                                                                                                                 |

## 2. 엔진·파서·변환기

별도 조사(2026-10-09)에서 옮겼다. 조사는 9종 모두 확인으로 적었지만 확인일이 작성일과 같고 일부 갱신일이 월 단위라서,
이 저장소가 검증한 값은 아니다. 라이선스가 한 가지로 정해지지 않은 곳은 `미확인`으로 뒀다.
"내부 공통 모델"은 조사가 디렉터리·클래스 이름 수준으로만 적었고 필드 정의는 확인되지 않았다.

| 자료                                                                                | 상태 | 메모                                                                                                                                                                                                                                                    |
| ----------------------------------------------------------------------------------- | ---- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [YARG.Core](https://github.com/YARC-Official/YARG.Core)                             | 확인 | C#, LGPL-3.0-or-later, 2026-10 활발. `.mid`(DryWetMidi)·`.chart`(Moonscraper) → 런타임 차트. 모델 `SongChart`, `InstrumentTrack`, `SyncTrack`, `ChartEvent`, `Note`. Moonscraper가 소스에 번들돼 BSD-3-Clause 고지. A                                   |
| [RhythmCodex](https://github.com/SaxxonPike/RhythmCodex)                            | 확인 | C#, MIT, 2025-05, 진행 중(WIP). 코나미 아케이드·콘솔 역공학 포맷, BMS, MIDI → MIDI, StepMania, WAV. 모델 `Chart`, `Event`, `Track`, `LinearData`. 1절의 rhythm-game-formats(SaxxonPike)와 **다른 저장소**다. A                                          |
| [MuConvert](https://github.com/MuNET-OSS/MuConvert)                                 | 확인 | C#, GPL-3.0, 2026 활발. maimai(Simai와 MA2 상호 변환), CHUNITHM(UGC와 C2S 상호 변환), ONGEKI(OGKR). 게임별 IR(`MaiChart`, `OgkChart`, `ChuniChart`)이고 README가 "IR에서 정보를 잃지 않는다"를 설계 목표로 밝힌다. 초기 `SaltNya/MuConvert`에서 이관. A |
| [rconv](https://github.com/prefixaut/rconv)                                         | 확인 | Rust, 2022 안정·보존. 라이선스는 조사에 "MIT 또는 Apache-2.0"으로 적혀 **미확인**. Malody `.mc`, osu!, BMS, StepMania, bmson 등 상호 변환. trait·구조체 기반 통합 차트 추상화. 변환 손실(타이밍 양자화) 유의 안내. B                                    |
| [simfile](https://github.com/garcia/simfile)                                        | 확인 | Python, MIT, 2026 활발. `.sm`·`.ssc` 양방향. 모델 `Simfile`(`SMSimfile`, `SSCSimfile`), `Chart`, `TimingData`. `ashastral/simfile`에서 계정명 변경. A                                                                                                   |
| [osu-parsers](https://github.com/kionell/osu-parsers)                               | 확인 | TypeScript, MIT, 2026 활발. `.osu`, `.osb`, `.osr`, `.db` 읽기, `.osu`·`.osb` 재인코딩. osu!lazer 객체 모델 준거. A                                                                                                                                     |
| [jubeatools](https://github.com/Stepland/jubeatools)                                | 확인 | Python, MIT, v3.1.0(2026-09-14, PyPI 확인). `.eve`, `.jbsq`, `.mc`, `memon` 등 상호 변환. 모델 `Song` 아래 복수 `Chart`, 4x4 패널 좌표. A                                                                                                               |
| [MaiConverter](https://github.com/donmai-me/MaiConverter)                           | 확인 | Python, MIT, 2026 활발. Simai, Ma2(평문), `S*T`(평문), `S*B`(AES 암복호). 1소절 = 384틱. `ma2tosimai`는 완전한 Simai 파일이 아니라 채보만 만든다고 README가 밝힘. A                                                                                     |
| [sonolus-pjsekai-engine](https://github.com/NonSpicyBurrito/sonolus-pjsekai-engine) | 확인 | TypeScript, MIT, 2025-12 공개 아카이브(읽기 전용), 포크가 활발. SUS → 중간 모델 USC → Sonolus LevelData(GZip JSON). A                                                                                                                                   |

조사가 정리한 내부 공통 모델의 세 패턴: ① 게임 특화 IR(MuConvert, MaiConverter, jubeatools, USC), ② 단일 게임군 런타임 IR(YARG.Core, simfile, osu-parsers),
③ 다포맷 IR(RhythmCodex, rconv). 곡 컨테이너와 채보를 나누는 모델(`Song`/`Chart`, `Simfile`/`Chart`)이 여럿이라는 점이 이 저장소 스키마의 `binder`/`pattern` 구분과 닿는다.
다만 이 도구들의 IR은 노트·타이밍 수준이고 이 저장소 스키마는 파일 식별과 묶음 관리 수준이라 층이 다르다(해석이며 조사가 한 말이 아니다).

## 3. 채보 에디터·저작 도구

별도의 에디터 조사와 재확인(2026-10-09)에서 옮겼다. 재확인이 1차 조사와 겹치는 6건(EOF, ArrowVortex, OCTAVE, Magma: Rok On Edition, K-Shoot, GrooveAuthor)은 재확인을 따랐다.
재확인 조사의 각주 번호는 주제가 맞지 않는 문헌을 가리켜서(예: KornShell) 쓰지 않고, 행 안의 직접 주소만 옮겼다. 해시와 날짜는 조사의 자기 보고다.

| 자료                                                                                                                                              | 상태   | 메모                                                                                                                                                                            |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Moonscraper Chart Editor](https://github.com/FireFox2000000/Moonscraper-Chart-Editor)                                                            | 부분   | BSD-3-Clause 표기, v1.5.13(2026-02-15). readme: Moonscraper 2 개발로 이 저장소는 주요 업데이트 중단. `.chart` 저장, 모델 미확인. A                                              |
| [Editor on Fire (EOF)](https://github.com/raynebc/editor-on-fire)                                                                                 | 확인   | `license.txt`는 수정 BSD 3조항 문안(GitHub은 NOASSERTION). 자체 프로젝트 형식 revision H(헤더 `EOFSONH\0`), `src/song.h`에 `EOF_SONG` 등 타입. 최신 커밋 2026-10-08. A (재확인) |
| [FeedBack (보존 저장소)](https://github.com/TurkeyMan/feedback-editor)                                                                            | 부분   | 원본 보존. 라이선스·저장 포맷 미확인. 버전·날짜(v0.97b, 2008-06-26)는 2차 자료 C                                                                                                |
| [C3 Authoring Tools 안내](https://rhythmgamingworld.com/c3-authoring-tools/)                                                                      | 부분   | REAPER·RBN 도구·Magma·Nautilus·Onyx 등 묶음의 입구. 2024-05-01. 단일 포맷·라이선스 없음. A                                                                                      |
| RBN 문서: [Magma](http://docs.c3universe.com/rbndocs/index.php?title=Magma) · [Reaper](http://docs.c3universe.com/rbndocs/index.php?title=Reaper) | 부분   | 원본 Magma 입력 MIDI+WAV, 출력 `.rba`. REAPER 4.22는 legacy. B                                                                                                                  |
| [Magma: Rok On Edition](https://github.com/NemosNautilus/magma-rok-on)                                                                            | 부분   | v4.1.0(2026-03-13). **프로젝트 라이선스 미확인**(README의 보존·교육 목적 문구는 허가문이 아니다). `.rbproj`/`.mid` → `.rba`. A                                                  |
| [Onyx](https://github.com/mtolly/onyx)                                                                                                            | 부분   | GPL v3(소프트웨어). 입출력 목록 미확인. A                                                                                                                                       |
| [Nautilus (Rock Band)](https://github.com/NemosNautilus/Nautilus)                                                                                 | 부분   | `LICENSE.txt` 존재, 본문 미검증. readme 기준일 2026-07-29. A                                                                                                                    |
| [OCTAVE](https://github.com/opria123/octave)                                                                                                      | 확인   | MIT, v1.0.28(2026-08-23). `.mid`/`.chart`/`song.ini` 입출력. "round-trip 무손실" 주장은 실행 검증하지 않았다. A (재확인)                                                        |
| [ArrowVortex](https://github.com/uvcat7/ArrowVortex)                                                                                              | 확인   | GPL-3.0-or-later이지만 README가 원저자에게 받은 원본 코드에 정식 라이선스가 없었다고 밝힌다. v1.0.1(2025-07-12). `.sm .ssc .dwi .osu` 읽기·쓰기. A (재확인)                     |
| [GrooveAuthor](https://github.com/PerryAsleep/GrooveAuthor)                                                                                       | 확인   | MIT, v1.1.4(2026-04-13), .NET 10. `.sm .ssc`. 1차 조사의 "v0.4.1"은 오래된 검색 색인으로 판정. A (재확인)                                                                       |
| [SMEditor](https://github.com/tillvit/smeditor)                                                                                                   | 부분   | MIT, v1.0.0(2024-08-14), 작업 중(WIP). `.sm .ssc`. A                                                                                                                            |
| [K-Shoot MANIA v1 (K-Shoot Editor)](https://github.com/kshootmania/ksm-v1)                                                                        | 확인   | 원본 편집기는 `kshooteditor.hsp`(HSP). `.ksh` 열기·저장. HSP 3.6 이상에서 정상 작동하지 않는다고 README가 밝힘. 코드 MIT, 일부 음원·바이너리 예외. A (재확인)                   |
| [Beafowl/ksm-editor](https://github.com/Beafowl/ksm-editor)                                                                                       | 확인   | 브라우저 도구. `.ksh` 입출력, `.kson`·ZIP 내보내기. **라이선스 표시 없음.** A (재확인)                                                                                          |
| [ArcCreate](https://github.com/Arcthesia/ArcCreate)                                                                                               | 부분   | Arcaea 커뮤니티 에디터. GPL-3.0, `.aff`, 1.2.21(2024-08-21). A                                                                                                                  |
| [Phichain](https://github.com/Ivan-1F/phichain)                                                                                                   | 부분   | Phigros 비공식 도구 체인(Rust·Bevy). LGPL-3.0, `assets/respack`은 CC BY-NC 4.0. 초기 단계, 저장 포맷 미확인. A                                                                  |
| [Re:PhiEdit](https://pgrfm.miraheze.org/wiki/Re:PhiEdit/en)                                                                                       | 부분   | 2차 위키만 근거. 원 배포·라이선스 미확인. C                                                                                                                                     |
| [Ched](https://github.com/paralleltree/Ched)                                                                                                      | 부분   | MIT, v3.2.0(2021-07-23). `.sus`(Sliding Universal Score) 내보내기. 본가 소프트웨어에 출력을 직접 입력하거나 본가 자산을 유용하는 것을 금지한다고 저장소가 밝힘. A               |
| [MikuMikuWorld (원본 후보)](https://github.com/crash5band/MikuMikuWorld)                                                                          | 미확인 | 조회 시 HTTP 404(삭제·비공개·이동 여부 단정 못 함). 파생 포크 다수. C                                                                                                           |
| [Margrete](https://umgr.inonote.jp/margrete/intro/)                                                                                               | 부분   | UMIGURI용. `*.ugc` 출력 확인. 소스·라이선스 미확인. A                                                                                                                           |
| [ChroMapper](https://github.com/Caeden117/ChroMapper)                                                                                             | 부분   | Beat Saber 맵 에디터. GPL-2.0, `pushed_at` 2026-10-06(릴리스 날짜 아님). 포맷 세부는 BSMG 위키 의존이라 B. A                                                                    |
| [Beat Saber 공식 문서](https://beatsaber.com/documentation/preparing-the-song/index.html)                                                         | 부분   | 공식 편집기 사용 문서. 출력 스키마·소스 공개 여부 미확인. BSMG를 공식 명세로 표기하지 않는다. A                                                                                 |
| [Heaven Studio](https://github.com/RHeavenStudio/HeavenStudio)                                                                                    | 부분   | MIT, 조직 목록에서 공개 아카이브(2024-06-14). RIQ 계열, Jukebox 저장소가 RIQ chart format 파서. Nintendo 지식재산권은 별도. A                                                   |
| [UNBEATABLE Chart Editor](https://dcellgames.itch.io/unbeatable-chart-editor)                                                                     | 부분   | v1.0.5(2026-05-12), All Rights Reserved. 채보 확장자 미확인. A                                                                                                                  |

보존 관점의 주의: 소스가 공개돼도 프로젝트 라이선스가 없는 도구(Magma: Rok On Edition, Beafowl/ksm-editor), GPL 표기와 원본 코드 권리 유보가 함께 있는 도구(ArrowVortex),
일부 자산만 비상업인 도구(Phichain)가 있다. FeedBack, Re:PhiEdit 원본, MikuMikuWorld 원본, Margrete, Beat Saber 편집기의 라이선스는 미확인이다.
