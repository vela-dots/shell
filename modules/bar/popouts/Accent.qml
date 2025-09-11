pragma ComponentBehavior: Bound

import qs.components
import qs.components.controls
import qs.config
import Quickshell
import QtQuick
import QtQuick.Layouts

Item {
    id: root

    required property var wrapper

    implicitWidth: layout.implicitWidth + Appearance.padding.normal * 2
    implicitHeight: layout.implicitHeight + Appearance.padding.normal * 2

    ColumnLayout {
        id: layout

        anchors.left: parent.left
        anchors.verticalCenter: parent.verticalCenter
        spacing: Appearance.spacing.small

        RowLayout {
            spacing: Appearance.spacing.small
            StyledText {
                text: qsTr("Language accents")
                font.weight: 500
            }
            StyledSwitch {
                checked: Colors.accentsEnabled
                onToggled: {
                    Colors.accentsEnabled = checked;
                    if (!checked) Colors.resetToScheme(); else Colors.applyOverrides();
                }
            }
        }

        StyledRect {
            Layout.topMargin: Appearance.spacing.normal
            implicitWidth: expandBtn.implicitWidth + Appearance.padding.normal * 2
            implicitHeight: expandBtn.implicitHeight + Appearance.padding.small
            radius: Appearance.rounding.normal
            color: Colors.palette.m3primaryContainer

            StateLayer {
                color: Colors.palette.m3onPrimaryContainer
                function onClicked(): void {
                    root.wrapper.hasCurrent = false;
                    // Prefer Vela Settings (QML) with Colors section
                    if (!Quickshell.execDetached(["app2unit", "--", "vela-settings", "--section", "colors"])) {
                        // Fallback to qmlscene loader if binary not present
                        if (!Quickshell.execDetached(["app2unit", "--", "qmlscene", Qt.resolvedUrl("file://" + Qt.resolvedUrl("~/.vela/settings/qml/main.qml")).slice(7), "--section", "colors"])) {
                        if (Config.general.apps.colors && Config.general.apps.colors.length > 0)
                            Quickshell.execDetached(["app2unit", "--", ...Config.general.apps.colors]);
                        else
                            Quickshell.execDetached(["app2unit", "--", "codium", "--open-url", "vela://settings/colors"]);
                        }
                    }
                }
            }

            RowLayout {
                id: expandBtn
                anchors.centerIn: parent
                spacing: Appearance.spacing.small

                StyledText {
                    Layout.leftMargin: Appearance.padding.smaller
                    text: qsTr("Open color settings")
                    color: Colors.palette.m3onPrimaryContainer
                }

                MaterialIcon {
                    text: "chevron_right"
                    color: Colors.palette.m3onPrimaryContainer
                    font.pointSize: Appearance.font.size.large
                }
            }
        }
    }
}
