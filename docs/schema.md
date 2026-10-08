# 스키마와 식별자

작성해 둔 MariaDB 스키마(`chicken_menu`)와 논리적 데이터 모델 설계서(2024-12-19, 2차)를 요약하고,
해시·고유식별자·IPFS 관점에서 검토한 결과를 적는다. 이 문서는 스키마 변경안을 설계하지 않고 관찰과 결정 사항만 남긴다.

## 1. 릴레이션 요약

| 릴레이션 | 속성 |
| --- | --- |
| `file` | `id`, `name`, `size`, `format_property`, `encoding_property`, `checksum_algorithm_identifier`, `checksum_value`, `ipfs_cid` |
| `binder` | `id`, `name` |
| `pattern` | `id`, `name`, `category`, `pattern_file_id` |
| `persona` | `id`, `name`, `introduction` |
| `binder_attached_pattern` (binder 포함 패턴) | `binder_id`, `attached_pattern_id`, `relative_path` |
| `binder_appendix_file` (binder 부속 파일) | `binder_id`, `appendix_file_id`, `relative_path` |
| `pattern_attached_file` (패턴 관련 파일) | `pattern_id`, `attached_file_id`, `relative_path` |
| `pattern_credits` (패턴 제작 참여) | `pattern_id`, `persona_id`, `role` (기본 `ARTIST`) |
| `persona_affiliation` (명의 소속) | `member_persona_id`, `group_persona_id` |
| `alternate_persona` (명의 별칭) | `main_persona_id`, `minor_persona_id` |

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

| 관찰 | 근거 | 영향 |
| --- | --- | --- |
| 파일당 체크섬이 1종이다 | 컬럼이 하나씩 | BMS 생태계는 채보를 **MD5 또는 SHA-256**(원시 바이트)으로 식별한다(난이도표의 `md5`/`sha256`, beatoraja, jbms-parser). SHA3-256만 있으면 난이도표·랭킹과 매칭할 수 없다 |
| `checksum_value`는 64바이트면 충분하다 | MD5 16, SHA-256·SHA3-256·BLAKE3-256 32, SHA-512 64 | 컬럼 크기를 바꿀 필요가 없다 |
| 설계서 본문은 해시 값에 `VARCHAR(128)`를 고려했다고 적었고, 표와 DDL은 `VARBINARY(64)`다 | 설계서 "검토 및 의도"와 DDL의 차이 | 문자열(16진)과 바이트 중 하나로 정해야 한다 |
| `ipfs_cid`는 `VARBINARY(128)`인데 CID는 문자열로 만들어진다 | `util-for-parsing` 브랜치의 `print_file_row`가 `ipfs add` 출력의 문자열 CID를 CSV로 쓰고 `LOAD DATA`로 적재한다 | 같은 해시도 CIDv0(`Qm…`)와 CIDv1(`bafy…`)의 문자열이 다르다. 저장 형태를 하나로 고정해야 한다(예: 정규화한 CID 바이트) |
| CID는 **생성 프로파일에 종속**된다 | kubo 기본(CIDv0, 256KiB, 링크 174)과 `unixfs-v1-2025`(CIDv1, 1MiB, 링크 1024)는 같은 파일에서 다른 CID를 만든다 | CID 옆에 프로파일 이름이 없으면 재현·비교할 수 없다 |
| 알고리즘 이름이 자유 문자열이다 | `VARCHAR(32)` | 철자를 정해야 한다. IANA 해시 이름 레지스트리는 `md5`, `sha-256`, `sha3-256`을 갖고 `blake3`는 없다 |
| 파일의 정체성은 내용 해시이고 이름이 아니다 | 같은 이름(`kick.wav` 등)이 여러 binder에 나온다 | `(checksum_algorithm_identifier, checksum_value)`를 유일 키로 보는 것이 자연스럽다. 경로를 연결 테이블(`relative_path`)로 분리해 둔 구조는 IPFS의 콘텐츠 주소 + 경로 모델과 잘 맞는다 |
| 패턴의 정체성은 패턴 파일의 해시다 | `pattern.name`은 유일하지 않고 `pattern_file_id`가 `file`을 참조한다 | 난이도표의 `md5`와 `file`의 MD5를 직접 연결할 수 있다 |
| binder에는 해시·CID가 없다 | `binder(id, name)` | 난이도표의 `ipfs` 필드는 **디렉터리**여야 한다. IPFS로 공유하려면 binder 단위 루트 CID가 필요하다 |
| 원문 참조 이름이 남지 않는다 | 연결 테이블은 `file_id`와 `relative_path`만 보관 | `#WAV01 kick.wav`가 실제로 `kick.ogg`로 해석되는 경우(확장자 대체)와 대소문자 차이를 복원할 수 없다. [BMS 문법](crate::docs::bms_grammar)의 `file-ref-command`가 (헤더, 인덱스, 원문 경로)를 내므로 보관할지 정해야 한다 |
| `format_property`·`encoding_property`로는 BMS 텍스트를 구분하기 어렵다 | libmagic이 BMS 내용을 인식하는지는 시험하지 않았다 | 확장자와 문법 적합도(알려진 명령 비율)로 판별하는 방법이 있다 |

정리: 최소한 **MD5 + SHA-256(원시 바이트) + SHA3-256 + CID(프로파일 이름 포함)** 를 한 파일에 대해 보관할 수 있어야 BMS 생태계와 IPFS를 함께 지원한다.
이를 위한 구조 변경(예: 파일-다이제스트 연결 테이블, binder CID, 참조 이름)은 아직 결정하지 않았다.

## 3. 메타데이터 스키마(METS)와의 관계

로드맵은 스키마를 METS에서 파생시키고 XML과 인코딩을 UTF-8로 두는 것이다. 위 `file` 릴레이션과 METS의 대응은 다음과 같이 정리된다(제안, 미검증).

| 이 스키마 | METS에서 대응하는 곳 |
| --- | --- |
| `file.name`, `size`, `format_property` | `fileSec`의 `file` 요소와 `SIZE`, `MIMETYPE` 속성 |
| `checksum_algorithm_identifier`, `checksum_value` | `CHECKSUMTYPE`, `CHECKSUM` (METS 2에서는 SHA3-256 등을 쓸 수 있고 METS 1.12.1에서는 불가) |
| `relative_path` | 파일 위치(`FLocat`) 또는 `structMap`의 구조 |
| `binder`, `pattern` | `structMap`의 구조 단위 |
| `persona`, `pattern_credits` | `dmdSec`에 담는 기술 메타데이터(MODS 또는 Dublin Core 후보) |

자세한 근거는 [기술 채택 판단](crate::docs::adoption)의 메타데이터 스키마 절에 있다.
