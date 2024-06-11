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
______________
Do not spawn more than 1 time for the same display. Tokio ctrl_c doesn't like it and can cause issues when calling the system shutdown.

For whatever reason if you still spawned it more than 1 time, you can use `killall swaytitlers` to kill the process (will kill all processes including the one used in the bar, so the bar will need to be reloaded).
