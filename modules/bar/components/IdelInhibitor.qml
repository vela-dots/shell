import qs.components
import qs.services
import qs.config
import Quickshell
import QtQuick

StyledRect {
    id: root

    implicitWidth: implicitHeight
    implicitHeight: icon.implicitHeight + Appearance.padding.small * 2

    radius: Appearance.rounding.full
    color: Qt.alpha(Colors.palette.m3primaryContainer, IdleInhibitor.enabled ? 1 : 0)

    StateLayer {
        function onClicked(): void {
        IdleInhibitor.enabled = !IdleInhibitor.enabled;
    }
    }

        MaterialIcon {
            id: icon

            anchors.centerIn: parent
            anchors.horizontalCenterOffset: -1

            text: "coffee"
            color: IdleInhibitor.enabled ? Colors.palette.m3onPrimaryContainer : Colors.palette.m3secondary
            font.bold: true
            font.pointSize: Appearance.font.size.normal
        }
}
