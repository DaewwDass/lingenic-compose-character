# Why This Didn't Exist Before

Font technology has existed for decades. Calligraphy theory has existed for centuries. Why has no one combined them until now?

## Font Formats Solved a Different Problem

OpenType, TrueType, UFO — these formats answer one question: what shape should appear on screen? Bezier outlines are sufficient for that. A curve is a curve. The rendering problem was solved.

No one needed to know *how* a character was written to display it in a PDF.

## Western-Centric Development

Font technology evolved in the West, for Latin scripts. Latin has serifs, stroke contrast, x-heights — but no codified theory of brush dynamics. There is no Western equivalent to:

- 入鋒/收鋒 — how the brush enters and exits
- 提按 — the lift and press of pressure
- 頓提折轉 — the actions at each point
- 筆勢 — the connection between strokes
- 鋒法 — the orientation of the tip

These concepts didn't exist in the minds of the engineers who designed the formats. So the formats had no place for them.

## Two Worlds That Don't Meet

Calligraphers think with brush and ink. They speak in classical terminology passed down through direct instruction: 藏鋒要深, 回鋒要圓, 頓要穩, 提要輕. This knowledge lives in practice, in muscle memory, in the relationship between master and student.

Font engineers think in bezier curves, hinting tables, kerning pairs, OpenType features. They optimize for rendering, file size, cross-platform compatibility.

These two communities rarely overlap. A calligraphy master typically doesn't write software. A font engineer typically hasn't spent years practicing 楷書 under a teacher.

## The Flattening Workflow

Most CJK fonts were created like this:

1. A calligrapher writes characters on paper
2. The paper is scanned
3. Outlines are traced from the scans
4. Engineers refine the curves for rendering

The semantic information — the entry styles, pressure dynamics, brush actions — existed in the calligrapher's mind and hand. It was never captured. The digital font retained only the final shape, a flattened shadow of the living brushwork.

## The Interdisciplinary Barrier

Creating a semantic font format requires:

- Deep knowledge of classical calligraphy theory
- Understanding of multiple CJK scripts and regional variants
- Expertise in format and markup design
- Software engineering capability

Few people have all of these. Calligraphy masters rarely design file formats. Format designers rarely read classical Chinese treatises on brushwork.

## No Obvious Use Case — Until Now

When font formats were designed, the goal was simple: put text on screen, put text on paper. Outlines accomplished this. Why capture more?

But new applications make semantic data valuable:

- **Style transfer** — Apply the dynamics of 楷書 to the structure of 行書
- **Stroke animation** — Animate the brush path with correct pressure and timing
- **Calligraphy education** — Teach not just what to draw, but how to move
- **AI training** — Train models on semantically labeled stroke data
- **Regional variants** — Share stroke structure, vary only the stylistic details

These use cases didn't drive the original font format designs. Now they do.

## The Knowledge Was Always There

The theory of 入鋒, 收鋒, 提按, 筆勢, 鋒法 has been written down for centuries. Treatises on brushwork exist in Classical Chinese, studied by calligraphers across China, Japan, Korea, Vietnam.

The technology to represent structured data has existed for decades. XML, JSON, custom markup languages — none of this is new.

The two just hadn't been combined. The calligraphy masters didn't need file formats. The format designers didn't know calligraphy theory.

## What Changed

Someone who understood both — classical terminology and modern format design — saw that they could be unified. That the knowledge didn't have to be flattened. That a font file could describe not just the shape of a character, but the movement that creates it.

The result is ｢‍｣ Lingenic 字.
