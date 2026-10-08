# BMS 문법

BMS 계열 파일(`.bms .bme .bml .pms`)의 문법 초안이다.
리듬게임 채보 전반이 대상이지만([지원 대상과 명세의 형식화](crate::docs::formats)), 이 문법은 BMS 계열 **한정**이다.
다른 계열의 문법은 각 포맷의 명세를 조사한 뒤 별도로 쓴다.
BMS에는 공식 형식 문법이 없어서 hitkey BMS command memo와 구동기·파서의 동작, 실제 파일을 근거로 썼다.
전문은 이 문서 끝에 있고 원본은 `docs/bms.abnf`다.

## 1. 저장소에 있던 초안

원본 `change_log.md`의 EBNF 초안이다(W3C XML 표기). 미완성이며 주석을 고려해 쓰던 중이었다.

```ebnf
; [ebnf for xml](https://www.w3.org/TR/2006/REC-xml11-20060816/#sec-notation)

single_line_comment ::= ( "//" | ";" ) ( WSP | CHAR )* CRLF
multi_line_comment ::= "/*" ( WSP | CHAR | CRLF )* "*/"
c_nl ::= single_line_comment | multi_line_comment | CRLF

block_comment ::= "/*" ( WSP | CHAR )* "*/"
b_c_wsp ::= block_comment | WSP

bms ::= bms_commend | bms_comment; 명령, 주석? 필수 commend를 안 정했나? 아닌데...

bms_comment ::= CHAR - "#" ( b_c_wsp | CHAR )* c_nl
bms_commend ::= header ( b_c_wsp | WSP*  parameter )+ c_nl  ; 이거 parameter 개수 맞게 바꿔야 함

header ::= "#" ( "TITLE" | ... )
```

초안과 이 문서의 ABNF 초안의 차이는 다음과 같다.

| 항목 | 저장소의 EBNF 초안 | ABNF 초안 |
| --- | --- | --- |
| 표기 | W3C XML EBNF(`::=`) | ABNF(RFC 5234). 따옴표 문자열이 기본적으로 대소문자 무시라 명령 이름에 맞다 |
| `#`로 시작하지 않는 줄 | `bms_comment` | `ignored-line`(같은 개념) |
| 주석 `;` `//` `/* */` | 문법에 포함 | 기본 언어에서 **제외**하고 선택적 전처리로 취급(아래 3절). 줄 맨 앞의 `;`나 `//` 줄은 어차피 무시되는 줄이다 |
| 헤더 | `"TITLE" \| ...`로 미완성 | 파일 참조, 슬롯 정의, 메타, 제어 흐름으로 분류해 나열하고, 모르는 헤더는 `generic-command`로 보존 |
| 헤더 값의 개수·형식 | TODO | 헤더별로 정수·실수·인덱스·경로·자유 텍스트를 구분 |
| 채널 문장 `#xxxCH:data` | 없음 | 있음(마디 3자리, 채널 2자리 base36, 객체 2자리 base36) |
| 제어 흐름 | 없음 | 2층에서 `IF…ENDIF`, `SWITCH…ENDSW`의 균형과 중첩 검사 |
| 필수 명령 | "필수 commend를 안 정했나?" | hitkey의 최소 요구사항 요약에 필수 헤더는 없다. 줄 순서도 자유이고 헤더는 어디에나 올 수 있다 |

## 2. 원본 기록의 헤더 분류와 문법의 대응

| 원본 기록의 분류 | 헤더 | ABNF 규칙 |
| --- | --- | --- |
| 패턴 정보용 | `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`, ... | `text-meta-name`, `number-meta-name` |
| 파일 정보용 | `#WAVxx`, `#BMPxx` | `file-ref-command`의 `indexed-file-name` |
| 파일 정보용 | `#EXWAVxx`, `#EXBMPxx` | `ex-file-name`(파라미터가 있어 값 전체를 보존) |
| 파일 정보용 | `#MIDIFILE`, `#PATH_WAV`, `#VIDEOFILE`, `#MOVIE`, `#BACKBMP`, `#CHARFILE`, `#STAGEFILE`, `#BANNER` | `plain-file-name` |
| (추가) | `#PREVIEW`, `#MATERIALSWAV`, `#MATERIALSBMP` | `plain-file-name` |

