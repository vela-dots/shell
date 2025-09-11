pragma Singleton

import QtQuick
import Quickshell.Io

Singleton {
    id: root

    property var palettes: ({})
    property string activeLanguage: ""
    readonly property var activeColors: palettes.colors ?? ({})

    function load(json) {
        try {
            const data = JSON.parse(json);
            root.palettes = data;
            root.activeLanguage = data.activeLanguage ?? "";
        } catch (e) {
            root.palettes = ({});
            root.activeLanguage = "";
        }
    }

    FileView {
        path: `${Paths.state}/palette_override.json`
        watchChanges: true
        onLoaded: root.load(text())
        onFileChanged: reload()
    }
}

