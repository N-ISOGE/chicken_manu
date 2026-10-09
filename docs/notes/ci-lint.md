# CI 린트 (super-linter)

PR을 만들거나 갱신하면 CI 린트가 통과하는지 확인하고, 통과하기 전에는 병합하지 않는다.

## 1. 구성

- `.github/workflows/lint.yml`이 super-linter(v8.7.0, 커밋 SHA 고정)로 push와 pull_request마다 코드와 문서를 검사한다.
- 설정은 `.github/linters/`에 있다.
  - `.markdown-lint.yml`: 줄 길이는 400자이고 표는 줄 길이 검사에서 뺀다(표 행은 줄을 나눌 수 없다). MD004, MD029, MD033, MD036은 끈다. MD060(표 열 정렬)은 켜져 있다.
  - `.codespellrc`: `lazer`, `readed`를 허용하고 `.gitignore`와 `LICENSES`는 검사에서 뺀다.
  - `.jscpd.json`: 중복 코드 검사. 임계값 0, `.md`는 제외한다.
- 마크다운 서식 검사(`MARKDOWN_PRETTIER`)는 모든 문서가 맞춰질 때까지 워크플로에서 끈다([프로젝트 노트](crate::docs::notes)의 "마크다운 서식").
- JSON 파일은 biome과 prettier의 서식 검사를 받는다.

## 2. 결과 확인

- `gh pr checks <번호>`로 검사별 결과를 본다.
- 실패하면 `gh run view <실행 번호> --log-failed`로 로그를 열고 `파일:줄 error 규칙` 형태의 줄을 찾는다.
- 실패 원인이 변경인지 환경인지는 로그의 `FATAL` 줄을 먼저 읽어 가른다.

## 3. 실행이 둘인 이유

한 PR에는 검사 실행이 둘 있다.

- push 실행은 그 브랜치만 검사한다.
- pull_request 실행은 대상 브랜치(`dev`)와 합친 상태를 검사한다.

그래서 브랜치는 통과해도 `dev`가 이미 깨져 있으면 pull_request 실행이 실패한다. 이때는 `dev`를 고치는 PR을 먼저 병합하거나, 같은 수정 커밋을 PR 브랜치에도 넣는다. 같은 수정이면 병합할 때 충돌하지 않는다. 브랜치가 `dev`보다 오래됐으면 `dev`를 먼저 병합해 맞춘 뒤 수정을 넣는다.

## 4. 표 열 정렬 (MD060)

MD060(`table-column-style`, `aligned`)은 한글, 한자 같은 전각 문자를 너비 2로 세어, 구분선 줄까지 열마다 같은 너비를 요구한다. 표의 한 칸만 바꿔도 걸린다. 그래서 표를 고치면 열 너비를 다시 맞춘다.

