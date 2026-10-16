# 포맷별 채보 단위와 파일 참조 필드

별도 조사(2026-10-09)에서 옮겼다. 조사는 열람 증거(HTTP 응답, 인용문)를 남기지 않았고 이 저장소에서 사실 확인을 하지 않았다.
"확인"은 조사의 자기 보고이므로 잠정으로 읽는다. 필수 여부 열은 대부분 "명시 확인 못함"이어서 스키마의 `NOT NULL` 근거로 쓰면 안 된다.
읽을 수 있는 사실은 bmson `info.init_bpm`(미지정 시 치명적 오류), Clone Hero `Name`(위키상 필수 태그), osu! 기본값(`PreviewTime` -1, `Mode` 0), bmson `resolution` 기본 240 정도다.

| 포맷                             | 파일당 채보                                                      | 다른 파일을 가리키는 필드                                                                                                                                                              | 시간 기저                                            | 조사의 확인 상태                                                                                                                                                        |
| -------------------------------- | ---------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| osu! `.osu` / `.osz`             | `.osu` 하나 = difficulty 하나. `.osz`(ZIP 기반)에 여러 개        | `AudioFilename`, `[Events]`의 배경·영상 이름. 비트맵 폴더 기준 상대 경로                                                                                                               | 오디오 시작 기준 ms, timing point는 beat length 기반 | 확인. 인코딩 규정은 못 찾음. 묶음·채보 외부 ID `BeatmapSetID`, `BeatmapID`                                                                                              |
| bmson 1.0.0-beta                 | 파일 하나 = 채보 하나                                            | `info.back_image`, `eyecatch_image`, `banner_image`, `preview_music`, `sound_channels[].name`, `bga.bga_header[].name`. bmson 파일 기준 상대 경로, `/`와 `\` 모두 허용하고 정규화 권고 | `y` pulse, `resolution` 기본 240 pulse/4분음표       | 확인                                                                                                                                                                    |
| StepMania `.sm` / `.ssc`         | `.sm`은 `#NOTES` 블록 여러 개, `.ssc`는 `#NOTEDATA` 블록 여러 개 | `#MUSIC`, `#BACKGROUND` 등 태그의 파일명                                                                                                                                               | measure/row 기반, `#BPMS`·`#STOPS`                   | 부분, 구현체 소스(`NotesLoaderSM.cpp`)에 기댐. 공식 위키는 못 읽음                                                                                                      |
| Clone Hero `.chart` / `song.ini` | `.chart` 하나에 여러 난이도·트랙                                 | 필드가 아니라 곡 폴더 안의 파일명 관행(`notes.chart`, `song.*`). 참조 필드를 모두 열람하지 못했다고 조사가 밝힘                                                                        | tick(`Resolution`)과 BPM·박자 이벤트                 | 부분. `song.ini` 섹션 표기가 조사 안에서 `[Song]`과 `[song]`으로 다름. 근거 주소도 `solamint.github.io`와 `thenathannator.github.io`로 다르고 어느 쪽이 현행인지 미확인 |
| KSH `.ksh`                       | 파일 하나 = 차트 하나(여러 난이도는 별도 파일 관행)              | `audio`, `jacket`(필드 이름은 `ksh_format.md`와 대조하지 못함)                                                                                                                         | measure/beat 기반                                    | 부분. 인코딩 미확인                                                                                                                                                     |
| Beat Saber `Info.dat`            | 여러 characteristic/difficulty 항목 가능(상한 미확인)            | `Info.dat`가 곡 메타와 비트맵·오디오·커버 경로를 가리킴                                                                                                                                | —                                                    | 부분(BSMG 위키, 공식 명세 아님)                                                                                                                                         |
| ADOFAI `.adofai`                 | "하나로 보임"(추정)                                              | 미확인                                                                                                                                                                                 | 각도 기반                                            | 부분(서드파티 파서 문서)                                                                                                                                                |
| BMS                              | 한 파일이 한 채보인 관행(근거 없는 서술)                         | `#WAVxx`, `#BMPxx` 번호와 파일명 ([BMS 문법][nav-1])                                                                                                                                   | 마디                                                 | 조사는 미확인. BM98 원전 주소 접속 실패                                                                                                                                 |
| Quaver `.qua` / `.qp`            | 한 맵셋에 복수 난이도 가능(상한 미확인)                          | 미확인                                                                                                                                                                                 | 미확인                                               | `.qp`는 부분, `.qua` 필드는 미확인. API에 `package_md5`                                                                                                                 |
| YARG `.sng`, DTX                 | 미확인                                                           | 미확인                                                                                                                                                                                 | 미확인                                               | 미확인. DTX 명세 주소 열기 실패                                                                                                                                         |

