<!-- markdownlint-disable MD024 -->
# 변경 기록

이 문서는 [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/) 형식과
[유의적 버전 2.0.0](https://semver.org/lang/ko/)을 따라 버전별 변동사항을 요약한다.

## 버전 규칙

- 아직 공개 API와 호환성 기준이 정해지지 않았으므로 **패치 번호만 올린다**(`0.1.x`).
- 마이너와 메이저는 공개 API나 데이터 형식(메타데이터 스키마)이 정해진 뒤에 올린다.
- 변경은 `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security`로 나눠 적는다.
- 로드맵과 풀어야 할 문제들은 이 문서가 아니라 `docs/notes.md`에 있다.
  이전 `change_log.md`의 시간 기반 기록은 아래 버전별 기록으로 대체했다.

`0.1.0` 이후의 버전 번호는 이 문서를 바꾸면서 Git 기록을 보고 소급해 붙인 것이다. 태그는 없다.
`Cargo.toml`의 버전은 아직 `0.1.0`이다.

## Unreleased

다음 릴리스는 `0.1.3`이 된다.

### Added

- `docs/`에 조사 문서를 추가하고 `cargo doc`으로 볼 수 있게 `src/docs.rs`에서 싣는다.
  - 기록·로드맵(`notes`), 참고 자료, 기술 채택 판단, 스키마와 식별자, 지원 대상과 명세의 형식화.
  - BMS 문법 초안(ABNF, `docs/bms.abnf`)과 실제 BMS 파일 189개로 한 검증 결과.
  - 다른 AI에게 조사를 맡기는 의뢰서를 `docs/briefs/`에 모았다(색인과 공통 규칙 포함).
    - B-01 `format-survey`: 리듬게임 채보 포맷·구동기·파서·규격의 기초 조사. BMS 계열뿐 아니라 기타 히어로 계열 chart, osu!, StepMania 등 채보 전반을 범위로 한다.
    - B-02 `medium-definition`: 매체의 구성과 형식을 사이버미디어 모델 등에 기대어 정의.
    - B-03 `copyright`: 저작권과 재배포 쟁점 조사.
    - B-04 `preservation-p2p`: p2p망에서 "열람만"이 가능한지, 보존 수준의 기준.

### Changed

- 린트 워크플로 폴더 이름의 오타를 고쳤다(`.github/workflow` → `.github/workflows`). GitHub Actions는 `workflows`만 읽는다.
- 린트 워크플로를 갱신했다. `actions/checkout` v4→v6(Node 20 지원 중단 경고), super-linter v6.8.0→v8.7.0, `ubuntu-latest`→`ubuntu-24.04`(2026-10-19부터 `ubuntu-latest`가 Ubuntu 26으로 바뀐다).
- 워크플로의 액션을 커밋 SHA로 고정하고 `persist-credentials: false`를 설정했다(zizmor 지적). codespell 설정(`.github/linters/.codespellrc`)을 추가했다.
- `src/main.rs`: 코드 검토(Copilot) 지적과 린트를 반영해 다시 정리했다.
  - `ipfs`를 셸(`pwsh -Command`) 없이 직접 실행하고 경로를 별도 인자로 넘긴다(파일명으로 명령이 실행될 수 있던 문제). 종료 상태와 CID 출력을 확인하고, 실행 파일은 `CHICKEN_MANU_IPFS`로 바꿀 수 있다.
  - libmagic은 하드코딩한 `D:/Tool/magic` 대신 시스템 기본 DB를 쓴다(`MAGIC` 환경 변수로 변경).
  - 읽기 버퍼를 길이 0으로 만들어 첫 줄이 NUL로 손상되던 문제를 고쳤다. 파일명은 `file_stem()`으로 구하고, 결과 폴더가 없으면 만든다.
  - 디렉터리나 읽을 수 없는 파일은 패닉 대신 `BMSReadError`로 돌려준다. `ToolFailure` 오류를 추가했다.
  - CSV의 파일명 필드를 인용한다.
  - 테스트가 임시 폴더에서 입력 파일을 직접 만들어 쓰고(ASCII, Shift-JIS, 잘못된 인코딩, 없는 파일, 디렉터리, CSV 인용), `rustfmt`와 clippy 경고를 정리했다.
- 린트가 지적한 문서의 용어 표기, 맨몸 URL을 고쳤다. 제목이 반복되는 `change_log.md`와 `docs/bms-parsers.md`는 파일 안에서만 MD024를 껐다.
- `Cargo.toml`: Cargo가 무시하던 `[env]` 항목을 지웠다.
- `README.md`를 정리했다. "매체"와 "에셋"을 정의하고(에셋은 매체가 참조하는 파일로 매체 바깥), 목적을 두 방향(매체의 형식 조사, 공유·보존 도구의 명세와 구현)으로 나눴다.
- `docs/index.md`: "표현"을 "매체"로 바꾸고 용어 절을 추가했다. 열린 결정에 사용자가 밝힌 방향(매체 정의와 저작권 검토는 다른 AI에게 위임, 도서관식 목록 관리, libp2p 기반 IPFS 후보와 추가 조사, 포맷 사이의 변환은 나중에)을 적었다.
- `change_log.md`를 버전별 변동사항 요약 문서로 바꿨다.
  로드맵과 문제들은 `docs/notes.md`로, 참고 자료는 `docs/references.md`로, BMS 문법 초안은 `docs/bms-grammar.md`로 옮겼다.
- `src/main.rs`: 크레이트 문서를 `docs/index.md`에서 가져오고 `docs` 모듈을 선언했다.

## 0.1.2 - 2024-08-09

### Added

- BMS 문법 초안(EBNF)을 적기 시작했다. 주석(`;`, `//`, `/* */`)을 고려했다.
- 참고 구동기에 Qwilight를 추가했다. 구동기를 추가하는 대신 구현을 보고 정리하는 방침을 기록했다.

### Changed

- 기록 문서의 구성을 정리했다.

## 0.1.1 - 2024-08-06

### Added

- BMS 항목: hitkey BMS command memo의 헤더 분류(패턴 정보용, 파일 정보용)와 고려할 점.
- BMSE가 대응하지 않는 header 목록.
- iBMSC와 μBMSC의 BMS 읽는 부분(`OpenBMS`, `SaveBMS`)과 다루는 header.
- `add-reference` 브랜치를 `dev`에 병합(#4).

## 0.1.0 - 2024-08-05

### Added

- 저장소 초기 설정: `README`, `.gitignore`(Rust, JetBrains, Visual Studio Code), Project IDX 환경(`.idx/dev.nix`)(2024-07-11부터).
- Cargo 패키지 `chicken_manu` 초기화(2024-08-02).
- 저장소 설정: `CODEOWNERS`, super-linter 린트 워크플로, `CODE_OF_CONDUCT.md`.
- BMS 파일 읽기: 문자표 기반(비 Unicode) 인코딩을 `encoding_rs`로 해석한다.
- 테스트 때 볼 로그 출력(`log`, `test-log`)과 함수 오류 형식.
- 로드맵과 기록 초안(`change_log.md`).
