# ｢‍｣ Lingenic Compose 字

｢‍｣ Lingenic Compose 字 encodes classical calligraphy theory as machine-readable primitives. A `.字` file describes how a brush moves to create each character: the entry and exit of the tip, pressure variations along the stroke, brush actions at each point, and connections between strokes. It captures not just what a character looks like, but how it is written.

**漢文** ｢‍｣ Lingenic 字，以古法書論為機讀元語。字檔記筆之運行：鋒之入出、輕重之變、頓提折轉、筆勢連斷。非徒錄其形，亦述其書法也。

**正體** ｢‍｣ Lingenic 字 將古典書法理論編碼為機器可讀的原語。`.字` 檔案描述毛筆如何運行以書寫每個字：筆鋒的入出、沿筆畫的輕重變化、每一點的筆法動作、筆畫之間的連接。不僅記錄字的外形，更記錄書寫的方式。

**简体** ｢‍｣ Lingenic 字 将古典书法理论编码为机器可读的原语。`.字` 文件描述毛笔如何运行以书写每个字：笔锋的入出、沿笔画的轻重变化、每一点的笔法动作、笔画之间的连接。不仅记录字的外形，更记录书写的方式。

**日本語** ｢‍｣ Lingenic 字 は古典書道理論を機械可読なプリミティブとして符号化する。`.字` ファイルは筆がどのように動いて各文字を書くかを記述する：穂先の入りと出、画に沿った軽重の変化、各点での筆法動作、画の間の連接。字の形だけでなく、書き方をも記す。

**한국어** ｢‍｣ Lingenic 字는 고전 서예 이론을 기계 판독 가능한 원시 요소로 부호화한다. `.字` 파일은 붓이 어떻게 움직여 각 글자를 쓰는지 기술한다: 붓끝의 입봉과 수봉, 획을 따른 경중의 변화, 각 점에서의 필법 동작, 획 사이의 연결. 글자의 모양뿐 아니라 쓰는 방법까지 기록한다.

**Tiếng Việt** ｢‍｣ Lingenic 字 mã hóa lý thuyết thư pháp cổ điển thành các nguyên thủy máy đọc được. Tệp `.字` mô tả cách bút lông di chuyển để viết mỗi chữ: sự nhập và xuất của đầu bút, biến đổi khinh trọng dọc theo nét, động tác bút pháp tại mỗi điểm, sự liên kết giữa các nét. Ghi lại không chỉ hình dạng chữ, mà còn cách viết.

## Dependencies

