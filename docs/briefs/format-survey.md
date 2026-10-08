# 기초 조사 지침: 리듬게임 채보 포맷·구동기·파서·규격

다른 AI 어시스턴트(Perplexity, ChatGPT, Gemini 등)가 조사를 이어받을 때 **먼저 읽는 문서**다.
이 문서만 읽어도 시작할 수 있게 썼다. 작성 기준일은 2026-10-08이다.

## 1. 배경과 목적

- 프로젝트: `chicken_manu`(치킨메뉴). 리듬게임 채보(chart, 譜面, beatmap, simfile 등)와 그에 딸린 파일(음원, 이미지, 영상)을 한 묶음으로 관리·공유·배포하는 Rust 도구다.
- **범위: 문서와 코드에서 "BMS, ..."라고 쓴 것은 BMS 하나가 아니라 리듬게임 채보 전반을 뜻한다.** BMS 계열(`.bms .bme .bml .pms`)은 도구가 시작한 곳이라 가장 깊이 조사했을 뿐이다. 기타 히어로 계열의 `.chart`와 `.mid`, osu!의 `.osu`, StepMania의 `.sm`과 `.ssc` 같은 포맷도 지원 후보다.
- 도구가 하는 일: 채보 파일을 읽어 메타데이터와 **파일 참조**(채보가 가리키는 음원, 배경, 배너, 영상 등)를 뽑고, 파일별 해시와 IPFS CID를 계산해 스키마(METS 파생 XML 예정)에 기록한다.
- 이 조사의 목적: 포맷, 구동기(플레이어), 에디터, 파서, 규격 문서를 **하나씩 세부 문서로** 정리해서, "어떤 구현이 채보와 딸린 파일을 어떻게 해석하는가"를 근거와 함께 알 수 있게 한다. 그 결과로 도구가 **최소한 지원할 대상**을 정한다.
- 현재 단계: **사실 수집**이다. 어떤 기술이 낫다는 평가나 추천은 하지 않는다. 채택 판단은 별도 문서의 몫이다.

## 2. 지켜야 할 원칙

1. **1차 출처 우선.** 공식 사이트, 명세, 소스 저장소, 개발자 문서 순으로 쓰고, 2차 자료는 따로 표시한다.
2. **추측 금지.** 버전, 날짜, 라이선스, DOI/ISBN, URL은 확인하지 못했으면 `미확인`으로 적는다. 비슷해 보이는 값으로 채우지 않는다.
3. **확인 상태와 신빙성을 분리한다.**
   - 확인 상태: `확인`(원문을 직접 열어 확인) / `부분`(일부만 읽음, 요약 도구 결과에 기댐) / `미확인`(찾지 못했거나 열지 못함. 이유 기록).
   - 신빙성: A(공식·1차 소스) / B(구현체 문서, 신뢰할 만한 커뮤니티 명세) / C(오래된 비공식 글, 2차 해설) / X(제외).
4. **원어 보존.** chart, 譜面, 谱面, 보면, beatmap, simfile, level, choreography처럼 출처가 쓰는 말을 그대로 적고, 대응 관계는 따로 붙인다. 통일하지 않는다. 한국어 설명을 덧붙이고 전문 용어는 원어를 병기한다.
5. **날짜를 적는다.** 모든 항목에 확인일(YYYY-MM-DD)을 적는다. 버전과 날짜는 "조사 시점의 값"이다.
6. **접근하지 못한 이유를 남긴다.** HTTP 상태 코드, 인증서 오류, 폐쇄 사이트, 다운로드 불가 등을 `미확인` 옆에 적는다. 웹 아카이브 사본을 쓰면 사본 URL과 스냅숏 날짜를 적는다.
7. **번역이 아니라 원문을 확인한다.** 번역 페이지나 기계 번역 요약에만 기댄 사실은 `부분`으로 둔다.
8. **코드와 문서를 대량으로 옮기지 않는다.** 짧은 인용(한두 문장, 한 함수명)만 쓴다. 라이선스가 표시되지 않은 저장소의 코드는 복사하지 않고 동작만 설명한다.
9. **AI가 자동 생성했을 가능성이 있는 사이트는 근거로 쓰지 않는다**(신빙성 X). 예: `deepwiki.com`, `<owner>-<repo>.mintlify.app` 형태의 자동 문서, `grokipedia.com`, `fretsonfire.wiki`, AI 블로그, 시장조사 스팸 사이트. 발견하면 "제외 출처"로 기록만 한다.
10. **충돌하는 증거는 둘 다 적는다.** 구현마다 동작이 다르거나 문서와 코드가 다르면 해소하려 하지 말고 나란히 기록한다.