- 이미 prettier로 맞춘 파일은 `npx prettier@3.3.3 --write <파일>`로 표 열도 같이 맞는다. 이 저장소에서 prettier로 맞춘 표는 이 검사를 통과했다.
- 아직 prettier로 맞추지 않은 파일은 prettier가 문서 전체를 바꾸므로, 표만 맞추는 [부록의 스크립트](#부록-표-정렬-스크립트-powershell)를 쓴다.
  - 스크립트가 바꾸려는 파일과 CI가 지적하는 파일이 같지 않을 수 있다. 먼저 `-DryRun`으로 보고, CI가 지적한 파일에만 적용한다.
  - 적용한 뒤 `git diff -w`로 구분선 줄의 대시 개수 말고 다른 내용이 바뀌지 않았는지 본다.

## 5. JSON 서식

biome은 탭 들여쓰기를, prettier는 공백 들여쓰기를 기준으로 삼아, 설정 없이는 여러 줄 JSON이 두 검사를 동시에 통과하지 못한다. 짧은 객체는 한 줄로 두고 80자를 넘기지 않는다.

## 6. 자주 걸리는 실패

| 실패                                       | 원인과 처리                                                                                                                                |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------ |
| MD060                                      | 문구를 일괄 치환하거나 표 칸을 고친 뒤 열 너비를 맞추지 않았다. 4절대로 정렬한다                                                           |
| biome과 prettier가 JSON에서 동시에 실패    | 여러 줄 JSON을 썼다. 한 줄로 되돌린다(5절)                                                                                                 |
| 브랜치는 통과인데 pull_request 실행만 실패 | `dev`가 이미 깨져 있다. 3절대로 `dev`의 수정을 먼저 병합하거나 같은 수정 커밋을 넣는다                                                     |
| TRIVY가 `walk dir error`로 실패            | CI 작업 폴더의 `target/debug/deps` 임시 파일을 읽다 생긴, 변경과 무관한 일시 실패다. 같은 브랜치에 push하거나 실행을 다시 돌리면 통과한다  |
| 먼저 병합된 PR 때문에 같은 줄이 충돌       | 브랜치에 `dev`를 병합하고 충돌을 푼 뒤 push하고, 검사 통과를 확인하고 병합한다. 충돌한 수정이 `dev`에 이미 들어 있으면 `dev`의 판을 택한다 |

같은 줄을 고치는 PR이 여럿이면 먼저 병합된 PR 뒤에 나머지가 충돌한다. 병합 순서를 정할 때 같은 파일을 고치는 PR을 먼저 찾아 둔다.

## 7. 병합 방식과 병합 전 확인

- 이 저장소의 이전 병합과 같은 방식이다: `gh pr merge <번호> --squash --subject "<PR 제목> (#<번호>)" --body ""`. 제목 끝에 PR 번호를 붙이고 본문은 비운다.
- 원격 브랜치는 이 명령으로 지우지 않는다(`--delete-branch`를 쓰지 않는다).
- 병합 전에 `gh pr checks <번호>`의 두 실행(push, pull_request)이 모두 통과했고, `gh pr view <번호> --json mergeable,mergeStateStatus`가 `MERGEABLE`과 `CLEAN`인지 본다.
- 병합 뒤에는 `dev`의 push 실행이 통과하는지 본다.

## 8. push 전에 로컬에서 할 수 있는 확인

- `cargo doc --no-deps`: 문서 링크와 모듈을 본다.
- 부록 스크립트의 `-DryRun`: 표 정렬이 필요한 파일이 있는지 본다.
- 일괄 치환을 썼다면 치환 결과의 diff를 읽는다.
- 변경한 JSON의 한 줄 길이를 본다.

## 부록. 표 정렬 스크립트 (PowerShell)

4절의 MD060 표 열 정렬에 쓴다. 한글, 한자 같은 전각 문자를 너비 2로 세어 열마다 가장 넓은 칸에 맞춰 패딩하고 구분선 줄의 대시 수도 맞춘다. `-DryRun`이면 파일을 쓰지 않고 어느 파일의 표 몇 개가 바뀔지만 보여 준다. 사용: `.\align-tables.ps1 -Paths docs\formats.md -DryRun`

```powershell
param([Parameter(Mandatory)][string[]]$Paths, [switch]$DryRun)
# 마크다운 표를 "aligned" 스타일로 맞춘다: 열마다 가장 넓은 칸에 맞춰 패딩, 한글·한자 등 동아시아 전각 문자는 너비 2.
$utf8 = New-Object System.Text.UTF8Encoding($false)

function Get-Width([string]$s) {
    $w = 0
    $i = 0
    while ($i -lt $s.Length) {
        $cp = [char]::ConvertToUtf32($s, $i)
        $i += if ($cp -gt 0xFFFF) { 2 } else { 1 }
        if (($cp -ge 0x0300 -and $cp -le 0x036F) -or ($cp -ge 0x200B -and $cp -le 0x200F) -or $cp -eq 0xFE0F) { continue }
        if (($cp -ge 0x1100 -and $cp -le 0x115F) -or ($cp -ge 0x2E80 -and $cp -le 0xA4CF) -or
            ($cp -ge 0xAC00 -and $cp -le 0xD7A3) -or ($cp -ge 0xF900 -and $cp -le 0xFAFF) -or
            ($cp -ge 0xFE30 -and $cp -le 0xFE6F) -or ($cp -ge 0xFF00 -and $cp -le 0xFF60) -or
            ($cp -ge 0xFFE0 -and $cp -le 0xFFE6) -or ($cp -ge 0x1F300 -and $cp -le 0x1F64F) -or
            ($cp -ge 0x1F900 -and $cp -le 0x1F9FF) -or ($cp -ge 0x20000 -and $cp -le 0x3FFFD)) { $w += 2 } else { $w += 1 }
    }
    return $w
}

function Split-Row([string]$line) {
    $t = $line.Trim()
    if ($t.StartsWith('|')) { $t = $t.Substring(1) }
    if ($t.EndsWith('|') -and -not $t.EndsWith('\|')) { $t = $t.Substring(0, $t.Length - 1) }
    $cells = New-Object System.Collections.Generic.List[string]
    $sb = New-Object System.Text.StringBuilder
    for ($k = 0; $k -lt $t.Length; $k++) {
        $c = $t[$k]
        if ($c -eq '\' -and $k + 1 -lt $t.Length -and $t[$k + 1] -eq '|') { [void]$sb.Append('\|'); $k++; continue }
        if ($c -eq '|') { $cells.Add($sb.ToString().Trim()); [void]$sb.Clear(); continue }
        [void]$sb.Append($c)
    }
    $cells.Add($sb.ToString().Trim())
    return ,$cells.ToArray()
}

foreach ($p in $Paths) {
    $text = [IO.File]::ReadAllText($p)
    $nl = if ($text.Contains("`r`n")) { "`r`n" } else { "`n" }
    $lines = [regex]::Split($text, "\r?\n")
    $out = New-Object System.Collections.Generic.List[string]
    $changed = 0
    $i = 0
    while ($i -lt $lines.Length) {
        $l = $lines[$i]
        $isRow = $l.TrimStart().StartsWith('|')
        $nextIsSep = ($i + 1 -lt $lines.Length) -and ($lines[$i + 1] -match '^\s*\|[\s:\-|]+\|\s*$')
        if ($isRow -and $nextIsSep) {
            $indent = $l.Substring(0, $l.Length - $l.TrimStart().Length)
            $j = $i
            $block = New-Object System.Collections.Generic.List[string]
            while ($j -lt $lines.Length -and $lines[$j].TrimStart().StartsWith('|')) { $block.Add($lines[$j]); $j++ }
            $rows = @($block | ForEach-Object { , (Split-Row $_) })
            $ncol = ($rows | ForEach-Object { $_.Count } | Measure-Object -Maximum).Maximum
            $widths = @(0) * $ncol
            for ($r = 0; $r -lt $rows.Count; $r++) {
                if ($r -eq 1) { continue }
                for ($c = 0; $c -lt $rows[$r].Count; $c++) { $w = Get-Width $rows[$r][$c]; if ($w -gt $widths[$c]) { $widths[$c] = $w } }
            }
            for ($c = 0; $c -lt $ncol; $c++) { if ($widths[$c] -lt 3) { $widths[$c] = 3 } }
            $newBlock = New-Object System.Collections.Generic.List[string]
            for ($r = 0; $r -lt $rows.Count; $r++) {
                $cells = @()
                for ($c = 0; $c -lt $ncol; $c++) {
                    $val = if ($c -lt $rows[$r].Count) { $rows[$r][$c] } else { '' }
                    if ($r -eq 1) {
                        $left = $val.StartsWith(':'); $right = $val.EndsWith(':')
                        $d = '-' * $widths[$c]
                        if ($left) { $d = ':' + $d.Substring(1) }
                        if ($right) { $d = $d.Substring(0, $d.Length - 1) + ':' }
                        $cells += $d
                    } else {
                        $cells += ($val + (' ' * ($widths[$c] - (Get-Width $val))))
                    }
                }
                $newBlock.Add($indent + '| ' + ($cells -join ' | ') + ' |')
            }
            for ($r = 0; $r -lt $block.Count; $r++) { if ($block[$r] -ne $newBlock[$r]) { $changed++ ; break } }
            foreach ($nb in $newBlock) { $out.Add($nb) }
            $i = $j
            continue
        }
        $out.Add($l)
        $i++
    }
    $new = $out -join $nl
    if ($new -ne $text) {
        if (-not $DryRun) { [IO.File]::WriteAllText($p, $new, $utf8) }
        "{0}: realigned {1} table(s){2}" -f $p, $changed, $(if ($DryRun) { ' (dry run)' } else { '' })
    } else { "{0}: already aligned" -f $p }
}
```
