# 상용·모바일·VR 포맷 후보

[← 길라잡이][project-guide]

## 처음 읽는다면

[형식 단서를 확인한 대상](#2-형식-단서를-확인한-대상)에서 형식 표기와 구성 단서를 먼저 살펴보실 수 있습니다. [도구를 찾은 대상](#3-도구를-찾았지만-형식-근거가-더-필요한-대상)과 [근거를 더 확보할 후보](#4-공개-근거를-더-확보할-후보)는 확인한 범위를 구분해 읽습니다.

## 목차

1. [발견 목록과 확인 수준](#1-발견-목록과-확인-수준)
2. [형식 단서를 확인한 대상](#2-형식-단서를-확인한-대상)
3. [도구를 찾았지만 형식 근거가 더 필요한 대상](#3-도구를-찾았지만-형식-근거가-더-필요한-대상)
4. [공개 근거를 더 확보할 후보](#4-공개-근거를-더-확보할-후보)
5. [상세 비교로 이어가기](#5-상세-비교로-이어가기)

### 관련 문서

- [조사 대상 파악][nav-1]: 발견 목록과 상세 조사 순서
- [채보 단위와 파일 참조 필드][units]: 형식별 단위와 자료 연결
- [추가 확인 사항][checks]: 아직 답하지 못한 질문

## 1. 발견 목록과 확인 수준

상용 게임과 모바일·VR 게임에서 매체가 어떤 형식으로 존재하는지 알아보기 위한 목록입니다. 공개 명세뿐 아니라 편집기·파서·변환기의 설명에서도 형식의 이름과 구성을 찾을 수 있습니다.

게임명은 자료를 찾는 단서로 사용합니다. 실제로 기록할 것은 그 게임과 관련된 **형식·판본·자료의 관계**입니다. 같은 게임에도 원 게임의 자료, 커스텀 채보, 편집기 작업 파일, 변환 도구의 중간 형식이 함께 등장할 수 있습니다.

존재가 확인된 형식은 바이너리 여부나 명세의 공개 여부와 관계없이 발견 목록에 남깁니다. 공개 명세나 읽을 수 있는 구현체가 있는 대상부터 상세 조사하고, 근거에 접근하기 어려운 대상은 미확인·후순위로 표시합니다. 게임이나 도구만 찾았고 형식의 직접 근거는 확보하지 못했다면 형식 미확인으로 구분합니다. 이 원칙은 [조사 대상 파악][nav-1]과 같습니다.

아래는 **2026-10-16에 고정한 커밋의 README를 읽은 결과**입니다. 문서에서 확인한 표기와 설명의 범위를 적었으며, 개별 파서 소스·실제 파일·게임 실행은 대조하지 않았습니다. 링크는 읽은 판본의 README로 연결합니다.

## 2. 형식 단서를 확인한 대상

| 대상           | README에서 확인한 형식·구성 단서                                                                             | 공개 근거                      | 상세 조사에서 확인할 것                                         |
| -------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------ | --------------------------------------------------------------- |
| pop'n music    | `.bin` 채보를 파싱하며, `.bin`이 `.ifs`에서 나온다고 설명. 타임스탬프가 있는 이벤트와 구·신 형식을 구별      | [popn-chart-utils][popn]       | 두 형식의 실제 바이트 구조, 채보 판본, 컨테이너와 음원 참조     |
| GITADORA       | `seqtool.py`의 SQ2·SQ3·DTX·JSON 입출력과 JSON 중간 표현을 설명. VA3는 키음 등 소리 자료를 담는 묶음으로 설명 | [gitadora-customs][gitadora]   | 채보와 소리 묶음의 연결, 중간 표현에 옮기는 필드, 변환 손실     |
| VOS            | VOS 자료의 파싱과 MIDI 변환 기능을 설명                                                                      | [VosDroid][vos]                | 입력 판본과 파일 구조, 원본 정보와 MIDI 표현의 대응             |
| Synchronica    | `.osu`에서 fumen으로 변환하는 도구로 설명                                                                    | [OSynchronica][sync]           | 실제로 읽는 입력 파일과 출력 구조, 원 게임 자료와의 관계        |
| OhShape        | `.YML` 맵 파일이 `.OGG` 곡 파일을 참조한다고 설명                                                            | [OhShapeEditor][ohshape]       | 음원 참조 필드와 경로 기준, 맵 내부의 단위                      |
| Audica         | `.audica` 읽기·내보내기와 메타데이터·난이도별 cue 접근 예시를 제공                                           | [Audica .NET Tools][audica]    | 묶음 내부 파일, 시간 단위, 난이도·음원 사이의 연결              |
| Synth Riders   | `.synth` 파일과 클립보드에서 쓰는 JSON 표현을 다루는 도구로 설명                                             | [Synth Mapping Helper][synth]  | 파일의 외부 포장과 내부 JSON을 구별하고, 관련 자료 참조 확인    |
| Audio Trip     | `.ats`를 Audio Trip 자료로 안내. `.synth`를 Audio Trip으로 옮기는 사용법을 제공                              | [TripSitter][trip]             | `.ats` 내부 단위와 참조, 원본과 변환 결과의 대응                |
| Trombone Champ | MIDI에서 `song.tmb`를 생성하며, 출력은 JSON으로 설명. 사용자 곡 폴더에 `song.ogg`를 함께 두도록 안내         | [Midi2TromboneChamp][trombone] | 채보와 음원 연결이 필드인지 파일명 관행인지, MIDI에서 옮기는 값 |
| Spin Rhythm XD | `.srtb` 읽기·편집과 별도 JSON 내보내기, 음원·앨범 이미지를 포함한 ZIP 저장을 설명                            | [SRTBEditor][spin]             | 각 출력의 역할, 묶음 단위, 원시 자료와 편집 표현의 차이         |
| Rhythm Doctor  | `.rdlevel`을 JSON으로 연결하고 레벨용 스키마를 안내. 커스텀 캐릭터 스키마는 별도로 제공                      | [rd-schema][rd]                | 스키마 대상 판본, 레벨과 캐릭터 자료의 관계, 표현하지 않는 제약 |

이 표의 한 행은 포맷 하나를 뜻하지 않습니다. 예를 들어 GITADORA 행에는 채보·중간 표현·소리 묶음이 함께 나오므로 각각의 역할을 나누어 기록해야 합니다. 파싱이나 변환 기능의 설명은 해당 구현이 읽고 쓰는 범위를 알아볼 근거이며, 원 게임 형식 전체의 명세와 같다고 보지 않습니다.

TripSitter는 README에서 Audio Trip → Synth Riders 역방향을 미구현으로 표시하고, 게임 간 요소 차이로 동일한 변환을 만들 수 없다고 설명합니다. 도구 소개의 양방향 목표와 실제 제공되는 변환 방향을 구별해야 하는 사례입니다.

## 3. 도구를 찾았지만 형식 근거가 더 필요한 대상

| 대상           | 확인한 공개 자료                                                                  | 이 문서에서 확인한 범위와 남은 것                                                                                                 |
| -------------- | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| DANCERUSH      | [DRS Chart Editor][drs], 추가 후보 [drs2dd](https://github.com/thomasasfk/drs2dd) | 편집기 README는 제목만 있어 저장 형식을 확인하지 못함. 변환기와 편집기 소스를 추가 대조해야 함                                    |
| Groove Coaster | [Groove-Coaster-Converter][groove]                                                | README는 곡 관리·변환·항목 편집 도구라고 설명하지만, 대상 파일의 확장자·구조는 설명하지 않음                                      |
| EZ2ON          | [Arrowgene.Ez2Off][ez]                                                            | EZ2ON의 서버 에뮬레이터로 설명. 서버 자료와 채보 형식을 구분할 직접 근거 필요                                                     |
| Ragnarock      | [Edda][edda]                                                                      | 커스텀 곡을 레벨로 만드는 편집기로 설명. 저장 파일·스키마·음원 참조는 README만으로 확인하지 못함                                  |
| NecroDancer    | [necrodancer-custom-music][necro]                                                 | 커스텀 음악을 가져오며 음원·비트맵·세이브 파일을 연결하는 도구로 설명. 비트맵의 확장자·구조와 세이브 파일의 역할은 추가 대조 필요 |
| VOEZ           | [VoezEditor][voez]                                                                | README는 프로젝트 이름만 있어 저장 형식이나 원 게임 자료와의 관계를 확인하지 못함                                                 |

이 항목들은 도구에 관한 자료와 형식에 관한 자료를 구분해 기록합니다. README에 형식 설명이 없다는 사실은 형식이나 파서가 없다는 뜻이 아닙니다. 소스나 별도 문서가 있다면 그 위치와 판본을 확인해 상세 조사를 이어갑니다.

## 4. 공개 근거를 더 확보할 후보

| 대상                                                  | 현재 남은 질문                                                           |
| ----------------------------------------------------- | ------------------------------------------------------------------------ |
| EZ2DJ·DJMAX·O2Jam The Beginning                       | 대상 작품·판본의 채보 형식을 직접 설명하는 명세·파서가 있는가?           |
| ADOFAI·Dynamix·Beat Banger                            | 편집·배포 형식과 해석 규칙을 연결할 공개 문서 또는 소스 판본은 무엇인가? |
| Lanota·Rotaeno·Fortnite Festival·Pistol Whip·Deemo II | 형식의 존재를 뒷받침할 직접 근거와 상세 조사 자료를 확보할 수 있는가?    |

이 문서에는 위 후보들의 형식·판본을 확정할 직접 근거가 아직 연결되지 않았습니다. 자료를 찾으면 발견 상태와 상세 조사 조건을 각각 갱신합니다. 명세가 비공개이거나 바이너리라는 이유만으로 발견 목록에서 제외하지 않습니다.

## 5. 상세 비교로 이어가기

상세 조사에서는 채보·파일·묶음의 단위, 관련 자료를 찾는 필드와 경로, 판본별 해석 차이를 확인합니다. 변환기가 있으면 원본·중간 표현·출력 사이에 어떤 정보가 대응하는지 살펴봅니다.

확인한 규칙은 공개 출처의 절이나 소스 위치와 함께 [채보 단위와 파일 참조 필드][units]에 연결합니다. 아직 답하지 못한 질문은 [추가 확인 사항][checks]에서 이어갑니다. 도구의 라이선스와 그 도구가 다루는 게임 자료의 권리는 따로 확인합니다.

[popn]: https://github.com/fishluv/popn-chart-utils/blob/3b9bc1228d8ab23c3e2f2a305d2d8b47b00ed839/README.md
[gitadora]: https://github.com/fisyher/gitadora-customs/blob/be26e0d6a0151e655c2e19a99b23619cf7e728f0/README.md
[vos]: https://github.com/ReVanTis/VosDroid/blob/692f3def3293aabcd2414b4a1ff0bad6550ad5a8/README.md
[ohshape]: https://github.com/OddersLab/OhShapeEditor/blob/de38c3ab55100a7a685dddb83cf0249268a17f7c/Readme.md
[audica]: https://github.com/octoberU/Audica-.NET-Tools/blob/4bd952388ecbced27ae3f1e2c211296407197625/README.md
[synth]: https://github.com/adosikas/synth_mapping_helper/blob/e98fb6cb21dbe0d338a1c86808c6ebb26d010982/README.md
[trip]: https://github.com/Blogshot/trip-sitter/blob/a3eadf605ddf0f5e2ec9df7771d6d9b1c808fae6/README.md
[trombone]: https://github.com/NyxTheShield/Midi2TromboneChamp/blob/5f82695002295ba43749942802a1791454b57902/README.md
[spin]: https://github.com/plushproto/SRTBEditor/blob/e4fde8cbd67163dbb7f1fa8f176e3f40bbdd9f50/README.md
[rd]: https://github.com/0f-0b/rd-schema/blob/3e51c469e712aa6adb1137bb8d97d5250add85c7/README.md
[sync]: https://github.com/akechi-haruka/OSynchronica/blob/adb5c074ad41fb085aeccd7b7b89d3483b9b563e/README.txt
[drs]: https://github.com/Helloyunho/drs-chart-editor-web/blob/95cd8e111f298b0de7d465e46f3460a29bfd1125/README.md
[groove]: https://github.com/oocyberkevinoo/Groove-Coaster-Converter/blob/a2a1c78c603baaec10d78a8fae5525650e9a1db0/README.md
[ez]: https://github.com/sebastian-heinz/Arrowgene.Ez2Off/blob/abadfc282a0347b98a8b20a27b4a15e344755466/README.md
[edda]: https://github.com/PKBeam/Edda/blob/a9e7fe2de798c177568a58f642135b3343b93d2e/README.md
[necro]: https://github.com/Supinic/necrodancer-custom-music/blob/1330bd4d5db8742e5ba9c2646ab83a7633f70451/README.md
[voez]: https://github.com/AndrewFM/VoezEditor/blob/99353b587696ea9935091b1b803514c9267dec1b/README.md
[nav-1]: ../formats.md
[units]: chart-units.md
[checks]: format-checks.md
[project-guide]: ../../index.md
