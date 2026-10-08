# 스키마와 식별자

작성해 둔 MariaDB 스키마(`chicken_menu`)와 논리적 데이터 모델 설계서(2024-12-19, 2차)를 요약하고,
해시·고유식별자·IPFS 관점에서 검토한 결과를 적는다. 이 문서는 스키마 변경안을 설계하지 않고 관찰과 결정 사항만 남긴다.

## 1. 릴레이션 요약

| 릴레이션                                     | 속성                                                                                                                        |
| -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `file`                                       | `id`, `name`, `size`, `format_property`, `encoding_property`, `checksum_algorithm_identifier`, `checksum_value`, `ipfs_cid` |
| `binder`                                     | `id`, `name`                                                                                                                |
| `pattern`                                    | `id`, `name`, `category`, `pattern_file_id`                                                                                 |
| `persona`                                    | `id`, `name`, `introduction`                                                                                                |
| `binder_attached_pattern` (binder 포함 패턴) | `binder_id`, `attached_pattern_id`, `relative_path`                                                                         |
| `binder_appendix_file` (binder 부속 파일)    | `binder_id`, `appendix_file_id`, `relative_path`                                                                            |
| `pattern_attached_file` (패턴 관련 파일)     | `pattern_id`, `attached_file_id`, `relative_path`                                                                           |
| `pattern_credits` (패턴 제작 참여)           | `pattern_id`, `persona_id`, `role` (기본 `ARTIST`)                                                                          |
| `persona_affiliation` (명의 소속)            | `member_persona_id`, `group_persona_id`                                                                                     |
| `alternate_persona` (명의 별칭)              | `main_persona_id`, `minor_persona_id`                                                                                       |

설계서에 적힌 결정은 다음과 같다.

- 문자셋은 다국어 지원을 위해 UTF-8(utf8mb4)로 통일했다.
- ID는 모두 `BIGINT UNSIGNED`다.
- 이름 길이는 실제 패턴을 조사해 정했다. 가장 많이 쓰이는 것으로 추정한 플레이어의 인터넷 랭킹 사이트에서 최장 123자(일본어)를 확인했고, 그 사이트의 이름 필드가 Shift-JIS 256바이트(128자)이므로 2배인 256자로 했다.
- 상대경로는 리눅스 초기 설정을 따라 `VARCHAR(4096)`이다.
- `format_property`는 IANA MIME 타입 표현의 최대 길이를 반영해 `VARCHAR(128)`이고, 확장자도 같이 담을 수 있게 했다.
- `pattern.category`는 "기존에 알려진 패턴 형식들의 식별자"를 쓰도록 정했다.
- 명의 별칭은 처음에 `persona`에 주 명의 외래키를 두려 했으나, 별도의 `alternate_persona` 릴레이션으로 분해했다.
- binder를 추가하는 절차: ① 파일을 모두 `file`에 넣고 ② 패턴 파일을 찾아 `pattern`과 `binder_attached_pattern`에 넣고 ③ 패턴이 참조하는 파일을 `pattern_attached_file`에 넣고 ④ 나머지를 `binder_appendix_file`에 넣고 ⑤ 명의와 `pattern_credits`를 채운다.

## 2. 해시와 고유식별자 검토

