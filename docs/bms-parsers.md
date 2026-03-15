# BMS 파서 구현 정리

이 문서는 BMS 형식을 실제로 파싱하는 주요 도구 및 구동기의 구현을 정리한 것입니다.
각 구현이 어떤 커맨드를 지원하는지, 인코딩을 어떻게 처리하는지,
그리고 구현별로 특이한 동작을 기록합니다.

---

## 목차

1. [대상 목록](#1-대상-목록)
2. [구현별 상세](#2-구현별-상세)
   - 2.1 [jbms-parser (Beatoraja)](#21-jbms-parser-beatoraja)
   - 2.2 [bms-js (Bemuse)](#22-bms-js-bemuse)
   - 2.3 [BMSE](#23-bmse)
   - 2.4 [iBMSC](#24-ibmsc)
   - 2.5 [μBMSC](#25-μbmsc)
   - 2.6 [Qwilight](#26-qwilight)
3. [커맨드 지원 비교표](#3-커맨드-지원-비교표)
4. [인코딩 처리 비교](#4-인코딩-처리-비교)
5. [구현 간 차이 및 주의사항](#5-구현-간-차이-및-주의사항)

---

## 1. 대상 목록

| 도구 | 종류 | 언어 | 저장소 |
|------|------|------|--------|
| [jbms-parser] | 구동기 파서 (Beatoraja) | Java | https://github.com/exch-bms2/jbms-parser |
| [bms-js] | 구동기 파서 (Bemuse) | TypeScript | https://github.com/bemusic/bemuse/tree/master/packages/bms |
| [BMSE] | 편집기 | VB.NET | https://github.com/Nekokan/BMSE |
| [iBMSC] | 편집기 | VB.NET | https://github.com/aqtq314/iBMSC |
| [μBMSC] | 편집기 (iBMSC 포크) | VB.NET | https://github.com/zardoru/iBMSC |
| [Qwilight] | 구동기 | — | https://taehui.ddns.net/ko (소스 비공개) |

[jbms-parser]: https://github.com/exch-bms2/jbms-parser
[bms-js]: https://github.com/bemusic/bemuse/tree/master/packages/bms
[BMSE]: https://github.com/Nekokan/BMSE
[iBMSC]: https://github.com/aqtq314/iBMSC
[μBMSC]: https://github.com/zardoru/iBMSC
[Qwilight]: https://taehui.ddns.net/ko

---

## 2. 구현별 상세

### 2.1 jbms-parser (Beatoraja)

- **파일**: `src/bms/model/BMSDecoder.java`
- **저장소**: https://github.com/exch-bms2/jbms-parser

#### 파싱 방식

- 한 줄씩 읽는 line-by-line 방식.
- `matchesReserveWord(line, keyword)` 함수로 대소문자 무관 비교.
- `#RANDOM` → `Deque<Integer>` 스택으로 중첩 처리.
- `#IF` / `#ENDIF` / `#ENDRANDOM` 분기 처리.
- `%KEY value` 형식 및 `@KEY value` 형식도 처리 (`model.getValues().put(key, value)`).

#### 인코딩

- `new InputStreamReader(stream, "MS932")` — **Shift_JIS (MS932) 하드코딩**.
- UTF-8이나 다른 인코딩은 자동 감지하지 않음.
- 파일 해시: MD5 + SHA-256 모두 계산.

#### 지원 커맨드 (CommandWord enum)

`PLAYER`, `GENRE`, `TITLE`, `SUBTITLE`, `ARTIST`, `SUBARTIST`, `PLAYLEVEL`,
`RANK`, `DEFEXRANK`, `TOTAL`, `VOLWAV`, `STAGEFILE`, `BACKBMP`, `PREVIEW`,
`LNOBJ`, `LNMODE`, `DIFFICULTY`, `BANNER`, `COMMENT`(stub; 파싱 후 무시),
`BASE`(62진수 확장)

#### 타이밍 관련 특이사항

- **`#SCROLLxx`**: `scrolltable: Map<Integer, Double>` 에 저장.
  인덱스는 `parseInt36(line, 7)` (36진수 파싱), 값은 `double`.
  음수 허용.
- `#BPM` 음수: WARNING 로그 후 무시.
- `#STOP` 음수: `Math.abs()` 적용 후 WARNING.

#### 채널 처리

- `Section` 객체로 분리하여 `makeTimeLines()` 호출.

---

### 2.2 bms-js (Bemuse)

- **파일**: `packages/bms/src/reader/index.ts`, `packages/bms/src/compiler/index.ts`
- **저장소**: https://github.com/bemusic/bemuse/tree/master/packages/bms

#### 파싱 방식

정규식 기반 매처 (`compiler/index.ts`):

| 패턴 | 처리 |
|------|------|
| `#RANDOM \d+` | 제어흐름 — 난수 범위 설정 |
| `#IF \d+` | 제어흐름 — 조건 분기 |
| `#ENDIF` | 제어흐름 — 분기 종료 |
| `#DDD02:\S+` | 박자 변경 (measure 02 채널) |
| `#DDD\S\S:\S+` | 채널 데이터 |
| `#\w+ \S+` | 헤더 (`chart.headers.set(key, value)`) |

- 헤더를 `Map<string, string>`으로 저장 (타입 검증 없음).
- DTX 형식도 지원 (`:` 구분자 방식).

#### 인코딩

- `bemuse-chardet` 라이브러리로 인코딩 자동 감지.
- `iconv-lite`로 디코딩.
- [hitkey의 `#CHARSET` 섹션 알고리즘](http://hitkey.nekokan.dyndns.info/cmds.htm#CHARSET)을 준거로 함.
- `forceEncoding` 옵션으로 수동 지정 가능.
- BOM(U+FEFF) 자동 제거.

#### 미구현 항목

`#ELSEIF`, `#SWITCH`, `#SETRANDOM` — 단순 `RANDOM` / `IF` / `ENDIF`만 처리.

---

### 2.3 BMSE

- **파일**: `modInput.vb`
- **저장소**: https://github.com/Nekokan/BMSE

#### 파싱 방식

- `Case` 문으로 키워드 분기 (대소문자 무관).
- `Case "#IF", "#RANDOM", "#RONDAM"` — `#RONDAM` 오타 변형도 처리.
- PLAYER_TYPE: `1P` / `2P` / `DP` / `PMS` / `OCT`.

#### 인코딩

- VB.NET 기본 코드페이지 (Windows 시스템 설정에 따름).

#### 채널 정의 (OBJ_CH enum 일부)

`CH_SCROLL(1020)`, `CH_SPEED(1033)` 포함 — **`#SCROLLxx` 지원**.

#### 파싱하지 않는 헤더

- 파일 관련: `#BANNER`, `#BACKBMP`, `#EXBMPzz`, `#VIDEOFILE`, `#CHARFILE`,
  `#MIDIFILE`, `#EXWAVzz`, `#PREVIEW`, `#MATERIALSWAV`, `#MATERIALSBMP`,
  `#PATH_WAV`, `#CDDA`
- 메타데이터 관련: `#SUBTITLE`, `#SUBARTIST`, `#MAKER`

---

### 2.4 iBMSC

- **파일**: `source/iBMSC/iBMSC/Form1.vb`
- **저장소**: https://github.com/aqtq314/iBMSC

#### 파싱 방식

`OpenBMS(ByVal xStrAll As String)` 함수가 파일 전체 문자열을 받아 줄 단위로 처리.

- `StartsWith` + `StringComparison.CurrentCultureIgnoreCase` 비교.
- `#IF` / `#ENDIF` / `#SWITCH` / `#SETSWITCH` / `#ENDSW` 스택 처리.
- `#RANDOM`은 `xExpansion` 문자열에 누적 후 별도 처리.

#### 인코딩

- `TextEncoding As System.Text.Encoding = System.Text.Encoding.Default` (기본값).
- 설정 UI에서 "Text Encoding" 항목으로 변경 가능.
- 파일 읽기: `My.Computer.FileSystem.ReadAllText(xPath, TextEncoding)`.

#### 지원 헤더

`#WAV`, `#BPM`(xx 및 초기값), `#STOP`, `#TITLE`, `#ARTIST`, `#GENRE`,
`#PLAYER`, `#RANK`, `#PLAYLEVEL`, `#SUBTITLE`, `#SUBARTIST`,
`#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#DIFFICULTY`, `#EXRANK`,
`#TOTAL`, `#COMMENT`, `#LNOBJ`

- `#LNTYPE`은 주석 처리 (TODO).
- `#SCROLLxx` **미지원** (분기 없음).
- `#BMP` 미지원 — BGA 대신 다른 방식을 사용하는 목적으로 설계.

#### 채널 처리

- 채널 식별자: `Mid(sLineTrim, 5, 2)`, 7번째 문자가 `":"` 인지 확인.
- BPM 범위: 0 < BPM < 65536 적용, 상한 초과 시 655359999로 클램프.
- STOP 범위: 동일하게 클램프.

---

### 2.5 μBMSC

- **파일**: `iBMSC/ChartIO.vb`
- **저장소**: https://github.com/zardoru/iBMSC (iBMSC 포크)

iBMSC에서 파생된 포크. 파싱 구조는 거의 동일하나 다음 차이가 있음.

#### iBMSC와의 주요 차이

| 항목 | iBMSC | μBMSC |
|------|-------|-------|
| `#SCROLLxx` | 미지원 | **지원** (`hSCROLL` 배열, `SC` 채널) |
| `#BMP` | 미지원 | **지원** (`hBMP` 배열) |
| `#DEFEXRANK` | `#EXRANK`로 처리 | `#DEFEXRANK` 명시 지원 |
| 기본 인코딩 | `Encoding.Default` | `Encoding.UTF8` |
| BPM/STOP 클램프 | 있음 | 없음 ("No limits on BPM editing") |

#### SCROLL 구현 상세

```vb
ReDim hSCROLL(1295)
...
ElseIf sLineTrim.StartsWith("#SCROLL", ...) Then
    hSCROLL(C36to10(Mid(sLineTrim, Len("#SCROLL") + 1, 2))) = _
        Val(Mid(sLineTrim, Len("#SCROLL") + 4)) * 10000
...
If Channel = "SC" Then .Value = hSCROLL(C36to10(Mid(sLineTrim, xI1, 2)))
```

채널 식별자 목록에 `"SC"` 포함: `BMSChannelList() = {"01", "03", ..., "SC"}`.

---

### 2.6 Qwilight

- **저장소**: https://taehui.ddns.net/ko (소스 비공개)

소스 코드를 직접 확인할 수 없음.
[hitkey BMS command memo](https://hitkey.nekokan.dyndns.info/cmds.htm) 에서 참고한
[Guide to understand BMS format](https://cosmic.mearie.org/2005/03/bmsguide/) 을 파싱 기준으로 삼는 것으로 알려져 있음.

---

## 3. 커맨드 지원 비교표

아래 표에서 ○ = 지원, △ = 부분 지원 또는 조건부, × = 미지원, — = 확인 불가.

### 메타데이터 헤더

| 커맨드 | jbms-parser | bms-js | BMSE | iBMSC | μBMSC |
|--------|:-----------:|:------:|:----:|:-----:|:-----:|
| `#TITLE` | ○ | ○ | ○ | ○ | ○ |
| `#SUBTITLE` | ○ | ○ | × | ○ | ○ |
| `#ARTIST` | ○ | ○ | ○ | ○ | ○ |
| `#SUBARTIST` | ○ | ○ | × | ○ | ○ |
| `#GENRE` | ○ | ○ | ○ | ○ | ○ |
| `#MAKER` | × | ○ | × | × | × |
| `#COMMENT` | △ (stub) | ○ | ○ | ○ | ○ |

### 파일 헤더

| 커맨드 | jbms-parser | bms-js | BMSE | iBMSC | μBMSC |
|--------|:-----------:|:------:|:----:|:-----:|:-----:|
| `#WAVxx` | ○ | ○ | ○ | ○ | ○ |
| `#BMPxx` | ○ | ○ | × | × | ○ |
| `#STAGEFILE` | ○ | ○ | × | ○ | ○ |
| `#BANNER` | ○ | ○ | × | ○ | ○ |
| `#BACKBMP` | ○ | ○ | × | ○ | ○ |
| `#PREVIEW` | ○ | ○ | × | × | × |
| `#EXWAVxx` | × | ○ | × | × | × |
| `#EXBMPxx` | × | ○ | × | × | × |
| `#VIDEOFILE` | × | ○ | × | × | × |
| `#MIDIFILE` | × | △ | × | × | × |
| `#PATH_WAV` | × | × | × | × | × |
| `#MATERIALSWAV` | × | × | × | × | × |
| `#MATERIALSBMP` | × | × | × | × | × |
| `#CDDA` | × | × | × | × | × |

### 타이밍 헤더

| 커맨드 | jbms-parser | bms-js | BMSE | iBMSC | μBMSC |
|--------|:-----------:|:------:|:----:|:-----:|:-----:|
| `#BPM` (초기값) | ○ | ○ | ○ | ○ | ○ |
| `#BPMxx` | ○ | ○ | ○ | ○ | ○ |
| `#BASEBPM` | × | × | × | × | × |
| `#STOPxx` | ○ | ○ | ○ | ○ | ○ |
| `#STP` | × | × | × | × | × |
| **`#SCROLLxx`** | **○** | × | **○** | × | **○** |

### 게임플레이 헤더

| 커맨드 | jbms-parser | bms-js | BMSE | iBMSC | μBMSC |
|--------|:-----------:|:------:|:----:|:-----:|:-----:|
| `#PLAYER` | ○ | ○ | ○ | ○ | ○ |
| `#RANK` | ○ | ○ | ○ | ○ | ○ |
| `#DEFEXRANK` | ○ | ○ | × | △ (`#EXRANK`) | ○ |
| `#DIFFICULTY` | ○ | ○ | × | ○ | ○ |
| `#PLAYLEVEL` | ○ | ○ | ○ | ○ | ○ |
| `#LNOBJ` | ○ | ○ | × | ○ | ○ |
| `#LNTYPE` | ○ | × | × | × (주석) | × (주석) |

### 게이지 / 볼륨 헤더

| 커맨드 | jbms-parser | bms-js | BMSE | iBMSC | μBMSC |
|--------|:-----------:|:------:|:----:|:-----:|:-----:|
| `#TOTAL` | ○ | ○ | × | ○ | ○ |
| `#VOLWAV` | ○ | ○ | × | × | × |

### 제어흐름

| 커맨드 | jbms-parser | bms-js | BMSE | iBMSC | μBMSC |
|--------|:-----------:|:------:|:----:|:-----:|:-----:|
| `#RANDOM` | ○ | ○ | ○ | ○ | ○ |
| `#SETRANDOM` | ○ | × | × | × | × |
| `#IF` | ○ | ○ | ○ | ○ | ○ |
| `#ELSEIF` | ○ | × | × | × | × |
| `#ELSE` | ○ | × | × | × | × |
| `#ENDIF` | ○ | ○ | ○ | ○ | ○ |
| `#ENDRANDOM` | ○ | × | × | × | × |
| `#SWITCH` | ○ | × | × | ○ | ○ |
| `#SETSWITCH` | × | × | × | ○ | ○ |
| `#ENDSW` | ○ | × | × | ○ | ○ |
| `#RONDAM` (오타) | × | × | ○ | × | × |

---

## 4. 인코딩 처리 비교

| 구현 | 인코딩 처리 방식 |
|------|----------------|
| jbms-parser | `MS932` (Shift_JIS) 하드코딩 |
| bms-js | `bemuse-chardet` 자동 감지 + `iconv-lite` 디코딩; hitkey `#CHARSET` 알고리즘 준거; BOM 제거; `forceEncoding` 옵션 |
| BMSE | Windows 기본 코드페이지 |
| iBMSC | `System.Text.Encoding.Default` (기본값); 설정 UI에서 변경 가능 |
| μBMSC | `System.Text.Encoding.UTF8` (기본값); 설정 UI에서 변경 가능 |
| Qwilight | 소스 비공개 — 확인 불가 |

`#CHARSET` 헤더를 명시적으로 파싱하는 구현은 bms-js 외에 확인되지 않음.

---

## 5. 구현 간 차이 및 주의사항

### `#SCROLLxx` 지원 여부

`#SCROLLxx`는 [hitkey BMS command memo](https://hitkey.nekokan.dyndns.info/cmds.htm)에
수록되어 있지 않으나, 여러 구현에서 실제로 지원하고 있음.

| 구현 | 지원 | 비고 |
|------|:----:|------|
| jbms-parser | ○ | `scrolltable: Map<Integer,Double>`, 36진수 인덱스, 음수 허용 |
| bms-js | × | — |
| BMSE | ○ | `OBJ_CH.CH_SCROLL(1020)` 채널, `Case "#SCROLL"` 분기 |
| iBMSC | × | — |
| μBMSC | ○ | `hSCROLL(1295)` 배열, 채널 `SC` |

`bms.ebnf`의 `timing_header`에서 `#SCROLLxx` 반영 여부는 별도 판단이 필요함.
복수의 주요 구현(jbms-parser, BMSE, μBMSC)에서 지원하므로 사실상 비공식 표준으로
취급할 수 있음.

### `#LNTYPE` vs `#LNOBJ`

- `#LNTYPE`은 jbms-parser만 명시 지원.
- iBMSC / μBMSC는 `#LNTYPE` 파싱 코드가 주석으로 남아 있으며,
  대신 `#LNOBJ`가 없으면 자동으로 `#LNTYPE 1`을 출력함.

### `#RONDAM` 오타 허용

BMSE는 `#RANDOM`의 오타 변형인 `#RONDAM`도 처리함.
실제 BMS 파일에 이 오타가 포함된 경우가 있음을 시사.

### BPM / STOP 범위 처리

- iBMSC: 0 < BPM < 65536, 상한 초과 시 `655359999`로 클램프.
- μBMSC: 범위 제한 없음 ("No limits on BPM editing").
- jbms-parser: `#BPM` 음수는 WARNING 후 무시; `#STOP` 음수는 `Math.abs()` 적용.

### `#DEFEXRANK` vs `#EXRANK`

- iBMSC는 `#DEFEXRANK` 분기 없이 `#EXRANK`로 처리.
- μBMSC는 `#DEFEXRANK`를 명시적으로 지원.