- [Lingenic Pen-Path](https://github.com/LingenicLLC/lingenic-compose-pen-path) — Rigid pen stroke primitives
- [Lingenic Brush-Path](https://github.com/LingenicLLC/lingenic-compose-brush-path) — Flexible brush stroke primitives

## Multilingual Keywords

The parser accepts English and CJK keywords. All variant forms are accepted:

| Form | Used in | Example |
|------|---------|---------|
| Traditional Chinese | 正體字, 繁體字 | 字體, 筆畫, 轉, 絲 |
| Simplified Chinese | 简体字 | 字体, 笔画, 转, 丝 |
| Japanese Kyūjitai | 旧字体 | 字體, 筆畫, 轉, 絲 |
| Japanese Shinjitai | 新字体 | 字体, 筆画, 転, 糸 |
| Korean Hanja | 漢字/한자 | 字體, 筆畫, 轉, 絲 |
| Vietnamese Hán tự | Chữ Hán | 字體, 筆畫, 轉, 絲 |
| Vietnamese Quốc ngữ | Tiếng Việt | tự-thể, bút-hoạ, chuyển, ti |

### Block Keywords

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `.font-begin` | `.字體始` | `.字体始` | `.字体始` | `.자체시` | `.tự-thể-thủy` | Begin font block |
| `.font-end` | `.字體終` | `.字体终` | `.字体終` | `.자체종` | `.tự-thể-chung` | End font block |
| `.brush-begin` | `.筆始` | `.笔始` | `.筆始` | `.필시` | `.bút-thủy` | Begin brush definition |
| `.brush-end` | `.筆終` | `.笔终` | `.筆終` | `.필종` | `.bút-chung` | End brush definition |
| `.ink-begin` | `.墨始` | `.墨始` | `.墨始` | `.묵시` | `.mặc-thủy` | Begin ink definition |
| `.ink-end` | `.墨終` | `.墨终` | `.墨終` | `.묵종` | `.mặc-chung` | End ink definition |
| `.character-begin` | `.字始` | `.字始` | `.字始` | `.자시` | `.tự-thủy` | Begin character |
| `.character-end` | `.字終` | `.字终` | `.字終` | `.자종` | `.tự-chung` | End character |
| `.stroke-begin` | `.畫始` | `.画始` | `.画始` | `.획시` | `.hoạ-thủy` | Begin stroke |
| `.stroke-end` | `.畫終` | `.画终` | `.画終` | `.획종` | `.hoạ-chung` | End stroke |
| `.pen-stroke` | `.筆劃` | `.笔划` | `.筆画` | `.필획` | `.bút-nét` | Pen stroke (typographic) |

### Properties

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `units-per-em` | `字面單位` | `字面单位` | `字面単位` | `자면단위` | `tự-diện-đơn-vị` | Units per em square |
| `script-type` | `文字` | `文字` | `文字` | `문자` | `văn-tự` | Script type |
| `regional-variant` | `地區` | `地区` | `地区` | `지역` | `địa-khu` | Regional variant |
| `advance-width` | `字寬` | `字宽` | `字幅` | `자폭` | `tự-khoan` | Character advance width |
| `stroke-count` | `畫數` | `画数` | `画数` | `획수` | `hoạ-số` | Stroke count |
| `brush-tip-width` | `鋒寬` | `锋宽` | `穂先幅` | `봉폭` | `phong-khoan` | Brush tip width |
| `brush-belly-width` | `腹寬` | `腹宽` | `腹幅` | `복폭` | `phúc-khoan` | Brush belly width |
| `brush-flexibility` | `彈性` | `弹性` | `弾性` | `탄성` | `đàn-tính` | Brush flexibility |
| `bristle-type` | `毫` | `毫` | `毫` | `호` | `hào` | Bristle type |
| `ink-concentration` | `濃度` | `浓度` | `濃度` | `농도` | `nồng-độ` | Ink concentration |
| `stroke-path` | `徑` | `径` | `径` | `경` | `kính` | Stroke path |

### Entry/Exit Semantics (入鋒/收鋒)

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `entry-style` | `入鋒` | `入锋` | `入鋒` | `입봉` | `nhập-phong` | Entry style |
| `exit-style` | `收鋒` | `收锋` | `収鋒` | `수봉` | `thâu-phong` | Exit style |
| `tip-exposed` | `露鋒` | `露锋` | `露鋒` | `노봉` | `lộ-phong` | Visible tip entry |
| `tip-hidden` | `藏鋒` | `藏锋` | `蔵鋒` | `장봉` | `tàng-phong` | Concealed entry |
| `tip-extend` | `出鋒` | `出锋` | `出鋒` | `출봉` | `xuất-phong` | Tapered extension |
| `tip-return` | `回鋒` | `回锋` | `回鋒` | `회봉` | `hồi-phong` | Return stroke |
| `tip-press-lift` | `頓收` | `顿收` | `頓収` | `돈수` | `độn-thâu` | Press then lift |

### Tip Mode (鋒法)

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `tip-mode` | `鋒法` | `锋法` | `鋒法` | `봉법` | `phong-pháp` | Tip mode |
| `tip-center` | `中鋒` | `中锋` | `中鋒` | `중봉` | `trung-phong` | Center tip |
| `brush-side-left` | `側鋒左` | `侧锋左` | `側鋒左` | `측봉좌` | `trắc-phong-tả` | Side brush left |
| `brush-side-right` | `側鋒右` | `侧锋右` | `側鋒右` | `측봉우` | `trắc-phong-hữu` | Side brush right |

### Brush Actions (提按)

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `brush-press-pause` | `頓` | `顿` | `頓` | `돈` | `độn` | Press and pause |
| `brush-lift` | `提` | `提` | `提` | `제` | `đề` | Lift brush |
| `brush-sharp-turn` | `折` | `折` | `折` | `절` | `chiết` | Sharp angular turn |
| `brush-round-turn` | `轉` | `转` | `転` | `전` | `chuyển` | Smooth pivot |
| `brush-twist` | `挫` | `挫` | `挫` | `좌` | `tỏa` | Twist brush |
| `brush-settle-down` | `蹲` | `蹲` | `蹲` | `준` | `tồn` | Settle down |
| `brush-quick-entry` | `搶` | `抢` | `搶` | `창` | `thương` | Quick entry |

### Pressure (輕重)

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `pressure-value` | `輕重` | `轻重` | `軽重` | `경중` | `khinh-trọng` | Pressure value (0.0–1.0) |

### Stroke Connections (筆勢)

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `stroke-connection` | `筆勢` | `笔势` | `筆勢` | `필세` | `bút-thế` | Connection style |
| `stroke-independent` | `獨立` | `独立` | `独立` | `독립` | `độc-lập` | No connection |
| `stroke-flowing` | `連綿` | `连绵` | `連綿` | `연면` | `liên-miên` | Flowing connection |
| `stroke-thin-link` | `牽絲` | `牵丝` | `牽糸` | `견사` | `khiên-ti` | Visible thin link |
| `stroke-airborne` | `飛渡` | `飞渡` | `飛渡` | `비도` | `phi-độ` | Implied connection |

### Bristle Types (毫)

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `bristle-goat` | `羊毫` | `羊毫` | `羊毛` | `양호` | `dương-hào` | Soft goat hair |
| `bristle-wolf` | `狼毫` | `狼毫` | `狼毛` | `낭호` | `lang-hào` | Stiff wolf hair |
| `bristle-mixed` | `兼毫` | `兼毫` | `兼毛` | `겸호` | `kiêm-hào` | Mixed hair |
| `bristle-rabbit` | `紫毫` | `紫毫` | `紫毛` | `자호` | `tử-hào` | Rabbit hair |

## Key Concepts

### Stroke Types (StrokeKind)

Basic strokes (基本筆畫):

| Kind | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|------|------|------|--------|--------|------|-------------|
| `Heng` | `橫` | `横` | `横` | `횡` | `hoành` | Horizontal |
| `Shu` | `豎` | `竖` | `竪` | `수` | `thụ` | Vertical |
| `Pie` | `撇` | `撇` | `撇` | `별` | `phiết` | Left-falling |
| `Na` | `捺` | `捺` | `捺` | `날` | `nại` | Right-falling |
| `Dian` | `點` | `点` | `点` | `점` | `điểm` | Dot |
| `Ti` | `提` | `提` | `提` | `제` | `đề` | Rising |

Compound strokes (複合筆畫):

| Kind | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|------|------|------|--------|--------|------|-------------|
| `HengZhe` | `橫折` | `横折` | `横折` | `횡절` | `hoành-chiết` | Horizontal-turn |
| `ShuZhe` | `豎折` | `竖折` | `竪折` | `수절` | `thụ-chiết` | Vertical-turn |
| `HengGou` | `橫鉤` | `横钩` | `横鉤` | `횡구` | `hoành-câu` | Horizontal-hook |
| `ShuGou` | `豎鉤` | `竖钩` | `竪鉤` | `수구` | `thụ-câu` | Vertical-hook |
| `HengZheGou` | `橫折鉤` | `横折钩` | `横折鉤` | `횡절구` | `hoành-chiết-câu` | Horizontal-turn-hook |
| `ShuWanGou` | `豎彎鉤` | `竖弯钩` | `竪彎鉤` | `수만구` | `thụ-oan-câu` | Vertical-curve-hook |

### Brush Actions (提按)

| Keyword | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `brush-press-pause` | `頓` | `顿` | `頓` | `돈` | `độn` | Press down and pause |
| `brush-lift` | `提` | `提` | `提` | `제` | `đề` | Lift brush |
| `brush-sharp-turn` | `折` | `折` | `折` | `절` | `chiết` | Sharp angular turn |
| `brush-round-turn` | `轉` | `转` | `転` | `전` | `chuyển` | Smooth pivot |
| `brush-twist` | `挫` | `挫` | `挫` | `좌` | `tỏa` | Twist brush |
| `brush-settle-down` | `蹲` | `蹲` | `蹲` | `준` | `tồn` | Settle brush down |
| `brush-quick-entry` | `搶` | `抢` | `搶` | `창` | `thương` | Quick entry stroke |

Actions appear inline in paths:
```
stroke-path 100,100 ⌒ 200,150 brush-press-pause 300,100 brush-sharp-turn 300,50 brush-lift
```

### Entry Styles (入鋒)

| Style | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|-------|------|------|--------|--------|------|-------------|
| `entry-style tip-exposed` | `露鋒` | `露锋` | `露鋒` | `노봉` | `lộ-phong` | Visible tip entry |
| `entry-style tip-hidden <angle>` | `藏鋒` | `藏锋` | `蔵鋒` | `장봉` | `tàng-phong` | Concealed entry from angle |

### Exit Styles (收鋒)

| Style | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|-------|------|------|--------|--------|------|-------------|
| `exit-style tip-extend` | `出鋒` | `出锋` | `出鋒` | `출봉` | `xuất-phong` | Tapered extension |
| `exit-style tip-return` | `回鋒` | `回锋` | `回鋒` | `회봉` | `hồi-phong` | Return stroke |
| `exit-style tip-press-lift` | `頓收` | `顿收` | `頓収` | `돈수` | `độn-thâu` | Press then lift |

### Stroke Connections (筆勢)

For running script (行書) and grass script (草書):

| Style | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|-------|------|------|--------|--------|------|-------------|
| `stroke-connection stroke-independent` | `獨立` | `独立` | `独立` | `독립` | `độc-lập` | No connection (楷書 default) |
| `stroke-connection stroke-flowing` | `連綿` | `连绵` | `連綿` | `연면` | `liên-miên` | Flowing to next stroke |
| `stroke-connection stroke-thin-link` | `牽絲` | `牵丝` | `牽糸` | `견사` | `khiên-ti` | Visible thin link |
| `stroke-connection stroke-airborne` | `飛渡` | `飞渡` | `飛渡` | `비도` | `phi-độ` | Implied connection |

### Path Connectors

Same as `.lettering` format:

| Symbol | Name | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|--------|------|------|------|--------|--------|------|-------------|
| `⏤` | Line | `直` | `直` | `直` | `직` | `trực` | Straight segment |
| `⌒` | Curve | `曲` | `曲` | `曲` | `곡` | `khúc` | Smooth curve (Hobby's algorithm) |
| `∠` | Corner | `角` | `角` | `角` | `각` | `giác` | Sharp angle break |
| `↻` | Cycle | `環` | `环` | `環` | `환` | `hoàn` | Close path to start |

### Pressure/Dynamics (輕重)

| Keyword | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `pressure-value` | `輕重` | `轻重` | `軽重` | `경중` | `khinh-trọng` | Pressure value (0.0–1.0) |

Inline modifier using classical 輕重 (light-heavy) concept:
```
stroke-path 100,100 pressure-value 0.3 ⌒ 200,150 pressure-value 0.8 ⌒ 300,100 pressure-value 0.5
```

Value range: 0.0 = light/輕/轻/khinh, 1.0 = heavy/重/trọng.

### Tip Mode (鋒法)

| Mode | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|------|------|------|--------|--------|------|-------------|
| `tip-mode tip-center` | `中鋒` | `中锋` | `中鋒` | `중봉` | `trung-phong` | Tip in center of stroke |
| `tip-mode brush-side-left` | `側鋒左` | `侧锋左` | `側鋒左` | `측봉좌` | `trắc-phong-tả` | Tip on left edge |
| `tip-mode brush-side-right` | `側鋒右` | `侧锋右` | `側鋒右` | `측봉우` | `trắc-phong-hữu` | Tip on right edge |

## Regional Variants

```
.character-begin 直 U+76F4
  regional-variant CN
  advance-width 1000
  # ... CN strokes ...
.character-end

.character-begin 直 U+76F4
  regional-variant JP
  advance-width 1000
  # ... JP variant strokes (subtle differences) ...
.character-end
```

## Multiple Scripts

```
.font-begin MixedFont
  script-type Han

  # Han characters use brush strokes
  .character-begin 字 U+5B57
    script-type Han
    # ...
  .character-end

  # Hiragana also uses brush strokes
  .character-begin あ U+3042
    script-type Hiragana
    # ...
  .character-end

  # Hangul may use pen strokes
  .character-begin 한 U+D55C
    script-type Hangul
    .pen-stroke mono 100,100 ⏤ 200,100 ⏤ 200,200
  .character-end

.font-end
```

## Pen Strokes (Typographic Styles)

For Song (宋體) and Hei (黑體) styles, use pen strokes like `.lettering`:

```
.font-begin SongFont
  .circular-pen-begin thin
    diameter 20
  .circular-pen-end

  .elliptical-pen-begin thick
    major-axis 60
    minor-axis 20
    angle 0
  .elliptical-pen-end

  .character-begin 一 U+4E00
    advance-width 1000
    .pen-stroke thick 100,500 ⏤ 900,500
  .character-end

.font-end
```

## Usage

```rust
use character::parse::parse;
use character::render::render_character_to_svg;
use brush_path::Color;

// Parse a .字 file
let source = std::fs::read_to_string("楷體.字")?;
let font = parse(&source)?;

// Render a character to SVG
if let Some(ch) = font.get_character(0x6C38) {  // 永
    let svg = render_character_to_svg(&font, ch, Color::BLACK);
}
```

## Example: Fully Classical

```
.字體始 楷書
  字面單位 1000
  文字 漢
  地區 CN

  .筆始 主
    鋒寬 8
    腹寬 40
    彈性 0.7
    毫 羊毫
  .筆終

  .字始 永 U+6C38
    字寬 1000
    畫數 5

    .畫始 主 點
      入鋒 露鋒
      收鋒 頓收
      鋒法 中鋒
      徑 500,850 輕重 0.4 ⌒ 520,800 輕重 0.7 頓 510,720 輕重 0.3
    .畫終

  .字終
.字體終
```

## Complete Example: 永

The character 永 (eternal) contains all basic stroke types, making it the traditional example for teaching calligraphy:

```
.font-begin YongExample
  units-per-em 1000
  script-type Han
  regional-variant CN

  .brush-begin main
    brush-tip-width 8
    brush-belly-width 40
    brush-flexibility 0.7
    bristle-type bristle-goat
  .brush-end

  .ink-begin black
    ink-concentration 0.85
  .ink-end

  .paper raw-xuan

  .character-begin 永 U+6C38
    advance-width 1000
    stroke-count 5

    # 1. 點 (dot) - top center
    .stroke-begin main Dian
      entry-style tip-exposed
      exit-style tip-press-lift
      tip-mode tip-center
      stroke-path 500,850 pressure-value 0.4 ⌒ 520,800 pressure-value 0.7 brush-press-pause 510,720 pressure-value 0.3
    .stroke-end

    # 2. 橫折鉤 (horizontal-turn-hook) - main frame
    .stroke-begin main HengZheGou
      entry-style tip-hidden 180
      exit-style tip-extend
      tip-mode tip-center
      stroke-path 200,600 pressure-value 0.5 ⌒ 450,620 pressure-value 0.6 brush-press-pause 500,600 pressure-value 0.8 brush-sharp-turn 500,300 pressure-value 0.6 ⌒ 400,150 pressure-value 0.4 brush-lift
    .stroke-end

    # 3. 橫撇 (horizontal-left-falling)
    .stroke-begin main Pie
      entry-style tip-hidden 180
      exit-style tip-extend
      stroke-path 500,500 pressure-value 0.5 ⌒ 400,450 pressure-value 0.5 brush-sharp-turn 200,250 pressure-value 0.2
    .stroke-end

    # 4. 撇 (left-falling) - lower left
    .stroke-begin main Pie
      entry-style tip-exposed
      exit-style tip-extend
      stroke-path 500,400 pressure-value 0.6 ⌒ 300,100 pressure-value 0.2
    .stroke-end

    # 5. 捺 (right-falling) - lower right
    .stroke-begin main Na
      entry-style tip-hidden 135
      exit-style tip-press-lift
      stroke-path 500,400 pressure-value 0.4 ⌒ 700,200 pressure-value 0.7 brush-press-pause 800,100 pressure-value 0.9
    .stroke-end

  .character-end

.font-end
```

## Reusable Components

Components enable defining strokes, radicals, and styles once and reusing them across characters. This follows the [｢‍｣ Lingenic Compose](https://compose.lingenic.com/specification/) component system.

### Stroke Templates

Define a reusable stroke pattern with parameters:

```
.define-stroke heng-standard
  entry-style tip-hidden 180
  exit-style tip-return
  tip-mode tip-center
  stroke-path %start%,%y% pressure-value 0.3 ⌒ %mid%,%y% pressure-value 0.6 ⌒ %end%,%y% pressure-value 0.4
.end-define

.define-stroke dian-standard
  entry-style tip-exposed
  exit-style tip-press-lift
  stroke-path %x%,%y% pressure-value 0.4 ⌒ %x%+20,%y%-50 pressure-value 0.7 brush-press-pause %x%+10,%y%-80 pressure-value 0.3
.end-define
```

Use with `.use-stroke`:

```
.stroke-begin main Heng
  .use-stroke heng-standard start=100 mid=500 end=900 y=600
.stroke-end
```

### Radical Components

Define radicals once, reuse across characters:

```
.define-component shui-radical
  # 氵 — three-dot water radical
  .stroke-begin %brush% Dian
    .use-stroke dian-standard x=%x%+50 y=%y%+200
  .stroke-end
  .stroke-begin %brush% Dian
    .use-stroke dian-standard x=%x%+30 y=%y%+100
  .stroke-end
  .stroke-begin %brush% Ti
    entry-style tip-hidden 225
    exit-style tip-extend
    stroke-path %x%,%y% pressure-value 0.3 ⌒ %x%+80,%y%+50 pressure-value 0.6
  .stroke-end
.end-define

.define-component mu-radical
  # 木 — tree radical
  .stroke-begin %brush% Heng
    .use-stroke heng-standard start=%x% mid=%x%+100 end=%x%+200 y=%y%+150
  .stroke-end
  .stroke-begin %brush% Shu
    stroke-path %x%+100,%y%+200 pressure-value 0.5 ⌒ %x%+100,%y% pressure-value 0.5
  .stroke-end
  .stroke-begin %brush% Pie
    stroke-path %x%+100,%y%+100 pressure-value 0.5 ⌒ %x%,%y% pressure-value 0.2
  .stroke-end
  .stroke-begin %brush% Na
    stroke-path %x%+100,%y%+100 pressure-value 0.4 ⌒ %x%+200,%y% pressure-value 0.7
  .stroke-end
.end-define
```

Use with `.use-component`:

```
.character-begin 河 U+6CB3
  advance-width 1000
  stroke-count 8

  .use-component shui-radical x=50 y=300 brush=main

  # Right side: 可
  .stroke-begin main Heng
    .use-stroke heng-standard start=350 mid=600 end=850 y=750
  .stroke-end
  # ... remaining strokes
.character-end

.character-begin 林 U+6797
  advance-width 1000
  stroke-count 8

  .use-component mu-radical x=50 y=200 brush=main
  .use-component mu-radical x=500 y=200 brush=main
.character-end
```

### Style Definitions

Define consistent brush and ink styles:

```
.define-style kaishu-heavy
  brush-flexibility 0.8
  default-pressure 0.6
  entry-weight 1.2
  exit-weight 0.8
.end-define

.define-style xingshu-flowing
  brush-flexibility 0.9
  default-pressure 0.5
  stroke-connection stroke-flowing
.end-define
```

Apply with `.apply-style`:

```
.stroke-begin main Heng
  .apply-style kaishu-heavy
  stroke-path 100,500 ⌒ 900,500
.stroke-end
```

### Component Libraries

Load external component definitions:

```
.include-components "radicals/water.字-component"
.include-components "radicals/wood.字-component"
.include-components "strokes/kaishu.字-component"
.include-styles "styles/kaishu.字-style"
```

### Component Keywords

| English | 正體 | 简体 | 日本語 | 한국어 | Việt | Description |
|---------|------|------|--------|--------|------|-------------|
| `.define-stroke` | `.定畫` | `.定画` | `.定画` | `.정획` | `.định-hoạ` | Define stroke template |
| `.end-define` | `.定終` | `.定终` | `.定終` | `.정종` | `.định-chung` | End definition |
| `.use-stroke` | `.用畫` | `.用画` | `.用画` | `.용획` | `.dụng-hoạ` | Use stroke template |
| `.define-component` | `.定部` | `.定部` | `.定部` | `.정부` | `.định-bộ` | Define component |
| `.use-component` | `.用部` | `.用部` | `.用部` | `.용부` | `.dụng-bộ` | Use component |
| `.define-style` | `.定式` | `.定式` | `.定式` | `.정식` | `.định-thức` | Define style |
| `.apply-style` | `.用式` | `.用式` | `.用式` | `.용식` | `.dụng-thức` | Apply style |
| `.include-components` | `.入部` | `.入部` | `.入部` | `.입부` | `.nhập-bộ` | Include component file |
| `.include-styles` | `.入式` | `.入式` | `.入式` | `.입식` | `.nhập-thức` | Include style file |

## Modules

- `parse` — Tokenizer and parser
- `font` — CJK font structure and metrics
- `character` — Character with strokes
- `stroke` — Brush and pen strokes
- `brush` — Brush definitions
- `region` — Regional variants
- `script` — Script types
- `render` — SVG and PDF output

## License

Apache-2.0

## Part of [｢‍｣ Lingenic Compose](https://compose.lingenic.com/)

｢‍｣ Lingenic Compose 字 is a component of the [｢‍｣ Lingenic Compose](https://compose.lingenic.com/) typesetting system.
