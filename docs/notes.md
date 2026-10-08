# 프로젝트 노트

이전 `change_log.md`에 있던 로드맵과 "문제들"을 옮긴 것이다.
버전별 변동사항은 저장소 루트의 `change_log.md`(rustdoc의 [변경 기록](crate::docs::changelog))에 있다.
참고 자료는 [참고 자료](crate::docs::references), BMS 문법 초안은 [BMS 문법](crate::docs::bms_grammar)으로 옮겼다.

## 로드맵

### 스키마 정의하기

- METS에서 파생시켜서 메타데이터에 대한 스키마를 직접 최신화하는 부분을 줄임.
  - XML, encoding은 UTF-8로.
- **표준을 따른다.** 메타데이터 형식을 직접 만들고 계속 고치는 일은 감당하기 어려우므로, 표준이 정한 요소를 그대로 쓰고
  표준에 없는 이 프로젝트 고유의 부분만 최소한으로 덧붙인다.
  보존에 필요한 메타데이터를 파악하는 일은 [의뢰서 B-05](crate::docs::briefs::preservation_metadata)에 맡긴다.

### 관리, 공유 및 배포 도구 작성

- rust 기반으로 작성.

## 작업 방식

### Git

- 브랜치는 `main`과 `dev` 두 개를 기본으로 한다.
  - `dev`: 통합용. 직접 푸시할 수 없고 PR로만 바꾼다.
  - `main`: 릴리스용. 다른 브랜치 작업이 끝난 뒤 `dev`에서 반영하기로 하고 지금은 멈춰 둔다.
- 작업은 `dev`에서 딴 **짧게 쓰고 지우는 브랜치**에서 하고, 끝나면 PR로 `dev`에 넣는다.
- 오래된 브랜치(`util-for-parsing`, `wrapper-bms-to-mets`, `add-reference`)는 이어 쓰지 않는다.
  내용은 PR로 `dev`에 들어가고, 그 뒤에 정리한다.

### 마크다운 서식

- md는 prettier(3.3.3)로 맞춘다: `npx prettier@3.3.3 --write <파일>`.
- 전체를 한 번에 바꾸지 않고 **고치는 파일부터** 맞춘다. 서식 검사(`MARKDOWN_PRETTIER`)는 모든 파일이 맞춰질 때까지 워크플로에서 끈다.
- 아직 맞추지 않은 파일은 `npx prettier@3.3.3 --check "**/*.md"`로 볼 수 있다.
  맞춘 파일: `docs/bms-parsers.md`.

## 문제들

이거가 있었나? -> BMSSearch  
이거 남아있긴 한건가? -> ?  
이 파일이 맞나? -> BMSSearch로는 부족  
이거를 뭐라 부르냐 -> 패턴이랑 관련 파일들?

*(정리 시 추가)* 위 질문들과 조사 결과의 대응.

| 문제 | 조사에서 알게 된 것 |
| --- | --- |
| 이거가 있었나? | BMS Search는 곡 목록 사이트이고 개발자용 API 문서는 찾지 못했다 |
| 이 파일이 맞나? | 채보 파일은 MD5 또는 SHA-256(원시 바이트)로 식별하는 것이 BMS 생태계의 관행이다([스키마와 식별자](crate::docs::schema)) |
| 이거를 뭐라 부르냐 | 스키마에서는 패턴(pattern), 파일 묶음은 binder로 정했다 |
