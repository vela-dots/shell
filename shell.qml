//@ pragma Env QS_NO_RELOAD_POPUP=1
//@ pragma env QSG_RENDER_LOOP=threaded
//@ Env QT_QUICK_FLICKABLE_WHEEL_DECELERATION=10000

import "modules"
import "modules/drawers"
import "modules/background"
import "modules/areapicker"
import "modules/lock"
import Quickshell

ShellRoot {
    Background {}
    Drawers {}
    AreaPicker {}
    Lock {}

    Shortcuts{}
}