# swaytitlers
Outputs title of the focused window using swayipc rust crate.

Useful for widgets in the eww bar.
Example of a simple widget:

```
(deflisten windows0 "~/.config/eww/scripts/swaytitlers DP-1")
(defwidget window_w0 []
  (box :halign "center"
    (revealer
      :reveal {windows0 != ""}
      :transition "none"
      (label
        :class "window_w"
        :limit-width 40
        :text "${windows0}"
      )
    )
  )
)
```
