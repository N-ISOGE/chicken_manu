# 기록

## 로드맵

### 스키마 정의하기

- [METS]에서 파생시켜서 메타데이터에 대한 스키마를 직접 최신화하는 부분을 줄임.
  - xml, encoding은 UTF-8로.

### 관리, 공유 및 배포 도구 작성

- rust 기반으로 작성.

## 시간 기반 기록

##### 2024-07-11

- idx 환경 파일 작성
  - rust랑 markup 언어 관련 설정
- readme, log 작성
  - 기본적인 기록 작성

## 의미 기반 기록

### 문제들

이거가 있었나? -> BMSSearch  
이거 남아있긴 한건가? -> ?  
이 파일이 맞나? -> BMSSearch로는 부족  
이거를 뭐라 부르냐 -> 패턴이랑 관련 파일들?

### 참고할 것들

#### [METS]

- 스키마 참고

[METS]: https://www.loc.gov/standards/mets/mets-home.html

#### [IPFS]

- 배포 및 공유 기반

**[IPFS] 구현**

- [iroh](https://iroh.computer/docs)
  - [iroh-rust](https://crates.io/crates/iroh)
- [rust-ipfs](https://crates.io/crates/rust-ipfs)

[IPFS]: https://docs.ipfs.tech/

### BMS

#### 개요

bms 파일 하나와 연관된 파일들을 묶어서 관리함.

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

#### 여러 사양들

##### [hitkey BMS command memo]

과거 기록들: [hitkey 2014 archive]  
현재 반영: [hitkey 2014 archive]

여러 관련 도구와 구동기들의 사양을 기반으로 사양을 정리한 문서.

---

**패턴 정보용 header**

`#TITLE`, `#ARTIST`, ...

**파일 정보용 header**

`#WAVXX`, `#EXWAVXX`, `#MIDIFILE`  
`#PATH_WAV`  
`#BMPXX`, `#EXBMPXX`, `#VIDEOFILE`, `#MOVIE`, `#BACKBMP`, `#CHARFILE`  
`#STAGEFILE`, `#BANNER`

MATERIAL 계열 header를 쓴 bms가 있나?

**고려할 점**

header 중복되면 EOL에 가까운, 뒤에 나온 header를 반영함.

- 예외도 있음 :
  - 메타데이터에선 `#SUBTITLE`, `#SUBARTIST` 등...
  - 파일 처리에선 고려하지 않음.

Comment 처리?

- `;`, `//`, (`/*`, `*/`)

소리 파일이 다른데 패턴 파일이 같은 경우를 다르게 등록시켜야 할 점.

[hitkey BMS command memo]: https://hitkey.nekokan.dyndns.info/cmds.htm

[hitkey 2014 archive]: https://web.archive.org/web/20240505175610/https://hitkey.nekokan.dyndns.info/cmds.htm

#### 관련 도구 및 구동기

##### **BMSE**

여기서 대응 안하는 header

- 파일 관련
  - `#BANNER`, `#BACKBMP`, `#EXBMPzz`, `#VIDEOFILE`, `#CHARFILE`
  - `#MIDIFILE`, `#EXWAVzz`, `#PREVIEW`
  - `#MATERIALSWAV`, `#MATERIALSBMP`, `#PATH_WAV`
  - `#CDDA`
- 메타데이터 관련
  - `#SUBTITLE`, `#SUBARTIST`, `#MAKER`

관련 링크

- [site](http://ucn.tokonats.net/software/bmse/)
- [github](https://github.com/Nekokan/BMSE)

##### **iBMSC**

관련 링크

- [github](https://github.com/aqtq314/iBMSC)

BGA 대신할 것이 있는 쪽을 지원하는 목적이라 `#BMP`을 뺌.

`source/iBMSC/iBMSC/Form1.vb`

- `Private Sub OpenBMS(ByVal As String)`
- `Private Function SaveBMS() As String`
  - 메타데이터 관련: `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`
  - 파일 정보 관련: `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#WAV`

##### **μBMSC**

관련 링크

- [github]([https://github.com/zardoru/iBMSC)

iBMSC와 비슷하게 `#BMP`을 뺌

`iBMSC/ChartIO.vb`

- `Private Sub OpenBMS(ByVal As String)`
  - 메타데이터 관련: `#TITLE`, `#ARTIST`, `#SUBTITLE`, `#SUBARTIST`
  - 파일 정보 관련: `#STAGEFILE`, `#BANNER`, `#BACKBMP`, `#WAV`

##### [Beatoraja]

bms parser: [jbms-parser]

[Beatoraja]: https://github.com/exch-bms2/beatoraja

[jbms-parser]: https://github.com/exch-bms2/jbms-parser

##### [bemuse]

bms parser: [bms-js]

[bms-js]: https://github.com/bemusic/bemuse/tree/master/packages/bms

[bemuse]: https://github.com/bemusic/bemuse
