# 포맷별 추가 확인 사항

별도 조사(2026-10-08)가 아래 포맷 9건에서 새로 확인한 사실이다. 이 저장소가 다시 확인한 값이 아니며, 확인 상태 열은 조사가 스스로 적은 값이다.

| 자료                       | 새로 확인한 것                                                                                            | 확인 상태                                                             |
| -------------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| RBN / C3 Documentation     | HTTPS 자체서명 인증서 문제로 열리지 않을 수 있다. 일반 HTTP로는 위키 전문이 열림                          | 확인                                                                  |
| simai(`maidata.txt`)       | atwiki `pages/25.html`은 메타데이터 헤더 규격이고 기보 문법은 `pages/1002.html`(2026-07-10 갱신판)에 있음 | 확인                                                                  |
| PEC(PhiEditor)             | 줄 단위 명령(`cp`, `cd`, `ca`, `cm`, `cr`, `cf` 등). Re:PhiEdit의 PEConverter로 상호 변환                 | 확인                                                                  |
| RPE(Re:PhiEdit) JSON       | `META` + `judgeLineList`. 시간 단위 `[beat, numerator, denominator]`                                      | 확인                                                                  |
| SOUND VOLTEX `.vox`        | Vox 10은 레이저 좌표 0~127 정수, Vox 12는 float(1/64 단위). 공식 명세는 비공개라 역공학 변환기로 실증     | 확인. 1차 명세 없음                                                   |
| `.chart`                   | 원본 FeedBack은 레인 0~4, Clone Hero 확장은 레인 5(강제), 6(탭), 7(오픈 스트럼), GHL 6프렛                | 확인                                                                  |
| UCS(Pump It Up)            | `ucs.piugame.com`이 PIU PHOENIX 전용으로 정상 운영(TLS 오류 없음). 블록 헤더 `:BPM :Delay :Beat :Split`   | 확인                                                                  |
| maimai ma2 / `S*T` / `S*B` | ma2는 평문(1소절 = 384틱). `S*B`는 `S*T`를 AES 암호화한 바이너리                                          | 확인. 세가 공식 명세는 비공개                                         |
| ADOFAI `.adofai`           | 이동 각도로 시간 결정: `t = (theta / 180) * (60000 / BPM)` ms. `pathData` 문자열 → `angleData` 실수 배열  | 확인. 근거가 Steam 커뮤니티 페이지·튜토리얼이라 1차 명세인지는 미확인 |

`.chart`의 근거 주소로 `github.com/TheNathannator/GuitarGame_ChartFormats`, `solamint.github.io/GuitarGame_ChartFormats/`, `thenathannator.github.io/…`가 나오는데 같은 자료인지는 확인하지 못했다. 세 값을 합치지 않았다.