| 관찰                                                                                     | 근거                                                                                                                                                                       | 영향                                                                                                                                                                                                                                                                                                                       |
| ---------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 파일당 체크섬이 1종이다                                                                  | 컬럼이 하나씩                                                                                                                                                              | BMS 생태계는 채보를 **MD5 또는 SHA-256**(원시 바이트)으로 식별한다(난이도표의 `md5`/`sha256`, beatoraja, jbms-parser). SHA3-256만 있으면 난이도표·랭킹과 매칭할 수 없다                                                                                                                                                    |
| `checksum_value`는 64바이트면 충분하다                                                   | MD5 16, SHA-256·SHA3-256·BLAKE3-256 32, SHA-512 64                                                                                                                         | 컬럼 크기를 바꿀 필요가 없다                                                                                                                                                                                                                                                                                               |
| 설계서 본문은 해시 값에 `VARCHAR(128)`를 고려했다고 적었고, 표와 DDL은 `VARBINARY(64)`다 | 설계서 "검토 및 의도"와 DDL의 차이                                                                                                                                         | 문자열(16진)과 바이트 중 하나로 정해야 한다                                                                                                                                                                                                                                                                                |
| `ipfs_cid`는 `VARBINARY(128)`인데 CID는 문자열로 만들어진다                              | `util-for-parsing` 브랜치의 `print_file_row`가 `ipfs add` 출력의 문자열 CID를 CSV로 쓰고 `LOAD DATA`로 적재한다                                                            | 같은 해시도 CIDv0(`Qm…`)와 CIDv1(`bafy…`)의 문자열이 다르다. 저장 형태를 하나로 고정해야 한다(예: 정규화한 CID 바이트)                                                                                                                                                                                                     |
| CID는 **생성 프로파일에 종속**된다                                                       | kubo 기본(CIDv0, 256KiB, 링크 174)과 `unixfs-v1-2025`(CIDv1, 1MiB, 링크 1024)는 같은 파일에서 다른 CID를 만든다                                                            | CID 옆에 프로파일 이름이 없으면 재현·비교할 수 없다                                                                                                                                                                                                                                                                        |
| 알고리즘 이름이 자유 문자열이다                                                          | `VARCHAR(32)`                                                                                                                                                              | 철자를 정해야 한다. IANA 해시 이름 레지스트리는 `md5`, `sha-256`, `sha3-256`을 갖고 `blake3`는 없다. 다른 표준의 철자도 제각각이다: BagIt은 이름을 소문자·영숫자로 줄이고(`sha256`), OCFL은 `sha512`·`sha256`·`blake2b-512`, Frictionless Data Package는 `sha1:` 접두, Internet Archive는 `md5`·`sha1`·`crc32` 필드를 쓴다 |
| 파일의 정체성은 내용 해시이고 이름이 아니다                                              | 같은 이름(`kick.wav` 등)이 여러 binder에 나온다                                                                                                                            | `(checksum_algorithm_identifier, checksum_value)`를 유일 키로 보는 것이 자연스럽다. 경로를 연결 테이블(`relative_path`)로 분리해 둔 구조는 IPFS의 콘텐츠 주소 + 경로 모델과 잘 맞는다                                                                                                                                      |
| 패턴의 정체성은 패턴 파일의 해시다                                                       | `pattern.name`은 유일하지 않고 `pattern_file_id`가 `file`을 참조한다                                                                                                       | 난이도표의 `md5`와 `file`의 MD5를 직접 연결할 수 있다. 단 **파일 하나가 채보 하나인 포맷**(BMS 계열은 관행, osu!·bmson·KSH)에서만 성립한다. 바로 아래 행을 본다                                                                                                                                                            |
| 파일 하나에 채보가 여럿인 포맷이 있다                                                    | StepMania `.sm`(`#NOTES` 여러 개)·`.ssc`(`#NOTEDATA` 여러 개), Clone Hero `.chart`(난이도·트랙 여러 개)([포맷별 채보 단위](crate::docs::formats::chart_units))             | 같은 파일의 채보 여럿이 같은 `pattern_file_id`를 공유하게 된다. 이 열에 유일 제약이 있는지는 설계서에서 확인하지 못했다. "파일 안의 어느 채보인가"를 가리키는 값(예: `.ssc`의 `#STEPSTYPE`+`#DIFFICULTY`+`#METER`)이 필요한지 정해야 한다                                                                                  |
| 곡 단위 메타가 채보 파일 밖에 있는 포맷이 있다                                           | Clone Hero는 `song.ini`, Beat Saber는 `Info.dat`. osu!와 StepMania는 채보 파일마다 중복해 적는다                                                                           | `pattern`과 `binder`에는 곡 제목·아티스트 같은 곡 단위 메타를 담을 곳이 없다(`persona`와 `pattern_credits`뿐). `song.ini` 같은 파일을 `binder_appendix_file`과 `pattern_attached_file` 중 어디에 넣는지도 정해야 한다                                                                                                      |
| `binder`에 묶음 종류와 외부 식별자가 없다                                                | 묶음이 ZIP 기반 압축 파일(`.osz`), 폴더(StepMania, Clone Hero, BMS), 단일 파일(`.adofai`)로 다르다. osu!는 `BeatmapSetID`·`BeatmapID`, Quaver API는 `package_md5`를 갖는다 | 압축 파일 자체를 `file`로 둘지 풀어서 폴더로 볼지, 외부 ID를 보관할지 정해야 한다. 압축 항목 이름의 문자 인코딩은 `.osz`를 포함해 전부 미확인이라 `relative_path`(`VARCHAR(4096)`, utf8mb4)에 항상 담긴다는 근거가 없다                                                                                                    |
| binder에는 해시·CID가 없다                                                               | `binder(id, name)`                                                                                                                                                         | 난이도표의 `ipfs` 필드는 **디렉터리**여야 한다. IPFS로 공유하려면 binder 단위 루트 CID가 필요하다                                                                                                                                                                                                                          |
| 원문 참조 이름이 남지 않는다                                                             | 연결 테이블은 `file_id`와 `relative_path`만 보관                                                                                                                           | `#WAV01 kick.wav`가 실제로 `kick.ogg`로 해석되는 경우(확장자 대체)와 대소문자 차이를 복원할 수 없다. [BMS 문법](crate::docs::bms_grammar)의 `file-ref-command`가 (헤더, 인덱스, 원문 경로)를 내므로 보관할지 정해야 한다                                                                                                   |
| `format_property`·`encoding_property`로는 BMS 텍스트를 구분하기 어렵다                   | libmagic이 BMS 내용을 인식하는지는 시험하지 않았다                                                                                                                         | 확장자와 문법 적합도(알려진 명령 비율)로 판별하는 방법이 있다                                                                                                                                                                                                                                                              |

