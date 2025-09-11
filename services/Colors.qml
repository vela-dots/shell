pragma Singleton
pragma ComponentBehavior: Bound

import qs.config
import qs.utils
import Vela
// Local singleton to read palette overrides written by the editor helper
import "./"
import Quickshell
import Quickshell.Io
import QtQuick

Singleton {
    id: root

    property bool showPreview
    // Toggle to enable/disable language accent overlays at runtime
    property bool accentsEnabled: true
    property string scheme
    property string flavor
    readonly property bool light: showPreview ? previewLight : currentLight
    property bool currentLight
    property bool previewLight
    readonly property M3Palette palette: showPreview ? preview : current
    readonly property M3TPalette tPalette: M3TPalette {}
    readonly property M3Palette current: M3Palette {}
    readonly property M3Palette preview: M3Palette {}
    readonly property Transparency transparency: Transparency {}
    property real wallLuminance

    function getLuminance(c: color): real {
        if (c.r == 0 && c.g == 0 && c.b == 0)
            return 0;
        return Math.sqrt(0.299 * (c.r ** 2) + 0.587 * (c.g ** 2) + 0.114 * (c.b ** 2));
    }

    function alterColor(c: color, a: real, layer: int): color {
        const luminance = getLuminance(c);

        const offset = (!light || layer == 1 ? 1 : -layer / 2) * (light ? 0.2 : 0.3) * (1 - transparency.base) * (1 + wallLuminance * (light ? (layer == 1 ? 3 : 1) : 2.5));
        const scale = (luminance + offset) / luminance;
        const r = Math.max(0, Math.min(1, c.r * scale));
        const g = Math.max(0, Math.min(1, c.g * scale));
        const b = Math.max(0, Math.min(1, c.b * scale));

        return Qt.rgba(r, g, b, a);
    }

    function layer(c: color, layer: var): color {
        if (!transparency.enabled)
            return c;

        return layer === 0 ? Qt.alpha(c, transparency.base) : alterColor(c, transparency.layers, layer ?? 1);
    }

    function on(c: color): color {
        if (c.hslLightness < 0.5)
            return Qt.hsla(c.hslHue, c.hslSaturation, 0.9, 1);
        return Qt.hsla(c.hslHue, c.hslSaturation, 0.1, 1);
    }

    function load(data: string, isPreview: bool): void {
        const colors = isPreview ? preview : current;
        const scheme = JSON.parse(data);

        if (!isPreview) {
            root.scheme = scheme.name;
            // accept both US and UK spellings for compatibility
            flavor = scheme.flavor ?? scheme.flavour;
            currentLight = scheme.mode === "light";
        } else {
            previewLight = scheme.mode === "light";
        }

        for (const [name, color] of Object.entries(scheme.colors ?? scheme.colours)) {
            const propName = name.startsWith("term") ? name : `m3${name}`;
            if (colors.hasOwnProperty(propName))
            colors[propName] = `#${color}`;
        }
        applyOverrides();
    }

    function applyOverrides(): void {
        if (!accentsEnabled)
            return;
        const overrides = ColorHandler.activeColors ?? {};
        const setIf = (prop, key) => {
            if (overrides[key]) {
                if (current.hasOwnProperty(prop)) current[prop] = `#${overrides[key]}`;
                if (preview.hasOwnProperty(prop)) preview[prop] = `#${overrides[key]}`;
            }
        };
        setIf('m3primary', 'primary');
        setIf('m3secondary', 'secondary');
        setIf('m3tertiary', 'tertiary');
        setIf('m3surfaceTint', 'surfaceTint');
    }

    function resetToScheme(): void {
        // Reload base scheme colors and do not apply overrides
        root.load(schemeFile.text, false);
    }

    function setMode(mode: string): void {
        Quickshell.execDetached(["vela", "scheme", "set", "--notify", "-m", mode]);
    }

    FileView {
        id: schemeFile
        path: `${Paths.state}/scheme.json`
        watchChanges: true
        onFileChanged: reload()
        onLoaded: root.load(text(), false)
    }

    Connections {
        target: ColorHandler
        function onActiveLanguageChanged(): void { applyOverrides(); }
        function onPalettesChanged(): void { applyOverrides(); }
    }

    Connections {
        target: Wallpapers

        function onCurrentChanged(): void {
            const current = Wallpapers.current;
            CUtils.getAverageLuminance(current, l => {
                                           if (Wallpapers.current == current)
                                           root.wallLuminance = l;
                                       });
        }
    }

    component Transparency: QtObject {
        readonly property bool enabled: Appearance.transparency.enabled
        readonly property real base: Appearance.transparency.base - (root.light ? 0.1 : 0)
        readonly property real layers: Appearance.transparency.layers
    }

    component M3TPalette: QtObject {
        readonly property color m3primary_paletteKeyColor: root.layer(root.palette.m3primary_paletteKeyColor)
        readonly property color m3secondary_paletteKeyColor: root.layer(root.palette.m3secondary_paletteKeyColor)
        readonly property color m3tertiary_paletteKeyColor: root.layer(root.palette.m3tertiary_paletteKeyColor)
        readonly property color m3neutral_paletteKeyColor: root.layer(root.palette.m3neutral_paletteKeyColor)
        readonly property color m3neutral_variant_paletteKeyColor: root.layer(root.palette.m3neutral_variant_paletteKeyColor)
        readonly property color m3background: root.layer(root.palette.m3background, 0)
        readonly property color m3onBackground: root.layer(root.palette.m3onBackground)
        readonly property color m3surface: root.layer(root.palette.m3surface, 0)
        readonly property color m3surfaceDim: root.layer(root.palette.m3surfaceDim, 0)
        readonly property color m3surfaceBright: root.layer(root.palette.m3surfaceBright, 0)
        readonly property color m3surfaceContainerLowest: root.layer(root.palette.m3surfaceContainerLowest)
        readonly property color m3surfaceContainerLow: root.layer(root.palette.m3surfaceContainerLow)
        readonly property color m3surfaceContainer: root.layer(root.palette.m3surfaceContainer)
        readonly property color m3surfaceContainerHigh: root.layer(root.palette.m3surfaceContainerHigh)
        readonly property color m3surfaceContainerHighest: root.layer(root.palette.m3surfaceContainerHighest)
        readonly property color m3onSurface: root.layer(root.palette.m3onSurface)
        readonly property color m3surfaceVariant: root.layer(root.palette.m3surfaceVariant, 0)
        readonly property color m3onSurfaceVariant: root.layer(root.palette.m3onSurfaceVariant)
        readonly property color m3inverseSurface: root.layer(root.palette.m3inverseSurface, 0)
        readonly property color m3inverseOnSurface: root.layer(root.palette.m3inverseOnSurface)
        readonly property color m3outline: root.layer(root.palette.m3outline)
        readonly property color m3outlineVariant: root.layer(root.palette.m3outlineVariant)
        readonly property color m3shadow: root.layer(root.palette.m3shadow)
        readonly property color m3scrim: root.layer(root.palette.m3scrim)
        readonly property color m3surfaceTint: root.layer(root.palette.m3surfaceTint)
        readonly property color m3primary: root.layer(root.palette.m3primary)
        readonly property color m3onPrimary: root.layer(root.palette.m3onPrimary)
        readonly property color m3primaryContainer: root.layer(root.palette.m3primaryContainer)
        readonly property color m3onPrimaryContainer: root.layer(root.palette.m3onPrimaryContainer)
        readonly property color m3inversePrimary: root.layer(root.palette.m3inversePrimary)
        readonly property color m3secondary: root.layer(root.palette.m3secondary)
        readonly property color m3onSecondary: root.layer(root.palette.m3onSecondary)
        readonly property color m3secondaryContainer: root.layer(root.palette.m3secondaryContainer)
        readonly property color m3onSecondaryContainer: root.layer(root.palette.m3onSecondaryContainer)
        readonly property color m3tertiary: root.layer(root.palette.m3tertiary)
        readonly property color m3onTertiary: root.layer(root.palette.m3onTertiary)
        readonly property color m3tertiaryContainer: root.layer(root.palette.m3tertiaryContainer)
        readonly property color m3onTertiaryContainer: root.layer(root.palette.m3onTertiaryContainer)
        readonly property color m3error: root.layer(root.palette.m3error)
        readonly property color m3onError: root.layer(root.palette.m3onError)
        readonly property color m3errorContainer: root.layer(root.palette.m3errorContainer)
        readonly property color m3onErrorContainer: root.layer(root.palette.m3onErrorContainer)
        readonly property color m3primaryFixed: root.layer(root.palette.m3primaryFixed)
        readonly property color m3primaryFixedDim: root.layer(root.palette.m3primaryFixedDim)
        readonly property color m3onPrimaryFixed: root.layer(root.palette.m3onPrimaryFixed)
        readonly property color m3onPrimaryFixedVariant: root.layer(root.palette.m3onPrimaryFixedVariant)
        readonly property color m3secondaryFixed: root.layer(root.palette.m3secondaryFixed)
        readonly property color m3secondaryFixedDim: root.layer(root.palette.m3secondaryFixedDim)
        readonly property color m3onSecondaryFixed: root.layer(root.palette.m3onSecondaryFixed)
        readonly property color m3onSecondaryFixedVariant: root.layer(root.palette.m3onSecondaryFixedVariant)
        readonly property color m3tertiaryFixed: root.layer(root.palette.m3tertiaryFixed)
        readonly property color m3tertiaryFixedDim: root.layer(root.palette.m3tertiaryFixedDim)
        readonly property color m3onTertiaryFixed: root.layer(root.palette.m3onTertiaryFixed)
        readonly property color m3onTertiaryFixedVariant: root.layer(root.palette.m3onTertiaryFixedVariant)
    }

    component M3Palette: QtObject {
        property color m3primary_paletteKeyColor: "#ff3e00"
        property color m3secondary_paletteKeyColor: "#312e81"
        property color m3tertiary_paletteKeyColor: "#1f2937"
        property color m3neutral_paletteKeyColor: "#111827"
        property color m3neutral_variant_paletteKeyColor: "#1e1b4b"

        property color m3background: "#0b0f19"
        property color m3onBackground: "#e5e7eb"

        property color m3surface: "#111827"
        property color m3surfaceDim: "#0b0f19"
        property color m3surfaceBright: "#1f2937"

        property color m3surfaceContainerLowest: "#0b0f19"
        property color m3surfaceContainerLow: "#111827"
        property color m3surfaceContainer: "#1f2937"
        property color m3surfaceContainerHigh: "#1f2937"
        property color m3surfaceContainerHighest: "#1f2937"

        property color m3onSurface: "#e5e7eb"
        property color m3surfaceVariant: "#1e1b4b"
        property color m3onSurfaceVariant: "#9ca3af"

        property color m3inverseSurface: "#e5e7eb"
        property color m3inverseOnSurface: "#111827"
        property color m3outline: "#9ca3af"
        property color m3outlineVariant: "#1e1b4b"

        property color m3shadow: "#000000"
        property color m3scrim: "#000000"

        property color m3surfaceTint: "#ff3e00"

        property color m3primary: "#ff3e00"
        property color m3onPrimary: "#0b0f19"
        property color m3primaryContainer: "#312e81"
        property color m3onPrimaryContainer: "#e5e7eb"
        property color m3inversePrimary: "#ff3e00"

        property color m3secondary: "#312e81"
        property color m3onSecondary: "#e5e7eb"
        property color m3secondaryContainer: "#1e1b4b"
        property color m3onSecondaryContainer: "#e5e7eb"

        property color m3tertiary: "#1f2937"
        property color m3onTertiary: "#e5e7eb"
        property color m3tertiaryContainer: "#111827"
        property color m3onTertiaryContainer: "#e5e7eb"

        property color m3error: "#ef4444"
        property color m3onError: "#0b0f19"
        property color m3errorContainer: "#111827"
        property color m3onErrorContainer: "#e5e7eb"

        property color m3primaryFixed: "#ff3e00"
        property color m3primaryFixedDim: "#ff3e00"
        property color m3onPrimaryFixed: "#0b0f19"
        property color m3onPrimaryFixedVariant: "#312e81"

        property color m3secondaryFixed: "#312e81"
        property color m3secondaryFixedDim: "#1e1b4b"
        property color m3onSecondaryFixed: "#e5e7eb"
        property color m3onSecondaryFixedVariant: "#9ca3af"

        property color m3tertiaryFixed: "#1f2937"
        property color m3tertiaryFixedDim: "#111827"
        property color m3onTertiaryFixed: "#e5e7eb"
        property color m3onTertiaryFixedVariant: "#9ca3af"

        property color term0: "#111827"
        property color term1: "#ef4444"
        property color term2: "#22c55e"
        property color term3: "#f59e0b"
        property color term4: "#312e81"
        property color term5: "#ff3e00"
        property color term6: "#9ca3af"
        property color term7: "#e5e7eb"

        property color term8: "#1f2937"
        property color term9: "#ef4444"
        property color term10: "#22c55e"
        property color term11: "#f59e0b"
        property color term12: "#1e1b4b"
        property color term13: "#ff3e00"
        property color term14: "#e5e7eb"
        property color term15: "#ffffff"
    }
}