## 3. 이미 확인된 것 (다시 조사하지 말 것)

새 조사와 어긋나면 어긋남을 기록한다.

### 3.1 BMS 계열

- 원 명세(1998, Urao Yane·NBK)는 `#`로 시작하는 줄만 명령이고 나머지는 무시한다. 명령은 대소문자를 구분하지 않고 줄 순서는 자유다. 인코딩은 규정하지 않는다.
- 줄은 두 종류다. 헤더 문장 `#HEADER value`와 채널 문장 `#xxxCH:data`(xxx는 마디 3자리, CH는 채널 2자리, data는 2자리 인덱스의 나열이며 `00`은 쉼표). 인덱스는 초기에 16진, 현재 구현은 36진(`0-9A-Za-z`, 1296칸)이다.
- hitkey BMS command memo(최종 갱신 2014-07-11)는 사실상 표준이다. **그러나 이 문서에는 `#PREVIEW`, `#BASE`, `#LNMODE`가 없다**(원문 전체를 확인). `#CHARSET`은 있으나 obsolete로 표시돼 있다. 2014년 이후의 확장은 반영하지 않는다.
- 같은 헤더가 중복되면 EOF에 가까운 쪽을 쓴다. 예외: `#ExtChr`, `#STP`, `#WAVCMD`, `#OPTION`, 제어 흐름. `#SUBTITLE`, `#SUBARTIST`, `#COMMENT`, `#LNOBJ`는 여러 개를 허용하는 구현이 있다.
- 주석 문법(`;`, `//`, `/* */`)은 IIDXv, HDX, outliner 등의 확장이다. 원 명세에는 없다.
- beatoraja의 파서인 jbms-parser가 하는 일(소스 확인): 인코딩은 BOM → EUC-KR → MS932 → UTF-8 → UTF-16/32 순으로 64KB 표본을 시험하고 기본값은 MS932다. `#BASE`는 사전 스캔하며 62 또는 36이다. `\`는 `/`로 바꾼다. MD5와 SHA-256을 파일 원시 바이트에서 계산한다. 이 저장소의 라이선스는 표시되어 있지 않다.
- Rust 파서 bms-rs 1.0.0은 인코딩 감지, BOM 제거, 경로 정규화(역슬래시·대소문자)를 하지 않는다. `#BASE 62` 지원은 확인하지 못했다.
- 채보 식별은 BMS 생태계에서 파일 원시 바이트의 MD5 또는 SHA-256이 관행이다(난이도표 spec의 `md5`/`sha256` 필드).
- 로컬에서 실제 BMS 파일 189개를 조사한 결과: 인코딩은 UTF-8 85, CP932 96, EUC-KR 8개였고 BOM이 있는 파일은 없었다. macOS 메타 파일 `._*`가 섞여 있었다. 한 컬렉션의 값이라 일반화하지 말 것.

### 3.2 다른 계열

이 프로젝트가 직접 원문을 확인한 사실은 **없다**. 아래 4.2절 표의 주소와 평가(신빙성, 등록 상태)는 사용자가 정리 중인 채보 포맷·명세 원장(119행)에서 옮긴 값이며, 각각 다시 확인해야 한다.

### 3.3 범위 밖

이 프로젝트의 기술 선택(IPFS, METS, iroh, Rust 크레이트)은 이 조사의 범위 밖이다.

## 4. 조사 대상 목록 (시작점)

아래 목록은 **시작점**이다. 접속 가능 여부, 현재 유지 상태, 후속 프로젝트가 있는지를 먼저 확인한다.
목록에 없는 구현을 발견하면 6절 서식으로 추가한다.

우선순위는 이 프로젝트가 "채보가 가리키는 파일과 인코딩의 해석"에 미치는 영향을 기준으로 한 **제안**이다.
P1은 널리 쓰이거나 명세·소스가 있어 근거를 얻기 쉬운 것, P2는 현행이지만 영향이 작거나 자료가 적은 것, P3는 과거 구현이나 자료가 없는 것이다.

### 4.0 계열 개요