`print_file_row`가 만드는 CSV는 파일명 필드를 큰따옴표로 감싸고 내부 큰따옴표를 두 번 쓴다(RFC 4180).
파일명에 쉼표가 있어도 열 수가 바뀌지 않게 하려는 것이다. `LOAD DATA`로 적재하는 설정도 이 인용 규칙에 맞아야 한다.
정확한 옵션(`ENCLOSED BY`, 기본 이스케이프 문자와 Windows 경로의 `\`)은 MariaDB 문서로 확인하지 못했다(`미확인`).

정리: 최소한 **MD5 + SHA-256(원시 바이트) + SHA3-256 + CID(프로파일 이름 포함)** 를 한 파일에 대해 보관할 수 있어야 BMS 생태계와 IPFS를 함께 지원한다.
다만 BagIt은 도구가 SHA-256·SHA-512 지원을 필수로 하고 새 bag에 SHA-512를 권고하며, OCFL은 콘텐츠 주소에 `sha512`·`sha256`만 허용(`sha512` 권장)한다. SHA3-256은 이 필수·허용 집합에 없다. 다른 보존 표준과 맞추려면 SHA-512 보관 여부도 정해야 한다.
이를 위한 구조 변경(예: 파일-다이제스트 연결 테이블, binder CID, 참조 이름)은 아직 결정하지 않았다.

## 3. 메타데이터 스키마(METS)와의 관계

로드맵은 스키마를 METS에서 파생시키고 XML과 인코딩을 UTF-8로 두는 것이다. 위 `file` 릴레이션과 METS의 대응은 다음과 같이 정리된다(제안, 미검증).

| 이 스키마                                         | METS에서 대응하는 곳                                                                      |
| ------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `file.name`, `size`, `format_property`            | `fileSec`의 `file` 요소와 `SIZE`, `MIMETYPE` 속성                                         |
| `checksum_algorithm_identifier`, `checksum_value` | `CHECKSUMTYPE`, `CHECKSUM` (METS 2에서는 SHA3-256 등을 쓸 수 있고 METS 1.12.1에서는 불가) |
| `relative_path`                                   | 파일 위치(`FLocat`) 또는 `structMap`의 구조                                               |
| `binder`, `pattern`                               | `structMap`의 구조 단위                                                                   |
| `persona`, `pattern_credits`                      | `dmdSec`에 담는 기술 메타데이터(MODS 또는 Dublin Core 후보)                               |

자세한 근거는 [기술 채택 판단](crate::docs::adoption)의 메타데이터 스키마 절에 있다.

## 4. 다른 보존·교환 포맷과의 비교

다른 AI의 조사(2026-10-09)가 모은 사실을 재배열한 것이다. 출처 링크와 확인 수준은 [참고 자료](crate::docs::references::preservation) 1절에 있고, 본문을 읽은 것은 BagIt과 OCFL뿐이다.
이 표는 채택 판단이 아니며, 조사가 밝힌 대로 대상들은 층위가 다르다(저장 규약, 참조 모델, 목록, 식별자).

| 대상                      | 메타데이터 직렬화                                                       | 해시 알고리즘                                 | 경로 규칙 요지                                                             |
| ------------------------- | ----------------------------------------------------------------------- | --------------------------------------------- | -------------------------------------------------------------------------- |
| BagIt (RFC 8493)          | 줄 단위 텍스트(`bagit.txt`, `manifest-<algorithm>.txt`, `bag-info.txt`) | 열린 목록. SHA-256·SHA-512 도구 필수          | 상대 경로, `/` 구분, 외부 참조 금지. 대소문자·정규화만 다른 파일 금지 권고 |
| OCFL 1.1                  | JSON(`inventory.json`) + 다이제스트 sidecar                             | 콘텐츠 `sha512`·`sha256`, fixity는 5종과 확장 | `.`·`..` 금지, 대소문자 구별, 링크 금지                                    |
| RO-Crate 1.2/1.3          | JSON-LD                                                                 | 미확인                                        | 미확인                                                                     |
| MAME software list        | XML(DTD 미열람)                                                         | 미확인                                        | 미확인                                                                     |
| No-Intro / Redump DAT     | XML 계열                                                                | `crc`, `md5`, `sha1`(검색 결과의 예)          | 미확인                                                                     |
| SWHID                     | 문자열                                                                  | SHA-1(`sha1_git`) 고정                        | `path` qualifier는 절대 경로, 대소문자 보존                                |
| Frictionless Data Package | JSON(`datapackage.json`)                                                | 선택 `hash`, 기본 MD5, 접두로 다른 알고리즘   | 상대 경로 또는 URL, 혼합 불가                                              |
| Internet Archive item     | XML sidecar + JSON API                                                  | `md5`, `sha1`, `crc32`                        | 미확인                                                                     |

이 표에서 읽을 수 있는 사실:

- BagIt(매니페스트와 태그 매니페스트), OCFL(인벤토리), Data Package(`datapackage.json`)는 묶음 단위 기술 파일에 파일별 다이제스트를 둔다. 이 스키마에서 `binder`에 해시·CID가 없다는 점과 대조된다.
- 로드맵의 "METS 파생 UTF-8 XML"과 달리 RO-Crate, OCFL, Data Package는 JSON이고 BagIt은 텍스트다. XML인 쪽은 MAME 목록, DAT 계열, Internet Archive의 `_meta.xml`이다.
- 경로의 대소문자·정규화는 BagIt(충돌 방지 권고), OCFL(대소문자 구별), SWHID(`path` 대소문자 보존)가 각자 정한다. 이 스키마의 "원문 참조 이름이 남지 않는다" 문제와 같은 종류다.