## 특정 판본의 소스 열람 (2026-10-14)

아래는 이 저장소에서 특정 판본의 공개 소스와 문서를 직접 열어 해당 줄을 대조한 내용이다. 확인한 것은 소스와 문서의 텍스트이고, 로더나 게임을 실행해서 얻은 값은 없다. 위 표의 다른 칸과 상태는 바뀌지 않는다.

StepMania (`stepmania/stepmania` 저장소, 커밋 `825467bc`, 2026-08-22 커밋):

- 곡 폴더를 읽는 `NotesLoader::LoadFromDir`(`src/NotesLoader.cpp`)는 `.ssc`, `.sma`, `.sm` 순으로 적용할 파일이 있는지 보고, 앞선 형식에 파일이 있으면 그 로더만 쓴다. 셋 모두 없을 때만 DWI, BMS, KSF를 같은 방식으로 시도한다.
- `.sm` 로더(`src/NotesLoaderSM.cpp`)는 `#NOTES`와 `#NOTES2` 태그마다 채보를 읽는다. 곡 태그 처리기에 `BPMS`와 `STOPS`가 있고 `FREEZES`는 `STOPS`와 같은 처리기를 쓴다.
- `.ssc` 로더(`src/NotesLoaderSSC.cpp`)는 `#NOTEDATA` 태그를 만나면 새 채보를 만들고, 이어지는 채보 태그와 `#NOTES`를 읽은 뒤 곡에 추가한다. 곡 수준의 `BPMS`, `STOPS`, `WARPS` 처리기와 채보별 처리기가 따로 있고, 채보별 타이밍은 파일 버전이 분리 타이밍 버전 이상일 때(또는 편집 파일일 때) 적용된다.
- 이미 있는 채보를 파일에서 다시 찾는 `SSCLoader::LoadNoteDataFromSimfile`은 채보 종류, 이름, 설명, 난이도, 레벨, 크레딧을 차례로 비교하고 하나라도 다르면 그 채보를 건너뛴다. 파일 버전이 채보 이름 태그 버전 이상이면 `CHARTNAME`을 채보 이름과, `DESCRIPTION`을 설명과 비교한다. 그보다 낮은 버전이면 `DESCRIPTION` 값을 채보 이름과 비교한다.

YARG (`YARC-Official/YARG.Core` 저장소, 커밋 `d42b96db`, 2026-10-11 커밋):

- `.chart` 리더(`MoonscraperChartParser/IO/Chart/ChartReader.cs`)는 첫째 섹션으로 `[Song]`, 둘째 섹션으로 `[SyncTrack]`을 요구하고, 없거나 순서가 다르면 예외를 던진 뒤 나머지 악기 섹션을 읽는다. `Resolution`이 없으면 192로 보며, 설정이 없을 때 HOPO 임계는 `Resolution * 65 / 192`다(해상도 192에서 65 tick). 이 값은 파일에 적힌 값이 아니라 구현의 기본값이다.
- `.mid` 리더(`MoonscraperChartParser/IO/Midi/MidReader.cs`)는 time division이 4분음표당 tick 방식이 아니면 예외를 던지고, 첫 트랙을 싱크 트랙으로 요구한다. 곡 전체 이벤트를 담은 EVENTS 트랙을 다른 트랙보다 먼저 찾아 읽는다. 설정이 없을 때 HOPO 임계는 `Resolution / 3 + 1`이다.

osu! (`ppy/osu-wiki` 저장소, 커밋 `85834aa`, 2026-10-13 커밋, osu! 파일 형식 문서):

- `Mode` 값 `3`이 osu!mania다. mania의 홀드 노트는 `x,y,time,type,hitSound,endTime:hitSample` 형태이고 `type`의 비트 7로 표시한다.
- 열 번호는 `floor(x * columnCount / 512)`로 계산하고 `0`과 `columnCount - 1` 사이로 제한한다. mania의 오브젝트는 `y`를 쓰지 않는다.