원본 기록의 질문 "MATERIAL 계열 header를 쓴 bms가 있나?"에 대해: 로컬 샘플 189개에는 없었다. hitkey는 `#MATERIALSWAV`를 nanasi 1.00 미만 전용의 obsolete 명령으로 표시한다.

원본 기록의 "고려할 점"은 다음과 같이 확인됐다(hitkey 원문).

- 헤더가 중복되면 EOL에 가까운, 뒤에 나온 것을 쓴다. 예외: `#ExtChr`, `#STP`, `#WAVCMD`, `#OPTION`과 제어 흐름. `#SUBTITLE`, `#SUBARTIST`, `#COMMENT`, `#LNOBJ`는 이 규칙이 적용되지 않을 수 있다(여러 개 허용하는 구현이 있다). 파일 처리에서는 이 예외를 고려하지 않는다는 원본의 방침과 같다.
- 주석은 `;`, `//`, `/* */`다. 다만 IIDXv, HDX, outliner 등이 지원하는 **확장**이며 원 명세에는 없다.
- 소리 파일이 다르고 패턴 파일이 같은 경우를 다르게 등록해야 한다. 스키마에서는 같은 패턴 파일을 참조하되 관련 파일이 다른 패턴을 별도 패턴으로 추가하도록 정했다([스키마와 식별자](crate::docs::schema)).

## 3. ABNF 초안의 구조

- **입력은 디코딩된 텍스트다.** 바이트에서 텍스트로의 변환(BOM, Shift_JIS, EUC-KR, UTF-8, `#CHARSET`)은 문법 밖이다.
- **1층: 줄 문법.** 줄 구분자는 CRLF, CR, LF이고 한 파일에 섞일 수 있다. `source-line = command-line / percent-line / ignored-line`.
  - 채널 문장은 `#` + 마디 3자리 + 채널 2자리 + `:` + 데이터다. 채널 `02`는 소수(마디 길이), `03`은 2자리 16진 정수(BPM), 그 밖의 채널은 2자리 base36 객체의 나열이다(`00`은 쉼표).
  - 헤더 문장은 `#NAME` + 구분자(공백·탭) + 값이다. 제어 흐름, 파일 참조, 슬롯 정의, 메타로 분류하고 모르는 헤더는 이름과 원문 값을 보존한다.
  - 시작 규칙이 둘이다. **`source-line`**(관대)은 모르는 헤더와 값이 어긋난 타입 헤더도 받아들인다. **`known-command-line`**(엄격)은 알려진 명령만 받는다. 관대 규칙에서 타입 있는 이름이 `generic-command`로만 맞으면 진단으로 취급하는 것이 의도다.
- **2층: 블록 구조.** 줄마다 토큰 하나(`L`, `RANDOM`, `IF`, `ENDIF` 등)로 바꾼 열에서 `IF…ELSEIF…ELSE…ENDIF`와 `SWITCH…ENDSW`의 균형과 중첩을 검사한다. `#RANDOM`은 값을 정할 뿐이라 `#ENDRANDOM`은 선택이다.

## 4. 검증 결과

로컬 샘플 `test_resource/`(.bms 71, .bme 96, .bml 15, .pms 11, `._` 접두 메타 파일 4개 제외 189개, 저장소에는 없음)로 시험했다.
도구는 Python의 `abnf` 2.9.0이며, 문법 파일을 규칙 단위로 읽어 줄마다 `parse_all`을 호출했다. 검증 스크립트는 저장소에 넣지 않았다.

