pragma Singleton

import ".."
import qs.config
import qs.utils
import Quickshell
import Quickshell.Io
import QtQuick

Searcher {
    id: root

    property string currentScheme
    property string currentVariant

    function transformSearch(search: string): string {
        return search.slice(`${Config.launcher.actionPrefix}scheme `.length);
    }

    function selector(item: var): string {
        return `${item.name} ${item.flavor}`;
    }

    function reload(): void {
        getCurrent.running = true;
    }

    list: schemes.instances
    useFuzzy: Config.launcher.useFuzzy.schemes
    keys: ["name", "flavor"]
    weights: [0.9, 0.1]

    Variants {
        id: schemes

        Scheme {}
    }

    Process {
        id: getSchemes

        running: true
        command: ["vela", "scheme", "list"]
        stdout: StdioCollector {
            onStreamFinished: {
                const schemeData = JSON.parse(text);
                const list = Object.entries(schemeData).map(([name, f]) => Object.entries(f).map(([flavor, colors]) => ({
                                                                                                                              name,
                                                                                                                              flavor,
                                                                                                                              colors
                                                                                                                          })));

                const flat = [];
                for (const s of list)
                for (const f of s)
                flat.push(f);

                schemes.model = flat.sort((a, b) => (a.name + a.flavor).localeCompare((b.name + b.flavor)));
            }
        }
    }

    Process {
        id: getCurrent

        running: true
        command: ["vela", "scheme", "get", "-nfv"]
        stdout: StdioCollector {
            onStreamFinished: {
                const [name, flavor, variant] = text.trim().split("\n");
                root.currentScheme = `${name} ${flavor}`;
                root.currentVariant = variant;
            }
        }
    }

    component Scheme: QtObject {
        required property var modelData
        readonly property string name: modelData.name
        readonly property string flavor: modelData.flavor
        readonly property var colours: modelData.colors ?? modelData.colours

        function onClicked(list: AppList): void {
            list.visibilities.launcher = false;
            Quickshell.execDetached(["vela", "scheme", "set", "-n", name, "-f", flavor]);
        }
    }
}
