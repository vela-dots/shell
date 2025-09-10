pragma ComponentBehavior: Bound

import Vela
import Quickshell.Widgets
import QtQuick

IconImage {
    id: root

    required property color color
    property color dominantColor

    asynchronous: true

    layer.enabled: true
    layer.effect: Colouriser {
        sourceColor: root.dominantColor
        colorizationColor: root.color
    }

    layer.onEnabledChanged: {
        if (layer.enabled && status === Image.Ready)
        CUtils.getDominantColor(this, c => dominantColor = c);
    }

    onStatusChanged: {
        if (layer.enabled && status === Image.Ready)
        CUtils.getDominantColor(this, c => dominantColor = c);
    }
}
