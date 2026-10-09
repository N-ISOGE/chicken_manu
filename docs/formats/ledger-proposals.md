# 원장 보강 제안

별도 조사(2026-10-08)는 원장의 '발견·식별' 항목을 보강하자고 제안했다. 이 조사는 원장을 바꾸지 않고 "제안 등록 상태"만 적었으며,
조사 제목은 8건인데 표는 9행이다. 원장 반영은 아직 하지 않았다.

| 원장 행    | 자료                       | 현재 → 제안            | 새로 확인한 것                                                                                            | 확인·신빙성                                                              |
| ---------- | -------------------------- | ---------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| FMT-GTR-03 | RBN / C3 Documentation     | 식별 → 검증            | 이전 열람 실패는 HTTPS 자체서명 인증서 문제. 일반 HTTP로는 위키 전문이 열림                               | 확인, A                                                                  |
| FMT-ARC-05 | simai(`maidata.txt`)       | 식별 → 추출            | atwiki `pages/25.html`은 메타데이터 헤더 규격이고 기보 문법은 `pages/1002.html`(2026-07-10 갱신판)에 있음 | 확인, B                                                                  |
| FMT-MOB-03 | PEC(PhiEditor)             | 발견 → 검증            | 줄 단위 명령(`cp`, `cd`, `ca`, `cm`, `cr`, `cf` 등). Re:PhiEdit의 PEConverter로 상호 변환                 | 확인, B                                                                  |
| FMT-MOB-04 | RPE(Re:PhiEdit) JSON       | 식별 → 추출            | `META` + `judgeLineList`. 시간 단위 `[beat, numerator, denominator]`                                      | 확인, B                                                                  |
| FMT-ANA-03 | SOUND VOLTEX `.vox`        | 식별 → 검증            | Vox 10은 레이저 좌표 0~127 정수, Vox 12는 float(1/64 단위). 공식 명세는 비공개라 역공학 변환기로 실증     | 확인, B. 1차 명세 없음                                                   |
| FMT-GTR-01 | `.chart`                   | 추출 → 추출(경계 규명) | 원본 FeedBack은 레인 0~4, Clone Hero 확장은 레인 5(강제), 6(탭), 7(오픈 스트럼), GHL 6프렛                | 확인, B                                                                  |
| FMT-PNL-06 | UCS(Pump It Up)            | 식별 → 검증            | `ucs.piugame.com`이 PIU PHOENIX 전용으로 정상 운영(TLS 오류 해소). 블록 헤더 `:BPM :Delay :Beat :Split`   | 확인, A                                                                  |
| FMT-ARC-06 | maimai ma2 / `S*T` / `S*B` | 식별 → 검증            | ma2는 평문(1소절 = 384틱). `S*B`는 `S*T`를 AES 암호화한 바이너리                                          | 확인, B. 세가 공식 명세는 비공개                                         |
| FMT-LVL-04 | ADOFAI `.adofai`           | 식별 → 검증            | 이동 각도로 시간 결정: `t = (theta / 180) * (60000 / BPM)` ms. `pathData` 문자열 → `angleData` 실수 배열  | 확인, A. 근거가 Steam 커뮤니티 페이지·튜토리얼이라 1차 명세인지는 미확인 |

GTR-01의 근거 주소(`github.com/TheNathannator/GuitarGame_ChartFormats`)와 다른 조사의 `solamint.github.io/GuitarGame_ChartFormats/`, 또 다른 자료의 `thenathannator.github.io/…`가 같은 자료인지는 확인하지 못했다. 세 값을 합치지 않았다.