| 항목 | 결과 |
| --- | --- |
| 단위 케이스 45개(허용과 거부) | 불일치 0 |
| 구조 케이스 14개(균형, 중첩, 잘못된 순서) | 불일치 0 |
| 483,213줄(서로 다른 줄 98,067개)에 관대 규칙 | 실패 0. 400자를 넘는 43줄은 같은 규칙의 정규식으로 확인 |
| 같은 줄들에 엄격 규칙 | 실패 4줄: `#SUBARTHIST`(오타) 2, `#TOTAL stand by OK` 2 |
| 189개 파일의 블록 구조 | 실패 0 |

한계: 문법 **수용** 검사일 뿐이다. 값의 의미(마디 길이가 양수인지 등)와 참조 파일의 존재는 검사하지 않았다. 샘플은 한 저장소의 파일이라 특정 시기와 제작자에 치우쳐 있다.

## 5. 실제 파일에서 알게 된 것

1. hitkey 원문 전체를 확인한 결과 `#PREVIEW`, `#BASE`, `#LNMODE`는 hitkey에 없다. 샘플에는 `#PREVIEW`와 `#LNMODE`가 각각 13개 파일에 있다. 실질 표준은 hitkey와 구동기 동작이다.
2. `#CHARSET`은 hitkey에 있으나 obsolete 표시이고(ruvit 2.0b5p2에서 제거), BOM이 있으면 `#CHARSET`보다 BOM을 따른다. EUC-KR과 Shift_JIS 외의 인코딩이면 BOM 있는 UTF-8로 저장·배포하기를 권장한다.
3. 샘플 189개 중 BOM이 있는 파일은 0개다. 인코딩 분포는 UTF-8 85, CP932 96, EUC-KR 8이다. BOM에만 의존하면 안 된다.
4. `._append_*.bms` 4개는 macOS AppleDouble 메타 파일이라 BMS가 아니다. `._` 접두 이름으로 먼저 제외해야 한다.
5. 샘플의 `#` 줄에는 주석 문법(`;` `//` `/* */`)이 하나도 없었다. 주석을 지원하려면 1층 앞에서 먼저 제거해야 하고(hitkey: 주석이 제어 구문보다 우선), 값 안의 `;`나 `//`(URL, 제목)와 충돌하는 구현의 차이를 정해야 한다.
6. `#STAGEFILE`과 `#GENRE`는 값이 빈 경우가 흔하고(각 16줄, 14줄), `#endif`는 소문자로 쓰인다. 채널 `02`에는 `.25`처럼 앞의 0이 없는 소수가 있다.
7. 샘플의 `#RANDOM` 2개는 모두 `#ENDRANDOM`이 없었다. `#IF` 12개는 `#ENDIF` 12개와 짝이 맞았다.
8. 줄 구분자는 샘플 전체가 CRLF였지만 hitkey는 CRLF, CR, LF가 한 파일에 섞일 수 있다고 한다.

## 6. 문법이 정하지 않은 것

구현할 때 정해야 하는 항목이다. 근거는 `docs/bms.abnf` 끝의 부록 주석에도 있다.

- 인코딩 판별 순서와 문법 밖 처리(BOM, 엄격한 UTF-8, 엄격한 Shift_JIS, EUC-KR, 손실 디코딩).
- 중복 헤더 처리와 같은 마디·채널의 병합(나중 `00`은 덮어쓰지 않음, 채널 01·02·A6는 예외).
- 비정상 채널 데이터(Shift_JIS 아트, `12.375f` 같은 값, 홀수 길이)는 명세에 없다. 이 문법은 채널 문장에서 거부한다.
- `#BASE 62`는 jbms-parser·beatoraja 전용이다. 지원하려면 인덱스를 대소문자 구분 62진으로 바꿔야 한다.
- 파일 참조의 경로 정규화(역슬래시, 대소문자, 확장자 대체).
- 주석 지원 여부.

## 7. 문법 전문 (ABNF)
