# 프로젝트 노트

이전 `change_log.md`에 있던 로드맵과 "문제들"을 옮긴 것이다.
버전별 변동사항은 저장소 루트의 `change_log.md`(rustdoc의 [변경 기록](crate::docs::changelog))에 있다.
참고 자료는 [참고 자료](crate::docs::references), BMS 문법 초안은 [BMS 문법](crate::docs::bms_grammar)으로 옮겼다.

## 로드맵

### 스키마 정의하기

- METS에서 파생시켜서 메타데이터에 대한 스키마를 직접 최신화하는 부분을 줄임.
  - XML, encoding은 UTF-8로.

### 관리, 공유 및 배포 도구 작성

- rust 기반으로 작성.

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
