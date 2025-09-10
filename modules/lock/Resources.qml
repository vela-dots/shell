import qs.components
import qs.components.controls
import qs.components.misc
import qs.services
import qs.config
import QtQuick
import QtQuick.Layouts

GridLayout {
    id: root

    anchors.left: parent.left
    anchors.right: parent.right
    anchors.margins: Appearance.padding.large

    rowSpacing: Appearance.spacing.large
    columnSpacing: Appearance.spacing.large
    rows: 2
    columns: 2

    Ref {
        service: SystemUsage
    }

    Resource {
        Layout.topMargin: Appearance.padding.large
        icon: "memory"
        value: SystemUsage.cpuPerc
        color: Colors.palette.m3primary
    }

    Resource {
        Layout.topMargin: Appearance.padding.large
        icon: "thermostat"
        value: Math.min(1, SystemUsage.cpuTemp / 90)
        color: Colors.palette.m3secondary
    }

    Resource {
        Layout.bottomMargin: Appearance.padding.large
        icon: "memory_alt"
        value: SystemUsage.memPerc
        color: Colors.palette.m3secondary
    }

    Resource {
        Layout.bottomMargin: Appearance.padding.large
        icon: "hard_disk"
        value: SystemUsage.storagePerc
        color: Colors.palette.m3tertiary
    }

    component Resource: StyledRect {
        id: res

        required property string icon
        required property real value
        required property color color

        Layout.fillWidth: true
        implicitHeight: width

        color: Colors.layer(Colors.palette.m3surfaceContainerHigh, 2)
        radius: Appearance.rounding.large

        CircularProgress {
            id: circ

            anchors.fill: parent
            value: res.value
            padding: Appearance.padding.large * 3
            fgColor: res.color
            bgColor: Colors.layer(Colors.palette.m3surfaceContainerHighest, 3)
            strokeWidth: width < 200 ? Appearance.padding.smaller : Appearance.padding.normal
        }

        MaterialIcon {
            id: icon

            anchors.centerIn: parent
            text: res.icon
            color: res.color
            font.pointSize: (circ.arcRadius * 0.7) || 1
            font.weight: 600
        }

        Behavior on value {
            Anim {
                duration: Appearance.anim.durations.large
            }
        }
    }
}
