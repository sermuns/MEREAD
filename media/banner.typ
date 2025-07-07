#let brown = rgb("#2e261d")
#let beige = rgb("#dad2c5")
#set page(width: 1073pt, height: 151pt, fill: none, margin: 0em)
#set text(fill: beige, font: "MonaspiceKr NFM", size: 100pt)
#set align(center + horizon)

#box(fill: brown, width: 100%, height: 100%, radius: 10%, stack(
  dir: ltr,
  spacing: .5em,
  [MEREAD],
  image(bytes(
    read("markdown-mark-solid.svg").replace(
      "<path",
      "<path fill='" + beige.to-hex() + "'",
    ),
  )),
))