| 계열 | 대표 포맷 | 원장 ID | 우선 |
| --- | --- | --- | --- |
| 건반·키음형(BMS 계열) | `.bms .bme .bml .pms`, bmson, DTX | KEY-01~15 | P1 (가장 깊이 조사됨, 4.1) |
| 기타·밴드형 | `.chart`, `.mid`, `song.ini`, `.sng` | GTR-01~10 | P1 |
| VSRG·osu! 계열 | `.osu`, `.qua`, `.mc` | VSR-01~11 | P1 |
| 패널·스텝형(StepMania 계열) | `.sm`, `.ssc`, `.dwi`, `.ksf` | PNL-01~11 | P1 |
| SDVX·아날로그형 | `.ksh`, `.kson` | ANA-01~03 | P2 |
| 노래·발성형 | UltraStar `.txt` | VOC-01 | P2 |
| 아케이드, 모바일, VR, 특수규칙·레벨형 | simai, `.aff`, Beat Saber, `.adofai` 등 | ARC, MOB, VRM, LVL | P3 (이후 단계) |
| 외부 표준·범용 후보 | MIDI, MusicXML, MEI, JAMS 등 | EXT-01~14 | P3 (참고용) |
| 색인·메타 자료 | 포맷 문서 모음 | IDX-01~09 | 입구로 사용 |

원장의 ID 규칙은 `FMT-<분류코드>-<번호>`다. 분류코드는 KEY, PNL, VSR, GTR, VOC, ANA, ARC, MOB, VRM, LVL, EXT, IDX, DRP(폐기)다.
원장은 사용자의 Google Drive 시트라 AI가 접근하지 못할 수 있다. 접근하지 못하면 이 문서의 표를 시드로 쓰고, 새 행은 CSV로 제출한다.

### 4.1 BMS 계열

#### 규격·문서

| 우선 | 이름 | 주소(조사 시작점) | 메모 |
| --- | --- | --- | --- |
| P1 | BMS Format Specification(BM98 원전) | `http://bm98.yaneu.com/bm98/bmsformat.html` | 1998-11-26. 접속 가능 여부 확인 |
| P1 | hitkey BMS command memo | `https://hitkey.bms.ms/cmds.htm` | 이미 확인. 원 호스트 `https://hitkey.nekokan.dyndns.info/cmds.htm`는 재확인하지 못함 |
| P1 | beatoraja 楽曲製作者向け資料(확장 명세) | `https://github.com/exch-bms2/beatoraja/wiki` | `#BASE`, `#LNMODE`, `#DEFEXRANK` 등 확장 |
| P1 | Bemuse BMS 지원·확장 문서 | `https://bemuse.ninja/project/docs/bms-support` | 확장(`#SCROLL` 등) 페이지를 찾을 것 |
| P1 | 난이도표 명세(jbmstable-parser wiki) | `https://github.com/exch-bms2/jbmstable-parser/wiki` | `md5`/`sha256`/`ipfs` 필드 |
| P1 | bmson 명세 | `https://bmson-spec.readthedocs.io/en/master/doc/index.html` | 1.0.0-beta. JSON Schema 파일이 있는지 |
| P2 | BMS extensions proposed by Sonorous | `https://cosmic.mearie.org/f/sonorous/bmsexts` | 2013-07-10부터 |
| P2 | Guide to understand BMS format | `https://cosmic.mearie.org/2005/03/bmsguide/` | 2005-03-22 |
| P2 | Angolmois Internals | `https://github.com/lifthrasiir/angolmois/blob/master/INTERNALS.md` | 2013-03-09 |
| P2 | Basic specification of BML(RDM) | `https://nvyu.net/rdm/rby_ex.php` | |
| P2 | DTX 사양 Q&A | `http://dtxmania.net/wiki.cgi?page=qa_dtx_spec_e` | DTX는 `.bms`도 읽음 |
| P2 | LR2 beta3 Skin csv specification | `http://right-stick.sub.jp/lr2skinhelp.html` | 스킨 사양. 채보 해석과는 간접 관련 |
| P2 | bmspec(실행 가능 명세) | `https://github.com/bemusic/bmspec` | Gherkin, 공식 명세 아님 |
| P2 | Base62 reference(bms-rs README가 링크) | bms-rs 저장소 README에서 찾을 것 | `#BASE 62`의 권위 문서가 있는지 |
| P3 | wiki.bms.ms `Bms:Spec` | `https://web.archive.org/web/*/http://wiki.bms.ms/Bms:Spec` | 웹 아카이브 |
| P3 | MGQ 표기 | `https://web.archive.org/web/*/http://ivy.pr.co.kr/rdm/jp/extension.htm` | 2001-06-21 |

#### 구동기(플레이어)