확인하지 못했거나 이 열람으로 말할 수 없는 것: StepMania 공식 위키의 서술, YARG 구현이 Clone Hero 같은 다른 게임의 동작과 같은지, 배포된 실행 파일이 위 커밋의 소스와 일치하는지, 같은 파일을 위 구현들이 실제로 어떻게 해석하는지.

관찰:

- `.sm`, `.ssc`, `.chart`는 파일 하나에 채보가 여럿이다. "채보 파일 하나 = 패턴 하나"라는 [스키마][nav-2]의 가정이 이 포맷들에서는 성립하지 않는다.
- 곡 단위 메타가 채보 파일 밖에 있는 포맷(Clone Hero `song.ini`, Beat Saber `Info.dat`)과 채보마다 중복 기재하는 포맷(osu!, StepMania)이 있다.
- 참조 필드의 모양이 BMS의 (헤더, 인덱스, 원문 경로)와 다르다. 키·태그형(osu!, StepMania, KSH), 배열 원소형(bmson), 이름 관행형(Clone Hero)이 있다.
- [지원 대상][nav-3]의 단계 표(1절)는 DTX, Quaver `.qua`, `.sng`를 지원 후보로 적었는데, 이 조사는 세 포맷 모두 필드를 확인하지 못했다. 다른 경로로 얻은 값일 수 있고 어느 쪽이 맞는지 모른다.

## 보컬형 매체의 표기 (2026-10-14)

이 저장소에서 아래 공개 문서와 특정 판본의 소스를 직접 열어 대조했다. 확인한 것은 문서와 소스의 텍스트이고, 프로그램을 실행해서 얻은 값은 없다.

UltraStar (`usdx.eu/format` 형식 문서, 페이지를 내려받아 본문 텍스트에서 대조했다. 열람 시점의 판은 1.1.0, 문서 표시 갱신일 2023-11-21):

- 이 문서는 1.1.0만 기술한다. 이전 판과의 차이나 판 표기가 없는 파일의 취급은 이 문서에서 확인할 수 없다.
- 필수 속성 항목에 `#AUDIO`와 `#VERSION`이 새로 올라 있고, `#AUDIO`는 `#MP3`를 대체한다(`#MP3`는 2025년에 없앨 예정이라고 적혀 있다).
- 본문 노트 종류는 `:` 일반, `*` 황금, `F` 프리스타일(점수 없음), `R` 랩, `G` 랩 황금이고, `-`는 구절의 끝이며 다음 구절의 시작 박을 함께 적는다.
- `#BPM`은 "4분음표 단위"이며 곡의 실제 BPM의 4배가 이상적이라고 적혀 있다. 어떤 곡이든 임의의 BPM으로 탭할 수 있어서 실제 템포와 꼭 관련되지는 않는다. MIDI와의 비교는 이 문서에 없다.

Rock Band 계열 보컬 MIDI (`YARC-Official/YARG.Core` 저장소, 커밋 `d42b96db`, `MidIOHelper.cs`):

- 트랙 이름 `PART VOCALS`와 하모니 트랙 `HARM1`~`HARM3`(The Beatles: Rock Band는 `PART HARM1`~`PART HARM3`)을 보컬 악기로 인식한다.
- 보컬 음높이 노트는 36~84, 타악은 96(재생)과 97(비재생)이다. 구절 표시는 105와 106이고, 소스 주석은 Rock Band 3가 이 대결용 구절 표시를 없애고 보컬에서는 105만 쓴다고 적는다. 스타 파워 노트는 116이다.
- 이 값들은 YARG 구현이 읽는 규칙이다. 다른 게임이나 도구가 같은 값을 쓰는지는 이 열람으로 말할 수 없다.

확인하지 못한 것: UltraStar 1.1.0 이전 판의 차이, Rock Band 보컬 MIDI의 원 제작 문서, Karaoke Revolution 판본별 내부 형식, 마이크 입력의 분석 방식.

[nav-1]: ../bms-grammar.md
[nav-2]: ../../cataloging/schema.md
[nav-3]: ../formats.md