| 우선 | 약칭 | 이름·버전(hitkey 2014 기재) | 조사 시작점 |
| --- | --- | --- | --- |
| P1 | LR2 | LunaticRave2 100201 | `https://web.archive.org/web/20110210225009/http://www.lr2.sakura.ne.jp/index2.html`. 소스 비공개. hitkey는 "일본의 사실상 표준"이라 적음 |
| P1 | beatoraja | beatoraja | `https://github.com/exch-bms2/beatoraja`, 파서 `https://github.com/exch-bms2/jbms-parser`. GPL-3.0(beatoraja) |
| P1 | Qwilight | Qwilight | `https://taehui.ddns.net/ko`. hitkey가 인용한 Guide to understand BMS format을 참고해 파싱한다고 알려짐 |
| P1 | Bemuse | Bemuse | `https://github.com/bemusic/bemuse`, 파서 `packages/bms` |
| P1 | ruvit | ruv-it! 2.0 b5p7 test #7 (2012-03-19) | `https://nvyu.net/rdm/`. 한국의 사실상 표준(hitkey) |
| P1 | Angolmois / Sonorous | Angolmois 2.0, Angolmois Rust Edition (2014-04-08), Sonorous 0.1.0-pre (2014-07-08) | `https://mearie.org/projects/angolmois/`, `https://github.com/lifthrasiir/angolmois-rust`, `https://cosmic.mearie.org/f/sonorous/` |
| P1 | uBMplay | uBMplay 1.5.2 | `http://ucn.tokonats.net/software/ubmplay/` |
| P2 | nanasi2 | nanasigroove2 beta (Toy Musical 3 Ver.2.2) | `http://d11x.sakura.ne.jp/asdf/` |
| P2 | pomu2 | Feeling Pomu Second Ver 0.8001 | `https://pmcc.nekokan.dyndns.info/pmcc2/download.html` |
| P2 | fgt# | forgetalia# (2011-04-16) | `https://cerebralmuddystream.nekokan.dyndns.info/soft/forgetalia_sp.zip` |
| P2 | HDX / IIDXv | charatbeatHDX VIOLET (v1.05) / BMIIDXView2010 v2.14 | `http://www.charatsoft.com/software/charatbeatHDX/index.html`, `http://www.charatsoft.com/software/bmview/index.html` |
| P2 | PMSee-V | PMSee-V v2.2.3 | `https://sakukoba.ninja-x.jp/ponila/` |
| P2 | o2mania, MyO2, D3beat | o2mania 1.2.0 / MyO2 2011-06-01 / D3beat ver1.1 | hitkey는 "to be tested"로 분류 |
| P3 | 과거 구동기 | BM98, BM98de, MGQ, DDR(Delight Delight Reduplication), RDM(rhythm-it 1.72a), MW(MixWaver), BmDx, bemaniaDX, nazo/nazoZZ, DXEmu, Mac(MacBeat), Aqua(Aqua'n Beats), nanasi, fgt++ | hitkey command memo의 "BMS apps" 표에 약칭과 주소가 있다 |

#### 에디터

| 우선 | 약칭 | 이름·버전 | 조사 시작점 |
| --- | --- | --- | --- |
| P1 | BMSE | BMx Sequence Editor 1.3.8 (+ dttvb 포크 `https://github.com/dtinth/UCN-BMSE`) | `http://ucn.tokonats.net/software/bmse/`, `https://github.com/Nekokan/BMSE` |
| P1 | iBMSC | iBMS BMS Creator 3.0.5 Delta | `https://github.com/aqtq314/iBMSC` |
| P1 | uBMSC | iBMSC의 후속(zardoru) | 정식 저장소 주소를 확인할 것. 기존 기록은 `https://github.com/zardoru/iBMSC`를 적음 |
| P2 | BmsONE | bmson 편집기 | 공식 저장소를 확인할 것 |
| P2 | DTXC | DTXCreator 026 (2014-07-07) | `https://en.osdn.jp/projects/dtxmania/releases/` |
| P3 | 그 밖의 과거 에디터 | BMSC(BMS Creator 2.0b1), beditor 1.3.1, GDAC2(GDA Creator), 774gsc | hitkey 표 참조 |

#### 뷰어·변환기·보조 도구

BMS를 읽는 코드가 있어 파일 참조 해석 사례로 쓸 수 있다. 우선순위는 낮다(P2 이하).
`bmx2wav`, `bme2wav`, `bms2wav`, `BmsToAvi`, `BGAEncoder`, `BGAEncAdv`, `in_bm`(WAview), `in_bm2`, `bmse.kpi`, `BMS Viewer`, `BMEV`, `nBMplay`, `o2play`, `PMChr-V`, `woslicerII/III`, `BMx Outliner`(웹), `bms diff tool`(웹), `Be-Music Helper`(bmhelper), `Mid2BMS`, `TechnicalGroove`, `otama`, `GALLI`, `BMS Printer`, `lr2_pmsview_helper`, `SP2DP`, `3-4toD-E`, `BM-ND`, `Starry Music Beat`(iOS).
이름과 주소는 hitkey command memo의 "BMS apps" 표에 있다.

#### 파서·라이브러리

| 우선 | 이름 | 언어 | 조사 시작점 |
| --- | --- | --- | --- |
| P1 | jbms-parser | Java | `https://github.com/exch-bms2/jbms-parser` (`src/bms/model/BMSDecoder.java`) |
| P1 | bms-js(Bemuse) | TypeScript | `https://github.com/bemusic/bemuse/tree/master/packages/bms` |
| P1 | bms-rs | Rust | `https://github.com/MikuroXina/bms-rs`, `https://docs.rs/bms-rs` |
| P2 | bms-table | Rust | `https://docs.rs/bms-table` |
| P2 | jbmstable-parser | Java | `https://github.com/exch-bms2/jbmstable-parser` |
| P2 | Angolmois, Sonorous 파서부 | C, Rust | 위 구동기 저장소 |
| 찾을 것 | 그 밖의 BMS/bmson 파서 | 언어 무관 | GitHub(topics `bms`, `bmson`), crates.io, npm, PyPI, NuGet, Maven에서 `bms`, `bmson` 검색 |

### 4.2 다른 계열의 포맷

주소와 평가는 원장에서 옮겼다(신빙성, 등록 상태는 2026-09-23 기준). 모두 재확인 대상이다.
각 포맷에 대해 **명세, 이를 읽는 구현(게임, 에디터, 변환기), 파서 라이브러리**를 찾는다. 구현 목록은 아직 없다.

#### 기타·밴드형 (Guitar Hero, Rock Band, Clone Hero 계열)

| 우선 | 원장 ID | 포맷 | 조사 시작점 | 원장 평가 |
| --- | --- | --- | --- | --- |
| P1 | GTR-01 | `.chart` (Feedback, Moonscraper, Clone Hero) | `https://thenathannator.github.io/GuitarGame_ChartFormats/` | B·추출, 문서 CC0 |
| P1 | GTR-02 | `.mid` (Guitar Hero·Rock Band MIDI 관례) | 위와 같음 | A·식별 |
| P1 | GTR-04 | `song.ini` | 위와 같음 | B·식별 |
| P1 | GTR-05 | `.sng` (컨테이너) | `https://github.com/mdsitton/SngFileFormat` | B·추출, MIT |
| P2 | GTR-03 | RBN / C3 저작 문서 | `http://docs.c3universe.com/rbndocs/index.php?title=Authoring` | A·식별 |
| P2 | GTR-09 | Phase Shift 확장 | `https://thenathannator.github.io/GuitarGame_ChartFormats/` | B·식별 |
| P3 | GTR-06, 07, 08, 10 | `songs.dta`, GH3 `.qb/.pak`, DJ Hero `.xmk`, Rocksmith `.psarc` | 원장 참조 | B~C·발견 |

#### VSRG·osu! 계열

| 우선 | 원장 ID | 포맷 | 조사 시작점 | 원장 평가 |
| --- | --- | --- | --- | --- |
| P1 | VSR-01 | `.osu` (v14 / lazer v128) | `https://osu.ppy.sh/wiki/en/Client/File_formats/osu_(file_format)` | A·추출 |
| P2 | VSR-02, 03 | `.osb` 스토리보드, `.osr` 리플레이 | `https://osu.ppy.sh/wiki/en/Storyboard/Scripting` 등 | A·추출 |
| P2 | VSR-04~06 | 포맷 논의, Beatmap Converts, lazer 용어 | `https://github.com/ppy/osu/discussions/12976` 등 | A·추출 |
| P2 | VSR-07 | Quaver `.qua` (YAML) | `https://github.com/Quaver/Quaver.API/blob/master/Quaver.API/Maps/Qua.cs` | A·추출 |
| P2 | VSR-08 | Malody `.mc` (JSON) | `https://prefixaut.github.io/rconv/rconv/malody.html` | B·추출 |
| P3 | VSR-09~11 | Rhythia `.sspm/.rhm`, FXF | 원장 참조 | B~C |

#### 패널·스텝형 (StepMania, DDR, Pump 계열)

| 우선 | 원장 ID | 포맷 | 조사 시작점 | 원장 평가 |
| --- | --- | --- | --- | --- |
| P1 | PNL-01 | `.sm` | `https://github.com/stepmania/stepmania/wiki/sm` | A·추출 |
| P1 | PNL-02 | `.ssc` (StepMania 5) | `https://github.com/stepmania/stepmania/wiki/ssc` | A·추출 |
| P2 | PNL-03 | Project OutFox (SM·KSF·TJA 흡수) | `https://outfox.wiki/en/dev/effects/EffectFiles-ChartSegments` | A·추출 |
| P2 | PNL-04, 05 | DWI, KSF | 원장 참조 | A~B·추출 |
| P3 | PNL-06~11 | UCS, STX/NOT5, 초기 DDR 시뮬레이터 포맷군, DDR `.ssq`, SMA, NotITG | 원장 참조 | B~C |

#### 그 밖

| 우선 | 원장 ID | 포맷 | 조사 시작점 | 원장 평가 |
| --- | --- | --- | --- | --- |
| P2 | ANA-01, 02 | KSH, KSON (K-Shoot MANIA) | `https://github.com/kshootmania/ksm-chart-format` | A·추출, CC0 |
| P2 | VOC-01 | UltraStar `.txt` | `https://github.com/UltraStar-Deluxe/format` | A·추출 |
| P3 | ARC, MOB, VRM, LVL, EXT | 아케이드(simai, TJA, SUS 등), 모바일(Arcaea `.aff` 등), VR(Beat Saber), 특수규칙(`.adofai` 등), 외부 표준(MIDI, MusicXML 등) | 원장 참조 | 이후 단계 |

#### 찾을 것 (원장 6절: 명세·역공학 문서를 아직 못 찾은 포맷)

EZ2DJ/EZ2ON, VOS, DJMAX(최신작), Lanota, Rotaeno, VOEZ, Deemo II, Groove Coaster, DANCERUSH, pop'n music·GITADORA 원본, Synchronica, O2Jam - The Beginning, Fortnite Festival, OhShape, Pistol Whip, Crypt of the NecroDancer, Muse Dash 공식 원본, Spin Rhythm XD(`.srtb`).

## 5. 대상마다 답해야 할 질문

모든 포맷과 구현(구동기, 에디터, 파서, 도구)에 같은 질문을 던져서 구현 간 비교표를 만들 수 있게 한다.
각 답에는 **근거**(URL, 소스 파일 경로와 함수명, 문서 구절)와 **확인 상태**를 붙인다. 답을 못 찾으면 `미확인`과 이유를 적는다.
"적용" 열은 그 질문이 어떤 대상에 해당하는지다. 해당하지 않으면 `해당 없음`이라고 쓴다(빈칸으로 두지 않는다).

| 번호 | 적용 | 질문 |
| --- | --- | --- |
| Q1 | 공통 | 정체: 원어 이름, 개발자·소속, 최초와 최종 릴리스의 버전·날짜, 유지 상태(활발/중단/종료), 플랫폼, 라이선스, 소스 공개 여부 |
| Q2 | 공통 | 읽는 파일: 확장자, 텍스트/바이너리, 컨테이너(ZIP 등) 여부, 버전 필드와 하위 호환, 암호화·난독화 여부 |
| Q3 | 공통 | **파일 참조**: 채보가 외부 파일(음원, 키음, 배경, 배너, 영상, 스토리보드, 폰트 등)을 가리키는 필드·헤더·섹션, 각각이 지원 / 파싱만 / 무시 / 오류 중 무엇으로 다뤄지는가 |
| Q4 | 공통 | 경로 규칙: 구분자(`\`와 `/`), 대소문자 구분, 하위 디렉터리, 상위 경로(`..`)와 절대경로, 확장자 대체, 못 찾았을 때의 동작, 최대 길이, 폴더 안 고정 파일명 규칙 |
| Q5 | 공통 | 인코딩: 기본 인코딩, 감지 방식(BOM, 휴리스틱, 헤더 선언), 파일명이 OS 로케일에 의존하는지, 알려진 문제 |
| Q6 | 공통 | 구문 허용 범위: 대소문자, 구분자, 들여쓰기, 값 생략, 비정상 값, 주석 문법, 줄바꿈(CRLF, CR, LF 혼용) |
| Q7 | BMS 계열 | 제어 흐름 `#RANDOM #SETRANDOM #IF #ELSEIF #ELSE #ENDIF #ENDRANDOM #SWITCH #SETSWITCH #CASE #SKIP #DEF #ENDSW`의 지원 범위, 중첩, `#ENDRANDOM` 생략 |
| Q8 | 포맷에 따라 | 중복 처리: 같은 키(헤더, 필드)가 반복될 때, 같은 위치의 데이터가 반복될 때(병합 여부) |
| Q9 | 공통 | 채보 식별: 어떤 해시(MD5, SHA-256, CRC32 등)를 어떤 바이트에서 계산하는가, 난이도표·랭킹·서버와의 연동 |
| Q10 | 포맷에 따라 | 확장 키·헤더와 그 동작(BMS의 `#LNTYPE #LNOBJ #LNMODE #BASE #SCROLL #SPEED #DEFEXRANK #DIFFICULTY #TOTAL #VOLWAV`처럼 포맷별로 정한다) |
| Q11 | 공통 | 알려진 버그, 비표준 동작, 다른 구현과 충돌하는 점 |
| Q12 | 공통 | 소스가 공개된 경우: 파서가 있는 파일 경로와 함수·클래스, 라이선스, 테스트(명세 테스트 포함) 유무 |
| Q13 | 공통 | **채보 단위**: 파일 하나에 채보가 몇 개 들어가는가(예: 한 파일 1채보, 한 파일 여러 난이도), 난이도·모드를 어떻게 구분하는가 |
| Q14 | 공통 | **묶음 단위**: 곡 메타데이터가 채보 파일 안에 있는가 별도 파일(`song.ini` 등)에 있는가, 폴더·압축 파일(`.osz`, `.sng` 등)의 구조와 확장자 |
| Q15 | 공통 | 시간 기저와 구조: 시간을 마디 격자, 틱, 박, 절대 시간 중 무엇으로 표현하는가, 구조 계열(헤더+채널, 태그 블록, INI 섹션, JSON/YAML, MIDI, 바이너리 등) |
| Q16 | 공통 | 배포 관행: 곡·채보를 어떻게 공유하는가(게임 내 서버, 외부 사이트, 압축 묶음)와 재배포에 관한 공개된 약관 |

닫힌 소스(예: LR2)는 문서와 커뮤니티 증거로만 답할 수 있다. 이때 신빙성을 C 이하로 적고, 어떤 증거인지 명시한다.

### 5.1 파일 참조(Q3)의 조사 힌트

아래는 어디를 보면 되는지에 대한 **힌트이며 확인된 사실이 아니다**(일반적인 지식에서 온 것으로, 조사하며 반드시 확인한다).

| 계열 | 볼 곳 |
| --- | --- |
| BMS | `#WAVxx`, `#BMPxx`, `#EXWAVxx`, `#EXBMPxx`, `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#PREVIEW`, `#CHARFILE`, `#MIDIFILE`, `#VIDEOFILE`, `#MOVIE`, `#PATH_WAV`, `#MATERIALSWAV`, `#MATERIALSBMP` |
| StepMania(`.sm`, `.ssc`) | 음악·배너·배경 등을 가리키는 태그(`#MUSIC`, `#BANNER`, `#BACKGROUND` 등)와 시뮬파일 폴더의 파일 규칙 |
| osu! | `[General]`의 오디오 파일 필드, `[Events]`의 배경·영상·스토리보드 항목, `.osz` 묶음 |
| 기타 히어로 계열 | `song.ini`와 폴더 안 고정 파일명, `.chart`의 `[Song]` 섹션, `.sng` 컨테이너 안의 파일 |

## 6. 항목 문서 서식

포맷 하나, 구현 하나, 규격 하나마다 마크다운 문서 한 개를 만든다. 아래 서식을 그대로 쓰고, 모르는 칸은 `미확인(이유)`로 채운다.

````text
# <원어 이름> (<한국어 설명>)

- 종류: 포맷 | 구동기 | 에디터 | 뷰어 | 변환기 | 파서 | 규격 | 컨테이너
- 계열: 건반·키음형 | 기타·밴드형 | VSRG·osu! | 패널·스텝형 | ...
- 원장 ID: FMT-xxx-nn (없으면 새로 제안)
- 약칭(slug): <소문자-하이픈>
- 확인일: YYYY-MM-DD
- 신빙성: A | B | C | X
- 관계: 후속·포크·참조하는 항목

## 개요
- 개발자·소속:
- 최초 릴리스(버전, 날짜):
- 최종 릴리스(버전, 날짜):
- 유지 상태:
- 플랫폼:
- 라이선스·소스:
- 주소: 공식 / 소스 / 웹 아카이브 사본(스냅숏 날짜)

## 질문별 답
| 번호 | 답 | 근거 | 상태 |
| --- | --- | --- | --- |
| Q2 | | | 확인 / 부분 / 미확인 |
| Q3 | | | |
(Q1~Q16 모두. 해당하지 않는 질문은 "해당 없음")

## 미확인 사항
- 항목, 이유(예: HTTP 403, 폐쇄 사이트)

## 조사 기록
- 사용한 검색어, 열어 본 문서, 배제한 출처(신빙성 X)와 이유
````

포맷과 규격 문서는 위 서식과 별도로, 이 프로젝트의 **채보 포맷·명세 원장**(22열)에 들어갈 CSV 행도 함께 낸다.
열 순서: `id`, 상위분류, 자료명, 자료유형, 포맷·확장자, 대응 게임·생태계, 원어 단위명, 시간 기저, 구조 계열, 입력·공간 모델, 선행 관행(에피스테메 가설), 공식성, 신빙성, 공개 소스·라이선스, 문서·예제·테스트, 유지보수 상태, 관계, 미확인 사항, 등록 상태, URL, 보조 URL, 확인일.
id는 `FMT-<분류코드>-<번호>` 형식이고, 해당 분류의 다음 번호부터 붙인다. 등록 상태는 발견 → 식별 → 검증 → 추출 → 비교 가능 → 폐기 중 하나다.

## 7. 파일 배치와 이름

- 항목 문서: `docs/refs/<slug>.md` (예: `docs/refs/lr2.md`, `docs/refs/osu-file-format.md`, `docs/refs/jbms-parser.md`).
- 계열 문서: `docs/refs/family-<계열>.md`. 계열의 개요, 포맷 목록, 구현 목록, 계열 안의 공통점과 차이를 한 장으로 요약한다.
- 색인: `docs/refs/index.md`에 한 줄씩 `약칭 | 종류 | 계열 | 우선순위 | 신빙성 | 확인 상태 요약 | 확인일`.
- 비교표: 같은 질문의 답을 모아 비교하는 표는 `docs/refs/matrix-<주제>.md`에 둔다. 예: `matrix-file-ref.md`(Q3), `matrix-path.md`(Q4), `matrix-encoding.md`(Q5), `matrix-container.md`(Q14), `matrix-time-base.md`(Q15).
- 문서를 rustdoc에 싣는 일은 저장소 관리자가 한다. 마크다운만 만들어서 넘기면 된다. 본문의 코드 블록에는 반드시 언어(`text`, `abnf` 등)를 적고, 대괄호는 백틱으로 감싼다.
- 문서 언어는 한국어이고 전문 용어는 원어를 병기한다. 표와 목록 중심으로 쓰고 서술은 최소화한다.

## 8. 하지 말 것

- 평가나 추천("이 구현이 더 좋다")을 쓰지 않는다. 사실과 근거만 적는다.
- 서로 다른 포맷의 요소를 같은 것으로 합치지 않는다("통합"은 이 단계의 일이 아니다). 대응 관계는 비교표에 기록만 한다.
- 확인하지 못한 값을 그럴듯하게 채우지 않는다.
- 요약 도구가 준 결과를 `확인`으로 표시하지 않는다.
- 다운로드한 실행 파일을 실행하지 않는다. 소스와 문서를 읽는 것으로 한정한다.
- 개인 정보(개발자의 연락처 등)는 공개 페이지에 이미 적힌 것이라도 소속과 닉네임 이상으로 옮기지 않는다.

## 9. 검색 어휘

- 일본어: `BMS 仕様`, `BMS フォーマット 仕様`, `BMS 拡張コマンド`, `BMS 文字コード`, `BMS 難易度表 仕様`, `譜面 フォーマット 仕様`.
- 한국어: `BMS 규격`, `BMS 확장 명령`, `채보 파일 포맷`.
- 영어: `Be-Music Source specification`, `bmson specification`, `chart file format`, `simfile format`, `beatmap file format`, `rhythm game chart format documentation`.
- 소스 저장소: GitHub topics `bms`, `bmson`, `beatmania`, `lunaticrave2`, `stepmania`, `osu`, `clone-hero`, `rhythm-game`; 패키지 레지스트리(crates.io, npm, PyPI, NuGet, Maven).
- 접속이 안 될 때: 웹 아카이브(`web.archive.org`)의 스냅숏을 쓰고, 사본임을 적는다.

## 10. 보고 형식

작업을 마치면 다음을 한 번에 보고한다.

1. 만든 항목 수(종류별, 계열별), 파일 목록.
2. 접속하지 못한 대상과 이유.
3. 구현끼리 충돌하는 증거의 목록.
4. 신빙성 X로 제외한 출처의 목록.
5. 이 지침과 어긋난 사실(특히 3절과 다른 발견, 원장 평가와 다른 발견).
6. 새로 발견해 원장에 추가할 행(CSV).
7. 다음에 조사할 만한 대상.
